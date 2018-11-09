// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Layer;
use RelationSet;
use RelationType;
use Role;
use State;
use StateSet;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Object(Object<ffi::AtkObject, ffi::AtkObjectClass>);

    match fn {
        get_type => || ffi::atk_object_get_type(),
    }
}

pub trait AtkObjectExt {
    fn add_relationship<P: IsA<Object>>(&self, relationship: RelationType, target: &P) -> bool;

    //fn connect_property_change_handler(&self, handler: /*Unknown conversion*//*Unimplemented*/PropertyChangeHandler) -> u32;

    //fn get_attributes(&self) -> /*Ignored*/Option<AttributeSet>;

    fn get_description(&self) -> Option<String>;

    fn get_index_in_parent(&self) -> i32;

    fn get_layer(&self) -> Layer;

    fn get_mdi_zorder(&self) -> i32;

    fn get_n_accessible_children(&self) -> i32;

    fn get_name(&self) -> Option<String>;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn get_object_locale(&self) -> Option<String>;

    fn get_parent(&self) -> Option<Object>;

    fn get_role(&self) -> Role;

    //fn initialize<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, data: P);

    fn notify_state_change(&self, state: State, value: bool);

    fn peek_parent(&self) -> Option<Object>;

    fn ref_accessible_child(&self, i: i32) -> Option<Object>;

    fn ref_relation_set(&self) -> Option<RelationSet>;

    fn ref_state_set(&self) -> Option<StateSet>;

    fn remove_property_change_handler(&self, handler_id: u32);

    fn remove_relationship<P: IsA<Object>>(&self, relationship: RelationType, target: &P) -> bool;

    fn set_description(&self, description: &str);

    fn set_name(&self, name: &str);

    fn set_parent<P: IsA<Object>>(&self, parent: &P);

    fn set_role(&self, role: Role);

    fn get_property_accessible_component_layer(&self) -> i32;

    fn get_property_accessible_component_mdi_zorder(&self) -> i32;

    fn get_property_accessible_description(&self) -> Option<String>;

    fn set_property_accessible_description<'a, P: Into<Option<&'a str>>>(&self, accessible_description: P);

    fn get_property_accessible_hypertext_nlinks(&self) -> i32;

    fn get_property_accessible_name(&self) -> Option<String>;

    fn set_property_accessible_name<'a, P: Into<Option<&'a str>>>(&self, accessible_name: P);

    fn get_property_accessible_parent(&self) -> Option<Object>;

