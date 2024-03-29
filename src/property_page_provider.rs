use crate::gtk_helpers;
use crate::tags_list;
use glib::translate::*;
use gtk;
use gtk::prelude::*;
use nautilus_extension::{FileInfo, PropertyPage, PropertyPageProvider};

pub struct TmsuPropertyPageProvider {}

impl PropertyPageProvider for TmsuPropertyPageProvider {
    fn get_pages(&self, files: &[FileInfo]) -> Vec<PropertyPage> {
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

        vec![PropertyPage::new(
            "TMSU tags",
            label.upcast::<gtk::Widget>().to_glib_full(),
            list.to_glib_full(),
        )]
    }
}
