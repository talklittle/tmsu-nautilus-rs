use glib_ffi::GType;
use gobject_ffi::GTypeModule;
use libc::c_int;
use nautilus_extension::NautilusModule;

use column_provider;
use info_provider;
use menu_provider;

nautilus_module!(init);

fn init(module: *mut GTypeModule) -> GType {
    println!("Initializing TMSU Nautilus");

    NautilusModule::new(module, "TmsuNautilusExtension")
        .add_column_provider(column_provider::TmsuColumnProvider {})
        .add_info_provider(info_provider::TmsuInfoProvider {})
        .add_menu_provider(menu_provider::TmsuMenuProvider {})
        .register()
}
