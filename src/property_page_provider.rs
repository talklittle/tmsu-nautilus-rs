use glib::translate::*;
use gtk;
use gtk::prelude::*;
use gtk_helpers;
use nautilus_extension::{FileInfo, PropertyPage, PropertyPageProvider};
use tags_list;

pub struct TmsuPropertyPageProvider {

}

impl PropertyPageProvider for TmsuPropertyPageProvider {
    fn get_pages(&self, files: &Vec<FileInfo>) -> Vec<PropertyPage> {
        gtk_helpers::init_gtk();

        let label_text = "TMSU tags";
        let label = gtk::Label::new(Some(label_text));

        let list = tags_list::new_widget(files);

        list.show_all();

        vec![
            PropertyPage {
                name: "TMSU tags".to_string(),
                raw_label: label.to_glib_none().0,
                raw_page: list.to_glib_none().0,
            }
        ]
    }
}