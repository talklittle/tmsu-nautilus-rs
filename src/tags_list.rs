// GTK List Box widget

use crate::tmsu_commands;
use gtk;
use gtk::builders::ScrolledWindowBuilder;
use gtk::prelude::*;
use nautilus_extension::FileInfo;
use percent_encoding;

pub fn new_widget(files: &[FileInfo]) -> gtk::Widget {
    let frame = gtk::Frame::new(None);

    let scrolled_window = ScrolledWindowBuilder::new().build();
    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Always);
    frame.add(&scrolled_window);

    let mut list_box = gtk::ListBox::new();
    for file in files {
        add_tag_rows_from_file(&mut list_box, file);
    }

    list_box.connect_row_activated(move |list_box, list_box_row| {
        on_row_activated(list_box, list_box_row);
    });
    list_box.show();

    scrolled_window.add(&list_box);
    scrolled_window.show();

    frame.show();

    frame.upcast::<gtk::Widget>()
}

fn add_tag_rows_from_file(list_box: &mut gtk::ListBox, file: &FileInfo) {
    let tags = match file.attributes.get("tmsu_tags") {
        Some(value) => value.split_whitespace().map(|s| s.to_owned()).collect(),
        None => tmsu_commands::tags(&get_path(file)),
    };

    for tag in tags {
        let row = list_box_row(&tag, file);
        list_box.add(&row);
    }
}

fn list_box_row(tag: &str, file: &FileInfo) -> gtk::ListBoxRow {
    let list_box_row = gtk::ListBoxRow::new();

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    list_box_row.add(&hbox);

    let tag_and_file_count_vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    hbox.pack_start(&tag_and_file_count_vbox, true, true, 6);

    let tag_label = gtk::Label::new(Some(tag));
    tag_label.set_halign(gtk::Align::Start);
    tag_and_file_count_vbox.pack_start(&tag_label, true, true, 0);

    let entry = gtk::Entry::new();
    tag_and_file_count_vbox.pack_start(&entry, true, true, 0);

    let tag_label_clone = tag_label.clone();
    entry.connect_focus_out_event(move |entry, _| {
        // user clicked away; ignore changes
        entry.hide();
        tag_label_clone.show();
        Inhibit(false)
    });

    let tag_label_clone = tag_label;
    let file_clone = file.clone();
    entry.connect_activate(move |entry| {
        // untag old and tag new

        let old_tag = tag_label_clone.text();
        let new_tags: Vec<String> = entry
            .text()
            .as_str()
            .split_whitespace()
            .map(String::from)
            .collect();

        let path = get_path(&file_clone);
        tmsu_commands::untag(&path, old_tag.as_str());
        tmsu_commands::add_tags(&[path], &new_tags);
        file_clone.invalidate_extension_info();

        tag_label_clone.set_text(entry.text().as_str());

        entry.hide();
        tag_label_clone.show();
    });

    let remove_button = gtk::Button::new();
    remove_button.set_label("Remove");
    hbox.pack_end(&remove_button, false, false, 0);

    let file_clone = file.clone();
    remove_button.connect_clicked(move |remove_button| {
        on_clicked_remove_cb(remove_button, &file_clone);
    });

    list_box_row.show_all();
    entry.hide();

    list_box_row
}

fn get_path(file_info: &FileInfo) -> String {
    let uri = file_info.get_uri();
    percent_encoding::percent_decode(uri[7..].as_ref())
        .decode_utf8_lossy()
        .into_owned()
}

fn on_clicked_remove_cb(button: &gtk::Button, file: &FileInfo) {
    let hbox = button.parent().unwrap();
    let list_box_row = hbox.parent().unwrap();
    let mut list_box = list_box_row
        .parent()
        .unwrap()
        .downcast::<gtk::ListBox>()
        .unwrap();

    let tag_and_file_count_vbox = hbox.downcast::<gtk::Container>().unwrap().children()[0].clone();

    let tag_label = tag_and_file_count_vbox
        .downcast::<gtk::Container>()
        .unwrap()
        .children()[0]
        .clone();
    let tag = tag_label.downcast::<gtk::Label>().unwrap().text();

    let path = get_path(file);
    tmsu_commands::untag(&path, tag.as_str());
    file.invalidate_extension_info();

    // remove all and repopulate list

    for row in list_box.children() {
        list_box.remove(&row);
    }

    add_tag_rows_from_file(&mut list_box, file);
    list_box.show();
}

fn on_row_activated(_list_box: &gtk::ListBox, list_box_row: &gtk::ListBoxRow) {
    let hbox = list_box_row.children()[0]
        .clone()
        .downcast::<gtk::Box>()
        .unwrap();
    let tag_and_file_count_vbox = hbox.children()[0].clone().downcast::<gtk::Box>().unwrap();

    let tag_label = tag_and_file_count_vbox.children()[0]
        .clone()
        .downcast::<gtk::Label>()
        .unwrap();
    let entry = tag_and_file_count_vbox.children()[1]
        .clone()
        .downcast::<gtk::Entry>()
        .unwrap();

    tag_label.hide();
    entry.show();

    entry.set_text(tag_label.text().as_str());
    entry.grab_focus();
}
