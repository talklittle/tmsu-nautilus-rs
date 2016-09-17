use gdk_ffi::{GdkEvent, GdkWindowTypeHint};
use glib_ffi::{GList, g_list_append, g_list_length, g_list_nth_data, gboolean, gpointer};
use gobject_ffi::{GConnectFlags, GObject, g_signal_connect_data};
use gtk_ffi::{GtkBox, GtkButton, GtkContainer, GtkEntry, GtkOrientation, GtkWidget, GtkWindow, GtkWindowType};
use gtk_ffi::{gtk_init, gtk_main, gtk_main_quit};
use gtk_ffi::{gtk_box_new, gtk_box_pack_start};
use gtk_ffi::{gtk_button_new, gtk_button_set_label};
use gtk_ffi::{gtk_container_add, gtk_container_set_border_width};
use gtk_ffi::{gtk_entry_get_text, gtk_entry_new};
use gtk_ffi::gtk_label_new;
use gtk_ffi::{gtk_widget_set_size_request, gtk_widget_show_all};
use gtk_ffi::{gtk_window_close, gtk_window_new, gtk_window_set_title, gtk_window_set_type_hint};
use libc::{c_char, c_void};
use nautilus_ffi::{NautilusFileInfo, NautilusMenuProviderIface};
use nautilus_ffi::{nautilus_file_info_get_uri, nautilus_file_info_get_uri_scheme, nautilus_file_info_invalidate_extension_info, nautilus_file_info_list_copy};
use nautilus_ffi::{nautilus_menu_new, nautilus_menu_append_item, nautilus_menu_item_new, nautilus_menu_item_set_submenu};
use std::ffi::{CStr, CString};
use std::mem;
use std::path::Path;
use std::process::Command;
use std::ptr;
use url;

#[no_mangle]
pub unsafe extern "C" fn tmsu_extension_menu_provider_iface_init(iface: gpointer, _: gpointer) {
    tmsu_extension_menu_provider_iface_struct(iface);
}

fn tmsu_extension_menu_provider_iface_struct(iface: gpointer) {
    let iface_struct = iface as *mut NautilusMenuProviderIface;
    unsafe {
        (*iface_struct).get_file_items = Some(tmsu_extension_get_file_items);
    }
}

#[no_mangle]
pub extern "C" fn tmsu_extension_get_file_items(_provider: *mut c_void, _window: *mut GtkWidget, files: *mut GList) -> *mut GList {
    if files.is_null() {
        return ptr::null_mut() as *mut GList;
    }

    let name = CString::new("TmsuNautilusExtension::TMSU").unwrap().into_raw();
    let label = CString::new("TMSU").unwrap().into_raw();
    let tip = CString::new("TMSU tags").unwrap().into_raw();
    let icon = ptr::null();

    let add_tag_name = CString::new("TmsuNautilusExtension::Add_Tag").unwrap().into_raw();
    let add_tag_label = CString::new("Add tags\u{2026}").unwrap().into_raw();
    let add_tag_tip = CString::new("Add tags\u{2026}").unwrap().into_raw();
    let add_tag_icon = ptr::null();

    let activate_name = CString::new("activate").unwrap().into_raw();

    unsafe {
        let top_menuitem = nautilus_menu_item_new(name, label, tip, icon);
        let submenu = nautilus_menu_new();
        nautilus_menu_item_set_submenu(top_menuitem, submenu);

        let add_tag_menuitem = nautilus_menu_item_new(add_tag_name, add_tag_label, add_tag_tip, add_tag_icon);
        g_signal_connect_data(
            add_tag_menuitem as *mut GObject,
            activate_name,
            Some(mem::transmute(add_tag_activate_cb as *mut c_void)),
            nautilus_file_info_list_copy(files) as *mut c_void,
            None,
            GConnectFlags::empty()
        );
        nautilus_menu_append_item(submenu, add_tag_menuitem);

        let mut items: *mut GList = ptr::null_mut();
        items = g_list_append(items, top_menuitem as *mut c_void);

        // deallocate CStrings
        CString::from_raw(name);
        CString::from_raw(label);
        CString::from_raw(tip);
        CString::from_raw(add_tag_name);
        CString::from_raw(add_tag_label);
        CString::from_raw(add_tag_tip);
        CString::from_raw(activate_name);

        return items;
    }
}

