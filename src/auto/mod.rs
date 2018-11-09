// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

mod action;
pub use self::action::Action;
pub use self::action::ActionExt;

mod component;
pub use self::component::Component;
pub use self::component::ComponentExt;

mod document;
pub use self::document::Document;
pub use self::document::DocumentExt;

mod editable_text;
pub use self::editable_text::EditableText;
pub use self::editable_text::EditableTextExt;

mod gobject_accessible;
pub use self::gobject_accessible::GObjectAccessible;
pub use self::gobject_accessible::GObjectAccessibleExt;

mod hyperlink;
pub use self::hyperlink::Hyperlink;
pub use self::hyperlink::HyperlinkExt;

mod hyperlink_impl;
pub use self::hyperlink_impl::HyperlinkImpl;
pub use self::hyperlink_impl::HyperlinkImplExt;

mod hypertext;
pub use self::hypertext::Hypertext;
pub use self::hypertext::HypertextExt;

mod image;
pub use self::image::Image;
pub use self::image::ImageExt;

mod misc;
pub use self::misc::Misc;
pub use self::misc::MiscExt;

mod no_op_object_factory;
pub use self::no_op_object_factory::NoOpObjectFactory;

mod object;
pub use self::object::Object;
pub use self::object::AtkObjectExt;

mod object_factory;
pub use self::object_factory::ObjectFactory;
pub use self::object_factory::ObjectFactoryExt;

mod plug;
pub use self::plug::Plug;
pub use self::plug::PlugExt;

mod registry;
pub use self::registry::Registry;
pub use self::registry::RegistryExt;

mod relation;
pub use self::relation::Relation;
pub use self::relation::RelationExt;

mod relation_set;
pub use self::relation_set::RelationSet;
pub use self::relation_set::RelationSetExt;

mod selection;
pub use self::selection::Selection;
pub use self::selection::SelectionExt;

mod socket;
pub use self::socket::Socket;
pub use self::socket::SocketExt;

mod state_set;
pub use self::state_set::StateSet;
pub use self::state_set::StateSetExt;

mod streamable_content;
pub use self::streamable_content::StreamableContent;
pub use self::streamable_content::StreamableContentExt;

mod table;
pub use self::table::Table;
pub use self::table::TableExt;

#[cfg(any(feature = "v2_12", feature = "dox"))]
mod table_cell;
#[cfg(any(feature = "v2_12", feature = "dox"))]
pub use self::table_cell::TableCell;
#[cfg(any(feature = "v2_12", feature = "dox"))]
pub use self::table_cell::TableCellExt;

mod text;
pub use self::text::Text;
pub use self::text::TextExt;

mod util;
pub use self::util::Util;

mod value;
pub use self::value::Value;
pub use self::value::ValueExt;

mod window;
pub use self::window::Window;
pub use self::window::WindowExt;

#[cfg(any(feature = "v2_12", feature = "dox"))]
mod range;
#[cfg(any(feature = "v2_12", feature = "dox"))]
pub use self::range::Range;

mod rectangle;
pub use self::rectangle::Rectangle;

mod text_range;
pub use self::text_range::TextRange;

mod enums;
pub use self::enums::CoordType;
pub use self::enums::Layer;
pub use self::enums::RelationType;
pub use self::enums::Role;
pub use self::enums::StateType;
pub use self::enums::TextAttribute;
pub use self::enums::TextBoundary;
pub use self::enums::TextClipType;
pub use self::enums::TextGranularity;
pub use self::enums::ValueType;

mod flags;
pub use self::flags::HyperlinkStateFlags;

mod alias;
pub use self::alias::State;

#[doc(hidden)]
pub mod traits {
    pub use super::ActionExt;
    pub use super::ComponentExt;
    pub use super::DocumentExt;
    pub use super::EditableTextExt;
    pub use super::GObjectAccessibleExt;
    pub use super::HyperlinkExt;
    pub use super::HyperlinkImplExt;
    pub use super::HypertextExt;
    pub use super::ImageExt;
    pub use super::MiscExt;
    pub use super::AtkObjectExt;
    pub use super::ObjectFactoryExt;
    pub use super::PlugExt;
    pub use super::RegistryExt;
    pub use super::RelationExt;
    pub use super::RelationSetExt;
    pub use super::SelectionExt;
    pub use super::SocketExt;
    pub use super::StateSetExt;
    pub use super::StreamableContentExt;
    pub use super::TableExt;
    #[cfg(any(feature = "v2_12", feature = "dox"))]
    pub use super::TableCellExt;
    pub use super::TextExt;
    pub use super::ValueExt;
    pub use super::WindowExt;
}
