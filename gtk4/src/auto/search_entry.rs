// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    ffi, Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, Editable, LayoutManager,
    Overflow, Widget,
};
#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
use crate::{InputHints, InputPurpose};
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkSearchEntry")]
    pub struct SearchEntry(Object<ffi::GtkSearchEntry>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, Editable;

    match fn {
        type_ => || ffi::gtk_search_entry_get_type(),
    }
}

impl SearchEntry {
    #[doc(alias = "gtk_search_entry_new")]
    pub fn new() -> SearchEntry {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_search_entry_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`SearchEntry`] objects.
    ///
    /// This method returns an instance of [`SearchEntryBuilder`](crate::builders::SearchEntryBuilder) which can be used to create [`SearchEntry`] objects.
    pub fn builder() -> SearchEntryBuilder {
        SearchEntryBuilder::new()
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "gtk_search_entry_get_input_hints")]
    #[doc(alias = "get_input_hints")]
    #[doc(alias = "input-hints")]
    pub fn input_hints(&self) -> InputHints {
        unsafe { from_glib(ffi::gtk_search_entry_get_input_hints(self.to_glib_none().0)) }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "gtk_search_entry_get_input_purpose")]
    #[doc(alias = "get_input_purpose")]
    #[doc(alias = "input-purpose")]
    pub fn input_purpose(&self) -> InputPurpose {
        unsafe {
            from_glib(ffi::gtk_search_entry_get_input_purpose(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_search_entry_get_key_capture_widget")]
    #[doc(alias = "get_key_capture_widget")]
    pub fn key_capture_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_search_entry_get_key_capture_widget(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_8")))]
    #[doc(alias = "gtk_search_entry_get_search_delay")]
    #[doc(alias = "get_search_delay")]
    #[doc(alias = "search-delay")]
    pub fn search_delay(&self) -> u32 {
        unsafe { ffi::gtk_search_entry_get_search_delay(self.to_glib_none().0) }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "gtk_search_entry_set_input_hints")]
    #[doc(alias = "input-hints")]
    pub fn set_input_hints(&self, hints: InputHints) {
        unsafe {
            ffi::gtk_search_entry_set_input_hints(self.to_glib_none().0, hints.into_glib());
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "gtk_search_entry_set_input_purpose")]
    #[doc(alias = "input-purpose")]
    pub fn set_input_purpose(&self, purpose: InputPurpose) {
        unsafe {
            ffi::gtk_search_entry_set_input_purpose(self.to_glib_none().0, purpose.into_glib());
        }
    }

    #[doc(alias = "gtk_search_entry_set_key_capture_widget")]
    pub fn set_key_capture_widget(&self, widget: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_search_entry_set_key_capture_widget(
                self.to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v4_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_8")))]
    #[doc(alias = "gtk_search_entry_set_search_delay")]
    #[doc(alias = "search-delay")]
    pub fn set_search_delay(&self, delay: u32) {
        unsafe {
            ffi::gtk_search_entry_set_search_delay(self.to_glib_none().0, delay);
        }
    }

    #[doc(alias = "activates-default")]
    pub fn activates_default(&self) -> bool {
        ObjectExt::property(self, "activates-default")
    }

    #[doc(alias = "activates-default")]
    pub fn set_activates_default(&self, activates_default: bool) {
        ObjectExt::set_property(self, "activates-default", activates_default)
    }

    #[doc(alias = "placeholder-text")]
    pub fn placeholder_text(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "placeholder-text")
    }

    #[doc(alias = "placeholder-text")]
    pub fn set_placeholder_text(&self, placeholder_text: Option<&str>) {
        ObjectExt::set_property(self, "placeholder-text", placeholder_text)
    }

    #[doc(alias = "activate")]
    pub fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<F: Fn(&SearchEntry) + 'static>(
            this: *mut ffi::GtkSearchEntry,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"activate".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    activate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_activate(&self) {
        self.emit_by_name::<()>("activate", &[]);
    }

    #[doc(alias = "next-match")]
    pub fn connect_next_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn next_match_trampoline<F: Fn(&SearchEntry) + 'static>(
            this: *mut ffi::GtkSearchEntry,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"next-match".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    next_match_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_next_match(&self) {
        self.emit_by_name::<()>("next-match", &[]);
    }

    #[doc(alias = "previous-match")]
    pub fn connect_previous_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn previous_match_trampoline<F: Fn(&SearchEntry) + 'static>(
            this: *mut ffi::GtkSearchEntry,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"previous-match".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    previous_match_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_previous_match(&self) {
        self.emit_by_name::<()>("previous-match", &[]);
    }

    #[doc(alias = "search-changed")]
    pub fn connect_search_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn search_changed_trampoline<F: Fn(&SearchEntry) + 'static>(
            this: *mut ffi::GtkSearchEntry,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"search-changed".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    search_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "search-started")]
    pub fn connect_search_started<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn search_started_trampoline<F: Fn(&SearchEntry) + 'static>(
            this: *mut ffi::GtkSearchEntry,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"search-started".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    search_started_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "stop-search")]
    pub fn connect_stop_search<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn stop_search_trampoline<F: Fn(&SearchEntry) + 'static>(
            this: *mut ffi::GtkSearchEntry,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"stop-search".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    stop_search_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_stop_search(&self) {
        self.emit_by_name::<()>("stop-search", &[]);
    }

    #[doc(alias = "activates-default")]
    pub fn connect_activates_default_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_activates_default_trampoline<F: Fn(&SearchEntry) + 'static>(
            this: *mut ffi::GtkSearchEntry,
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
                c"notify::activates-default".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_activates_default_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "input-hints")]
    pub fn connect_input_hints_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_input_hints_trampoline<F: Fn(&SearchEntry) + 'static>(
            this: *mut ffi::GtkSearchEntry,
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
                c"notify::input-hints".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_input_hints_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[doc(alias = "input-purpose")]
    pub fn connect_input_purpose_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_input_purpose_trampoline<F: Fn(&SearchEntry) + 'static>(
            this: *mut ffi::GtkSearchEntry,
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
                c"notify::input-purpose".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_input_purpose_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "placeholder-text")]
    pub fn connect_placeholder_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_placeholder_text_trampoline<F: Fn(&SearchEntry) + 'static>(
            this: *mut ffi::GtkSearchEntry,
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
                c"notify::placeholder-text".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_placeholder_text_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_8")))]
    #[doc(alias = "search-delay")]
    pub fn connect_search_delay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_delay_trampoline<F: Fn(&SearchEntry) + 'static>(
            this: *mut ffi::GtkSearchEntry,
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
                c"notify::search-delay".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_search_delay_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for SearchEntry {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`SearchEntry`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct SearchEntryBuilder {
    builder: glib::object::ObjectBuilder<'static, SearchEntry>,
}

impl SearchEntryBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn activates_default(self, activates_default: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("activates-default", activates_default),
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    pub fn input_hints(self, input_hints: InputHints) -> Self {
        Self {
            builder: self.builder.property("input-hints", input_hints),
        }
    }

    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    pub fn input_purpose(self, input_purpose: InputPurpose) -> Self {
        Self {
            builder: self.builder.property("input-purpose", input_purpose),
        }
    }

    pub fn placeholder_text(self, placeholder_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("placeholder-text", placeholder_text.into()),
        }
    }

    #[cfg(feature = "v4_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_8")))]
    pub fn search_delay(self, search_delay: u32) -> Self {
        Self {
            builder: self.builder.property("search-delay", search_delay),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    pub fn editable(self, editable: bool) -> Self {
        Self {
            builder: self.builder.property("editable", editable),
        }
    }

    pub fn enable_undo(self, enable_undo: bool) -> Self {
        Self {
            builder: self.builder.property("enable-undo", enable_undo),
        }
    }

    pub fn max_width_chars(self, max_width_chars: i32) -> Self {
        Self {
            builder: self.builder.property("max-width-chars", max_width_chars),
        }
    }

    pub fn text(self, text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("text", text.into()),
        }
    }

    pub fn width_chars(self, width_chars: i32) -> Self {
        Self {
            builder: self.builder.property("width-chars", width_chars),
        }
    }

    pub fn xalign(self, xalign: f32) -> Self {
        Self {
            builder: self.builder.property("xalign", xalign),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`SearchEntry`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> SearchEntry {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
