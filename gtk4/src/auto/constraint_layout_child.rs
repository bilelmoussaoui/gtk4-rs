// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::LayoutChild;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct ConstraintLayoutChild(Object<ffi::GtkConstraintLayoutChild, ffi::GtkConstraintLayoutChildClass>) @extends LayoutChild;

    match fn {
        get_type => || ffi::gtk_constraint_layout_child_get_type(),
    }
}

impl ConstraintLayoutChild {}

pub const NONE_CONSTRAINT_LAYOUT_CHILD: Option<&ConstraintLayoutChild> = None;

impl fmt::Display for ConstraintLayoutChild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ConstraintLayoutChild")
    }
}
