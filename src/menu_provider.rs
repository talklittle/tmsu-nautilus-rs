use gdk;
use glib_ffi::gpointer;
use gobject_ffi::GObject;
use gtk;
use gtk::prelude::*;
use gtk_ffi::GtkWidget;
use nautilus_extension::{FileInfo, Menu, MenuItem, MenuProvider};
use std::path::Path;
use std::process::Command;
use url;

pub struct TmsuMenuProvider {

}

impl MenuProvider for TmsuMenuProvider {
    fn get_file_items<'a>(&self, _window: *mut GtkWidget, _files: &Vec<FileInfo>) -> Vec<MenuItem> {
        let mut top_menuitem = MenuItem::new(
            "TmsuNautilusExtension::TMSU".to_string(), "TMSU".to_string(), "TMSU tags".to_string(), None
        );

        let mut add_tag_menuitem = MenuItem::new(
            "TmsuNautilusExtension::Add_Tag".to_string(), "Add tags\u{2026}".to_string(), "Add tags\u{2026}".to_string(), None
        );
        add_tag_menuitem.set_activate_cb(add_tag_activate_cb);

        let submenu = Menu::new(vec![add_tag_menuitem]);

        top_menuitem.set_submenu(submenu);

        vec![top_menuitem]
    }
}

nautilus_menu_item_activate_cb!(add_tag_activate_cb, show_add_tag_window);

fn show_add_tag_window(files: Vec<FileInfo>) {
    gtk::init().unwrap();

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("TMSU");
    window.set_size_request(200, 10);
    window.set_border_width(10);
    window.set_type_hint(gdk::WindowTypeHint::Dialog);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 6);
    window.add(&vbox);

    let files_count = files.len();
    let prompt_text = format!("Add (space-separated) tags to {} file{}", files_count, if files_count == 1 { "" } else { "s" });
    let prompt_label = gtk::Label::new(Some(&prompt_text));
    vbox.pack_start(&prompt_label, true, true, 0);

    let entry = gtk::Entry::new();
    vbox.pack_start(&entry, true, true, 0);

    let button = gtk::Button::new();
    button.set_label("Add");
    vbox.pack_start(&button, true, true, 0);

    let files_clone = files.clone();
    let window_clone = window.clone();
    entry.connect_activate(move |entry_ref| {
        add_tags(entry_ref, &files_clone, &window_clone);
    });

    let entry_clone = entry.clone();
    let files_clone = files.clone();
    let window_clone = window.clone();
    button.connect_clicked(move |_| {
        add_tags(&entry_clone, &files_clone, &window_clone);
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}

fn add_tags(entry: &gtk::Entry, file_infos: &Vec<FileInfo>, window: &gtk::Window) {
    let entry_text = entry.get_text().unwrap();
    let filenames = filenames(file_infos);

    for tag in entry_text.split_whitespace() {
        Command::new("tmsu")
                .arg("tag")
                .arg(format!("--tags=\"{}\"", tag))
                .args(&filenames)
                .current_dir(Path::new(&filenames[0]).parent().unwrap())
                .output()
                .expect("failed to tag files");
    }

    invalidate_file_infos(file_infos);

    window.close();
}

fn filenames(files: &Vec<FileInfo>) -> Vec<String> {
    let mut filenames = vec![];
    let length = files.len();
    for i in 0..length {
        let ref file_info = files[i];
        let uri_scheme = file_info.get_uri_scheme();
        if uri_scheme != "file" {
            continue;
        }

        let uri = file_info.get_uri();
        let path = url::percent_encoding::percent_decode(&uri[7..].as_ref()).decode_utf8_lossy().into_owned();
        filenames.push(path);
    }
    filenames
}

fn invalidate_file_infos(files: &Vec<FileInfo>) {
    let length = files.len();
    for i in 0..length {
        let ref file_info = files[i];
        file_info.invalidate_extension_info();
    }
}
