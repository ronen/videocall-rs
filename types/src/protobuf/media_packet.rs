// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `types/media_packet.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:MediaPacket)
pub struct MediaPacket {
    // message fields
    // @@protoc_insertion_point(field:MediaPacket.email)
    pub email: ::std::string::String,
    // @@protoc_insertion_point(field:MediaPacket.video)
    pub video: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:MediaPacket.audio)
    pub audio: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:MediaPacket.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MediaPacket {
    fn default() -> &'a MediaPacket {
        <MediaPacket as ::protobuf::Message>::default_instance()
    }
}

impl MediaPacket {
    pub fn new() -> MediaPacket {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "email",
            |m: &MediaPacket| { &m.email },
            |m: &mut MediaPacket| { &mut m.email },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "video",
            |m: &MediaPacket| { &m.video },
            |m: &mut MediaPacket| { &mut m.video },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "audio",
            |m: &MediaPacket| { &m.audio },
            |m: &mut MediaPacket| { &mut m.audio },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MediaPacket>(
            "MediaPacket",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MediaPacket {
    const NAME: &'static str = "MediaPacket";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.email = is.read_string()?;
                },
                18 => {
                    self.video = is.read_bytes()?;
                },
                26 => {
                    self.audio = is.read_bytes()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.email.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.email);
        }
        if !self.video.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.video);
        }
        if !self.audio.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.audio);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.email.is_empty() {
            os.write_string(1, &self.email)?;
        }
        if !self.video.is_empty() {
            os.write_bytes(2, &self.video)?;
        }
        if !self.audio.is_empty() {
            os.write_bytes(3, &self.audio)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MediaPacket {
        MediaPacket::new()
    }

    fn clear(&mut self) {
        self.email.clear();
        self.video.clear();
        self.audio.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MediaPacket {
        static instance: MediaPacket = MediaPacket {
            email: ::std::string::String::new(),
            video: ::std::vec::Vec::new(),
            audio: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MediaPacket {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MediaPacket").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MediaPacket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MediaPacket {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18types/media_packet.proto\"O\n\x0bMediaPacket\x12\x14\n\x05email\
    \x18\x01\x20\x01(\tR\x05email\x12\x14\n\x05video\x18\x02\x20\x01(\x0cR\
    \x05video\x12\x14\n\x05audio\x18\x03\x20\x01(\x0cR\x05audioJ\xcf\x01\n\
    \x06\x12\x04\0\0\x06\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\
    \x12\x04\x02\0\x06\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x13\n\x0b\n\
    \x04\x04\0\x02\0\x12\x03\x03\x02\x13\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\
    \x03\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\t\x0e\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\x03\x11\x12\n\x0b\n\x04\x04\0\x02\x01\x12\x03\
    \x04\x02\x12\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x04\x02\x07\n\x0c\n\
    \x05\x04\0\x02\x01\x01\x12\x03\x04\x08\r\n\x0c\n\x05\x04\0\x02\x01\x03\
    \x12\x03\x04\x10\x11\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x05\x02\x12\n\x0c\
    \n\x05\x04\0\x02\x02\x05\x12\x03\x05\x02\x07\n\x0c\n\x05\x04\0\x02\x02\
    \x01\x12\x03\x05\x08\r\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x05\x10\x11\
    b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MediaPacket::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
