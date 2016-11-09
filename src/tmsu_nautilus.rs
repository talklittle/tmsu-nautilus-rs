use glib_ffi::GType;
use gobject_ffi::{GInterfaceInfo, GTypeModule};
use gobject_ffi::g_type_module_add_interface;
use libc::c_int;
use nautilus_extension::NautilusModule;
use nautilus_ffi::{nautilus_info_provider_get_type, nautilus_menu_provider_get_type};
use std::ptr;

use column_provider;
use info_provider;
use menu_provider;

nautilus_module!(register_type);

fn register_type(module: *mut GTypeModule) -> GType {
    let module_type =
        NautilusModule::new(module, "TmsuNautilusExtension")
        .add_column_provider(column_provider::TmsuColumnProvider {})
        .register();

    let info_provider_iface_info = GInterfaceInfo {
        interface_init: Some(info_provider::tmsu_extension_info_provider_iface_init),
        interface_finalize: None,
        interface_data: ptr::null_mut()
    };
    let menu_provider_iface_info = GInterfaceInfo {
        interface_init: Some(menu_provider::tmsu_extension_menu_provider_iface_init),
        interface_finalize: None,
        interface_data: ptr::null_mut()
    };

    unsafe {
        g_type_module_add_interface(module, module_type, nautilus_info_provider_get_type(), &info_provider_iface_info);
        g_type_module_add_interface(module, module_type, nautilus_menu_provider_get_type(), &menu_provider_iface_info);
    }

    module_type
}
