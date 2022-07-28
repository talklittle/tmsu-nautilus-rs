use crate::glib_ffi::GType;
use crate::gobject_ffi::GTypeModule;
use libc::c_int;
use nautilus_extension::NautilusModule;

use crate::column_provider;
use crate::info_provider;
use crate::menu_provider;
use crate::property_page_provider;

nautilus_module!(init);

fn init(module: *mut GTypeModule) -> GType {
    println!("Initializing TMSU Nautilus {}", env!("CARGO_PKG_VERSION"));

    NautilusModule::new(module, "TmsuNautilusExtension")
        .add_column_provider(column_provider::TmsuColumnProvider {})
        .add_info_provider(info_provider::TmsuInfoProvider {})
        .add_menu_provider(menu_provider::TmsuMenuProvider {})
        .add_property_page_provider(property_page_provider::TmsuPropertyPageProvider {})
        .register()
}