#[no_mangle]
pub unsafe extern "C" fn add_tag_activate_cb(_nautilusmenuitem: *mut GObject, user_data: gpointer) {
    show_add_tag_window(user_data);
}

#[no_mangle]
pub unsafe extern "C" fn on_delete_window_cb(_window: *mut GtkWidget, _event: *mut GdkEvent, user_data: gpointer) -> gboolean {
    destroy_window_data(user_data);

    // send "destroy" signal
    false as gboolean
}

#[no_mangle]
pub unsafe extern "C" fn on_destroy_window_cb(_window: *mut GtkWidget, _user_data: gpointer) {
    gtk_main_quit();
}

#[no_mangle]
pub unsafe extern "C" fn on_entry_activated_cb(_entry: *mut GtkWidget, user_data: gpointer) {
    add_tags(user_data);
}

#[no_mangle]
pub unsafe extern "C" fn on_button_clicked_cb(_button: *mut GtkWidget, user_data: gpointer) {
    add_tags(user_data);
}

#[repr(C)]
struct AddTagsWindowData {
    window: *mut GtkWidget,
    entry: *mut GtkEntry,
    files: *mut GList,
    raw_c_strings: Vec<*mut c_char>
}

fn init_gtk() {
    let mut argc = 0;
    unsafe {
        gtk_init(&mut argc, ptr::null_mut());
    }
}

fn show_add_tag_window(user_data: gpointer) {
    let button_text = CString::new("Add").unwrap().into_raw();
    let activate_name = CString::new("activate").unwrap().into_raw();
    let clicked_name = CString::new("clicked").unwrap().into_raw();
    let delete_event_name = CString::new("delete-event").unwrap().into_raw();
    let destroy_name = CString::new("destroy").unwrap().into_raw();
    let title = CString::new("TMSU").unwrap().into_raw();

    let files = user_data as *mut GList;

    // required before using Gtk
    init_gtk();

    unsafe {
        // create window

        let window = gtk_window_new(GtkWindowType::Toplevel);
        gtk_window_set_title(window as *mut GtkWindow, title);
        gtk_widget_set_size_request(window, 200, 100);
        gtk_container_set_border_width(window as *mut GtkContainer, 10);
        gtk_window_set_type_hint(window as *mut GtkWindow, GdkWindowTypeHint::Dialog);

        // add Gtk widgets

        let vbox = gtk_box_new(GtkOrientation::Vertical, 6);
        gtk_container_add(window as *mut GtkContainer, vbox);

        let files_count = g_list_length(files);
        let prompt_text = format!("Add (space-separated) tags to {} file{}", files_count, if files_count == 1 { "" } else { "s" });
        let prompt_text_c = CString::new(prompt_text).unwrap().into_raw();

        let prompt_label = gtk_label_new(prompt_text_c);
        gtk_box_pack_start(vbox as *mut GtkBox, prompt_label, true as gboolean, true as gboolean, 0);

        let entry = gtk_entry_new();
        gtk_box_pack_start(vbox as *mut GtkBox, entry, true as gboolean, true as gboolean, 0);

        let button = gtk_button_new();
        gtk_button_set_label(button as *mut GtkButton, button_text);
        gtk_box_pack_start(vbox as *mut GtkBox, button, true as gboolean, true as gboolean, 0);

        // keep track of CStrings to deallocate

        let raw_c_strings = vec![
            activate_name,
            button_text,
            clicked_name,
            delete_event_name,
            destroy_name,
            prompt_text_c,
            title
        ];

        let add_tags_window_data = Box::new(AddTagsWindowData {
            window: window,
            entry: entry as *mut GtkEntry,
            files: files,
            raw_c_strings: raw_c_strings
        });
        let add_tags_window_data_raw = Box::into_raw(mem::transmute(add_tags_window_data));

        // hit Enter in text entry box
        g_signal_connect_data(
            entry as *mut GObject,
            activate_name,
            Some(mem::transmute(on_entry_activated_cb as *mut c_void)),
            add_tags_window_data_raw,
            None,
            GConnectFlags::empty()
        );

        // click button
        g_signal_connect_data(
            button as *mut GObject,
            clicked_name,
            Some(mem::transmute(on_button_clicked_cb as *mut c_void)),
            add_tags_window_data_raw,
            None,
            GConnectFlags::empty()
        );

        // close window
        g_signal_connect_data(
            window as *mut GObject,
            delete_event_name,
            Some(mem::transmute(on_delete_window_cb as *mut c_void)),
            add_tags_window_data_raw,
            None,
            GConnectFlags::empty()
        );
        g_signal_connect_data(
            window as *mut GObject,
            destroy_name,
            Some(mem::transmute(on_destroy_window_cb as *mut c_void)),
            ptr::null_mut(),
            None,
            GConnectFlags::empty()
        );

        gtk_widget_show_all(window);

        gtk_main();
    }
}

