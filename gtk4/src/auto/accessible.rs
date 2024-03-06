// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "v4_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
use crate::{ATContext, AccessiblePlatformState};
use crate::{AccessibleProperty, AccessibleRelation, AccessibleRole, AccessibleState};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkAccessible")]
    pub struct Accessible(Interface<ffi::GtkAccessible, ffi::GtkAccessibleInterface>);

    match fn {
        type_ => || ffi::gtk_accessible_get_type(),
    }
}

impl Accessible {
    pub const NONE: Option<&'static Accessible> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Accessible>> Sealed for T {}
}

pub trait AccessibleExt: IsA<Accessible> + sealed::Sealed + 'static {
    //#[cfg(feature = "v4_14")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    //#[doc(alias = "gtk_accessible_announce")]
    //fn announce(&self, message: &str, priority: /*Ignored*/AccessibleAnnouncementPriority) {
    //    unsafe { TODO: call ffi:gtk_accessible_announce() }
    //}

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_accessible_get_accessible_parent")]
    #[doc(alias = "get_accessible_parent")]
    #[must_use]
    fn accessible_parent(&self) -> Option<Accessible> {
        unsafe {
            from_glib_full(ffi::gtk_accessible_get_accessible_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_accessible_get_accessible_role")]
    #[doc(alias = "get_accessible_role")]
    fn accessible_role(&self) -> AccessibleRole {
        unsafe {
            from_glib(ffi::gtk_accessible_get_accessible_role(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_accessible_get_at_context")]
    #[doc(alias = "get_at_context")]
    fn at_context(&self) -> ATContext {
        unsafe {
            from_glib_full(ffi::gtk_accessible_get_at_context(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_accessible_get_bounds")]
    #[doc(alias = "get_bounds")]
    fn bounds(&self) -> Option<(i32, i32, i32, i32)> {
        unsafe {
            let mut x = std::mem::MaybeUninit::uninit();
            let mut y = std::mem::MaybeUninit::uninit();
            let mut width = std::mem::MaybeUninit::uninit();
            let mut height = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_accessible_get_bounds(
                self.as_ref().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            ));
            if ret {
                Some((
                    x.assume_init(),
                    y.assume_init(),
                    width.assume_init(),
                    height.assume_init(),
                ))
            } else {
                None
            }
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_accessible_get_first_accessible_child")]
    #[doc(alias = "get_first_accessible_child")]
    #[must_use]
    fn first_accessible_child(&self) -> Option<Accessible> {
        unsafe {
            from_glib_full(ffi::gtk_accessible_get_first_accessible_child(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_accessible_get_next_accessible_sibling")]
    #[doc(alias = "get_next_accessible_sibling")]
    #[must_use]
    fn next_accessible_sibling(&self) -> Option<Accessible> {
        unsafe {
            from_glib_full(ffi::gtk_accessible_get_next_accessible_sibling(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_accessible_get_platform_state")]
    #[doc(alias = "get_platform_state")]
    fn platform_state(&self, state: AccessiblePlatformState) -> bool {
        unsafe {
            from_glib(ffi::gtk_accessible_get_platform_state(
                self.as_ref().to_glib_none().0,
                state.into_glib(),
            ))
        }
    }

    #[doc(alias = "gtk_accessible_reset_property")]
    fn reset_property(&self, property: AccessibleProperty) {
        unsafe {
            ffi::gtk_accessible_reset_property(
                self.as_ref().to_glib_none().0,
                property.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_accessible_reset_relation")]
    fn reset_relation(&self, relation: AccessibleRelation) {
        unsafe {
            ffi::gtk_accessible_reset_relation(
                self.as_ref().to_glib_none().0,
                relation.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_accessible_reset_state")]
    fn reset_state(&self, state: AccessibleState) {
        unsafe {
            ffi::gtk_accessible_reset_state(self.as_ref().to_glib_none().0, state.into_glib());
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_accessible_set_accessible_parent")]
    fn set_accessible_parent(
        &self,
        parent: Option<&impl IsA<Accessible>>,
        next_sibling: Option<&impl IsA<Accessible>>,
    ) {
        unsafe {
            ffi::gtk_accessible_set_accessible_parent(
                self.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                next_sibling.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_accessible_update_next_accessible_sibling")]
    fn update_next_accessible_sibling(&self, new_sibling: Option<&impl IsA<Accessible>>) {
        unsafe {
            ffi::gtk_accessible_update_next_accessible_sibling(
                self.as_ref().to_glib_none().0,
                new_sibling.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "accessible-role")]
    fn set_accessible_role(&self, accessible_role: AccessibleRole) {
        ObjectExt::set_property(self.as_ref(), "accessible-role", accessible_role)
    }

    #[doc(alias = "accessible-role")]
    fn connect_accessible_role_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accessible_role_trampoline<
            P: IsA<Accessible>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkAccessible,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Accessible::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accessible-role\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_accessible_role_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Accessible>> AccessibleExt for O {}
