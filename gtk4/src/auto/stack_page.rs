// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Widget;
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

glib::glib_wrapper! {
    pub struct StackPage(Object<ffi::GtkStackPage>) @implements Accessible;

    match fn {
        get_type => || ffi::gtk_stack_page_get_type(),
    }
}

impl StackPage {
    pub fn get_child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_stack_page_get_child(self.to_glib_none().0)) }
    }

    pub fn get_icon_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_stack_page_get_icon_name(self.to_glib_none().0)) }
    }

    pub fn get_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_stack_page_get_name(self.to_glib_none().0)) }
    }

    pub fn get_needs_attention(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_page_get_needs_attention(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_title(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_stack_page_get_title(self.to_glib_none().0)) }
    }

    pub fn get_use_underline(&self) -> bool {
        unsafe { from_glib(ffi::gtk_stack_page_get_use_underline(self.to_glib_none().0)) }
    }

    pub fn get_visible(&self) -> bool {
        unsafe { from_glib(ffi::gtk_stack_page_get_visible(self.to_glib_none().0)) }
    }

    pub fn set_icon_name(&self, setting: &str) {
        unsafe {
            ffi::gtk_stack_page_set_icon_name(self.to_glib_none().0, setting.to_glib_none().0);
        }
    }

    pub fn set_name(&self, setting: &str) {
        unsafe {
            ffi::gtk_stack_page_set_name(self.to_glib_none().0, setting.to_glib_none().0);
        }
    }

    pub fn set_needs_attention(&self, setting: bool) {
        unsafe {
            ffi::gtk_stack_page_set_needs_attention(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_title(&self, setting: &str) {
        unsafe {
            ffi::gtk_stack_page_set_title(self.to_glib_none().0, setting.to_glib_none().0);
        }
    }

    pub fn set_use_underline(&self, setting: bool) {
        unsafe {
            ffi::gtk_stack_page_set_use_underline(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_stack_page_set_visible(self.to_glib_none().0, visible.to_glib());
        }
    }

    pub fn connect_property_icon_name_notify<F: Fn(&StackPage) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<F: Fn(&StackPage) + 'static>(
            this: *mut ffi::GtkStackPage,
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
                b"notify::icon-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_needs_attention_notify<F: Fn(&StackPage) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_needs_attention_trampoline<F: Fn(&StackPage) + 'static>(
            this: *mut ffi::GtkStackPage,
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
                b"notify::needs-attention\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_needs_attention_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_title_notify<F: Fn(&StackPage) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&StackPage) + 'static>(
            this: *mut ffi::GtkStackPage,
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
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_use_underline_notify<F: Fn(&StackPage) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_underline_trampoline<F: Fn(&StackPage) + 'static>(
            this: *mut ffi::GtkStackPage,
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
                b"notify::use-underline\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_underline_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_visible_notify<F: Fn(&StackPage) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_trampoline<F: Fn(&StackPage) + 'static>(
            this: *mut ffi::GtkStackPage,
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
                b"notify::visible\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct StackPageBuilder {
    child: Option<Widget>,
    icon_name: Option<String>,
    name: Option<String>,
    needs_attention: Option<bool>,
    title: Option<String>,
    use_underline: Option<bool>,
    visible: Option<bool>,
    accessible_role: Option<AccessibleRole>,
}

impl StackPageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> StackPage {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref icon_name) = self.icon_name {
            properties.push(("icon-name", icon_name));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref needs_attention) = self.needs_attention {
            properties.push(("needs-attention", needs_attention));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        if let Some(ref use_underline) = self.use_underline {
            properties.push(("use-underline", use_underline));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref accessible_role) = self.accessible_role {
            properties.push(("accessible-role", accessible_role));
        }
        let ret = glib::Object::new(StackPage::static_type(), &properties)
            .expect("object new")
            .downcast::<StackPage>()
            .expect("downcast");
        ret
    }

    pub fn child<P: IsA<Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn needs_attention(mut self, needs_attention: bool) -> Self {
        self.needs_attention = Some(needs_attention);
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn use_underline(mut self, use_underline: bool) -> Self {
        self.use_underline = Some(use_underline);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn accessible_role(mut self, accessible_role: AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }
}

impl fmt::Display for StackPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StackPage")
    }
}
