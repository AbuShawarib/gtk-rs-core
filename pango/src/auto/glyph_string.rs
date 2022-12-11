// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Analysis, Font, Rectangle};
use glib::{prelude::*, translate::*};
use std::mem;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct GlyphString(Boxed<ffi::PangoGlyphString>);

    match fn {
        copy => |ptr| ffi::pango_glyph_string_copy(mut_override(ptr)),
        free => |ptr| ffi::pango_glyph_string_free(ptr),
        type_ => || ffi::pango_glyph_string_get_type(),
    }
}

impl GlyphString {
    #[doc(alias = "pango_glyph_string_new")]
    pub fn new() -> GlyphString {
        unsafe { from_glib_full(ffi::pango_glyph_string_new()) }
    }

    #[doc(alias = "pango_glyph_string_extents")]
    pub fn extents(&mut self, font: &impl IsA<Font>) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_glyph_string_extents(
                self.to_glib_none_mut().0,
                font.as_ref().to_glib_none().0,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    #[doc(alias = "pango_glyph_string_extents_range")]
    pub fn extents_range(
        &mut self,
        start: i32,
        end: i32,
        font: &impl IsA<Font>,
    ) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_glyph_string_extents_range(
                self.to_glib_none_mut().0,
                start,
                end,
                font.as_ref().to_glib_none().0,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    #[doc(alias = "pango_glyph_string_get_width")]
    #[doc(alias = "get_width")]
    pub fn width(&self) -> i32 {
        unsafe { ffi::pango_glyph_string_get_width(mut_override(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_glyph_string_index_to_x")]
    pub fn index_to_x(
        &self,
        text: &str,
        analysis: &mut Analysis,
        index_: i32,
        trailing: bool,
    ) -> i32 {
        let length = text.len() as _;
        unsafe {
            let mut x_pos = mem::MaybeUninit::uninit();
            ffi::pango_glyph_string_index_to_x(
                mut_override(self.to_glib_none().0),
                text.to_glib_none().0,
                length,
                analysis.to_glib_none_mut().0,
                index_,
                trailing.into_glib(),
                x_pos.as_mut_ptr(),
            );
            x_pos.assume_init()
        }
    }

    //#[cfg(any(feature = "v1_50", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
    //#[doc(alias = "pango_glyph_string_index_to_x_full")]
    //pub fn index_to_x_full(&mut self, text: &str, analysis: &mut Analysis, attrs: /*Ignored*/Option<&mut LogAttr>, index_: i32, trailing: bool) -> i32 {
    //    unsafe { TODO: call ffi:pango_glyph_string_index_to_x_full() }
    //}

    #[doc(alias = "pango_glyph_string_set_size")]
    pub fn set_size(&mut self, new_len: i32) {
        unsafe {
            ffi::pango_glyph_string_set_size(self.to_glib_none_mut().0, new_len);
        }
    }

    #[doc(alias = "pango_glyph_string_x_to_index")]
    pub fn x_to_index(&self, text: &str, analysis: &mut Analysis, x_pos: i32) -> (i32, i32) {
        let length = text.len() as _;
        unsafe {
            let mut index_ = mem::MaybeUninit::uninit();
            let mut trailing = mem::MaybeUninit::uninit();
            ffi::pango_glyph_string_x_to_index(
                mut_override(self.to_glib_none().0),
                text.to_glib_none().0,
                length,
                analysis.to_glib_none_mut().0,
                x_pos,
                index_.as_mut_ptr(),
                trailing.as_mut_ptr(),
            );
            (index_.assume_init(), trailing.assume_init())
        }
    }
}

impl Default for GlyphString {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for GlyphString {}
unsafe impl Sync for GlyphString {}
