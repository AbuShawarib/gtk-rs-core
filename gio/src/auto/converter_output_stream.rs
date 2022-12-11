// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Converter, FilterOutputStream, OutputStream, PollableOutputStream};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GConverterOutputStream")]
    pub struct ConverterOutputStream(Object<ffi::GConverterOutputStream, ffi::GConverterOutputStreamClass>) @extends FilterOutputStream, OutputStream, @implements PollableOutputStream;

    match fn {
        type_ => || ffi::g_converter_output_stream_get_type(),
    }
}

impl ConverterOutputStream {
    pub const NONE: Option<&'static ConverterOutputStream> = None;

    #[doc(alias = "g_converter_output_stream_new")]
    pub fn new(
        base_stream: &impl IsA<OutputStream>,
        converter: &impl IsA<Converter>,
    ) -> ConverterOutputStream {
        unsafe {
            OutputStream::from_glib_full(ffi::g_converter_output_stream_new(
                base_stream.as_ref().to_glib_none().0,
                converter.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ConverterOutputStream`] objects.
    ///
    /// This method returns an instance of [`ConverterOutputStreamBuilder`](crate::builders::ConverterOutputStreamBuilder) which can be used to create [`ConverterOutputStream`] objects.
    pub fn builder() -> ConverterOutputStreamBuilder {
        ConverterOutputStreamBuilder::default()
    }
}

impl Default for ConverterOutputStream {
    fn default() -> Self {
        glib::object::Object::new::<Self>(&[])
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ConverterOutputStream`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ConverterOutputStreamBuilder {
    converter: Option<Converter>,
    base_stream: Option<OutputStream>,
    close_base_stream: Option<bool>,
}

impl ConverterOutputStreamBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ConverterOutputStreamBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ConverterOutputStream`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ConverterOutputStream {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref converter) = self.converter {
            properties.push(("converter", converter));
        }
        if let Some(ref base_stream) = self.base_stream {
            properties.push(("base-stream", base_stream));
        }
        if let Some(ref close_base_stream) = self.close_base_stream {
            properties.push(("close-base-stream", close_base_stream));
        }
        glib::Object::new::<ConverterOutputStream>(&properties)
    }

    pub fn converter(mut self, converter: &impl IsA<Converter>) -> Self {
        self.converter = Some(converter.clone().upcast());
        self
    }

    pub fn base_stream(mut self, base_stream: &impl IsA<OutputStream>) -> Self {
        self.base_stream = Some(base_stream.clone().upcast());
        self
    }

    pub fn close_base_stream(mut self, close_base_stream: bool) -> Self {
        self.close_base_stream = Some(close_base_stream);
        self
    }
}

pub trait ConverterOutputStreamExt: 'static {
    #[doc(alias = "g_converter_output_stream_get_converter")]
    #[doc(alias = "get_converter")]
    fn converter(&self) -> Converter;
}

impl<O: IsA<ConverterOutputStream>> ConverterOutputStreamExt for O {
    fn converter(&self) -> Converter {
        unsafe {
            from_glib_none(ffi::g_converter_output_stream_get_converter(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for ConverterOutputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ConverterOutputStream")
    }
}
