// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
use crate::ShortcutsGroup;
use crate::{
    ffi, Accessible, AccessibleRole, Align, BaselinePosition, Box, Buildable, ConstraintTarget,
    LayoutManager, Orientable, Orientation, Overflow, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkShortcutsSection")]
    pub struct ShortcutsSection(Object<ffi::GtkShortcutsSection, ffi::GtkShortcutsSectionClass>) @extends Box, Widget, @implements Accessible, Buildable, ConstraintTarget, Orientable;

    match fn {
        type_ => || ffi::gtk_shortcuts_section_get_type(),
    }
}

impl ShortcutsSection {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ShortcutsSection`] objects.
    ///
    /// This method returns an instance of [`ShortcutsSectionBuilder`](crate::builders::ShortcutsSectionBuilder) which can be used to create [`ShortcutsSection`] objects.
    pub fn builder() -> ShortcutsSectionBuilder {
        ShortcutsSectionBuilder::new()
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[cfg(feature = "v4_14")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[allow(deprecated)]
    #[doc(alias = "gtk_shortcuts_section_add_group")]
    pub fn add_group(&self, group: &ShortcutsGroup) {
        unsafe {
            ffi::gtk_shortcuts_section_add_group(self.to_glib_none().0, group.to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "max-height")]
    pub fn max_height(&self) -> u32 {
        ObjectExt::property(self, "max-height")
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "max-height")]
    pub fn set_max_height(&self, max_height: u32) {
        ObjectExt::set_property(self, "max-height", max_height)
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "section-name")]
    pub fn section_name(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "section-name")
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "section-name")]
    pub fn set_section_name(&self, section_name: Option<&str>) {
        ObjectExt::set_property(self, "section-name", section_name)
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn title(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "title")
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn set_title(&self, title: Option<&str>) {
        ObjectExt::set_property(self, "title", title)
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "view-name")]
    pub fn view_name(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "view-name")
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "view-name")]
    pub fn set_view_name(&self, view_name: Option<&str>) {
        ObjectExt::set_property(self, "view-name", view_name)
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "max-height")]
    pub fn connect_max_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_height_trampoline<F: Fn(&ShortcutsSection) + 'static>(
            this: *mut ffi::GtkShortcutsSection,
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
                c"notify::max-height".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_max_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "section-name")]
    pub fn connect_section_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_section_name_trampoline<F: Fn(&ShortcutsSection) + 'static>(
            this: *mut ffi::GtkShortcutsSection,
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
                c"notify::section-name".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_section_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "title")]
    pub fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&ShortcutsSection) + 'static>(
            this: *mut ffi::GtkShortcutsSection,
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
                c"notify::title".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[doc(alias = "view-name")]
    pub fn connect_view_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_view_name_trampoline<F: Fn(&ShortcutsSection) + 'static>(
            this: *mut ffi::GtkShortcutsSection,
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
                c"notify::view-name".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_view_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ShortcutsSection`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ShortcutsSectionBuilder {
    builder: glib::object::ObjectBuilder<'static, ShortcutsSection>,
}

impl ShortcutsSectionBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn max_height(self, max_height: u32) -> Self {
        Self {
            builder: self.builder.property("max-height", max_height),
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn section_name(self, section_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("section-name", section_name.into()),
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn title(self, title: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("title", title.into()),
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    pub fn view_name(self, view_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("view-name", view_name.into()),
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    pub fn baseline_child(self, baseline_child: i32) -> Self {
        Self {
            builder: self.builder.property("baseline-child", baseline_child),
        }
    }

    pub fn baseline_position(self, baseline_position: BaselinePosition) -> Self {
        Self {
            builder: self
                .builder
                .property("baseline-position", baseline_position),
        }
    }

    pub fn homogeneous(self, homogeneous: bool) -> Self {
        Self {
            builder: self.builder.property("homogeneous", homogeneous),
        }
    }

    pub fn spacing(self, spacing: i32) -> Self {
        Self {
            builder: self.builder.property("spacing", spacing),
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

    pub fn orientation(self, orientation: Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ShortcutsSection`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ShortcutsSection {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
