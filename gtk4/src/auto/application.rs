// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ApplicationInhibitFlags;
use crate::Window;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkApplication")]
    pub struct Application(Object<ffi::GtkApplication, ffi::GtkApplicationClass>) @extends gio::Application, @implements gio::ActionGroup, gio::ActionMap;

    match fn {
        type_ => || ffi::gtk_application_get_type(),
    }
}

impl Application {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`Application`].
    ///
    /// This method returns an instance of [`ApplicationBuilder`] which can be used to create a [`Application`].
    pub fn builder() -> ApplicationBuilder {
        ApplicationBuilder::default()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`Application`].
pub struct ApplicationBuilder {
    menubar: Option<gio::MenuModel>,
    register_session: Option<bool>,
    action_group: Option<gio::ActionGroup>,
    application_id: Option<String>,
    flags: Option<gio::ApplicationFlags>,
    inactivity_timeout: Option<u32>,
    resource_base_path: Option<String>,
}

impl ApplicationBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ApplicationBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Application`].
    pub fn build(self) -> Application {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref menubar) = self.menubar {
            properties.push(("menubar", menubar));
        }
        if let Some(ref register_session) = self.register_session {
            properties.push(("register-session", register_session));
        }
        if let Some(ref action_group) = self.action_group {
            properties.push(("action-group", action_group));
        }
        if let Some(ref application_id) = self.application_id {
            properties.push(("application-id", application_id));
        }
        if let Some(ref flags) = self.flags {
            properties.push(("flags", flags));
        }
        if let Some(ref inactivity_timeout) = self.inactivity_timeout {
            properties.push(("inactivity-timeout", inactivity_timeout));
        }
        if let Some(ref resource_base_path) = self.resource_base_path {
            properties.push(("resource-base-path", resource_base_path));
        }
        let ret = glib::Object::new::<Application>(&properties)
            .expect("Failed to create an instance of Application");
        {
            Application::register_startup_hook(&ret);
        }
        ret
    }

    pub fn menubar<P: IsA<gio::MenuModel>>(mut self, menubar: &P) -> Self {
        self.menubar = Some(menubar.clone().upcast());
        self
    }

    pub fn register_session(mut self, register_session: bool) -> Self {
        self.register_session = Some(register_session);
        self
    }

    pub fn action_group<P: IsA<gio::ActionGroup>>(mut self, action_group: &P) -> Self {
        self.action_group = Some(action_group.clone().upcast());
        self
    }

    pub fn application_id(mut self, application_id: &str) -> Self {
        self.application_id = Some(application_id.to_string());
        self
    }

    pub fn flags(mut self, flags: gio::ApplicationFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    pub fn inactivity_timeout(mut self, inactivity_timeout: u32) -> Self {
        self.inactivity_timeout = Some(inactivity_timeout);
        self
    }

    pub fn resource_base_path(mut self, resource_base_path: &str) -> Self {
        self.resource_base_path = Some(resource_base_path.to_string());
        self
    }
}

pub const NONE_APPLICATION: Option<&Application> = None;

pub trait GtkApplicationExt: 'static {
    #[doc(alias = "gtk_application_add_window")]
    fn add_window<P: IsA<Window>>(&self, window: &P);

    #[doc(alias = "gtk_application_get_accels_for_action")]
    #[doc(alias = "get_accels_for_action")]
    fn accels_for_action(&self, detailed_action_name: &str) -> Vec<glib::GString>;

    #[doc(alias = "gtk_application_get_actions_for_accel")]
    #[doc(alias = "get_actions_for_accel")]
    fn actions_for_accel(&self, accel: &str) -> Vec<glib::GString>;

    #[doc(alias = "gtk_application_get_active_window")]
    #[doc(alias = "get_active_window")]
    fn active_window(&self) -> Option<Window>;

    #[doc(alias = "gtk_application_get_menu_by_id")]
    #[doc(alias = "get_menu_by_id")]
    fn menu_by_id(&self, id: &str) -> Option<gio::Menu>;

    #[doc(alias = "gtk_application_get_menubar")]
    #[doc(alias = "get_menubar")]
    fn menubar(&self) -> Option<gio::MenuModel>;

    #[doc(alias = "gtk_application_get_window_by_id")]
    #[doc(alias = "get_window_by_id")]
    fn window_by_id(&self, id: u32) -> Option<Window>;

    #[doc(alias = "gtk_application_get_windows")]
    #[doc(alias = "get_windows")]
    fn windows(&self) -> Vec<Window>;

    #[doc(alias = "gtk_application_inhibit")]
    fn inhibit<P: IsA<Window>>(
        &self,
        window: Option<&P>,
        flags: ApplicationInhibitFlags,
        reason: Option<&str>,
    ) -> u32;

    #[doc(alias = "gtk_application_list_action_descriptions")]
    fn list_action_descriptions(&self) -> Vec<glib::GString>;

    #[doc(alias = "gtk_application_remove_window")]
    fn remove_window<P: IsA<Window>>(&self, window: &P);

    #[doc(alias = "gtk_application_set_accels_for_action")]
    fn set_accels_for_action(&self, detailed_action_name: &str, accels: &[&str]);

    #[doc(alias = "gtk_application_set_menubar")]
    fn set_menubar<P: IsA<gio::MenuModel>>(&self, menubar: Option<&P>);

    #[doc(alias = "gtk_application_uninhibit")]
    fn uninhibit(&self, cookie: u32);

    #[doc(alias = "register-session")]
    fn is_register_session(&self) -> bool;

    #[doc(alias = "register-session")]
    fn set_register_session(&self, register_session: bool);

    #[doc(alias = "screensaver-active")]
    fn is_screensaver_active(&self) -> bool;

    #[doc(alias = "query-end")]
    fn connect_query_end<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "window-added")]
    fn connect_window_added<F: Fn(&Self, &Window) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "window-removed")]
    fn connect_window_removed<F: Fn(&Self, &Window) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "active-window")]
    fn connect_active_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "menubar")]
    fn connect_menubar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "register-session")]
    fn connect_register_session_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "screensaver-active")]
    fn connect_screensaver_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Application>> GtkApplicationExt for O {
    fn add_window<P: IsA<Window>>(&self, window: &P) {
        unsafe {
            ffi::gtk_application_add_window(
                self.as_ref().to_glib_none().0,
                window.as_ref().to_glib_none().0,
            );
        }
    }

    fn accels_for_action(&self, detailed_action_name: &str) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_get_accels_for_action(
                self.as_ref().to_glib_none().0,
                detailed_action_name.to_glib_none().0,
            ))
        }
    }

    fn actions_for_accel(&self, accel: &str) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_get_actions_for_accel(
                self.as_ref().to_glib_none().0,
                accel.to_glib_none().0,
            ))
        }
    }

    fn active_window(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_active_window(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn menu_by_id(&self, id: &str) -> Option<gio::Menu> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_menu_by_id(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
            ))
        }
    }

    fn menubar(&self) -> Option<gio::MenuModel> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_menubar(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn window_by_id(&self, id: u32) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_window_by_id(
                self.as_ref().to_glib_none().0,
                id,
            ))
        }
    }

    fn windows(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_application_get_windows(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn inhibit<P: IsA<Window>>(
        &self,
        window: Option<&P>,
        flags: ApplicationInhibitFlags,
        reason: Option<&str>,
    ) -> u32 {
        unsafe {
            ffi::gtk_application_inhibit(
                self.as_ref().to_glib_none().0,
                window.map(|p| p.as_ref()).to_glib_none().0,
                flags.into_glib(),
                reason.to_glib_none().0,
            )
        }
    }

    fn list_action_descriptions(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_list_action_descriptions(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_window<P: IsA<Window>>(&self, window: &P) {
        unsafe {
            ffi::gtk_application_remove_window(
                self.as_ref().to_glib_none().0,
                window.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_accels_for_action(&self, detailed_action_name: &str, accels: &[&str]) {
        unsafe {
            ffi::gtk_application_set_accels_for_action(
                self.as_ref().to_glib_none().0,
                detailed_action_name.to_glib_none().0,
                accels.to_glib_none().0,
            );
        }
    }

    fn set_menubar<P: IsA<gio::MenuModel>>(&self, menubar: Option<&P>) {
        unsafe {
            ffi::gtk_application_set_menubar(
                self.as_ref().to_glib_none().0,
                menubar.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn uninhibit(&self, cookie: u32) {
        unsafe {
            ffi::gtk_application_uninhibit(self.as_ref().to_glib_none().0, cookie);
        }
    }

    fn is_register_session(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"register-session\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `register-session` getter")
        }
    }

    fn set_register_session(&self, register_session: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"register-session\0".as_ptr() as *const _,
                register_session.to_value().to_glib_none().0,
            );
        }
    }

    fn is_screensaver_active(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"screensaver-active\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `screensaver-active` getter")
        }
    }

    #[doc(alias = "query-end")]
    fn connect_query_end<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn query_end_trampoline<P: IsA<Application>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkApplication,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"query-end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    query_end_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "window-added")]
    fn connect_window_added<F: Fn(&Self, &Window) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn window_added_trampoline<
            P: IsA<Application>,
            F: Fn(&P, &Window) + 'static,
        >(
            this: *mut ffi::GtkApplication,
            window: *mut ffi::GtkWindow,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &Application::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(window),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"window-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    window_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "window-removed")]
    fn connect_window_removed<F: Fn(&Self, &Window) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn window_removed_trampoline<
            P: IsA<Application>,
            F: Fn(&P, &Window) + 'static,
        >(
            this: *mut ffi::GtkApplication,
            window: *mut ffi::GtkWindow,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &Application::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(window),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"window-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    window_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "active-window")]
    fn connect_active_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_window_trampoline<
            P: IsA<Application>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active-window\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_active_window_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "menubar")]
    fn connect_menubar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_menubar_trampoline<P: IsA<Application>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::menubar\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_menubar_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "register-session")]
    fn connect_register_session_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_register_session_trampoline<
            P: IsA<Application>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::register-session\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_register_session_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "screensaver-active")]
    fn connect_screensaver_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_screensaver_active_trampoline<
            P: IsA<Application>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::screensaver-active\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_screensaver_active_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Application {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Application")
    }
}
