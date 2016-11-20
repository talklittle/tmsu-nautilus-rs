use gtk;
use gtk_ffi::gtk_init;
use std::ptr;

// workaround for https://github.com/gtk-rs/gtk/issues/405
pub fn init_gtk() {
    let mut argc = 0;
    unsafe {
        gtk_init(&mut argc, ptr::null_mut());
        gtk::set_initialized();
    }
}
