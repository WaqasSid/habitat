// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct EventEnvelope {
    // message fields
    field_type: ::std::option::Option<EventEnvelope_Type>,
    payload: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    timestamp: ::std::option::Option<u64>,
    member_id: ::std::option::Option<u64>,
    service: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EventEnvelope {}

impl EventEnvelope {
    pub fn new() -> EventEnvelope {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EventEnvelope {
        static mut instance: ::protobuf::lazy::Lazy<EventEnvelope> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EventEnvelope,
        };
        unsafe {
            instance.get(EventEnvelope::new)
        }
    }

    // optional .EventEnvelope.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: EventEnvelope_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> EventEnvelope_Type {
        self.field_type.unwrap_or(EventEnvelope_Type::ProtoBuf)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<EventEnvelope_Type> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<EventEnvelope_Type> {
        &mut self.field_type
    }

    // optional bytes payload = 2;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.payload.is_none() {
            self.payload.set_default();
        };
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        self.payload.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_payload(&self) -> &[u8] {
        match self.payload.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_payload_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.payload
    }

    fn mut_payload_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.payload
    }

    // optional uint64 timestamp = 3;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp.unwrap_or(0)
    }

    fn get_timestamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.timestamp
    }

    // optional uint64 member_id = 4;

    pub fn clear_member_id(&mut self) {
        self.member_id = ::std::option::Option::None;
    }

    pub fn has_member_id(&self) -> bool {
        self.member_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_member_id(&mut self, v: u64) {
        self.member_id = ::std::option::Option::Some(v);
    }

    pub fn get_member_id(&self) -> u64 {
        self.member_id.unwrap_or(0)
    }

    fn get_member_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.member_id
    }

    fn mut_member_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.member_id
    }

    // optional string service = 5;

    pub fn clear_service(&mut self) {
        self.service.clear();
    }

    pub fn has_service(&self) -> bool {
        self.service.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service(&mut self, v: ::std::string::String) {
        self.service = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service(&mut self) -> &mut ::std::string::String {
        if self.service.is_none() {
            self.service.set_default();
        };
        self.service.as_mut().unwrap()
    }

    // Take field
    pub fn take_service(&mut self) -> ::std::string::String {
        self.service.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_service(&self) -> &str {
        match self.service.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_service_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.service
    }

    fn mut_service_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.service
    }
}

impl ::protobuf::Message for EventEnvelope {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.payload)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.member_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.service)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.payload.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.member_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.service.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.payload.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.timestamp {
            os.write_uint64(3, v)?;
        };
        if let Some(v) = self.member_id {
            os.write_uint64(4, v)?;
        };
        if let Some(v) = self.service.as_ref() {
            os.write_string(5, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EventEnvelope {
    fn new() -> EventEnvelope {
        EventEnvelope::new()
    }

    fn descriptor_static(_: ::std::option::Option<EventEnvelope>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EventEnvelope_Type>>(
                    "type",
                    EventEnvelope::get_field_type_for_reflect,
                    EventEnvelope::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "payload",
                    EventEnvelope::get_payload_for_reflect,
                    EventEnvelope::mut_payload_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp",
                    EventEnvelope::get_timestamp_for_reflect,
                    EventEnvelope::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "member_id",
                    EventEnvelope::get_member_id_for_reflect,
                    EventEnvelope::mut_member_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "service",
                    EventEnvelope::get_service_for_reflect,
                    EventEnvelope::mut_service_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EventEnvelope>(
                    "EventEnvelope",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EventEnvelope {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_payload();
        self.clear_timestamp();
        self.clear_member_id();
        self.clear_service();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EventEnvelope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EventEnvelope {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EventEnvelope_Type {
    ProtoBuf = 1,
    JSON = 2,
    TOML = 3,
}

impl ::protobuf::ProtobufEnum for EventEnvelope_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EventEnvelope_Type> {
        match value {
            1 => ::std::option::Option::Some(EventEnvelope_Type::ProtoBuf),
            2 => ::std::option::Option::Some(EventEnvelope_Type::JSON),
            3 => ::std::option::Option::Some(EventEnvelope_Type::TOML),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EventEnvelope_Type] = &[
            EventEnvelope_Type::ProtoBuf,
            EventEnvelope_Type::JSON,
            EventEnvelope_Type::TOML,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EventEnvelope_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EventEnvelope_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EventEnvelope_Type {
}

impl ::protobuf::reflect::ProtobufValue for EventEnvelope_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SysInfo {
    // message fields
    ip: ::protobuf::SingularField<::std::string::String>,
    hostname: ::protobuf::SingularField<::std::string::String>,
    gossip_ip: ::protobuf::SingularField<::std::string::String>,
    gossip_port: ::protobuf::SingularField<::std::string::String>,
    http_gateway_ip: ::protobuf::SingularField<::std::string::String>,
    http_gateway_port: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SysInfo {}

impl SysInfo {
    pub fn new() -> SysInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SysInfo {
        static mut instance: ::protobuf::lazy::Lazy<SysInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SysInfo,
        };
        unsafe {
            instance.get(SysInfo::new)
        }
    }

    // optional string ip = 1;

    pub fn clear_ip(&mut self) {
        self.ip.clear();
    }

    pub fn has_ip(&self) -> bool {
        self.ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ip(&mut self, v: ::std::string::String) {
        self.ip = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ip(&mut self) -> &mut ::std::string::String {
        if self.ip.is_none() {
            self.ip.set_default();
        };
        self.ip.as_mut().unwrap()
    }

    // Take field
    pub fn take_ip(&mut self) -> ::std::string::String {
        self.ip.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ip(&self) -> &str {
        match self.ip.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ip_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ip
    }

    fn mut_ip_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ip
    }

    // optional string hostname = 2;

    pub fn clear_hostname(&mut self) {
        self.hostname.clear();
    }

    pub fn has_hostname(&self) -> bool {
        self.hostname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostname(&mut self, v: ::std::string::String) {
        self.hostname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hostname(&mut self) -> &mut ::std::string::String {
        if self.hostname.is_none() {
            self.hostname.set_default();
        };
        self.hostname.as_mut().unwrap()
    }

    // Take field
    pub fn take_hostname(&mut self) -> ::std::string::String {
        self.hostname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hostname(&self) -> &str {
        match self.hostname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_hostname_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.hostname
    }

    fn mut_hostname_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.hostname
    }

    // optional string gossip_ip = 3;

    pub fn clear_gossip_ip(&mut self) {
        self.gossip_ip.clear();
    }

    pub fn has_gossip_ip(&self) -> bool {
        self.gossip_ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gossip_ip(&mut self, v: ::std::string::String) {
        self.gossip_ip = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gossip_ip(&mut self) -> &mut ::std::string::String {
        if self.gossip_ip.is_none() {
            self.gossip_ip.set_default();
        };
        self.gossip_ip.as_mut().unwrap()
    }

    // Take field
    pub fn take_gossip_ip(&mut self) -> ::std::string::String {
        self.gossip_ip.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_gossip_ip(&self) -> &str {
        match self.gossip_ip.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_gossip_ip_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.gossip_ip
    }

    fn mut_gossip_ip_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.gossip_ip
    }

    // optional string gossip_port = 4;

    pub fn clear_gossip_port(&mut self) {
        self.gossip_port.clear();
    }

    pub fn has_gossip_port(&self) -> bool {
        self.gossip_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gossip_port(&mut self, v: ::std::string::String) {
        self.gossip_port = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gossip_port(&mut self) -> &mut ::std::string::String {
        if self.gossip_port.is_none() {
            self.gossip_port.set_default();
        };
        self.gossip_port.as_mut().unwrap()
    }

    // Take field
    pub fn take_gossip_port(&mut self) -> ::std::string::String {
        self.gossip_port.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_gossip_port(&self) -> &str {
        match self.gossip_port.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_gossip_port_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.gossip_port
    }

    fn mut_gossip_port_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.gossip_port
    }

    // optional string http_gateway_ip = 5;

    pub fn clear_http_gateway_ip(&mut self) {
        self.http_gateway_ip.clear();
    }

    pub fn has_http_gateway_ip(&self) -> bool {
        self.http_gateway_ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_http_gateway_ip(&mut self, v: ::std::string::String) {
        self.http_gateway_ip = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_http_gateway_ip(&mut self) -> &mut ::std::string::String {
        if self.http_gateway_ip.is_none() {
            self.http_gateway_ip.set_default();
        };
        self.http_gateway_ip.as_mut().unwrap()
    }

    // Take field
    pub fn take_http_gateway_ip(&mut self) -> ::std::string::String {
        self.http_gateway_ip.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_http_gateway_ip(&self) -> &str {
        match self.http_gateway_ip.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_http_gateway_ip_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.http_gateway_ip
    }

    fn mut_http_gateway_ip_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.http_gateway_ip
    }

    // optional string http_gateway_port = 6;

    pub fn clear_http_gateway_port(&mut self) {
        self.http_gateway_port.clear();
    }

    pub fn has_http_gateway_port(&self) -> bool {
        self.http_gateway_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_http_gateway_port(&mut self, v: ::std::string::String) {
        self.http_gateway_port = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_http_gateway_port(&mut self) -> &mut ::std::string::String {
        if self.http_gateway_port.is_none() {
            self.http_gateway_port.set_default();
        };
        self.http_gateway_port.as_mut().unwrap()
    }

    // Take field
    pub fn take_http_gateway_port(&mut self) -> ::std::string::String {
        self.http_gateway_port.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_http_gateway_port(&self) -> &str {
        match self.http_gateway_port.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_http_gateway_port_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.http_gateway_port
    }

    fn mut_http_gateway_port_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.http_gateway_port
    }
}

impl ::protobuf::Message for SysInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ip)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hostname)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.gossip_ip)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.gossip_port)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.http_gateway_ip)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.http_gateway_port)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.ip.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.hostname.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.gossip_ip.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.gossip_port.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.http_gateway_ip.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        if let Some(v) = self.http_gateway_port.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ip.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.hostname.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.gossip_ip.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.gossip_port.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.http_gateway_ip.as_ref() {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.http_gateway_port.as_ref() {
            os.write_string(6, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SysInfo {
    fn new() -> SysInfo {
        SysInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<SysInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ip",
                    SysInfo::get_ip_for_reflect,
                    SysInfo::mut_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hostname",
                    SysInfo::get_hostname_for_reflect,
                    SysInfo::mut_hostname_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "gossip_ip",
                    SysInfo::get_gossip_ip_for_reflect,
                    SysInfo::mut_gossip_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "gossip_port",
                    SysInfo::get_gossip_port_for_reflect,
                    SysInfo::mut_gossip_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "http_gateway_ip",
                    SysInfo::get_http_gateway_ip_for_reflect,
                    SysInfo::mut_http_gateway_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "http_gateway_port",
                    SysInfo::get_http_gateway_port_for_reflect,
                    SysInfo::mut_http_gateway_port_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SysInfo>(
                    "SysInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SysInfo {
    fn clear(&mut self) {
        self.clear_ip();
        self.clear_hostname();
        self.clear_gossip_ip();
        self.clear_gossip_port();
        self.clear_http_gateway_ip();
        self.clear_http_gateway_port();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SysInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SysInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PackageIdent {
    // message fields
    origin: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    version: ::protobuf::SingularField<::std::string::String>,
    release: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PackageIdent {}

impl PackageIdent {
    pub fn new() -> PackageIdent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PackageIdent {
        static mut instance: ::protobuf::lazy::Lazy<PackageIdent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PackageIdent,
        };
        unsafe {
            instance.get(PackageIdent::new)
        }
    }

    // optional string origin = 1;

    pub fn clear_origin(&mut self) {
        self.origin.clear();
    }

    pub fn has_origin(&self) -> bool {
        self.origin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: ::std::string::String) {
        self.origin = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin(&mut self) -> &mut ::std::string::String {
        if self.origin.is_none() {
            self.origin.set_default();
        };
        self.origin.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin(&mut self) -> ::std::string::String {
        self.origin.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_origin(&self) -> &str {
        match self.origin.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_origin_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.origin
    }

    fn mut_origin_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.origin
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string version = 3;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        if self.version.is_none() {
            self.version.set_default();
        };
        self.version.as_mut().unwrap()
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        self.version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_version(&self) -> &str {
        match self.version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_version_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.version
    }

    // optional string release = 4;

    pub fn clear_release(&mut self) {
        self.release.clear();
    }

    pub fn has_release(&self) -> bool {
        self.release.is_some()
    }

    // Param is passed by value, moved
    pub fn set_release(&mut self, v: ::std::string::String) {
        self.release = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_release(&mut self) -> &mut ::std::string::String {
        if self.release.is_none() {
            self.release.set_default();
        };
        self.release.as_mut().unwrap()
    }

    // Take field
    pub fn take_release(&mut self) -> ::std::string::String {
        self.release.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_release(&self) -> &str {
        match self.release.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_release_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.release
    }

    fn mut_release_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.release
    }
}

impl ::protobuf::Message for PackageIdent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.origin)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.version)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.release)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.origin.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.version.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.release.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.version.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.release.as_ref() {
            os.write_string(4, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PackageIdent {
    fn new() -> PackageIdent {
        PackageIdent::new()
    }

    fn descriptor_static(_: ::std::option::Option<PackageIdent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "origin",
                    PackageIdent::get_origin_for_reflect,
                    PackageIdent::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    PackageIdent::get_name_for_reflect,
                    PackageIdent::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version",
                    PackageIdent::get_version_for_reflect,
                    PackageIdent::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "release",
                    PackageIdent::get_release_for_reflect,
                    PackageIdent::mut_release_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PackageIdent>(
                    "PackageIdent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PackageIdent {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_name();
        self.clear_version();
        self.clear_release();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PackageIdent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PackageIdent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CensusEntry {
    // message fields
    member_id: ::protobuf::SingularField<::std::string::String>,
    service: ::protobuf::SingularField<::std::string::String>,
    group: ::protobuf::SingularField<::std::string::String>,
    org: ::protobuf::SingularField<::std::string::String>,
    cfg: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    sys: ::protobuf::SingularPtrField<SysInfo>,
    pkg: ::protobuf::SingularPtrField<PackageIdent>,
    leader: ::std::option::Option<bool>,
    follower: ::std::option::Option<bool>,
    update_leader: ::std::option::Option<bool>,
    update_follower: ::std::option::Option<bool>,
    election_is_running: ::std::option::Option<bool>,
    election_is_no_quorum: ::std::option::Option<bool>,
    election_is_finished: ::std::option::Option<bool>,
    update_election_is_running: ::std::option::Option<bool>,
    update_election_is_no_quorum: ::std::option::Option<bool>,
    update_election_is_finished: ::std::option::Option<bool>,
    initialized: ::std::option::Option<bool>,
    alive: ::std::option::Option<bool>,
    suspect: ::std::option::Option<bool>,
    confirmed: ::std::option::Option<bool>,
    persistent: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CensusEntry {}

impl CensusEntry {
    pub fn new() -> CensusEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CensusEntry {
        static mut instance: ::protobuf::lazy::Lazy<CensusEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CensusEntry,
        };
        unsafe {
            instance.get(CensusEntry::new)
        }
    }

    // optional string member_id = 1;

    pub fn clear_member_id(&mut self) {
        self.member_id.clear();
    }

    pub fn has_member_id(&self) -> bool {
        self.member_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_member_id(&mut self, v: ::std::string::String) {
        self.member_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_member_id(&mut self) -> &mut ::std::string::String {
        if self.member_id.is_none() {
            self.member_id.set_default();
        };
        self.member_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_member_id(&mut self) -> ::std::string::String {
        self.member_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_member_id(&self) -> &str {
        match self.member_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_member_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.member_id
    }

    fn mut_member_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.member_id
    }

    // optional string service = 2;

    pub fn clear_service(&mut self) {
        self.service.clear();
    }

    pub fn has_service(&self) -> bool {
        self.service.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service(&mut self, v: ::std::string::String) {
        self.service = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service(&mut self) -> &mut ::std::string::String {
        if self.service.is_none() {
            self.service.set_default();
        };
        self.service.as_mut().unwrap()
    }

    // Take field
    pub fn take_service(&mut self) -> ::std::string::String {
        self.service.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_service(&self) -> &str {
        match self.service.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_service_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.service
    }

    fn mut_service_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.service
    }

    // optional string group = 3;

    pub fn clear_group(&mut self) {
        self.group.clear();
    }

    pub fn has_group(&self) -> bool {
        self.group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group(&mut self, v: ::std::string::String) {
        self.group = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_group(&mut self) -> &mut ::std::string::String {
        if self.group.is_none() {
            self.group.set_default();
        };
        self.group.as_mut().unwrap()
    }

    // Take field
    pub fn take_group(&mut self) -> ::std::string::String {
        self.group.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_group(&self) -> &str {
        match self.group.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_group_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.group
    }

    fn mut_group_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.group
    }

    // optional string org = 4;

    pub fn clear_org(&mut self) {
        self.org.clear();
    }

    pub fn has_org(&self) -> bool {
        self.org.is_some()
    }

    // Param is passed by value, moved
    pub fn set_org(&mut self, v: ::std::string::String) {
        self.org = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_org(&mut self) -> &mut ::std::string::String {
        if self.org.is_none() {
            self.org.set_default();
        };
        self.org.as_mut().unwrap()
    }

    // Take field
    pub fn take_org(&mut self) -> ::std::string::String {
        self.org.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_org(&self) -> &str {
        match self.org.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_org_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.org
    }

    fn mut_org_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.org
    }

    // optional bytes cfg = 5;

    pub fn clear_cfg(&mut self) {
        self.cfg.clear();
    }

    pub fn has_cfg(&self) -> bool {
        self.cfg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cfg(&mut self, v: ::std::vec::Vec<u8>) {
        self.cfg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cfg(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.cfg.is_none() {
            self.cfg.set_default();
        };
        self.cfg.as_mut().unwrap()
    }

    // Take field
    pub fn take_cfg(&mut self) -> ::std::vec::Vec<u8> {
        self.cfg.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_cfg(&self) -> &[u8] {
        match self.cfg.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_cfg_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.cfg
    }

    fn mut_cfg_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.cfg
    }

    // optional .SysInfo sys = 6;

    pub fn clear_sys(&mut self) {
        self.sys.clear();
    }

    pub fn has_sys(&self) -> bool {
        self.sys.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sys(&mut self, v: SysInfo) {
        self.sys = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sys(&mut self) -> &mut SysInfo {
        if self.sys.is_none() {
            self.sys.set_default();
        };
        self.sys.as_mut().unwrap()
    }

    // Take field
    pub fn take_sys(&mut self) -> SysInfo {
        self.sys.take().unwrap_or_else(|| SysInfo::new())
    }

    pub fn get_sys(&self) -> &SysInfo {
        self.sys.as_ref().unwrap_or_else(|| SysInfo::default_instance())
    }

    fn get_sys_for_reflect(&self) -> &::protobuf::SingularPtrField<SysInfo> {
        &self.sys
    }

    fn mut_sys_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SysInfo> {
        &mut self.sys
    }

    // optional .PackageIdent pkg = 7;

    pub fn clear_pkg(&mut self) {
        self.pkg.clear();
    }

    pub fn has_pkg(&self) -> bool {
        self.pkg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pkg(&mut self, v: PackageIdent) {
        self.pkg = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pkg(&mut self) -> &mut PackageIdent {
        if self.pkg.is_none() {
            self.pkg.set_default();
        };
        self.pkg.as_mut().unwrap()
    }

    // Take field
    pub fn take_pkg(&mut self) -> PackageIdent {
        self.pkg.take().unwrap_or_else(|| PackageIdent::new())
    }

    pub fn get_pkg(&self) -> &PackageIdent {
        self.pkg.as_ref().unwrap_or_else(|| PackageIdent::default_instance())
    }

    fn get_pkg_for_reflect(&self) -> &::protobuf::SingularPtrField<PackageIdent> {
        &self.pkg
    }

    fn mut_pkg_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PackageIdent> {
        &mut self.pkg
    }

    // optional bool leader = 8;

    pub fn clear_leader(&mut self) {
        self.leader = ::std::option::Option::None;
    }

    pub fn has_leader(&self) -> bool {
        self.leader.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leader(&mut self, v: bool) {
        self.leader = ::std::option::Option::Some(v);
    }

    pub fn get_leader(&self) -> bool {
        self.leader.unwrap_or(false)
    }

    fn get_leader_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.leader
    }

    fn mut_leader_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.leader
    }

    // optional bool follower = 9;

    pub fn clear_follower(&mut self) {
        self.follower = ::std::option::Option::None;
    }

    pub fn has_follower(&self) -> bool {
        self.follower.is_some()
    }

    // Param is passed by value, moved
    pub fn set_follower(&mut self, v: bool) {
        self.follower = ::std::option::Option::Some(v);
    }

    pub fn get_follower(&self) -> bool {
        self.follower.unwrap_or(false)
    }

    fn get_follower_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.follower
    }

    fn mut_follower_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.follower
    }

    // optional bool update_leader = 10;

    pub fn clear_update_leader(&mut self) {
        self.update_leader = ::std::option::Option::None;
    }

    pub fn has_update_leader(&self) -> bool {
        self.update_leader.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_leader(&mut self, v: bool) {
        self.update_leader = ::std::option::Option::Some(v);
    }

    pub fn get_update_leader(&self) -> bool {
        self.update_leader.unwrap_or(false)
    }

    fn get_update_leader_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.update_leader
    }

    fn mut_update_leader_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.update_leader
    }

    // optional bool update_follower = 11;

    pub fn clear_update_follower(&mut self) {
        self.update_follower = ::std::option::Option::None;
    }

    pub fn has_update_follower(&self) -> bool {
        self.update_follower.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_follower(&mut self, v: bool) {
        self.update_follower = ::std::option::Option::Some(v);
    }

    pub fn get_update_follower(&self) -> bool {
        self.update_follower.unwrap_or(false)
    }

    fn get_update_follower_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.update_follower
    }

    fn mut_update_follower_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.update_follower
    }

    // optional bool election_is_running = 12;

    pub fn clear_election_is_running(&mut self) {
        self.election_is_running = ::std::option::Option::None;
    }

    pub fn has_election_is_running(&self) -> bool {
        self.election_is_running.is_some()
    }

    // Param is passed by value, moved
    pub fn set_election_is_running(&mut self, v: bool) {
        self.election_is_running = ::std::option::Option::Some(v);
    }

    pub fn get_election_is_running(&self) -> bool {
        self.election_is_running.unwrap_or(false)
    }

    fn get_election_is_running_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.election_is_running
    }

    fn mut_election_is_running_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.election_is_running
    }

    // optional bool election_is_no_quorum = 13;

    pub fn clear_election_is_no_quorum(&mut self) {
        self.election_is_no_quorum = ::std::option::Option::None;
    }

    pub fn has_election_is_no_quorum(&self) -> bool {
        self.election_is_no_quorum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_election_is_no_quorum(&mut self, v: bool) {
        self.election_is_no_quorum = ::std::option::Option::Some(v);
    }

    pub fn get_election_is_no_quorum(&self) -> bool {
        self.election_is_no_quorum.unwrap_or(false)
    }

    fn get_election_is_no_quorum_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.election_is_no_quorum
    }

    fn mut_election_is_no_quorum_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.election_is_no_quorum
    }

    // optional bool election_is_finished = 14;

    pub fn clear_election_is_finished(&mut self) {
        self.election_is_finished = ::std::option::Option::None;
    }

    pub fn has_election_is_finished(&self) -> bool {
        self.election_is_finished.is_some()
    }

    // Param is passed by value, moved
    pub fn set_election_is_finished(&mut self, v: bool) {
        self.election_is_finished = ::std::option::Option::Some(v);
    }

    pub fn get_election_is_finished(&self) -> bool {
        self.election_is_finished.unwrap_or(false)
    }

    fn get_election_is_finished_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.election_is_finished
    }

    fn mut_election_is_finished_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.election_is_finished
    }

    // optional bool update_election_is_running = 15;

    pub fn clear_update_election_is_running(&mut self) {
        self.update_election_is_running = ::std::option::Option::None;
    }

    pub fn has_update_election_is_running(&self) -> bool {
        self.update_election_is_running.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_election_is_running(&mut self, v: bool) {
        self.update_election_is_running = ::std::option::Option::Some(v);
    }

    pub fn get_update_election_is_running(&self) -> bool {
        self.update_election_is_running.unwrap_or(false)
    }

    fn get_update_election_is_running_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.update_election_is_running
    }

    fn mut_update_election_is_running_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.update_election_is_running
    }

    // optional bool update_election_is_no_quorum = 16;

    pub fn clear_update_election_is_no_quorum(&mut self) {
        self.update_election_is_no_quorum = ::std::option::Option::None;
    }

    pub fn has_update_election_is_no_quorum(&self) -> bool {
        self.update_election_is_no_quorum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_election_is_no_quorum(&mut self, v: bool) {
        self.update_election_is_no_quorum = ::std::option::Option::Some(v);
    }

    pub fn get_update_election_is_no_quorum(&self) -> bool {
        self.update_election_is_no_quorum.unwrap_or(false)
    }

    fn get_update_election_is_no_quorum_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.update_election_is_no_quorum
    }

    fn mut_update_election_is_no_quorum_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.update_election_is_no_quorum
    }

    // optional bool update_election_is_finished = 17;

    pub fn clear_update_election_is_finished(&mut self) {
        self.update_election_is_finished = ::std::option::Option::None;
    }

    pub fn has_update_election_is_finished(&self) -> bool {
        self.update_election_is_finished.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_election_is_finished(&mut self, v: bool) {
        self.update_election_is_finished = ::std::option::Option::Some(v);
    }

    pub fn get_update_election_is_finished(&self) -> bool {
        self.update_election_is_finished.unwrap_or(false)
    }

    fn get_update_election_is_finished_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.update_election_is_finished
    }

    fn mut_update_election_is_finished_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.update_election_is_finished
    }

    // optional bool initialized = 18;

    pub fn clear_initialized(&mut self) {
        self.initialized = ::std::option::Option::None;
    }

    pub fn has_initialized(&self) -> bool {
        self.initialized.is_some()
    }

    // Param is passed by value, moved
    pub fn set_initialized(&mut self, v: bool) {
        self.initialized = ::std::option::Option::Some(v);
    }

    pub fn get_initialized(&self) -> bool {
        self.initialized.unwrap_or(false)
    }

    fn get_initialized_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.initialized
    }

    fn mut_initialized_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.initialized
    }

    // optional bool alive = 19;

    pub fn clear_alive(&mut self) {
        self.alive = ::std::option::Option::None;
    }

    pub fn has_alive(&self) -> bool {
        self.alive.is_some()
    }

    // Param is passed by value, moved
    pub fn set_alive(&mut self, v: bool) {
        self.alive = ::std::option::Option::Some(v);
    }

    pub fn get_alive(&self) -> bool {
        self.alive.unwrap_or(false)
    }

    fn get_alive_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.alive
    }

    fn mut_alive_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.alive
    }

    // optional bool suspect = 20;

    pub fn clear_suspect(&mut self) {
        self.suspect = ::std::option::Option::None;
    }

    pub fn has_suspect(&self) -> bool {
        self.suspect.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suspect(&mut self, v: bool) {
        self.suspect = ::std::option::Option::Some(v);
    }

    pub fn get_suspect(&self) -> bool {
        self.suspect.unwrap_or(false)
    }

    fn get_suspect_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.suspect
    }

    fn mut_suspect_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.suspect
    }

    // optional bool confirmed = 21;

    pub fn clear_confirmed(&mut self) {
        self.confirmed = ::std::option::Option::None;
    }

    pub fn has_confirmed(&self) -> bool {
        self.confirmed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_confirmed(&mut self, v: bool) {
        self.confirmed = ::std::option::Option::Some(v);
    }

    pub fn get_confirmed(&self) -> bool {
        self.confirmed.unwrap_or(false)
    }

    fn get_confirmed_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.confirmed
    }

    fn mut_confirmed_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.confirmed
    }

    // optional bool persistent = 22;

    pub fn clear_persistent(&mut self) {
        self.persistent = ::std::option::Option::None;
    }

    pub fn has_persistent(&self) -> bool {
        self.persistent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_persistent(&mut self, v: bool) {
        self.persistent = ::std::option::Option::Some(v);
    }

    pub fn get_persistent(&self) -> bool {
        self.persistent.unwrap_or(false)
    }

    fn get_persistent_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.persistent
    }

    fn mut_persistent_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.persistent
    }
}

impl ::protobuf::Message for CensusEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.member_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.service)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.group)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.org)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.cfg)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.sys)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pkg)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.leader = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.follower = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.update_leader = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.update_follower = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.election_is_running = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.election_is_no_quorum = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.election_is_finished = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.update_election_is_running = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.update_election_is_no_quorum = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.update_election_is_finished = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.initialized = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.alive = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.suspect = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.confirmed = ::std::option::Option::Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.persistent = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.member_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.service.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.group.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.org.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.cfg.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        };
        if let Some(v) = self.sys.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.pkg.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.leader {
            my_size += 2;
        };
        if let Some(v) = self.follower {
            my_size += 2;
        };
        if let Some(v) = self.update_leader {
            my_size += 2;
        };
        if let Some(v) = self.update_follower {
            my_size += 2;
        };
        if let Some(v) = self.election_is_running {
            my_size += 2;
        };
        if let Some(v) = self.election_is_no_quorum {
            my_size += 2;
        };
        if let Some(v) = self.election_is_finished {
            my_size += 2;
        };
        if let Some(v) = self.update_election_is_running {
            my_size += 2;
        };
        if let Some(v) = self.update_election_is_no_quorum {
            my_size += 3;
        };
        if let Some(v) = self.update_election_is_finished {
            my_size += 3;
        };
        if let Some(v) = self.initialized {
            my_size += 3;
        };
        if let Some(v) = self.alive {
            my_size += 3;
        };
        if let Some(v) = self.suspect {
            my_size += 3;
        };
        if let Some(v) = self.confirmed {
            my_size += 3;
        };
        if let Some(v) = self.persistent {
            my_size += 3;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.member_id.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.service.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.group.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.org.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.cfg.as_ref() {
            os.write_bytes(5, &v)?;
        };
        if let Some(v) = self.sys.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.pkg.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.leader {
            os.write_bool(8, v)?;
        };
        if let Some(v) = self.follower {
            os.write_bool(9, v)?;
        };
        if let Some(v) = self.update_leader {
            os.write_bool(10, v)?;
        };
        if let Some(v) = self.update_follower {
            os.write_bool(11, v)?;
        };
        if let Some(v) = self.election_is_running {
            os.write_bool(12, v)?;
        };
        if let Some(v) = self.election_is_no_quorum {
            os.write_bool(13, v)?;
        };
        if let Some(v) = self.election_is_finished {
            os.write_bool(14, v)?;
        };
        if let Some(v) = self.update_election_is_running {
            os.write_bool(15, v)?;
        };
        if let Some(v) = self.update_election_is_no_quorum {
            os.write_bool(16, v)?;
        };
        if let Some(v) = self.update_election_is_finished {
            os.write_bool(17, v)?;
        };
        if let Some(v) = self.initialized {
            os.write_bool(18, v)?;
        };
        if let Some(v) = self.alive {
            os.write_bool(19, v)?;
        };
        if let Some(v) = self.suspect {
            os.write_bool(20, v)?;
        };
        if let Some(v) = self.confirmed {
            os.write_bool(21, v)?;
        };
        if let Some(v) = self.persistent {
            os.write_bool(22, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CensusEntry {
    fn new() -> CensusEntry {
        CensusEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<CensusEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "member_id",
                    CensusEntry::get_member_id_for_reflect,
                    CensusEntry::mut_member_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "service",
                    CensusEntry::get_service_for_reflect,
                    CensusEntry::mut_service_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "group",
                    CensusEntry::get_group_for_reflect,
                    CensusEntry::mut_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "org",
                    CensusEntry::get_org_for_reflect,
                    CensusEntry::mut_org_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "cfg",
                    CensusEntry::get_cfg_for_reflect,
                    CensusEntry::mut_cfg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SysInfo>>(
                    "sys",
                    CensusEntry::get_sys_for_reflect,
                    CensusEntry::mut_sys_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PackageIdent>>(
                    "pkg",
                    CensusEntry::get_pkg_for_reflect,
                    CensusEntry::mut_pkg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "leader",
                    CensusEntry::get_leader_for_reflect,
                    CensusEntry::mut_leader_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "follower",
                    CensusEntry::get_follower_for_reflect,
                    CensusEntry::mut_follower_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "update_leader",
                    CensusEntry::get_update_leader_for_reflect,
                    CensusEntry::mut_update_leader_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "update_follower",
                    CensusEntry::get_update_follower_for_reflect,
                    CensusEntry::mut_update_follower_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "election_is_running",
                    CensusEntry::get_election_is_running_for_reflect,
                    CensusEntry::mut_election_is_running_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "election_is_no_quorum",
                    CensusEntry::get_election_is_no_quorum_for_reflect,
                    CensusEntry::mut_election_is_no_quorum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "election_is_finished",
                    CensusEntry::get_election_is_finished_for_reflect,
                    CensusEntry::mut_election_is_finished_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "update_election_is_running",
                    CensusEntry::get_update_election_is_running_for_reflect,
                    CensusEntry::mut_update_election_is_running_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "update_election_is_no_quorum",
                    CensusEntry::get_update_election_is_no_quorum_for_reflect,
                    CensusEntry::mut_update_election_is_no_quorum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "update_election_is_finished",
                    CensusEntry::get_update_election_is_finished_for_reflect,
                    CensusEntry::mut_update_election_is_finished_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "initialized",
                    CensusEntry::get_initialized_for_reflect,
                    CensusEntry::mut_initialized_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "alive",
                    CensusEntry::get_alive_for_reflect,
                    CensusEntry::mut_alive_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "suspect",
                    CensusEntry::get_suspect_for_reflect,
                    CensusEntry::mut_suspect_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "confirmed",
                    CensusEntry::get_confirmed_for_reflect,
                    CensusEntry::mut_confirmed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "persistent",
                    CensusEntry::get_persistent_for_reflect,
                    CensusEntry::mut_persistent_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CensusEntry>(
                    "CensusEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CensusEntry {
    fn clear(&mut self) {
        self.clear_member_id();
        self.clear_service();
        self.clear_group();
        self.clear_org();
        self.clear_cfg();
        self.clear_sys();
        self.clear_pkg();
        self.clear_leader();
        self.clear_follower();
        self.clear_update_leader();
        self.clear_update_follower();
        self.clear_election_is_running();
        self.clear_election_is_no_quorum();
        self.clear_election_is_finished();
        self.clear_update_election_is_running();
        self.clear_update_election_is_no_quorum();
        self.clear_update_election_is_finished();
        self.clear_initialized();
        self.clear_alive();
        self.clear_suspect();
        self.clear_confirmed();
        self.clear_persistent();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CensusEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CensusEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x15, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73, 0x2f, 0x65, 0x76, 0x65, 0x6e,
    0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xd1, 0x01, 0x0a, 0x0d, 0x45, 0x76, 0x65, 0x6e,
    0x74, 0x45, 0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x12, 0x27, 0x0a, 0x04, 0x74, 0x79, 0x70,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x13, 0x2e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x45,
    0x6e, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79,
    0x70, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x12, 0x1c, 0x0a, 0x09,
    0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x1b, 0x0a, 0x09, 0x6d, 0x65,
    0x6d, 0x62, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x6d,
    0x65, 0x6d, 0x62, 0x65, 0x72, 0x49, 0x64, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63,
    0x65, 0x22, 0x28, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0c, 0x0a, 0x08, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x42, 0x75, 0x66, 0x10, 0x01, 0x12, 0x08, 0x0a, 0x04, 0x4a, 0x53, 0x4f, 0x4e, 0x10,
    0x02, 0x12, 0x08, 0x0a, 0x04, 0x54, 0x4f, 0x4d, 0x4c, 0x10, 0x03, 0x22, 0xc7, 0x01, 0x0a, 0x07,
    0x53, 0x79, 0x73, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x70, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x70, 0x12, 0x1a, 0x0a, 0x08, 0x68, 0x6f, 0x73, 0x74, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x68, 0x6f, 0x73, 0x74, 0x6e,
    0x61, 0x6d, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x67, 0x6f, 0x73, 0x73, 0x69, 0x70, 0x5f, 0x69, 0x70,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x67, 0x6f, 0x73, 0x73, 0x69, 0x70, 0x49, 0x70,
    0x12, 0x1f, 0x0a, 0x0b, 0x67, 0x6f, 0x73, 0x73, 0x69, 0x70, 0x5f, 0x70, 0x6f, 0x72, 0x74, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x67, 0x6f, 0x73, 0x73, 0x69, 0x70, 0x50, 0x6f, 0x72,
    0x74, 0x12, 0x26, 0x0a, 0x0f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x67, 0x61, 0x74, 0x65, 0x77, 0x61,
    0x79, 0x5f, 0x69, 0x70, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x68, 0x74, 0x74, 0x70,
    0x47, 0x61, 0x74, 0x65, 0x77, 0x61, 0x79, 0x49, 0x70, 0x12, 0x2a, 0x0a, 0x11, 0x68, 0x74, 0x74,
    0x70, 0x5f, 0x67, 0x61, 0x74, 0x65, 0x77, 0x61, 0x79, 0x5f, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x68, 0x74, 0x74, 0x70, 0x47, 0x61, 0x74, 0x65, 0x77, 0x61,
    0x79, 0x50, 0x6f, 0x72, 0x74, 0x22, 0x6e, 0x0a, 0x0c, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65,
    0x49, 0x64, 0x65, 0x6e, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x12, 0x0a,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x18, 0x0a, 0x07, 0x72,
    0x65, 0x6c, 0x65, 0x61, 0x73, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x72, 0x65,
    0x6c, 0x65, 0x61, 0x73, 0x65, 0x22, 0x9e, 0x06, 0x0a, 0x0b, 0x43, 0x65, 0x6e, 0x73, 0x75, 0x73,
    0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x1b, 0x0a, 0x09, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x49, 0x64, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x07, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x14, 0x0a, 0x05,
    0x67, 0x72, 0x6f, 0x75, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x67, 0x72, 0x6f,
    0x75, 0x70, 0x12, 0x10, 0x0a, 0x03, 0x6f, 0x72, 0x67, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x03, 0x6f, 0x72, 0x67, 0x12, 0x10, 0x0a, 0x03, 0x63, 0x66, 0x67, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x03, 0x63, 0x66, 0x67, 0x12, 0x1a, 0x0a, 0x03, 0x73, 0x79, 0x73, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x53, 0x79, 0x73, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x03, 0x73,
    0x79, 0x73, 0x12, 0x1f, 0x0a, 0x03, 0x70, 0x6b, 0x67, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0d, 0x2e, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x52, 0x03,
    0x70, 0x6b, 0x67, 0x12, 0x16, 0x0a, 0x06, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x06, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x1a, 0x0a, 0x08, 0x66,
    0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x65, 0x72, 0x18, 0x09, 0x20, 0x01, 0x28, 0x08, 0x52, 0x08, 0x66,
    0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x65, 0x72, 0x12, 0x23, 0x0a, 0x0d, 0x75, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x5f, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0c,
    0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x27, 0x0a, 0x0f,
    0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x65, 0x72, 0x18,
    0x0b, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0e, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x46, 0x6f, 0x6c,
    0x6c, 0x6f, 0x77, 0x65, 0x72, 0x12, 0x2e, 0x0a, 0x13, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x5f, 0x69, 0x73, 0x5f, 0x72, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x18, 0x0c, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x11, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x73, 0x52, 0x75,
    0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x12, 0x31, 0x0a, 0x15, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x5f, 0x69, 0x73, 0x5f, 0x6e, 0x6f, 0x5f, 0x71, 0x75, 0x6f, 0x72, 0x75, 0x6d, 0x18, 0x0d,
    0x20, 0x01, 0x28, 0x08, 0x52, 0x12, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x73,
    0x4e, 0x6f, 0x51, 0x75, 0x6f, 0x72, 0x75, 0x6d, 0x12, 0x30, 0x0a, 0x14, 0x65, 0x6c, 0x65, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x73, 0x5f, 0x66, 0x69, 0x6e, 0x69, 0x73, 0x68, 0x65, 0x64,
    0x18, 0x0e, 0x20, 0x01, 0x28, 0x08, 0x52, 0x12, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x49, 0x73, 0x46, 0x69, 0x6e, 0x69, 0x73, 0x68, 0x65, 0x64, 0x12, 0x3b, 0x0a, 0x1a, 0x75, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x5f, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x73,
    0x5f, 0x72, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x08, 0x52, 0x17,
    0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x45, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x73,
    0x52, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x12, 0x3e, 0x0a, 0x1c, 0x75, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x5f, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x73, 0x5f, 0x6e, 0x6f,
    0x5f, 0x71, 0x75, 0x6f, 0x72, 0x75, 0x6d, 0x18, 0x10, 0x20, 0x01, 0x28, 0x08, 0x52, 0x18, 0x75,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x45, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x73, 0x4e,
    0x6f, 0x51, 0x75, 0x6f, 0x72, 0x75, 0x6d, 0x12, 0x3d, 0x0a, 0x1b, 0x75, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x5f, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x73, 0x5f, 0x66, 0x69,
    0x6e, 0x69, 0x73, 0x68, 0x65, 0x64, 0x18, 0x11, 0x20, 0x01, 0x28, 0x08, 0x52, 0x18, 0x75, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x45, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x73, 0x46, 0x69,
    0x6e, 0x69, 0x73, 0x68, 0x65, 0x64, 0x12, 0x20, 0x0a, 0x0b, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x61,
    0x6c, 0x69, 0x7a, 0x65, 0x64, 0x18, 0x12, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0b, 0x69, 0x6e, 0x69,
    0x74, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x61, 0x6c, 0x69, 0x76,
    0x65, 0x18, 0x13, 0x20, 0x01, 0x28, 0x08, 0x52, 0x05, 0x61, 0x6c, 0x69, 0x76, 0x65, 0x12, 0x18,
    0x0a, 0x07, 0x73, 0x75, 0x73, 0x70, 0x65, 0x63, 0x74, 0x18, 0x14, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x07, 0x73, 0x75, 0x73, 0x70, 0x65, 0x63, 0x74, 0x12, 0x1c, 0x0a, 0x09, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x72, 0x6d, 0x65, 0x64, 0x18, 0x15, 0x20, 0x01, 0x28, 0x08, 0x52, 0x09, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x72, 0x6d, 0x65, 0x64, 0x12, 0x1e, 0x0a, 0x0a, 0x70, 0x65, 0x72, 0x73, 0x69, 0x73,
    0x74, 0x65, 0x6e, 0x74, 0x18, 0x16, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x70, 0x65, 0x72, 0x73,
    0x69, 0x73, 0x74, 0x65, 0x6e, 0x74, 0x4a, 0x83, 0x17, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x38,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x02, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x02, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04, 0x00, 0x12, 0x04, 0x03, 0x02, 0x07,
    0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x03, 0x07, 0x0b, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x04, 0x11, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x04, 0x0c, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x04, 0x0f, 0x10, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x05, 0x04, 0x0d, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x05, 0x04, 0x08, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x05, 0x0b, 0x0c, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x06, 0x04, 0x0d, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x06, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x06, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x09, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x09, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x17,
    0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x02, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x0a, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x0b, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0b, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x12, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x1e, 0x1f, 0x0a, 0x2e, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x02, 0x20, 0x22, 0x21, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x3a,
    0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x62,
    0x65, 0x20, 0x61, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x0c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x0c, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x0c, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x0d, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x10, 0x00, 0x17, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x10, 0x08, 0x0f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x11, 0x02, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x11, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x11, 0x12, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x11, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x12,
    0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x12, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x12, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x12, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x02, 0x12, 0x03, 0x13, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x13, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x13, 0x12,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x13, 0x1e, 0x1f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x14, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x14, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x14, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x14, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x15, 0x02,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x15, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x15, 0x12, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x15, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x05, 0x12, 0x03, 0x16, 0x02, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x04, 0x12,
    0x03, 0x16, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12, 0x03, 0x16,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x16, 0x12, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x16, 0x26, 0x27, 0x0a, 0x56,
    0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x1a, 0x00, 0x1f, 0x01, 0x1a, 0x4a, 0x20, 0x69, 0x64, 0x65,
    0x61, 0x6c, 0x6c, 0x79, 0x20, 0x77, 0x65, 0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x70, 0x75,
    0x6c, 0x6c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x49,
    0x64, 0x65, 0x6e, 0x74, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x66, 0x72, 0x6f, 0x6d, 0x20, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x65, 0x72, 0x2d, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x1a,
    0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x02, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x1b, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x1c, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1c,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1c, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1c, 0x12, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1c, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1d, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x1d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x1d, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1d, 0x1c,
    0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1e, 0x02, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x1e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x1e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x1e, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x1e, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x21, 0x00,
    0x38, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x21, 0x08, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x22, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x22, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x22, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x22, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x22, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x23, 0x02, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x23, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x23, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x23, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x23, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02,
    0x12, 0x03, 0x24, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x24, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x24, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x24, 0x12, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x24, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x25, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x25, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x25, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x25, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x25,
    0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x26, 0x02, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x03, 0x26, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x04, 0x05, 0x12, 0x03, 0x26, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x26, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x26, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x05, 0x12,
    0x03, 0x27, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x04, 0x12, 0x03, 0x27,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x06, 0x12, 0x03, 0x27, 0x0b, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x01, 0x12, 0x03, 0x27, 0x13, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x03, 0x12, 0x03, 0x27, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x06, 0x12, 0x03, 0x28, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x06, 0x04, 0x12, 0x03, 0x28, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x06,
    0x12, 0x03, 0x28, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x01, 0x12, 0x03,
    0x28, 0x18, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x03, 0x12, 0x03, 0x28, 0x1e,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x07, 0x12, 0x03, 0x29, 0x02, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x04, 0x12, 0x03, 0x29, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x07, 0x05, 0x12, 0x03, 0x29, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x07, 0x01, 0x12, 0x03, 0x29, 0x10, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07,
    0x03, 0x12, 0x03, 0x29, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x08, 0x12, 0x03,
    0x2a, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x04, 0x12, 0x03, 0x2a, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x05, 0x12, 0x03, 0x2a, 0x0b, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x01, 0x12, 0x03, 0x2a, 0x10, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x08, 0x03, 0x12, 0x03, 0x2a, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x09, 0x12, 0x03, 0x2b, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x09,
    0x04, 0x12, 0x03, 0x2b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x09, 0x05, 0x12,
    0x03, 0x2b, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x09, 0x01, 0x12, 0x03, 0x2b,
    0x10, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x09, 0x03, 0x12, 0x03, 0x2b, 0x20, 0x22,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x0a, 0x12, 0x03, 0x2c, 0x02, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x2c, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x0a, 0x01, 0x12, 0x03, 0x2c, 0x10, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0a, 0x03,
    0x12, 0x03, 0x2c, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x0b, 0x12, 0x03, 0x2d,
    0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x2d, 0x0b, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x2d, 0x10, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x2d, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x0c, 0x12, 0x03, 0x2e, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0c, 0x04,
    0x12, 0x03, 0x2e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0c, 0x05, 0x12, 0x03,
    0x2e, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x2e, 0x10,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x2e, 0x28, 0x2a, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x0d, 0x12, 0x03, 0x2f, 0x02, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x2f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x0d, 0x05, 0x12, 0x03, 0x2f, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0d,
    0x01, 0x12, 0x03, 0x2f, 0x10, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0d, 0x03, 0x12,
    0x03, 0x2f, 0x27, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x0e, 0x12, 0x03, 0x30, 0x02,
    0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0e, 0x04, 0x12, 0x03, 0x30, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0e, 0x05, 0x12, 0x03, 0x30, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x30, 0x10, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x30, 0x2d, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x0f, 0x12, 0x03, 0x31, 0x02, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0f, 0x04, 0x12,
    0x03, 0x31, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0f, 0x05, 0x12, 0x03, 0x31,
    0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x31, 0x10, 0x2c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x31, 0x2f, 0x31, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x10, 0x12, 0x03, 0x32, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x10, 0x04, 0x12, 0x03, 0x32, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x10, 0x05, 0x12, 0x03, 0x32, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x10, 0x01,
    0x12, 0x03, 0x32, 0x10, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x10, 0x03, 0x12, 0x03,
    0x32, 0x2e, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x11, 0x12, 0x03, 0x33, 0x02, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x11, 0x04, 0x12, 0x03, 0x33, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x11, 0x05, 0x12, 0x03, 0x33, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x11, 0x01, 0x12, 0x03, 0x33, 0x10, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x11, 0x03, 0x12, 0x03, 0x33, 0x1e, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x12,
    0x12, 0x03, 0x34, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x12, 0x04, 0x12, 0x03,
    0x34, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x12, 0x05, 0x12, 0x03, 0x34, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x12, 0x01, 0x12, 0x03, 0x34, 0x10, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x12, 0x03, 0x12, 0x03, 0x34, 0x18, 0x1a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x13, 0x12, 0x03, 0x35, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x13, 0x04, 0x12, 0x03, 0x35, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x13,
    0x05, 0x12, 0x03, 0x35, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x13, 0x01, 0x12,
    0x03, 0x35, 0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x13, 0x03, 0x12, 0x03, 0x35,
    0x1a, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x14, 0x12, 0x03, 0x36, 0x02, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x14, 0x04, 0x12, 0x03, 0x36, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x14, 0x05, 0x12, 0x03, 0x36, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x14, 0x01, 0x12, 0x03, 0x36, 0x10, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x14, 0x03, 0x12, 0x03, 0x36, 0x1c, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x15, 0x12,
    0x03, 0x37, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x15, 0x04, 0x12, 0x03, 0x37,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x15, 0x05, 0x12, 0x03, 0x37, 0x0b, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x15, 0x01, 0x12, 0x03, 0x37, 0x10, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x15, 0x03, 0x12, 0x03, 0x37, 0x1d, 0x1f,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
