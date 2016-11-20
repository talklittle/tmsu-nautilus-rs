// GTK List Box widget

use glib_ffi::{g_list_nth_data, gpointer, GFALSE, GTRUE};
use gobject_ffi::{g_signal_connect_data, GConnectFlags, GObject};
use gtk_ffi::{GtkAlign, GtkBox, GtkButton, GtkContainer, GtkLabel, GtkOrientation, GtkPolicyType, GtkScrolledWindow, GtkWidget};
use gtk_ffi::{gtk_box_new, gtk_box_pack_end, gtk_box_pack_start};
use gtk_ffi::{gtk_button_new, gtk_button_set_label};
use gtk_ffi::{gtk_container_add, gtk_container_foreach, gtk_container_get_children, gtk_container_remove};
use gtk_ffi::{gtk_label_new, gtk_label_get_text};
use gtk_ffi::gtk_list_box_new;
use gtk_ffi::{gtk_scrolled_window_new, gtk_scrolled_window_set_policy};
use gtk_ffi::{gtk_widget_get_parent, gtk_widget_set_halign, gtk_widget_show_all};
use libc::{c_char, c_void};
use nautilus_extension::FileInfo;
use std::ffi::CStr;
use std::mem;
use std::ptr;
use tmsu_commands;
use url;

pub fn new_widget(files: &Vec<FileInfo>) -> *mut GtkWidget {
    let scrolled_window = unsafe { gtk_scrolled_window_new(ptr::null_mut(), ptr::null_mut()) };
    let list_box = unsafe { gtk_list_box_new() };

    for file in files {
        add_tag_rows_from_file(list_box, &file);
    }

    unsafe {
        gtk_container_add(scrolled_window as *mut GtkContainer, list_box);
        gtk_scrolled_window_set_policy(scrolled_window as *mut GtkScrolledWindow, GtkPolicyType::Automatic, GtkPolicyType::Always);
    }

    scrolled_window
}

fn add_tag_rows_from_file(list_box: *mut GtkWidget, file: &FileInfo) {
    let tags_string =
        match file.attributes.get("tmsu_tags") {
            Some(value) => value.to_string(),
            None => tmsu_commands::tags(&get_path(&file)),  // TODO query the tags now
        };

    if tags_string.len() > 0 {
        for tag in tags_string.split(" ") {
            let row = list_box_row(tag, &file);
            unsafe {
                gtk_container_add(list_box as *mut GtkContainer, row);
            }
        }
    }
}

fn list_box_row(tag: &str, file: &FileInfo) -> *mut GtkWidget {
    let tag_c = format!("{}\0", tag);

    unsafe {
        let hbox = gtk_box_new(GtkOrientation::Horizontal, 6);

        let tag_and_file_count_vbox = gtk_box_new(GtkOrientation::Vertical, 0);
        gtk_box_pack_start(hbox as *mut GtkBox, tag_and_file_count_vbox, GTRUE, GTRUE, 6);

        let tag_label = gtk_label_new(tag_c.as_ptr() as *const c_char);
        gtk_widget_set_halign(tag_label, GtkAlign::Start);
        gtk_box_pack_start(tag_and_file_count_vbox as *mut GtkBox, tag_label, GTRUE, GTRUE, 0);

        // TODO small label saying e.g. "2 files" if the tag applies to 2 of the selected files

        let remove_button = gtk_button_new();
        gtk_button_set_label(remove_button as *mut GtkButton, "Remove\0".as_ptr() as *const c_char);
        gtk_box_pack_end(hbox as *mut GtkBox, remove_button, GFALSE, GFALSE, 0);

        g_signal_connect_data(
            remove_button as *mut GObject,
            "clicked\0".as_ptr() as *const c_char,
            Some(mem::transmute(on_clicked_remove_cb as *mut c_void)),
            mem::transmute(Box::into_raw(Box::new(file.clone()))),
            None,
            GConnectFlags::empty()
        );

        hbox
    }
}

fn get_path(file_info: &FileInfo) -> String {
    let uri = file_info.get_uri();
    url::percent_encoding::percent_decode(&uri[7..].as_ref()).decode_utf8_lossy().into_owned()
}

#[no_mangle]
pub unsafe extern "C" fn on_clicked_remove_cb(button: *mut GtkWidget, user_data: gpointer) {
    let file_box:Box<FileInfo> = Box::from_raw(mem::transmute(user_data));
    let file = *file_box;

    let hbox = gtk_widget_get_parent(button);
    let list_box_row = gtk_widget_get_parent(hbox);
    let list_box = gtk_widget_get_parent(list_box_row);

    let tag_and_file_count_vbox = g_list_nth_data(gtk_container_get_children(hbox as *mut GtkContainer), 0);
    let tag_label = g_list_nth_data(gtk_container_get_children(tag_and_file_count_vbox as *mut GtkContainer), 0);
    let tag = CStr::from_ptr(gtk_label_get_text(tag_label as *mut GtkLabel)).to_str().unwrap();

    let path = get_path(&file);
    tmsu_commands::untag(&path, tag);
    file.invalidate_extension_info();

    // remove all and repopulate list
    gtk_container_foreach(list_box as *mut GtkContainer, Some(remove_from_container), list_box as gpointer);
    add_tag_rows_from_file(list_box, &file);
    gtk_widget_show_all(list_box);
}

#[no_mangle]
pub unsafe extern "C" fn remove_from_container(child: *mut GtkWidget, user_data: gpointer) {
    let container = user_data as *mut GtkContainer;
    gtk_container_remove(container, child);
}
