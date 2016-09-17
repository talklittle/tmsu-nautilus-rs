use glib_ffi::{GList, g_list_append, gpointer};
use libc::c_void;
use nautilus_ffi::{NautilusColumnProviderIface, nautilus_column_new};
use std::ffi::CString;
use std::ptr;

#[no_mangle]
pub unsafe extern "C" fn tmsu_extension_column_provider_iface_init(iface: gpointer, _: gpointer) {
    tmsu_extension_column_provider_iface_struct(iface);
}

fn tmsu_extension_column_provider_iface_struct(iface: gpointer) {
    let iface_struct = iface as *mut NautilusColumnProviderIface;
    unsafe {
        (*iface_struct).get_columns = Some(tmsu_extension_get_columns);
    }
}

#[no_mangle]
pub extern "C" fn tmsu_extension_get_columns(_provider: gpointer) -> *mut GList {
    let name = CString::new("TmsuNautilusExtension::tmsu_tags_column").unwrap().into_raw();
    let attribute = CString::new("tmsu_tags").unwrap().into_raw();
    let label = CString::new("TMSU tags").unwrap().into_raw();
    let description = CString::new("List of TMSU tags").unwrap().into_raw();

    unsafe {
        let column = nautilus_column_new(name, attribute, label, description);
        let result = g_list_append(ptr::null_mut(), column as *mut c_void);

        // deallocate CStrings
        CString::from_raw(name);
        CString::from_raw(attribute);
        CString::from_raw(label);
        CString::from_raw(description);

        result
    }
}
