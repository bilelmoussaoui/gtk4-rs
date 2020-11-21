// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct DirectoryList(Object<ffi::GtkDirectoryList, ffi::GtkDirectoryListClass>) @implements gio::ListModel;

    match fn {
        get_type => || ffi::gtk_directory_list_get_type(),
    }
}

impl DirectoryList {
    pub fn new<P: IsA<gio::File>>(attributes: Option<&str>, file: Option<&P>) -> DirectoryList {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_directory_list_new(
                attributes.to_glib_none().0,
                file.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }
}

#[derive(Clone, Default)]
pub struct DirectoryListBuilder {
    attributes: Option<String>,
    file: Option<gio::File>,
    io_priority: Option<i32>,
    monitored: Option<bool>,
}

impl DirectoryListBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> DirectoryList {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref attributes) = self.attributes {
            properties.push(("attributes", attributes));
        }
        if let Some(ref file) = self.file {
            properties.push(("file", file));
        }
        if let Some(ref io_priority) = self.io_priority {
            properties.push(("io-priority", io_priority));
        }
        if let Some(ref monitored) = self.monitored {
            properties.push(("monitored", monitored));
        }
        let ret = glib::Object::new(DirectoryList::static_type(), &properties)
            .expect("object new")
            .downcast::<DirectoryList>()
            .expect("downcast");
        ret
    }

    pub fn attributes(mut self, attributes: &str) -> Self {
        self.attributes = Some(attributes.to_string());
        self
    }

    pub fn file<P: IsA<gio::File>>(mut self, file: &P) -> Self {
        self.file = Some(file.clone().upcast());
        self
    }

    pub fn io_priority(mut self, io_priority: i32) -> Self {
        self.io_priority = Some(io_priority);
        self
    }

    pub fn monitored(mut self, monitored: bool) -> Self {
        self.monitored = Some(monitored);
        self
    }
}

pub const NONE_DIRECTORY_LIST: Option<&DirectoryList> = None;

pub trait DirectoryListExt: 'static {
    fn get_attributes(&self) -> Option<glib::GString>;

    fn get_error(&self) -> Option<glib::Error>;

    fn get_file(&self) -> Option<gio::File>;

    fn get_io_priority(&self) -> i32;

    fn get_monitored(&self) -> bool;

    fn is_loading(&self) -> bool;

    fn set_attributes(&self, attributes: Option<&str>);

    fn set_file<P: IsA<gio::File>>(&self, file: Option<&P>);

    fn set_io_priority(&self, io_priority: i32);

    fn set_monitored(&self, monitored: bool);

    fn get_property_loading(&self) -> bool;

    fn connect_property_attributes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_error_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_io_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_loading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_monitored_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DirectoryList>> DirectoryListExt for O {
    fn get_attributes(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_directory_list_get_attributes(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_error(&self) -> Option<glib::Error> {
        unsafe {
            from_glib_none(ffi::gtk_directory_list_get_error(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_file(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(ffi::gtk_directory_list_get_file(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_io_priority(&self) -> i32 {
        unsafe { ffi::gtk_directory_list_get_io_priority(self.as_ref().to_glib_none().0) }
    }

    fn get_monitored(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_directory_list_get_monitored(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_loading(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_directory_list_is_loading(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_attributes(&self, attributes: Option<&str>) {
        unsafe {
            ffi::gtk_directory_list_set_attributes(
                self.as_ref().to_glib_none().0,
                attributes.to_glib_none().0,
            );
        }
    }

    fn set_file<P: IsA<gio::File>>(&self, file: Option<&P>) {
        unsafe {
            ffi::gtk_directory_list_set_file(
                self.as_ref().to_glib_none().0,
                file.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_io_priority(&self, io_priority: i32) {
        unsafe {
            ffi::gtk_directory_list_set_io_priority(self.as_ref().to_glib_none().0, io_priority);
        }
    }

    fn set_monitored(&self, monitored: bool) {
        unsafe {
            ffi::gtk_directory_list_set_monitored(
                self.as_ref().to_glib_none().0,
                monitored.to_glib(),
            );
        }
    }

    fn get_property_loading(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"loading\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `loading` getter")
                .unwrap()
        }
    }

    fn connect_property_attributes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_attributes_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkDirectoryList,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DirectoryList>,
        {
            let f: &F = &*(f as *const F);
            f(&DirectoryList::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::attributes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_attributes_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_error_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_error_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkDirectoryList,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DirectoryList>,
        {
            let f: &F = &*(f as *const F);
            f(&DirectoryList::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::error\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_error_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_file_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkDirectoryList,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DirectoryList>,
        {
            let f: &F = &*(f as *const F);
            f(&DirectoryList::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::file\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_file_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_io_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_io_priority_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkDirectoryList,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DirectoryList>,
        {
            let f: &F = &*(f as *const F);
            f(&DirectoryList::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::io-priority\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_io_priority_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_loading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_loading_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkDirectoryList,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DirectoryList>,
        {
            let f: &F = &*(f as *const F);
            f(&DirectoryList::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::loading\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_loading_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_monitored_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_monitored_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkDirectoryList,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DirectoryList>,
        {
            let f: &F = &*(f as *const F);
            f(&DirectoryList::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::monitored\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_monitored_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DirectoryList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DirectoryList")
    }
}
