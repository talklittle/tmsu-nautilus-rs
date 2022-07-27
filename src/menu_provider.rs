use gdk;
use glib_ffi::gpointer;
use gobject_ffi::GObject;
use gtk;
use gtk::prelude::*;
use gtk_ffi::GtkWidget;
use gtk_helpers;
use nautilus_extension::{FileInfo, Menu, MenuItem, MenuProvider};
use percent_encoding;
use tags_list;
use tmsu_commands;

pub struct TmsuMenuProvider {

}

impl MenuProvider for TmsuMenuProvider {
    fn get_file_items(&self, _window: *mut GtkWidget, files: &[FileInfo]) -> Vec<MenuItem> {
        let mut top_menuitem = MenuItem::new(
            "TmsuNautilusExtension::TMSU", "TMSU", "TMSU tags", None
        );

        let mut sub_items: Vec<MenuItem> = Vec::new();

        let mut add_tag_menuitem = MenuItem::new(
            "TmsuNautilusExtension::Add_Tag", "Add tags\u{2026}", "Add tags\u{2026}", None
        );
        add_tag_menuitem.set_activate_cb(add_tag_activate_cb);
        sub_items.push(add_tag_menuitem);

        // TODO Edit multiple selected files
        if files.len() == 1 {
            let mut edit_tags_menuitem = MenuItem::new(
                "TmsuNautilusExtension::Edit_Tags", "Edit tags\u{2026}", "Edit tags\u{2026}", None
            );
            edit_tags_menuitem.set_activate_cb(edit_tags_activate_cb);
            sub_items.push(edit_tags_menuitem);
        }

        let submenu = Menu::new(&sub_items);

        top_menuitem.set_submenu(&submenu);

        vec![top_menuitem]
    }
}

nautilus_menu_item_activate_cb!(add_tag_activate_cb, show_add_tag_window);
nautilus_menu_item_activate_cb!(edit_tags_activate_cb, show_edit_tags_window);

fn show_add_tag_window(files: Vec<FileInfo>) {
    gtk_helpers::init_gtk();

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("TMSU");
    window.set_size_request(200, 10);
    window.set_border_width(10);
    window.set_type_hint(gdk::WindowTypeHint::Dialog);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 6);
    window.add(&vbox);

    let files_count = files.len();
    let prompt_text = format!("Add (space-separated) tags to {} file{}", files_count, if files_count == 1 { "" } else { "s" });
    let prompt_label = gtk::Label::new(Some(&*prompt_text));
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

    let files_clone = files.clone();
    let window_clone = window.clone();
    button.connect_clicked(move |_| {
        add_tags(&entry, &files_clone, &window_clone);
    });

    window.connect_delete_event(move |_, _| {
        invalidate_file_infos(&files);
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}

fn add_tags(entry: &gtk::Entry, file_infos: &[FileInfo], window: &gtk::Window) {
    let entry_text = entry.text();
    let filenames = filenames(file_infos);

    let tags: Vec<String> = entry_text.as_str().split_whitespace().map(String::from).collect();
    tmsu_commands::add_tags(&filenames, &tags);

    window.close();
}

fn filenames(files: &[FileInfo]) -> Vec<String> {
    let mut filenames = Vec::new();
    for file_info in files {
        let uri_scheme = file_info.get_uri_scheme();
        if uri_scheme != "file" {
            continue;
        }

        let uri = file_info.get_uri();
        let path = percent_encoding::percent_decode(uri[7..].as_ref()).decode_utf8_lossy().into_owned();
        filenames.push(path);
    }
    filenames
}

fn invalidate_file_infos(files: &[FileInfo]) {
    for file_info in files {
        file_info.invalidate_extension_info();
    }
}

fn show_edit_tags_window(files: Vec<FileInfo>) {
    gtk_helpers::init_gtk();

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("TMSU");
    window.set_size_request(450, 500);
    window.set_border_width(10);
    window.set_type_hint(gdk::WindowTypeHint::Dialog);

    let tags_list = tags_list::new_widget(&files);

    window.add(&tags_list);

    window.connect_delete_event(move |_, _| {
        invalidate_file_infos(&files);
        gtk::main_quit();
        Inhibit(false)
    });

    window.show();
    gtk::main();
}