    fn set_property_accessible_parent<P: IsA<Object> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, accessible_parent: Option<&P>);

    fn get_property_accessible_role(&self) -> i32;

    fn set_property_accessible_role(&self, accessible_role: i32);

    fn get_property_accessible_table_caption(&self) -> Option<String>;

    fn set_property_accessible_table_caption<'a, P: Into<Option<&'a str>>>(&self, accessible_table_caption: P);

    fn get_property_accessible_table_caption_object(&self) -> Option<Object>;

    fn set_property_accessible_table_caption_object<P: IsA<Object> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, accessible_table_caption_object: Option<&P>);

    fn get_property_accessible_table_column_description(&self) -> Option<String>;

    fn set_property_accessible_table_column_description<'a, P: Into<Option<&'a str>>>(&self, accessible_table_column_description: P);

    fn get_property_accessible_table_column_header(&self) -> Option<Object>;

    fn set_property_accessible_table_column_header<P: IsA<Object> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, accessible_table_column_header: Option<&P>);

    fn get_property_accessible_table_row_description(&self) -> Option<String>;

    fn set_property_accessible_table_row_description<'a, P: Into<Option<&'a str>>>(&self, accessible_table_row_description: P);

    fn get_property_accessible_table_row_header(&self) -> Option<Object>;

    fn set_property_accessible_table_row_header<P: IsA<Object> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, accessible_table_row_header: Option<&P>);

    fn get_property_accessible_table_summary(&self) -> Option<Object>;

    fn set_property_accessible_table_summary<P: IsA<Object> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, accessible_table_summary: Option<&P>);

    fn get_property_accessible_value(&self) -> f64;

    fn set_property_accessible_value(&self, accessible_value: f64);

    //fn connect_active_descendant_changed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    //fn connect_children_changed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v2_9_4", deprecated)]
    fn connect_focus_event<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId;

    //fn connect_property_change<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_state_change<F: Fn(&Self, &str, bool) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_visible_data_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accessible_component_layer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accessible_component_mdi_zorder_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accessible_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accessible_hypertext_nlinks_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accessible_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accessible_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accessible_role_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accessible_table_caption_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accessible_table_caption_object_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accessible_table_column_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accessible_table_column_header_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accessible_table_row_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accessible_table_row_header_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accessible_table_summary_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accessible_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Object> + IsA<glib::object::Object>> AtkObjectExt for O {
    fn add_relationship<P: IsA<Object>>(&self, relationship: RelationType, target: &P) -> bool {
        unsafe {
            from_glib(ffi::atk_object_add_relationship(self.to_glib_none().0, relationship.to_glib(), target.to_glib_none().0))
        }
    }

    //fn connect_property_change_handler(&self, handler: /*Unknown conversion*//*Unimplemented*/PropertyChangeHandler) -> u32 {
    //    unsafe { TODO: call ffi::atk_object_connect_property_change_handler() }
    //}

    //fn get_attributes(&self) -> /*Ignored*/Option<AttributeSet> {
    //    unsafe { TODO: call ffi::atk_object_get_attributes() }
    //}

    fn get_description(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::atk_object_get_description(self.to_glib_none().0))
        }
    }

    fn get_index_in_parent(&self) -> i32 {
        unsafe {
            ffi::atk_object_get_index_in_parent(self.to_glib_none().0)
        }
    }

    fn get_layer(&self) -> Layer {
        unsafe {
            from_glib(ffi::atk_object_get_layer(self.to_glib_none().0))
        }
    }

    fn get_mdi_zorder(&self) -> i32 {
        unsafe {
            ffi::atk_object_get_mdi_zorder(self.to_glib_none().0)
        }
    }

    fn get_n_accessible_children(&self) -> i32 {
        unsafe {
            ffi::atk_object_get_n_accessible_children(self.to_glib_none().0)
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::atk_object_get_name(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn get_object_locale(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::atk_object_get_object_locale(self.to_glib_none().0))
        }
    }

    fn get_parent(&self) -> Option<Object> {
        unsafe {
            from_glib_none(ffi::atk_object_get_parent(self.to_glib_none().0))
        }
    }

    fn get_role(&self) -> Role {
        unsafe {
            from_glib(ffi::atk_object_get_role(self.to_glib_none().0))
        }
    }

    //fn initialize<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, data: P) {
    //    unsafe { TODO: call ffi::atk_object_initialize() }
    //}

    fn notify_state_change(&self, state: State, value: bool) {
        unsafe {
            ffi::atk_object_notify_state_change(self.to_glib_none().0, state, value.to_glib());
        }
    }

    fn peek_parent(&self) -> Option<Object> {
        unsafe {
            from_glib_none(ffi::atk_object_peek_parent(self.to_glib_none().0))
        }
    }

    fn ref_accessible_child(&self, i: i32) -> Option<Object> {
        unsafe {
            from_glib_full(ffi::atk_object_ref_accessible_child(self.to_glib_none().0, i))
        }
    }

    fn ref_relation_set(&self) -> Option<RelationSet> {
        unsafe {
            from_glib_full(ffi::atk_object_ref_relation_set(self.to_glib_none().0))
        }
    }

    fn ref_state_set(&self) -> Option<StateSet> {
        unsafe {
            from_glib_full(ffi::atk_object_ref_state_set(self.to_glib_none().0))
        }
    }

    fn remove_property_change_handler(&self, handler_id: u32) {
        unsafe {
            ffi::atk_object_remove_property_change_handler(self.to_glib_none().0, handler_id);
        }
    }

    fn remove_relationship<P: IsA<Object>>(&self, relationship: RelationType, target: &P) -> bool {
        unsafe {
            from_glib(ffi::atk_object_remove_relationship(self.to_glib_none().0, relationship.to_glib(), target.to_glib_none().0))
        }
    }

    fn set_description(&self, description: &str) {
        unsafe {
            ffi::atk_object_set_description(self.to_glib_none().0, description.to_glib_none().0);
        }
    }

    fn set_name(&self, name: &str) {
        unsafe {
            ffi::atk_object_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn set_parent<P: IsA<Object>>(&self, parent: &P) {
        unsafe {
            ffi::atk_object_set_parent(self.to_glib_none().0, parent.to_glib_none().0);
        }
    }

    fn set_role(&self, role: Role) {
        unsafe {
            ffi::atk_object_set_role(self.to_glib_none().0, role.to_glib());
        }
    }

    fn get_property_accessible_component_layer(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accessible-component-layer".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_accessible_component_mdi_zorder(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accessible-component-mdi-zorder".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_accessible_description(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accessible-description".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_accessible_description<'a, P: Into<Option<&'a str>>>(&self, accessible_description: P) {
        let accessible_description = accessible_description.into();
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "accessible-description".to_glib_none().0, Value::from(accessible_description).to_glib_none().0);
        }
    }

    fn get_property_accessible_hypertext_nlinks(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accessible-hypertext-nlinks".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_accessible_name(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accessible-name".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_accessible_name<'a, P: Into<Option<&'a str>>>(&self, accessible_name: P) {
        let accessible_name = accessible_name.into();
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "accessible-name".to_glib_none().0, Value::from(accessible_name).to_glib_none().0);
        }
    }

    fn get_property_accessible_parent(&self) -> Option<Object> {
        unsafe {
            let mut value = Value::from_type(<Object as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accessible-parent".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_accessible_parent<P: IsA<Object> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, accessible_parent: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "accessible-parent".to_glib_none().0, Value::from(accessible_parent).to_glib_none().0);
        }
    }

    fn get_property_accessible_role(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accessible-role".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_accessible_role(&self, accessible_role: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "accessible-role".to_glib_none().0, Value::from(&accessible_role).to_glib_none().0);
        }
    }

    fn get_property_accessible_table_caption(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accessible-table-caption".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_accessible_table_caption<'a, P: Into<Option<&'a str>>>(&self, accessible_table_caption: P) {
        let accessible_table_caption = accessible_table_caption.into();
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "accessible-table-caption".to_glib_none().0, Value::from(accessible_table_caption).to_glib_none().0);
        }
    }

    fn get_property_accessible_table_caption_object(&self) -> Option<Object> {
        unsafe {
            let mut value = Value::from_type(<Object as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accessible-table-caption-object".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_accessible_table_caption_object<P: IsA<Object> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, accessible_table_caption_object: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "accessible-table-caption-object".to_glib_none().0, Value::from(accessible_table_caption_object).to_glib_none().0);
        }
    }

    fn get_property_accessible_table_column_description(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accessible-table-column-description".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_accessible_table_column_description<'a, P: Into<Option<&'a str>>>(&self, accessible_table_column_description: P) {
        let accessible_table_column_description = accessible_table_column_description.into();
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "accessible-table-column-description".to_glib_none().0, Value::from(accessible_table_column_description).to_glib_none().0);
        }
    }

    fn get_property_accessible_table_column_header(&self) -> Option<Object> {
        unsafe {
            let mut value = Value::from_type(<Object as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accessible-table-column-header".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_accessible_table_column_header<P: IsA<Object> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, accessible_table_column_header: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "accessible-table-column-header".to_glib_none().0, Value::from(accessible_table_column_header).to_glib_none().0);
        }
    }

    fn get_property_accessible_table_row_description(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accessible-table-row-description".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_accessible_table_row_description<'a, P: Into<Option<&'a str>>>(&self, accessible_table_row_description: P) {
        let accessible_table_row_description = accessible_table_row_description.into();
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "accessible-table-row-description".to_glib_none().0, Value::from(accessible_table_row_description).to_glib_none().0);
        }
    }

    fn get_property_accessible_table_row_header(&self) -> Option<Object> {
        unsafe {
            let mut value = Value::from_type(<Object as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accessible-table-row-header".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_accessible_table_row_header<P: IsA<Object> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, accessible_table_row_header: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "accessible-table-row-header".to_glib_none().0, Value::from(accessible_table_row_header).to_glib_none().0);
        }
    }

    fn get_property_accessible_table_summary(&self) -> Option<Object> {
        unsafe {
            let mut value = Value::from_type(<Object as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accessible-table-summary".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_accessible_table_summary<P: IsA<Object> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, accessible_table_summary: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "accessible-table-summary".to_glib_none().0, Value::from(accessible_table_summary).to_glib_none().0);
        }
    }

    fn get_property_accessible_value(&self) -> f64 {
        unsafe {
            let mut value = Value::from_type(<f64 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "accessible-value".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_accessible_value(&self, accessible_value: f64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "accessible-value".to_glib_none().0, Value::from(&accessible_value).to_glib_none().0);
        }
    }

    //fn connect_active_descendant_changed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented arg1: *.Pointer
    //}

    //fn connect_children_changed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented arg2: *.Pointer
    //}

    fn connect_focus_event<F: Fn(&Self, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, bool) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "focus-event",
                transmute(focus_event_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    //fn connect_property_change<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented arg1: *.Pointer
    //}

    fn connect_state_change<F: Fn(&Self, &str, bool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str, bool) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "state-change",
                transmute(state_change_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_visible_data_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "visible-data-changed",
                transmute(visible_data_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_accessible_component_layer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accessible-component-layer",
                transmute(notify_accessible_component_layer_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_accessible_component_mdi_zorder_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accessible-component-mdi-zorder",
                transmute(notify_accessible_component_mdi_zorder_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_accessible_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accessible-description",
                transmute(notify_accessible_description_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_accessible_hypertext_nlinks_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accessible-hypertext-nlinks",
                transmute(notify_accessible_hypertext_nlinks_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_accessible_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accessible-name",
                transmute(notify_accessible_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_accessible_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accessible-parent",
                transmute(notify_accessible_parent_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_accessible_role_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accessible-role",
                transmute(notify_accessible_role_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_accessible_table_caption_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accessible-table-caption",
                transmute(notify_accessible_table_caption_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_accessible_table_caption_object_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accessible-table-caption-object",
                transmute(notify_accessible_table_caption_object_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_accessible_table_column_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accessible-table-column-description",
                transmute(notify_accessible_table_column_description_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_accessible_table_column_header_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accessible-table-column-header",
                transmute(notify_accessible_table_column_header_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_accessible_table_row_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accessible-table-row-description",
                transmute(notify_accessible_table_row_description_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_accessible_table_row_header_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accessible-table-row-header",
                transmute(notify_accessible_table_row_header_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_accessible_table_summary_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accessible-table-summary",
                transmute(notify_accessible_table_summary_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_accessible_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accessible-value",
                transmute(notify_accessible_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn focus_event_trampoline<P>(this: *mut ffi::AtkObject, arg1: glib_ffi::gboolean, f: glib_ffi::gpointer)
where P: IsA<Object> {
    let f: &&(Fn(&P, bool) + 'static) = transmute(f);
    f(&Object::from_glib_borrow(this).downcast_unchecked(), from_glib(arg1))
}

unsafe extern "C" fn state_change_trampoline<P>(this: *mut ffi::AtkObject, arg1: *mut libc::c_char, arg2: glib_ffi::gboolean, f: glib_ffi::gpointer)
where P: IsA<Object> {
    let f: &&(Fn(&P, &str, bool) + 'static) = transmute(f);
    f(&Object::from_glib_borrow(this).downcast_unchecked(), &String::from_glib_none(arg1), from_glib(arg2))
}

unsafe extern "C" fn visible_data_changed_trampoline<P>(this: *mut ffi::AtkObject, f: glib_ffi::gpointer)
where P: IsA<Object> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Object::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_accessible_component_layer_trampoline<P>(this: *mut ffi::AtkObject, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Object> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Object::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_accessible_component_mdi_zorder_trampoline<P>(this: *mut ffi::AtkObject, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Object> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Object::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_accessible_description_trampoline<P>(this: *mut ffi::AtkObject, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Object> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Object::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_accessible_hypertext_nlinks_trampoline<P>(this: *mut ffi::AtkObject, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Object> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Object::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_accessible_name_trampoline<P>(this: *mut ffi::AtkObject, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Object> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Object::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_accessible_parent_trampoline<P>(this: *mut ffi::AtkObject, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Object> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Object::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_accessible_role_trampoline<P>(this: *mut ffi::AtkObject, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Object> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Object::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_accessible_table_caption_trampoline<P>(this: *mut ffi::AtkObject, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Object> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Object::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_accessible_table_caption_object_trampoline<P>(this: *mut ffi::AtkObject, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Object> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Object::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_accessible_table_column_description_trampoline<P>(this: *mut ffi::AtkObject, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Object> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Object::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_accessible_table_column_header_trampoline<P>(this: *mut ffi::AtkObject, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Object> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Object::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_accessible_table_row_description_trampoline<P>(this: *mut ffi::AtkObject, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Object> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Object::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_accessible_table_row_header_trampoline<P>(this: *mut ffi::AtkObject, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Object> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Object::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_accessible_table_summary_trampoline<P>(this: *mut ffi::AtkObject, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Object> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Object::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_accessible_value_trampoline<P>(this: *mut ffi::AtkObject, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Object> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Object::from_glib_borrow(this).downcast_unchecked())
}
