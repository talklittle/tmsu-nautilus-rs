extern crate gdk_sys as gdk_ffi;
extern crate gio_sys as gio_ffi;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gtk_sys as gtk_ffi;
extern crate libc;

extern crate nautilus_extension;
#[macro_use(nautilus_module)]
extern crate nautilus_extension_sys;

extern crate url;

pub mod tmsu_nautilus;
pub mod nautilus_ffi;
pub mod column_provider;
pub mod info_provider;
pub mod menu_provider;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
