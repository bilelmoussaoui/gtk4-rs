// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ContentFormats;
use crate::ContentProvider;
use crate::Display;
use crate::Texture;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GdkClipboard")]
    pub struct Clipboard(Object<ffi::GdkClipboard>);

    match fn {
        type_ => || ffi::gdk_clipboard_get_type(),
    }
}

impl Clipboard {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`Clipboard`].
    ///
    /// This method returns an instance of [`ClipboardBuilder`] which can be used to create a [`Clipboard`].
    pub fn builder() -> ClipboardBuilder {
        ClipboardBuilder::default()
    }

    #[doc(alias = "gdk_clipboard_get_content")]
    #[doc(alias = "get_content")]
    pub fn content(&self) -> Option<ContentProvider> {
        unsafe { from_glib_none(ffi::gdk_clipboard_get_content(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_clipboard_get_display")]
    #[doc(alias = "get_display")]
    pub fn display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_clipboard_get_display(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_clipboard_get_formats")]
    #[doc(alias = "get_formats")]
    pub fn formats(&self) -> Option<ContentFormats> {
        unsafe { from_glib_none(ffi::gdk_clipboard_get_formats(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_clipboard_is_local")]
    pub fn is_local(&self) -> bool {
        unsafe { from_glib(ffi::gdk_clipboard_is_local(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_clipboard_set_content")]
    pub fn set_content<P: IsA<ContentProvider>>(
        &self,
        provider: Option<&P>,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gdk_clipboard_set_content(
                    self.to_glib_none().0,
                    provider.map(|p| p.as_ref()).to_glib_none().0
                ),
                "Can't set new clipboard content"
            )
        }
    }

    #[doc(alias = "gdk_clipboard_set_text")]
    pub fn set_text(&self, text: &str) {
        unsafe {
            ffi::gdk_clipboard_set_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_clipboard_set_texture")]
    pub fn set_texture<P: IsA<Texture>>(&self, texture: &P) {
        unsafe {
            ffi::gdk_clipboard_set_texture(
                self.to_glib_none().0,
                texture.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "changed")]
    pub fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<F: Fn(&Clipboard) + 'static>(
            this: *mut ffi::GdkClipboard,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "content")]
    pub fn connect_content_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_trampoline<F: Fn(&Clipboard) + 'static>(
            this: *mut ffi::GdkClipboard,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::content\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_content_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "formats")]
    pub fn connect_formats_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_formats_trampoline<F: Fn(&Clipboard) + 'static>(
            this: *mut ffi::GdkClipboard,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::formats\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_formats_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "local")]
    pub fn connect_local_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_local_trampoline<F: Fn(&Clipboard) + 'static>(
            this: *mut ffi::GdkClipboard,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::local\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_local_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`Clipboard`].
pub struct ClipboardBuilder {
    display: Option<Display>,
}

impl ClipboardBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ClipboardBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Clipboard`].
    pub fn build(self) -> Clipboard {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref display) = self.display {
            properties.push(("display", display));
        }
        glib::Object::new::<Clipboard>(&properties)
            .expect("Failed to create an instance of Clipboard")
    }

    pub fn display<P: IsA<Display>>(mut self, display: &P) -> Self {
        self.display = Some(display.clone().upcast());
        self
    }
}

impl fmt::Display for Clipboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Clipboard")
    }
}
