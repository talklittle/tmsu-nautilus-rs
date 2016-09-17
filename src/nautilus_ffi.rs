use gio_ffi::{GFile, GFileType, GMount};
use glib_ffi::{GList, GType, gboolean, gpointer};
use gobject_ffi::{GClosure, GTypeInterface};
use gtk_ffi::GtkWidget;
use libc::c_char;

#[repr(C)]
pub struct NautilusFileInfo {
    g_iface: GTypeInterface,
    is_gone: Option<extern fn(*mut NautilusFileInfo) -> gboolean>,
    get_name: Option<extern fn(*mut NautilusFileInfo) -> *mut c_char>,
    get_uri: Option<extern fn(*mut NautilusFileInfo) -> *mut c_char>,
    get_parent_uri: Option<extern fn(*mut NautilusFileInfo) -> *mut c_char>,
    get_uri_scheme: Option<extern fn(*mut NautilusFileInfo) -> *mut c_char>,
    get_mime_type: Option<extern fn(*mut NautilusFileInfo) -> *mut c_char>,
    is_mime_type: Option<extern fn(*mut NautilusFileInfo) -> gboolean>,
    is_directory: Option<extern fn(*mut NautilusFileInfo) -> gboolean>,
    add_emblem: Option<extern fn(*mut NautilusFileInfo)>,
    get_string_attribute: Option<extern fn(*mut NautilusFileInfo) -> *mut c_char>,
    add_string_attribute: Option<extern fn(*mut NautilusFileInfo)>,
    invalidate_extension_info: Option<extern fn(*mut NautilusFileInfo)>,
    get_activation_uri: Option<extern fn(*mut NautilusFileInfo) -> *mut c_char>,
    get_file_type: Option<extern fn(*mut NautilusFileInfo) -> GFileType>,
    get_location: Option<extern fn(*mut NautilusFileInfo) -> *mut GFile>,
    get_parent_location: Option<extern fn(*mut NautilusFileInfo) -> *mut GFile>,
    get_parent_info: Option<extern fn(*mut NautilusFileInfo) -> *mut NautilusFileInfo>,
    get_mount: Option<extern fn(*mut NautilusFileInfo) -> *mut GMount>,
    can_write: Option<extern fn(*mut NautilusFileInfo) -> gboolean>,
}

#[repr(C)]
pub struct NautilusColumnProviderIface {
    g_iface: GTypeInterface,
    pub get_columns: Option<extern fn(gpointer) -> *mut GList>
}

#[repr(C)]
pub struct NautilusInfoProviderIface {
    g_iface: GTypeInterface,
    pub update_file_info: Option<extern fn(*mut NautilusInfoProvider, *mut NautilusFileInfo, *mut GClosure, *mut *mut NautilusOperationHandle) -> NautilusOperationResult>,
    pub cancel_update: Option<extern fn(*mut NautilusInfoProvider, *mut NautilusOperationHandle)>
}

#[repr(C)]
pub struct NautilusMenuProviderIface {
    g_iface: GTypeInterface,
    pub get_file_items: Option<extern fn(gpointer, *mut GtkWidget, *mut GList) -> *mut GList>,
    pub get_background_items: Option<extern fn(gpointer, *mut GtkWidget, *mut NautilusFileInfo) -> *mut GList>,
}

pub enum NautilusColumn {}
pub enum NautilusInfoProvider {}
pub enum NautilusMenu {}
pub enum NautilusMenuItem {}
pub enum NautilusOperationHandle {}

#[link(name = "nautilus-extension")]
extern {
    pub fn nautilus_column_new(name: *const c_char, attribute: *const c_char, label: *const c_char, description: *const c_char) -> *mut NautilusColumn;
    pub fn nautilus_column_provider_get_type() -> GType;
    pub fn nautilus_file_info_add_string_attribute(file: *mut NautilusFileInfo, attribute_name: *const c_char, value: *const c_char);
    pub fn nautilus_file_info_get_uri(file_info: *mut NautilusFileInfo) -> *mut c_char;
    pub fn nautilus_file_info_get_uri_scheme(file_info: *mut NautilusFileInfo) -> *mut c_char;
    pub fn nautilus_file_info_invalidate_extension_info(file: *mut NautilusFileInfo);
    pub fn nautilus_file_info_list_copy(files: *mut GList) -> *mut GList;
    pub fn nautilus_info_provider_get_type() -> GType;
    pub fn nautilus_info_provider_update_complete_invoke(update_complete: *mut GClosure,
                                                         provider: *mut NautilusInfoProvider,
                                                         handle: *mut NautilusOperationHandle,
                                                         result: NautilusOperationResult);
    pub fn nautilus_menu_append_item(menu: *mut NautilusMenu, item: *mut NautilusMenuItem);
    pub fn nautilus_menu_item_new(name: *const c_char, label: *const c_char, tip: *const c_char, icon: *const c_char) -> *mut NautilusMenuItem;
    pub fn nautilus_menu_item_set_submenu(item: *mut NautilusMenuItem, menu: *mut NautilusMenu);
    pub fn nautilus_menu_new() -> *mut NautilusMenu;
    pub fn nautilus_menu_provider_get_type() -> GType;
}

#[repr(C)]
pub enum NautilusOperationResult {
    NAUTILUS_OPERATION_COMPLETE,
    NAUTILUS_OPERATION_FAILED,
    NAUTILUS_OPERATION_IN_PROGRESS,
}
