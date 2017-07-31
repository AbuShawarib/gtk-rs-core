// This file was generated by gir (f00d658) from gir-files (0bcaef9)
// DO NOT EDIT

use Coverage;
use EngineShape;
use FontDescription;
use FontMap;
use FontMetrics;
use Glyph;
use Language;
use Rectangle;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Font(Object<ffi::PangoFont>);

    match fn {
        get_type => || ffi::pango_font_get_type(),
    }
}

pub trait FontExt {
    fn describe(&self) -> Option<FontDescription>;

    fn describe_with_absolute_size(&self) -> Option<FontDescription>;

    fn find_shaper(&self, language: &Language, ch: u32) -> Option<EngineShape>;

    fn get_coverage(&self, language: &Language) -> Option<Coverage>;

    fn get_font_map(&self) -> Option<FontMap>;

    fn get_glyph_extents(&self, glyph: Glyph) -> (Rectangle, Rectangle);

    fn get_metrics<'a, P: Into<Option<&'a Language>>>(&self, language: P) -> Option<FontMetrics>;
}

impl<O: IsA<Font>> FontExt for O {
    fn describe(&self) -> Option<FontDescription> {
        unsafe {
            from_glib_full(ffi::pango_font_describe(self.to_glib_none().0))
        }
    }

    fn describe_with_absolute_size(&self) -> Option<FontDescription> {
        unsafe {
            from_glib_full(ffi::pango_font_describe_with_absolute_size(self.to_glib_none().0))
        }
    }

    fn find_shaper(&self, language: &Language, ch: u32) -> Option<EngineShape> {
        unsafe {
            from_glib_none(ffi::pango_font_find_shaper(self.to_glib_none().0, mut_override(language.to_glib_none().0), ch))
        }
    }

    fn get_coverage(&self, language: &Language) -> Option<Coverage> {
        unsafe {
            from_glib_full(ffi::pango_font_get_coverage(self.to_glib_none().0, mut_override(language.to_glib_none().0)))
        }
    }

    fn get_font_map(&self) -> Option<FontMap> {
        unsafe {
            from_glib_none(ffi::pango_font_get_font_map(self.to_glib_none().0))
        }
    }

    fn get_glyph_extents(&self, glyph: Glyph) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_font_get_glyph_extents(self.to_glib_none().0, glyph, ink_rect.to_glib_none_mut().0, logical_rect.to_glib_none_mut().0);
            (ink_rect, logical_rect)
        }
    }

    fn get_metrics<'a, P: Into<Option<&'a Language>>>(&self, language: P) -> Option<FontMetrics> {
        let language = language.into();
        unsafe {
            from_glib_full(ffi::pango_font_get_metrics(self.to_glib_none().0, mut_override(language.to_glib_none().0)))
        }
    }
}
