// GTK List Box widget

use gtk;
use gtk::prelude::*;
use nautilus_extension::FileInfo;
use tmsu_commands;
use url;

pub fn new_widget(files: &Vec<FileInfo>) -> gtk::Widget {
    let scrolled_window = gtk::ScrolledWindow::new(None, None);
    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Always);

    let mut list_box = gtk::ListBox::new();
    for file in files {
        add_tag_rows_from_file(&mut list_box, &file);
    }

    scrolled_window.add(&list_box);

    scrolled_window.upcast::<gtk::Widget>()
}

fn add_tag_rows_from_file(list_box: &mut gtk::ListBox, file: &FileInfo) {
    let tags_string =
        match file.attributes.get("tmsu_tags") {
            Some(value) => value.to_string(),
            None => tmsu_commands::tags(&get_path(&file)),  // TODO query the tags now
        };

    if tags_string.len() > 0 {
        for tag in tags_string.split(" ") {
            let row = list_box_row(tag, &file);
            list_box.add(&row);
        }
    }
}

fn list_box_row(tag: &str, file: &FileInfo) -> gtk::Widget {
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 6);

    let tag_and_file_count_vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    hbox.pack_start(&tag_and_file_count_vbox, true, true, 6);

    let tag_label = gtk::Label::new(Some(tag));
    tag_label.set_halign(gtk::Align::Start);
    tag_and_file_count_vbox.pack_start(&tag_label, true, true, 0);

    // TODO small label saying e.g. "2 files" if the tag applies to 2 of the selected files

    let remove_button = gtk::Button::new();
    remove_button.set_label("Remove");
    hbox.pack_end(&remove_button, false, false, 0);

    let file_clone = file.clone();
    remove_button.connect_clicked(move |remove_button| {
        on_clicked_remove_cb(&remove_button, &file_clone);
    });

    hbox.upcast::<gtk::Widget>()
}

fn get_path(file_info: &FileInfo) -> String {
    let uri = file_info.get_uri();
    url::percent_encoding::percent_decode(&uri[7..].as_ref()).decode_utf8_lossy().into_owned()
}

fn on_clicked_remove_cb(button: &gtk::Button, file: &FileInfo) {
    let hbox = button.get_parent().unwrap();
    let list_box_row = hbox.get_parent().unwrap();
    let mut list_box = list_box_row.get_parent().unwrap().downcast::<gtk::ListBox>().unwrap();

    let tag_and_file_count_vbox = hbox.downcast::<gtk::Container>().unwrap().get_children()[0].clone();

    let tag_label = tag_and_file_count_vbox.downcast::<gtk::Container>().unwrap().get_children()[0].clone();
    let tag = tag_label.downcast::<gtk::Label>().unwrap().get_text().unwrap();

    let path = get_path(&file);
    tmsu_commands::untag(&path, &tag);
    file.invalidate_extension_info();

    // remove all and repopulate list

    for row in list_box.get_children() {
        list_box.remove(&row);
    }

    add_tag_rows_from_file(&mut list_box, &file);
    list_box.show_all();
}
