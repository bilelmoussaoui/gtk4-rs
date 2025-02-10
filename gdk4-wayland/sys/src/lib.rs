// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use glib_sys as glib;

#[cfg(unix)]
#[allow(unused_imports)]
use libc::{dev_t, gid_t, pid_t, socklen_t, uid_t};
#[allow(unused_imports)]
use libc::{intptr_t, off_t, size_t, ssize_t, time_t, uintptr_t, FILE};
#[allow(unused_imports)]
use std::ffi::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Callbacks
pub type GdkWaylandToplevelExported =
    Option<unsafe extern "C" fn(*mut GdkWaylandToplevel, *const c_char, gpointer)>;

// Records
#[repr(C)]
#[allow(dead_code)]
pub struct _GdkWaylandDeviceClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GdkWaylandDeviceClass = _GdkWaylandDeviceClass;

#[repr(C)]
#[allow(dead_code)]
pub struct _GdkWaylandDisplayClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GdkWaylandDisplayClass = _GdkWaylandDisplayClass;

#[repr(C)]
#[allow(dead_code)]
pub struct _GdkWaylandGLContextClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GdkWaylandGLContextClass = _GdkWaylandGLContextClass;

#[repr(C)]
#[allow(dead_code)]
pub struct _GdkWaylandMonitorClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GdkWaylandMonitorClass = _GdkWaylandMonitorClass;

#[repr(C)]
#[allow(dead_code)]
pub struct _GdkWaylandSeatClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GdkWaylandSeatClass = _GdkWaylandSeatClass;

// Classes
#[repr(C)]
#[allow(dead_code)]
pub struct GdkWaylandDevice {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GdkWaylandDevice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkWaylandDevice @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct GdkWaylandDisplay {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GdkWaylandDisplay {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkWaylandDisplay @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct GdkWaylandGLContext {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GdkWaylandGLContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkWaylandGLContext @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct GdkWaylandMonitor {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GdkWaylandMonitor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkWaylandMonitor @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct GdkWaylandPopup {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GdkWaylandPopup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkWaylandPopup @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct GdkWaylandSeat {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GdkWaylandSeat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkWaylandSeat @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct GdkWaylandSurface {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GdkWaylandSurface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkWaylandSurface @ {self:p}"))
            .finish()
    }
}

#[repr(C)]
#[allow(dead_code)]
pub struct GdkWaylandToplevel {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GdkWaylandToplevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkWaylandToplevel @ {self:p}"))
            .finish()
    }
}

extern "C" {

    //=========================================================================
    // GdkWaylandDevice
    //=========================================================================
    pub fn gdk_wayland_device_get_type() -> GType;
    pub fn gdk_wayland_device_get_node_path(device: *mut GdkWaylandDevice) -> *const c_char;
    pub fn gdk_wayland_device_get_wl_keyboard(device: *mut GdkWaylandDevice) -> gpointer;
    pub fn gdk_wayland_device_get_wl_pointer(device: *mut GdkWaylandDevice) -> gpointer;
    pub fn gdk_wayland_device_get_wl_seat(device: *mut GdkWaylandDevice) -> gpointer;
    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    pub fn gdk_wayland_device_get_xkb_keymap(device: *mut GdkWaylandDevice) -> gpointer;

    //=========================================================================
    // GdkWaylandDisplay
    //=========================================================================
    pub fn gdk_wayland_display_get_type() -> GType;
    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    pub fn gdk_wayland_display_get_egl_display(display: *mut GdkWaylandDisplay) -> gpointer;
    pub fn gdk_wayland_display_get_startup_notification_id(
        display: *mut GdkWaylandDisplay,
    ) -> *const c_char;
    pub fn gdk_wayland_display_get_wl_compositor(display: *mut GdkWaylandDisplay) -> gpointer;
    pub fn gdk_wayland_display_get_wl_display(display: *mut GdkWaylandDisplay) -> gpointer;
    pub fn gdk_wayland_display_query_registry(
        display: *mut GdkWaylandDisplay,
        global: *const c_char,
    ) -> gboolean;
    pub fn gdk_wayland_display_set_cursor_theme(
        display: *mut GdkWaylandDisplay,
        name: *const c_char,
        size: c_int,
    );
    pub fn gdk_wayland_display_set_startup_notification_id(
        display: *mut GdkWaylandDisplay,
        startup_id: *const c_char,
    );

    //=========================================================================
    // GdkWaylandGLContext
    //=========================================================================
    pub fn gdk_wayland_gl_context_get_type() -> GType;

    //=========================================================================
    // GdkWaylandMonitor
    //=========================================================================
    pub fn gdk_wayland_monitor_get_type() -> GType;
    pub fn gdk_wayland_monitor_get_wl_output(monitor: *mut GdkWaylandMonitor) -> gpointer;

    //=========================================================================
    // GdkWaylandPopup
    //=========================================================================
    pub fn gdk_wayland_popup_get_type() -> GType;

    //=========================================================================
    // GdkWaylandSeat
    //=========================================================================
    pub fn gdk_wayland_seat_get_type() -> GType;
    pub fn gdk_wayland_seat_get_wl_seat(seat: *mut GdkWaylandSeat) -> gpointer;

    //=========================================================================
    // GdkWaylandSurface
    //=========================================================================
    pub fn gdk_wayland_surface_get_type() -> GType;
    pub fn gdk_wayland_surface_force_next_commit(surface: *mut gdk::GdkSurface);
    pub fn gdk_wayland_surface_get_wl_surface(surface: *mut GdkWaylandSurface) -> gpointer;

    //=========================================================================
    // GdkWaylandToplevel
    //=========================================================================
    pub fn gdk_wayland_toplevel_get_type() -> GType;
    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    pub fn gdk_wayland_toplevel_drop_exported_handle(
        toplevel: *mut GdkWaylandToplevel,
        handle: *const c_char,
    );
    pub fn gdk_wayland_toplevel_export_handle(
        toplevel: *mut GdkWaylandToplevel,
        callback: GdkWaylandToplevelExported,
        user_data: gpointer,
        destroy_func: glib::GDestroyNotify,
    ) -> gboolean;
    pub fn gdk_wayland_toplevel_set_application_id(
        toplevel: *mut GdkWaylandToplevel,
        application_id: *const c_char,
    );
    pub fn gdk_wayland_toplevel_set_transient_for_exported(
        toplevel: *mut GdkWaylandToplevel,
        parent_handle_str: *const c_char,
    ) -> gboolean;
    pub fn gdk_wayland_toplevel_unexport_handle(toplevel: *mut GdkWaylandToplevel);

}
