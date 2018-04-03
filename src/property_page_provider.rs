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
        // TODO Edit tags for multiple selected files
        if files.len() != 1 {
            return Vec::new();
        }

        gtk_helpers::init_gtk();

        let label_text = "TMSU tags";
        let label = gtk::Label::new(Some(label_text));

        let list = tags_list::new_widget(files);
        let margin = 10;
        list.set_margin_top(margin);
        list.set_margin_bottom(margin);
        list.set_margin_start(margin);
        list.set_margin_end(margin);

        list.show();

        vec![
            PropertyPage::new("TMSU tags", label.to_glib_none().0, list.to_glib_none().0)
        ]
    }
}
