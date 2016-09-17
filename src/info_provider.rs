use glib_ffi::gpointer;
use gobject_ffi::{GClosure, g_closure_ref};
use nautilus_ffi::{NautilusFileInfo, NautilusInfoProvider, NautilusInfoProviderIface, NautilusOperationHandle, NautilusOperationResult};
use nautilus_ffi::{nautilus_file_info_add_string_attribute, nautilus_file_info_get_uri, nautilus_file_info_get_uri_scheme};
use nautilus_ffi::nautilus_info_provider_update_complete_invoke;
use std::ffi::{CStr, CString};
use std::mem;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use url;

struct MyOperationHandle {
    skip_response: bool
}

#[no_mangle]
pub unsafe extern "C" fn tmsu_extension_info_provider_iface_init(iface: gpointer, _: gpointer) {
    tmsu_extension_info_provider_iface_struct(iface);
}

fn tmsu_extension_info_provider_iface_struct(iface: gpointer) {
    let iface_struct = iface as *mut NautilusInfoProviderIface;
    unsafe {
        (*iface_struct).update_file_info = Some(tmsu_extension_update_file_info);
        (*iface_struct).cancel_update = Some(tmsu_extension_cancel_update);
    }
}

#[no_mangle]
pub extern "C" fn tmsu_extension_update_file_info(provider: *mut NautilusInfoProvider,
                                                  file: *mut NautilusFileInfo,
                                                  update_complete: *mut GClosure,
                                                  handle: *mut *mut NautilusOperationHandle) -> NautilusOperationResult {
    let uri_scheme = get_uri_scheme(file);

    if uri_scheme != "file" {
        return NautilusOperationResult::NAUTILUS_OPERATION_COMPLETE;
    }

    let (tx, rx) = channel();
    let my_handle = Arc::new(MyOperationHandle { skip_response: false });
    Box::new(thread::spawn(move || {
        let (provider, file, update_complete, handle) = rx.recv().unwrap();
        update_file_info(provider, file, update_complete, handle);
    }));
    unsafe {
        let closure_copy = g_closure_ref(update_complete);
        *handle = mem::transmute(Box::into_raw(Box::new(my_handle)));
        tx.send((&mut *provider, &mut *file, &mut *closure_copy, &mut **handle)).unwrap();
    }

    return NautilusOperationResult::NAUTILUS_OPERATION_IN_PROGRESS;
}

#[no_mangle]
pub extern "C" fn tmsu_extension_cancel_update(_provider: *mut NautilusInfoProvider, handle: *mut NautilusOperationHandle) {
    unsafe {
        let ref mut my_handle_arc = *(handle as *mut Arc<MyOperationHandle>);
        let mut my_handle = Arc::get_mut(my_handle_arc).unwrap();
        my_handle.skip_response = true;
    }
}

fn get_uri_scheme(file: *mut NautilusFileInfo) -> String {
    unsafe {
        CStr::from_ptr(nautilus_file_info_get_uri_scheme(file)).to_string_lossy().into_owned()
    }
}

fn get_path(file: *mut NautilusFileInfo) -> String {
    unsafe {
        let uri = CStr::from_ptr(nautilus_file_info_get_uri(file)).to_str().unwrap();
        url::percent_encoding::percent_decode(&uri[7..].as_ref()).decode_utf8_lossy().into_owned()
    }
}

fn update_file_info(provider: &mut NautilusInfoProvider, file: &mut NautilusFileInfo, update_complete: &mut GClosure, handle: &mut NautilusOperationHandle) {
    let path = get_path(file);

    let output = Command::new("tmsu")
                         .arg("tags")
                         .arg(&path)
                         .current_dir(Path::new(&path).parent().unwrap())
                         .output()
                         .expect("could not query tmsu tags");

    let output_string = String::from_utf8(output.stdout.into_iter().collect()).unwrap();
    let output_str = output_string.trim();
    let find_result = output_str.find(": ");

    unsafe {
        let attr_name = CString::new("tmsu_tags").unwrap().into_raw();
        let attr_value =
            match find_result {
                Some(idx) => CString::new(&output_str[idx+2..]).unwrap().into_raw(),
                None => CString::new("").unwrap().into_raw()
            };

        let handle_ptr = handle as *mut NautilusOperationHandle;
        let ref mut my_handle_arc = *(handle_ptr as *mut Arc<MyOperationHandle>);
        let my_handle = Arc::get_mut(my_handle_arc).unwrap();

        if !my_handle.skip_response {
            nautilus_file_info_add_string_attribute(file, attr_name, attr_value);
            nautilus_info_provider_update_complete_invoke(update_complete, provider, handle, NautilusOperationResult::NAUTILUS_OPERATION_COMPLETE);
        }

        // deallocate CStrings
        CString::from_raw(attr_name);
        CString::from_raw(attr_value);
    }
}