fn add_tags(user_data: *mut c_void) {
    unsafe {
        // don't use Box::from_raw; keep raw pointer under Glib control until Gtk window destroyed
        let add_tags_window_ptr: *mut AddTagsWindowData = user_data as *mut AddTagsWindowData;
        let ref add_tags_window = *add_tags_window_ptr;

        let entry_text = CStr::from_ptr(gtk_entry_get_text(add_tags_window.entry)).to_str().unwrap();
        let filenames = filenames(add_tags_window.files);

        for tag in entry_text.split_whitespace() {
            Command::new("tmsu")
                    .arg("tag")
                    .arg(format!("--tags=\"{}\"", tag))
                    .args(&filenames)
                    .current_dir(Path::new(&filenames[0]).parent().unwrap())
                    .output()
                    .expect("failed to tag files");
        }

        invalidate_file_infos(add_tags_window.files);

        gtk_window_close(add_tags_window.window as *mut GtkWindow);
    }
}

fn filenames(files: *mut GList) -> Vec<String> {
    let mut filenames = vec![];
    unsafe {
        let length = g_list_length(files);
        for i in 0..length {
            let file = g_list_nth_data(files, i) as *mut NautilusFileInfo;
            let uri_scheme = CStr::from_ptr(nautilus_file_info_get_uri_scheme(file)).to_str().unwrap();
            if uri_scheme != "file" {
                continue;
            }

            let uri = CStr::from_ptr(nautilus_file_info_get_uri(file)).to_str().unwrap();
            let path = url::percent_encoding::percent_decode(&uri[7..].as_ref()).decode_utf8_lossy().into_owned();
            filenames.push(path);
        }
    }
    filenames
}

fn invalidate_file_infos(files: *mut GList) {
    unsafe {
        let length = g_list_length(files);
        for i in 0..length {
            let file = g_list_nth_data(files, i) as *mut NautilusFileInfo;
            nautilus_file_info_invalidate_extension_info(file);
        }
    }
}

fn destroy_window_data(user_data: *mut c_void) {
    unsafe {
        let boxed_add_tags_window: Box<AddTagsWindowData> = mem::transmute(Box::from_raw(user_data));
        let add_tags_window = *boxed_add_tags_window;
        deallocate_window_strings(&add_tags_window);
    }
}

fn deallocate_window_strings(add_tags_window: &AddTagsWindowData) {
    for raw_c_string in &add_tags_window.raw_c_strings {
        unsafe {
            CString::from_raw(*raw_c_string);
        }
    }
}
