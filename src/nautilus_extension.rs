use glib_ffi::GType;
use gobject_ffi::{GInterfaceInfo, GObjectClass, GTypeFlags, GTypeInfo, GTypeModule, GTypeQuery, GTypeValueTable};
use gobject_ffi::{g_type_module_add_interface, g_type_module_register_type, g_type_query};
use gobject_ffi::G_TYPE_OBJECT;
use libc::{c_char, c_int};
use nautilus_ffi::{nautilus_column_provider_get_type, nautilus_info_provider_get_type, nautilus_menu_provider_get_type};
use std::ffi::CString;
use std::mem;
use std::ptr;

use column_provider;
use info_provider;
use menu_provider;

static mut module_type: GType = 0;
static mut module_type_list: [GType; 1] = [0];

const VALUE_TABLE: GTypeValueTable = GTypeValueTable {
    value_init: None,
    value_free: None,
    value_copy: None,
    value_peek_pointer: None,
    collect_format: 0 as *const c_char,
    collect_value: None,
    lcopy_format: 0 as *const c_char,
    lcopy_value: None
};

#[repr(C)]
struct TmsuNautilusExtensionClass {
    _parent_slot: GObjectClass
}

#[no_mangle]
pub extern "C" fn nautilus_module_initialize(module: *mut GTypeModule) {
    println!("Initializing TMSU Nautilus Extension");

    register_type(module);
    unsafe {
        module_type_list[0] = module_type;
    }
}

#[no_mangle]
pub extern "C" fn nautilus_module_list_types(types: *mut *const GType, num_types: *mut c_int) {
    unsafe {
        *types = module_type_list.as_ptr();
        *num_types = module_type_list.len() as c_int;
    }
}

#[no_mangle]
pub extern "C" fn nautilus_module_shutdown() {
    println!("Shutting down TMSU Nautilus Extension");
    // Any module-specific shutdown code
}

fn register_type(module: *mut GTypeModule) {
    let name = CString::new("TmsuNautilusExtension").unwrap();

    let info = GTypeInfo {
        class_size: mem::size_of::<TmsuNautilusExtensionClass>() as u16,
        base_init: None,
        base_finalize: None,
        class_init: None,
        class_finalize: None,
        class_data: ptr::null(),
        instance_size: g_object_instance_size(),
        n_preallocs: 0,
        instance_init: None,
        value_table: &VALUE_TABLE
    };

    let column_provider_iface_info = GInterfaceInfo {
        interface_init: Some(column_provider::tmsu_extension_column_provider_iface_init),
        interface_finalize: None,
        interface_data: ptr::null_mut()
    };
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
        module_type = g_type_module_register_type(module, G_TYPE_OBJECT, name.as_ptr(), &info, GTypeFlags::empty());
        g_type_module_add_interface(module, module_type, nautilus_column_provider_get_type(), &column_provider_iface_info);
        g_type_module_add_interface(module, module_type, nautilus_info_provider_get_type(), &info_provider_iface_info);
        g_type_module_add_interface(module, module_type, nautilus_menu_provider_get_type(), &menu_provider_iface_info);
    }
}

fn g_object_instance_size() -> u16 {
    let mut query: GTypeQuery = GTypeQuery {
        instance_size: 0,
        class_size: 0,
        type_name: 0 as *const c_char,
        type_: 0
    };
    unsafe {
        g_type_query(G_TYPE_OBJECT, &mut query);
    }
    return query.instance_size as u16;
}