// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkListItemFactory")]
    pub struct ListItemFactory(Object<ffi::GtkListItemFactory, ffi::GtkListItemFactoryClass>);

    match fn {
        type_ => || ffi::gtk_list_item_factory_get_type(),
    }
}

impl ListItemFactory {}

pub const NONE_LIST_ITEM_FACTORY: Option<&ListItemFactory> = None;

impl fmt::Display for ListItemFactory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ListItemFactory")
    }
}
