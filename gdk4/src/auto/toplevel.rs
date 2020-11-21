// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Device;
use crate::Event;
use crate::FullscreenMode;
use crate::Surface;
use crate::SurfaceEdge;
use crate::Texture;
use crate::ToplevelLayout;
use crate::ToplevelState;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct Toplevel(Interface<ffi::GdkToplevel>) @requires Surface;

    match fn {
        get_type => || ffi::gdk_toplevel_get_type(),
    }
}

pub const NONE_TOPLEVEL: Option<&Toplevel> = None;

pub trait ToplevelExt: 'static {
    fn begin_move(&self, device: &Device, button: i32, x: f64, y: f64, timestamp: u32);

    fn begin_resize(
        &self,
        edge: SurfaceEdge,
        device: Option<&Device>,
        button: i32,
        x: f64,
        y: f64,
        timestamp: u32,
    );

    fn focus(&self, timestamp: u32);

    fn get_state(&self) -> ToplevelState;

    fn inhibit_system_shortcuts<P: IsA<Event>>(&self, event: Option<&P>);

    fn lower(&self) -> bool;

    fn minimize(&self) -> bool;

    fn present(&self, layout: &ToplevelLayout) -> bool;

    fn restore_system_shortcuts(&self);

    fn set_decorated(&self, decorated: bool);

    fn set_deletable(&self, deletable: bool);

    fn set_icon_list(&self, surfaces: &[Texture]);

    fn set_modal(&self, modal: bool);

    fn set_startup_id(&self, startup_id: &str);

    fn set_title(&self, title: &str);

    fn set_transient_for(&self, parent: &Surface);

    fn show_window_menu<P: IsA<Event>>(&self, event: &P) -> bool;

    fn supports_edge_constraints(&self) -> bool;

    fn get_property_decorated(&self) -> bool;

    fn get_property_deletable(&self) -> bool;

    fn get_property_fullscreen_mode(&self) -> FullscreenMode;

    fn set_property_fullscreen_mode(&self, fullscreen_mode: FullscreenMode);

    //fn get_property_icon_list(&self) -> /*Unimplemented*/Fundamental: Pointer;

    fn get_property_modal(&self) -> bool;

    fn get_property_shortcuts_inhibited(&self) -> bool;

    fn get_property_startup_id(&self) -> Option<glib::GString>;

    fn get_property_title(&self) -> Option<glib::GString>;

    fn get_property_transient_for(&self) -> Option<Surface>;

    //fn connect_compute_size<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_property_decorated_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_deletable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_fullscreen_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_icon_list_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_shortcuts_inhibited_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_startup_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_transient_for_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Toplevel>> ToplevelExt for O {
    fn begin_move(&self, device: &Device, button: i32, x: f64, y: f64, timestamp: u32) {
        unsafe {
            ffi::gdk_toplevel_begin_move(
                self.as_ref().to_glib_none().0,
                device.to_glib_none().0,
                button,
                x,
                y,
                timestamp,
            );
        }
    }

    fn begin_resize(
        &self,
        edge: SurfaceEdge,
        device: Option<&Device>,
        button: i32,
        x: f64,
        y: f64,
        timestamp: u32,
    ) {
        unsafe {
            ffi::gdk_toplevel_begin_resize(
                self.as_ref().to_glib_none().0,
                edge.to_glib(),
                device.to_glib_none().0,
                button,
                x,
                y,
                timestamp,
            );
        }
    }

    fn focus(&self, timestamp: u32) {
        unsafe {
            ffi::gdk_toplevel_focus(self.as_ref().to_glib_none().0, timestamp);
        }
    }

    fn get_state(&self) -> ToplevelState {
        unsafe { from_glib(ffi::gdk_toplevel_get_state(self.as_ref().to_glib_none().0)) }
    }

    fn inhibit_system_shortcuts<P: IsA<Event>>(&self, event: Option<&P>) {
        unsafe {
            ffi::gdk_toplevel_inhibit_system_shortcuts(
                self.as_ref().to_glib_none().0,
                event.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn lower(&self) -> bool {
        unsafe { from_glib(ffi::gdk_toplevel_lower(self.as_ref().to_glib_none().0)) }
    }

    fn minimize(&self) -> bool {
        unsafe { from_glib(ffi::gdk_toplevel_minimize(self.as_ref().to_glib_none().0)) }
    }

    fn present(&self, layout: &ToplevelLayout) -> bool {
        unsafe {
            from_glib(ffi::gdk_toplevel_present(
                self.as_ref().to_glib_none().0,
                layout.to_glib_none().0,
            ))
        }
    }

    fn restore_system_shortcuts(&self) {
        unsafe {
            ffi::gdk_toplevel_restore_system_shortcuts(self.as_ref().to_glib_none().0);
        }
    }

    fn set_decorated(&self, decorated: bool) {
        unsafe {
            ffi::gdk_toplevel_set_decorated(self.as_ref().to_glib_none().0, decorated.to_glib());
        }
    }

    fn set_deletable(&self, deletable: bool) {
        unsafe {
            ffi::gdk_toplevel_set_deletable(self.as_ref().to_glib_none().0, deletable.to_glib());
        }
    }

    fn set_icon_list(&self, surfaces: &[Texture]) {
        unsafe {
            ffi::gdk_toplevel_set_icon_list(
                self.as_ref().to_glib_none().0,
                surfaces.to_glib_none().0,
            );
        }
    }

    fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gdk_toplevel_set_modal(self.as_ref().to_glib_none().0, modal.to_glib());
        }
    }

    fn set_startup_id(&self, startup_id: &str) {
        unsafe {
            ffi::gdk_toplevel_set_startup_id(
                self.as_ref().to_glib_none().0,
                startup_id.to_glib_none().0,
            );
        }
    }

    fn set_title(&self, title: &str) {
        unsafe {
            ffi::gdk_toplevel_set_title(self.as_ref().to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn set_transient_for(&self, parent: &Surface) {
        unsafe {
            ffi::gdk_toplevel_set_transient_for(
                self.as_ref().to_glib_none().0,
                parent.to_glib_none().0,
            );
        }
    }

    fn show_window_menu<P: IsA<Event>>(&self, event: &P) -> bool {
        unsafe {
            from_glib(ffi::gdk_toplevel_show_window_menu(
                self.as_ref().to_glib_none().0,
                event.as_ref().to_glib_none().0,
            ))
        }
    }

    fn supports_edge_constraints(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_toplevel_supports_edge_constraints(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_property_decorated(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"decorated\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `decorated` getter")
                .unwrap()
        }
    }

    fn get_property_deletable(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"deletable\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `deletable` getter")
                .unwrap()
        }
    }

    fn get_property_fullscreen_mode(&self) -> FullscreenMode {
        unsafe {
            let mut value = Value::from_type(<FullscreenMode as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"fullscreen-mode\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `fullscreen-mode` getter")
                .unwrap()
        }
    }

    fn set_property_fullscreen_mode(&self, fullscreen_mode: FullscreenMode) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"fullscreen-mode\0".as_ptr() as *const _,
                Value::from(&fullscreen_mode).to_glib_none().0,
            );
        }
    }

    //fn get_property_icon_list(&self) -> /*Unimplemented*/Fundamental: Pointer {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"icon-list\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get().expect("Return Value for property `icon-list` getter").unwrap()
    //    }
    //}

    fn get_property_modal(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"modal\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `modal` getter")
                .unwrap()
        }
    }

    fn get_property_shortcuts_inhibited(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"shortcuts-inhibited\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `shortcuts-inhibited` getter")
                .unwrap()
        }
    }

    fn get_property_startup_id(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"startup-id\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `startup-id` getter")
        }
    }

    fn get_property_title(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"title\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `title` getter")
        }
    }

    fn get_property_transient_for(&self) -> Option<Surface> {
        unsafe {
            let mut value = Value::from_type(<Surface as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"transient-for\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `transient-for` getter")
        }
    }

    //fn connect_compute_size<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Out size: Gdk.ToplevelSize
    //}

    fn connect_property_decorated_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_decorated_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Toplevel>,
        {
            let f: &F = &*(f as *const F);
            f(&Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::decorated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_decorated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_deletable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_deletable_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Toplevel>,
        {
            let f: &F = &*(f as *const F);
            f(&Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::deletable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_deletable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_fullscreen_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_fullscreen_mode_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Toplevel>,
        {
            let f: &F = &*(f as *const F);
            f(&Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::fullscreen-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_fullscreen_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_icon_list_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_list_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Toplevel>,
        {
            let f: &F = &*(f as *const F);
            f(&Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-list\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_list_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modal_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Toplevel>,
        {
            let f: &F = &*(f as *const F);
            f(&Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::modal\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_modal_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_shortcuts_inhibited_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_shortcuts_inhibited_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Toplevel>,
        {
            let f: &F = &*(f as *const F);
            f(&Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::shortcuts-inhibited\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_shortcuts_inhibited_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_startup_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_startup_id_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Toplevel>,
        {
            let f: &F = &*(f as *const F);
            f(&Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::startup-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_startup_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Toplevel>,
        {
            let f: &F = &*(f as *const F);
            f(&Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::state\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_state_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Toplevel>,
        {
            let f: &F = &*(f as *const F);
            f(&Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_transient_for_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_transient_for_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Toplevel>,
        {
            let f: &F = &*(f as *const F);
            f(&Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::transient-for\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_transient_for_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Toplevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Toplevel")
    }
}
