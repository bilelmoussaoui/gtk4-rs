// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Align;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::LayoutManager;
use crate::Overflow;
use crate::SelectionModel;
use crate::StackPage;
use crate::StackTransitionType;
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

glib::wrapper! {
    #[doc(alias = "GtkStack")]
    pub struct Stack(Object<ffi::GtkStack>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_stack_get_type(),
    }
}

impl Stack {
    #[doc(alias = "gtk_stack_new")]
    pub fn new() -> Stack {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_stack_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`Stack`].
    ///
    /// This method returns an instance of [`StackBuilder`] which can be used to create a [`Stack`].
    pub fn builder() -> StackBuilder {
        StackBuilder::default()
    }

    #[doc(alias = "gtk_stack_add_child")]
    pub fn add_child<P: IsA<Widget>>(&self, child: &P) -> StackPage {
        unsafe {
            from_glib_none(ffi::gtk_stack_add_child(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_stack_add_named")]
    pub fn add_named<P: IsA<Widget>>(&self, child: &P, name: Option<&str>) -> StackPage {
        unsafe {
            from_glib_none(ffi::gtk_stack_add_named(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_stack_add_titled")]
    pub fn add_titled<P: IsA<Widget>>(
        &self,
        child: &P,
        name: Option<&str>,
        title: &str,
    ) -> StackPage {
        unsafe {
            from_glib_none(ffi::gtk_stack_add_titled(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                title.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_stack_get_child_by_name")]
    #[doc(alias = "get_child_by_name")]
    pub fn child_by_name(&self, name: &str) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_stack_get_child_by_name(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_stack_get_hhomogeneous")]
    #[doc(alias = "get_hhomogeneous")]
    pub fn is_hhomogeneous(&self) -> bool {
        unsafe { from_glib(ffi::gtk_stack_get_hhomogeneous(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_get_interpolate_size")]
    #[doc(alias = "get_interpolate_size")]
    pub fn interpolates_size(&self) -> bool {
        unsafe { from_glib(ffi::gtk_stack_get_interpolate_size(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_get_page")]
    #[doc(alias = "get_page")]
    pub fn page<P: IsA<Widget>>(&self, child: &P) -> Option<StackPage> {
        unsafe {
            from_glib_none(ffi::gtk_stack_get_page(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_stack_get_pages")]
    #[doc(alias = "get_pages")]
    pub fn pages(&self) -> SelectionModel {
        unsafe { from_glib_full(ffi::gtk_stack_get_pages(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_get_transition_duration")]
    #[doc(alias = "get_transition_duration")]
    pub fn transition_duration(&self) -> u32 {
        unsafe { ffi::gtk_stack_get_transition_duration(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_stack_get_transition_running")]
    #[doc(alias = "get_transition_running")]
    pub fn is_transition_running(&self) -> bool {
        unsafe { from_glib(ffi::gtk_stack_get_transition_running(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_get_transition_type")]
    #[doc(alias = "get_transition_type")]
    pub fn transition_type(&self) -> StackTransitionType {
        unsafe { from_glib(ffi::gtk_stack_get_transition_type(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_get_vhomogeneous")]
    #[doc(alias = "get_vhomogeneous")]
    pub fn is_vhomogeneous(&self) -> bool {
        unsafe { from_glib(ffi::gtk_stack_get_vhomogeneous(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_get_visible_child")]
    #[doc(alias = "get_visible_child")]
    pub fn visible_child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_stack_get_visible_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_get_visible_child_name")]
    #[doc(alias = "get_visible_child_name")]
    pub fn visible_child_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_stack_get_visible_child_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_remove")]
    pub fn remove<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_stack_remove(self.to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_stack_set_hhomogeneous")]
    pub fn set_hhomogeneous(&self, hhomogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_hhomogeneous(self.to_glib_none().0, hhomogeneous.into_glib());
        }
    }

    #[doc(alias = "gtk_stack_set_interpolate_size")]
    pub fn set_interpolate_size(&self, interpolate_size: bool) {
        unsafe {
            ffi::gtk_stack_set_interpolate_size(
                self.to_glib_none().0,
                interpolate_size.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_stack_set_transition_duration")]
    pub fn set_transition_duration(&self, duration: u32) {
        unsafe {
            ffi::gtk_stack_set_transition_duration(self.to_glib_none().0, duration);
        }
    }

    #[doc(alias = "gtk_stack_set_transition_type")]
    pub fn set_transition_type(&self, transition: StackTransitionType) {
        unsafe {
            ffi::gtk_stack_set_transition_type(self.to_glib_none().0, transition.into_glib());
        }
    }

    #[doc(alias = "gtk_stack_set_vhomogeneous")]
    pub fn set_vhomogeneous(&self, vhomogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_vhomogeneous(self.to_glib_none().0, vhomogeneous.into_glib());
        }
    }

    #[doc(alias = "gtk_stack_set_visible_child")]
    pub fn set_visible_child<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_stack_set_visible_child(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_stack_set_visible_child_full")]
    pub fn set_visible_child_full(&self, name: &str, transition: StackTransitionType) {
        unsafe {
            ffi::gtk_stack_set_visible_child_full(
                self.to_glib_none().0,
                name.to_glib_none().0,
                transition.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_stack_set_visible_child_name")]
    pub fn set_visible_child_name(&self, name: &str) {
        unsafe {
            ffi::gtk_stack_set_visible_child_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[doc(alias = "hhomogeneous")]
    pub fn connect_hhomogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_hhomogeneous_trampoline<F: Fn(&Stack) + 'static>(
            this: *mut ffi::GtkStack,
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
                b"notify::hhomogeneous\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_hhomogeneous_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "interpolate-size")]
    pub fn connect_interpolate_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_interpolate_size_trampoline<F: Fn(&Stack) + 'static>(
            this: *mut ffi::GtkStack,
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
                b"notify::interpolate-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_interpolate_size_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pages")]
    pub fn connect_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pages_trampoline<F: Fn(&Stack) + 'static>(
            this: *mut ffi::GtkStack,
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
                b"notify::pages\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pages_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "transition-duration")]
    pub fn connect_transition_duration_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_duration_trampoline<F: Fn(&Stack) + 'static>(
            this: *mut ffi::GtkStack,
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
                b"notify::transition-duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_transition_duration_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "transition-running")]
    pub fn connect_transition_running_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_running_trampoline<F: Fn(&Stack) + 'static>(
            this: *mut ffi::GtkStack,
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
                b"notify::transition-running\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_transition_running_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "transition-type")]
    pub fn connect_transition_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_type_trampoline<F: Fn(&Stack) + 'static>(
            this: *mut ffi::GtkStack,
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
                b"notify::transition-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_transition_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "vhomogeneous")]
    pub fn connect_vhomogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vhomogeneous_trampoline<F: Fn(&Stack) + 'static>(
            this: *mut ffi::GtkStack,
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
                b"notify::vhomogeneous\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_vhomogeneous_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "visible-child")]
    pub fn connect_visible_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_child_trampoline<F: Fn(&Stack) + 'static>(
            this: *mut ffi::GtkStack,
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
                b"notify::visible-child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "visible-child-name")]
    pub fn connect_visible_child_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_child_name_trampoline<F: Fn(&Stack) + 'static>(
            this: *mut ffi::GtkStack,
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
                b"notify::visible-child-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_child_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`Stack`].
pub struct StackBuilder {
    hhomogeneous: Option<bool>,
    interpolate_size: Option<bool>,
    transition_duration: Option<u32>,
    transition_type: Option<StackTransitionType>,
    vhomogeneous: Option<bool>,
    visible_child: Option<Widget>,
    visible_child_name: Option<String>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    halign: Option<Align>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    accessible_role: Option<AccessibleRole>,
}

impl StackBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`StackBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Stack`].
    pub fn build(self) -> Stack {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref hhomogeneous) = self.hhomogeneous {
            properties.push(("hhomogeneous", hhomogeneous));
        }
        if let Some(ref interpolate_size) = self.interpolate_size {
            properties.push(("interpolate-size", interpolate_size));
        }
        if let Some(ref transition_duration) = self.transition_duration {
            properties.push(("transition-duration", transition_duration));
        }
        if let Some(ref transition_type) = self.transition_type {
            properties.push(("transition-type", transition_type));
        }
        if let Some(ref vhomogeneous) = self.vhomogeneous {
            properties.push(("vhomogeneous", vhomogeneous));
        }
        if let Some(ref visible_child) = self.visible_child {
            properties.push(("visible-child", visible_child));
        }
        if let Some(ref visible_child_name) = self.visible_child_name {
            properties.push(("visible-child-name", visible_child_name));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref accessible_role) = self.accessible_role {
            properties.push(("accessible-role", accessible_role));
        }
        glib::Object::new::<Stack>(&properties).expect("Failed to create an instance of Stack")
    }

    pub fn hhomogeneous(mut self, hhomogeneous: bool) -> Self {
        self.hhomogeneous = Some(hhomogeneous);
        self
    }

    pub fn interpolate_size(mut self, interpolate_size: bool) -> Self {
        self.interpolate_size = Some(interpolate_size);
        self
    }

    pub fn transition_duration(mut self, transition_duration: u32) -> Self {
        self.transition_duration = Some(transition_duration);
        self
    }

    pub fn transition_type(mut self, transition_type: StackTransitionType) -> Self {
        self.transition_type = Some(transition_type);
        self
    }

    pub fn vhomogeneous(mut self, vhomogeneous: bool) -> Self {
        self.vhomogeneous = Some(vhomogeneous);
        self
    }

    pub fn visible_child<P: IsA<Widget>>(mut self, visible_child: &P) -> Self {
        self.visible_child = Some(visible_child.clone().upcast());
        self
    }

    pub fn visible_child_name(mut self, visible_child_name: &str) -> Self {
        self.visible_child_name = Some(visible_child_name.to_string());
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn layout_manager<P: IsA<LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn accessible_role(mut self, accessible_role: AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Stack")
    }
}
