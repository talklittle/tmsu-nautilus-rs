use glib_ffi::gboolean;
use gtk_ffi::{GtkBox, GtkButton, GtkOrientation};
use gtk_ffi::{gtk_box_new, gtk_box_pack_start};
use gtk_ffi::{gtk_button_new, gtk_button_set_label};
use gtk_ffi::gtk_label_new;
use gtk_ffi::gtk_widget_show;
use nautilus_extension::{FileInfo, PropertyPage, PropertyPageProvider};
use std::ffi::CString;

pub struct TmsuPropertyPageProvider {

}

impl PropertyPageProvider for TmsuPropertyPageProvider {
    fn get_pages(&self, files: &Vec<FileInfo>) -> Vec<PropertyPage> {

        let label_text = "TMSU tags";
        let label_text_c = CString::new(label_text).unwrap().into_raw();
        let label = unsafe { gtk_label_new(label_text_c) };

        let vbox = unsafe { gtk_box_new(GtkOrientation::Vertical, 6) };

        let button_text = CString::new("TODO").unwrap().into_raw();
        let button = unsafe { gtk_button_new() };
        unsafe {
            gtk_button_set_label(button as *mut GtkButton, button_text);
            gtk_box_pack_start(vbox as *mut GtkBox, button, true as gboolean, true as gboolean, 0);

            gtk_widget_show(label);
            gtk_widget_show(vbox);
            gtk_widget_show(button);
        }

        vec![
            PropertyPage {
                name: "TMSU tags".to_string(),
                raw_label: label,
                raw_page: vbox,
            }
        ]
    }
}