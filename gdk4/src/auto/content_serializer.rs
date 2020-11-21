// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct ContentSerializer(Object<ffi::GdkContentSerializer>);

    match fn {
        get_type => || ffi::gdk_content_serializer_get_type(),
    }
}

impl ContentSerializer {
    pub fn get_cancellable(&self) -> Option<gio::Cancellable> {
        unsafe {
            from_glib_none(ffi::gdk_content_serializer_get_cancellable(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_gtype(&self) -> glib::types::Type {
        unsafe { from_glib(ffi::gdk_content_serializer_get_gtype(self.to_glib_none().0)) }
    }

    pub fn get_mime_type(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::gdk_content_serializer_get_mime_type(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_output_stream(&self) -> gio::OutputStream {
        unsafe {
            from_glib_none(ffi::gdk_content_serializer_get_output_stream(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_priority(&self) -> i32 {
        unsafe { ffi::gdk_content_serializer_get_priority(self.to_glib_none().0) }
    }

    pub fn get_value(&self) -> glib::Value {
        unsafe { from_glib_none(ffi::gdk_content_serializer_get_value(self.to_glib_none().0)) }
    }

    pub fn return_error(&self, error: &mut glib::Error) {
        unsafe {
            ffi::gdk_content_serializer_return_error(
                self.to_glib_none().0,
                error.to_glib_none_mut().0,
            );
        }
    }

    pub fn return_success(&self) {
        unsafe {
            ffi::gdk_content_serializer_return_success(self.to_glib_none().0);
        }
    }
}

impl fmt::Display for ContentSerializer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ContentSerializer")
    }
}
