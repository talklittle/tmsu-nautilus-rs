use gtk_ffi::gtk_label_new;
use gtk_ffi::gtk_widget_show_all;
use nautilus_extension::{FileInfo, PropertyPage, PropertyPageProvider};
use std::ffi::CString;
use tags_list;

pub struct TmsuPropertyPageProvider {

}

impl PropertyPageProvider for TmsuPropertyPageProvider {
    fn get_pages(&self, files: &Vec<FileInfo>) -> Vec<PropertyPage> {

        let label_text = "TMSU tags";
        let label_text_c = CString::new(label_text).unwrap().into_raw();
        let label = unsafe { gtk_label_new(label_text_c) };

        let list = tags_list::list_box_new(files);

        unsafe {
            gtk_widget_show_all(list);
        }

        vec![
            PropertyPage {
                name: "TMSU tags".to_string(),
                raw_label: label,
                raw_page: list,
            }
        ]
    }
}