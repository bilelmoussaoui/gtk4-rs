// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RenderNode;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct TextNode(Object<ffi::GskTextNode>) @extends RenderNode;

    match fn {
        get_type => || ffi::gsk_text_node_get_type(),
    }
}

impl TextNode {
    pub fn new<P: IsA<pango::Font>>(
        font: &P,
        glyphs: &mut pango::GlyphString,
        color: &gdk::RGBA,
        offset: &graphene::Point,
    ) -> Option<TextNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gsk_text_node_new(
                font.as_ref().to_glib_none().0,
                glyphs.to_glib_none_mut().0,
                color.to_glib_none().0,
                offset.to_glib_none().0,
            ))
        }
    }

    pub fn get_num_glyphs(&self) -> u32 {
        unsafe { ffi::gsk_text_node_get_num_glyphs(self.to_glib_none().0) }
    }

    pub fn get_offset(&self) -> Option<graphene::Point> {
        unsafe { from_glib_none(ffi::gsk_text_node_get_offset(self.to_glib_none().0)) }
    }

    pub fn has_color_glyphs(&self) -> bool {
        unsafe { from_glib(ffi::gsk_text_node_has_color_glyphs(self.to_glib_none().0)) }
    }

    pub fn peek_color(&self) -> Option<gdk::RGBA> {
        unsafe { from_glib_none(ffi::gsk_text_node_peek_color(self.to_glib_none().0)) }
    }

    pub fn peek_font(&self) -> Option<pango::Font> {
        unsafe { from_glib_none(ffi::gsk_text_node_peek_font(self.to_glib_none().0)) }
    }

    //pub fn peek_glyphs(&self) -> /*Ignored*/Vec<pango::GlyphInfo> {
    //    unsafe { TODO: call ffi:gsk_text_node_peek_glyphs() }
    //}
}

impl fmt::Display for TextNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextNode")
    }
}
