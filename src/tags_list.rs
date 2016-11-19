// GTK List Box widget

use glib_ffi::{GFALSE, GTRUE};
use gtk_ffi::{GtkAlign, GtkBox, GtkButton, GtkContainer, GtkOrientation, GtkWidget};
use gtk_ffi::{gtk_box_new, gtk_box_pack_end, gtk_box_pack_start};
use gtk_ffi::{gtk_button_new, gtk_button_set_label};
use gtk_ffi::gtk_container_add;
use gtk_ffi::gtk_label_new;
use gtk_ffi::gtk_list_box_new;
use gtk_ffi::gtk_widget_set_halign;
use libc::c_char;
use nautilus_extension::FileInfo;
use tmsu_commands;
use url;

pub fn list_box_new(files: &Vec<FileInfo>) -> *mut GtkWidget {
    let list_box = unsafe { gtk_list_box_new() };

    for file in files {
        let tags_string =
            match file.attributes.get("tmsu_tags") {
                Some(value) => value.to_string(),
                None => tmsu_commands::tags(&get_path(&file)),  // TODO query the tags now
            };

        for tag in tags_string.split(" ") {
            println!("tag is: {}", tag);
            let row = list_box_row(tag, &file);
            unsafe {
                gtk_container_add(list_box as *mut GtkContainer, row);
            }
        }
    }

    list_box
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

        hbox
    }
}

fn get_path(file_info: &FileInfo) -> String {
    let uri = file_info.get_uri();
    url::percent_encoding::percent_decode(&uri[7..].as_ref()).decode_utf8_lossy().into_owned()
}
