/* GENERATED WITH SIDEX. DO NOT MODIFY! */

pub mod bsp {
    #![doc = "Rugix BSP configuration (`rugix-bsp.toml`).\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "BSP configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct BspConfig {
        #[doc = "BSP metadata.\n"]
        pub bsp: ::std::option::Option<BspMetadata>,
        #[doc = "Image configuration.\n"]
        pub image: BspImage,
        #[doc = "Bundle payload mapping.\n"]
        pub bundle: ::std::option::Option<BspBundle>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for BspConfig {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::bsp::BspConfig",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("bsp", ::sidex::shape::shape_of::<BspMetadata>)
                    .with_docs("BSP metadata.\n")
                    .optional(),
                ::sidex::shape::FieldShape::new("image", ::sidex::shape::shape_of::<BspImage>)
                    .with_docs("Image configuration.\n"),
                ::sidex::shape::FieldShape::new("bundle", ::sidex::shape::shape_of::<BspBundle>)
                    .with_docs("Bundle payload mapping.\n")
                    .optional(),
            ])),
        )
        .with_docs("BSP configuration.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[::sidex::shape::Attr::Assign {
                path: "rename_all",
                value: ::sidex::shape::AttrValue::String("kebab-case"),
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for BspConfig {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for BspConfig {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .bsp
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.image)),
                2usize => self
                    .bundle
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for BspConfig {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<BspMetadata>::None;
            let mut __field_1 = ::std::option::Option::<BspImage>::None;
            let mut __field_2 = ::std::option::Option::<BspBundle>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <BspMetadata as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <BspImage as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <BspBundle as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                bsp: __field_0,
                image: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `image`"))?,
                bundle: __field_2,
            })
        }
    }
    impl BspConfig {
        #[doc = "Creates a new [`BspConfig`]."]
        pub fn new(image: BspImage) -> Self {
            Self {
                image: image,
                bsp: ::std::default::Default::default(),
                bundle: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `bsp`."]
        pub fn set_bsp(&mut self, bsp: ::std::option::Option<BspMetadata>) -> &mut Self {
            self.bsp = bsp;
            self
        }
        #[doc = "Sets the value of `bsp`."]
        pub fn with_bsp(mut self, bsp: ::std::option::Option<BspMetadata>) -> Self {
            self.bsp = bsp;
            self
        }
        #[doc = "Sets the value of `image`."]
        pub fn set_image(&mut self, image: BspImage) -> &mut Self {
            self.image = image;
            self
        }
        #[doc = "Sets the value of `image`."]
        pub fn with_image(mut self, image: BspImage) -> Self {
            self.image = image;
            self
        }
        #[doc = "Sets the value of `bundle`."]
        pub fn set_bundle(&mut self, bundle: ::std::option::Option<BspBundle>) -> &mut Self {
            self.bundle = bundle;
            self
        }
        #[doc = "Sets the value of `bundle`."]
        pub fn with_bundle(mut self, bundle: ::std::option::Option<BspBundle>) -> Self {
            self.bundle = bundle;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for BspConfig {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for BspConfig {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "BSP image configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct BspImage {
        #[doc = "Image layout.\n"]
        pub layout: BspImageLayout,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for BspImage {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::bsp::BspImage",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "layout",
                    ::sidex::shape::shape_of::<BspImageLayout>,
                )
                .with_docs("Image layout.\n"),
            ])),
        )
        .with_docs("BSP image configuration.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for BspImage {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for BspImage {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.layout)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for BspImage {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<BspImageLayout>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <BspImageLayout as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                layout: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `layout`"))?,
            })
        }
    }
    impl BspImage {
        #[doc = "Creates a new [`BspImage`]."]
        pub fn new(layout: BspImageLayout) -> Self {
            Self { layout: layout }
        }
        #[doc = "Sets the value of `layout`."]
        pub fn set_layout(&mut self, layout: BspImageLayout) -> &mut Self {
            self.layout = layout;
            self
        }
        #[doc = "Sets the value of `layout`."]
        pub fn with_layout(mut self, layout: BspImageLayout) -> Self {
            self.layout = layout;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for BspImage {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for BspImage {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "BSP metadata.\n"]
    #[derive(Clone, Debug)]
    pub struct BspMetadata {
        #[doc = "Name of the BSP.\n"]
        pub name: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Architecture.\n"]
        pub architecture: ::std::option::Option<::sidex::types::Str>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for BspMetadata {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::bsp::BspMetadata",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "name",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Name of the BSP.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "architecture",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Architecture.\n")
                .optional(),
            ])),
        )
        .with_docs("BSP metadata.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for BspMetadata {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for BspMetadata {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .name
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => self
                    .architecture
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for BspMetadata {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<::sidex::types::Str>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                name: __field_0,
                architecture: __field_1,
            })
        }
    }
    impl BspMetadata {
        #[doc = "Creates a new [`BspMetadata`]."]
        pub fn new() -> Self {
            Self {
                name: ::std::default::Default::default(),
                architecture: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(&mut self, name: ::std::option::Option<::sidex::types::Str>) -> &mut Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: ::std::option::Option<::sidex::types::Str>) -> Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `architecture`."]
        pub fn set_architecture(
            &mut self,
            architecture: ::std::option::Option<::sidex::types::Str>,
        ) -> &mut Self {
            self.architecture = architecture;
            self
        }
        #[doc = "Sets the value of `architecture`."]
        pub fn with_architecture(
            mut self,
            architecture: ::std::option::Option<::sidex::types::Str>,
        ) -> Self {
            self.architecture = architecture;
            self
        }
    }
    impl ::std::default::Default for BspMetadata {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for BspMetadata {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for BspMetadata {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Image layout specification.\n"]
    #[derive(Clone, Debug)]
    pub struct BspImageLayout {
        #[doc = "Partition table type.\n"]
        pub ty: super::images::PartitionTableType,
        #[doc = "Raw firmware blobs written at fixed byte offsets.\n"]
        pub raw_blobs: ::std::option::Option<::sidex::types::Seq<BspRawBlob>>,
        #[doc = "Partitions.\n"]
        pub partitions: ::std::option::Option<::sidex::types::Seq<BspPartition>>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for BspImageLayout {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::bsp::BspImageLayout",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "ty",
                    ::sidex::shape::shape_of::<super::images::PartitionTableType>,
                )
                .with_docs("Partition table type.\n")
                .with_attrs(&[::sidex::shape::Attr::List {
                    path: "json",
                    args: &[::sidex::shape::Attr::Assign {
                        path: "name",
                        value: ::sidex::shape::AttrValue::String("type"),
                    }],
                }])
                .json_name("type"),
                ::sidex::shape::FieldShape::new(
                    "raw_blobs",
                    ::sidex::shape::shape_of::<::sidex::types::Seq<BspRawBlob>>,
                )
                .with_docs("Raw firmware blobs written at fixed byte offsets.\n")
                .optional()
                .json_name("raw-blobs"),
                ::sidex::shape::FieldShape::new(
                    "partitions",
                    ::sidex::shape::shape_of::<::sidex::types::Seq<BspPartition>>,
                )
                .with_docs("Partitions.\n")
                .optional(),
            ])),
        )
        .with_docs("Image layout specification.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[::sidex::shape::Attr::Assign {
                path: "rename_all",
                value: ::sidex::shape::AttrValue::String("kebab-case"),
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for BspImageLayout {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for BspImageLayout {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.ty)),
                1usize => self
                    .raw_blobs
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .partitions
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for BspImageLayout {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<super::images::PartitionTableType>::None;
            let mut __field_1 = ::std::option::Option::<::sidex::types::Seq<BspRawBlob>>::None;
            let mut __field_2 = ::std::option::Option::<::sidex::types::Seq<BspPartition>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <super::images::PartitionTableType as ::sidex::Decode>::decode(
                                decoder,
                            )?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::sidex::types::Seq<BspRawBlob> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::sidex::types::Seq<BspPartition> as ::sidex::Decode>::decode(
                                decoder,
                            )?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                ty: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `type`"))?,
                raw_blobs: __field_1,
                partitions: __field_2,
            })
        }
    }
    impl BspImageLayout {
        #[doc = "Creates a new [`BspImageLayout`]."]
        pub fn new(ty: super::images::PartitionTableType) -> Self {
            Self {
                ty: ty,
                raw_blobs: ::std::default::Default::default(),
                partitions: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `ty`."]
        pub fn set_ty(&mut self, ty: super::images::PartitionTableType) -> &mut Self {
            self.ty = ty;
            self
        }
        #[doc = "Sets the value of `ty`."]
        pub fn with_ty(mut self, ty: super::images::PartitionTableType) -> Self {
            self.ty = ty;
            self
        }
        #[doc = "Sets the value of `raw_blobs`."]
        pub fn set_raw_blobs(
            &mut self,
            raw_blobs: ::std::option::Option<::sidex::types::Seq<BspRawBlob>>,
        ) -> &mut Self {
            self.raw_blobs = raw_blobs;
            self
        }
        #[doc = "Sets the value of `raw_blobs`."]
        pub fn with_raw_blobs(
            mut self,
            raw_blobs: ::std::option::Option<::sidex::types::Seq<BspRawBlob>>,
        ) -> Self {
            self.raw_blobs = raw_blobs;
            self
        }
        #[doc = "Sets the value of `partitions`."]
        pub fn set_partitions(
            &mut self,
            partitions: ::std::option::Option<::sidex::types::Seq<BspPartition>>,
        ) -> &mut Self {
            self.partitions = partitions;
            self
        }
        #[doc = "Sets the value of `partitions`."]
        pub fn with_partitions(
            mut self,
            partitions: ::std::option::Option<::sidex::types::Seq<BspPartition>>,
        ) -> Self {
            self.partitions = partitions;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for BspImageLayout {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for BspImageLayout {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Raw firmware blob written at a fixed offset on the disk.\n"]
    #[derive(Clone, Debug)]
    pub struct BspRawBlob {
        #[doc = "Path to the blob file (relative to the BSP archive root).\n"]
        pub file: ::sidex::types::Str,
        #[doc = "Byte offset from the start of the disk image.\n"]
        pub offset: super::foreign::NumBytes,
        #[doc = "Reserved size for the blob region. When set, partition offsets are\ncomputed from `offset + size` instead of `offset + file_size`. Must\nbe at least as large as the actual blob file.\n"]
        pub size: ::std::option::Option<super::foreign::NumBytes>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for BspRawBlob {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::rugix_bakery::bsp::BspRawBlob" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("file" , :: sidex :: shape :: shape_of :: < :: sidex :: types :: Str > ,) . with_docs ("Path to the blob file (relative to the BSP archive root).\n") , :: sidex :: shape :: FieldShape :: new ("offset" , :: sidex :: shape :: shape_of :: < super :: foreign :: NumBytes > ,) . with_docs ("Byte offset from the start of the disk image.\n") , :: sidex :: shape :: FieldShape :: new ("size" , :: sidex :: shape :: shape_of :: < super :: foreign :: NumBytes > ,) . with_docs ("Reserved size for the blob region. When set, partition offsets are\ncomputed from `offset + size` instead of `offset + file_size`. Must\nbe at least as large as the actual blob file.\n") . optional ()]))) . with_docs ("Raw firmware blob written at a fixed offset on the disk.\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for BspRawBlob {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for BspRawBlob {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.file)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.offset)),
                2usize => self
                    .size
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for BspRawBlob {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<super::foreign::NumBytes>::None;
            let mut __field_2 = ::std::option::Option::<super::foreign::NumBytes>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <super::foreign::NumBytes as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <super::foreign::NumBytes as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                file: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `file`"))?,
                offset: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `offset`"))?,
                size: __field_2,
            })
        }
    }
    impl BspRawBlob {
        #[doc = "Creates a new [`BspRawBlob`]."]
        pub fn new(
            file: impl ::std::convert::Into<::sidex::types::Str>,
            offset: super::foreign::NumBytes,
        ) -> Self {
            Self {
                file: file.into(),
                offset: offset,
                size: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `file`."]
        pub fn set_file(
            &mut self,
            file: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.file = file.into();
            self
        }
        #[doc = "Sets the value of `file`."]
        pub fn with_file(mut self, file: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            self.file = file.into();
            self
        }
        #[doc = "Sets the value of `offset`."]
        pub fn set_offset(&mut self, offset: super::foreign::NumBytes) -> &mut Self {
            self.offset = offset;
            self
        }
        #[doc = "Sets the value of `offset`."]
        pub fn with_offset(mut self, offset: super::foreign::NumBytes) -> Self {
            self.offset = offset;
            self
        }
        #[doc = "Sets the value of `size`."]
        pub fn set_size(
            &mut self,
            size: ::std::option::Option<super::foreign::NumBytes>,
        ) -> &mut Self {
            self.size = size;
            self
        }
        #[doc = "Sets the value of `size`."]
        pub fn with_size(mut self, size: ::std::option::Option<super::foreign::NumBytes>) -> Self {
            self.size = size;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for BspRawBlob {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for BspRawBlob {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Partition in the BSP disk layout.\n"]
    #[derive(Clone, Debug)]
    pub struct BspPartition {
        #[doc = "Partition label.\n"]
        pub name: ::sidex::types::Str,
        #[doc = "Size of the partition.\n"]
        pub size: ::std::option::Option<super::foreign::NumBytes>,
        #[doc = "Filesystem type and options.\n"]
        pub filesystem: ::std::option::Option<super::images::Filesystem>,
        #[doc = "Partition type (GUID or MBR hex value).\n"]
        pub ty: ::std::option::Option<super::images::PartitionType>,
        #[doc = "Root directory name in the layer.\n"]
        pub root: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Mark the partition bootable.\n"]
        pub bootable: ::std::option::Option<bool>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for BspPartition {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::bsp::BspPartition",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "name",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Partition label.\n"),
                ::sidex::shape::FieldShape::new(
                    "size",
                    ::sidex::shape::shape_of::<super::foreign::NumBytes>,
                )
                .with_docs("Size of the partition.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "filesystem",
                    ::sidex::shape::shape_of::<super::images::Filesystem>,
                )
                .with_docs("Filesystem type and options.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "ty",
                    ::sidex::shape::shape_of::<super::images::PartitionType>,
                )
                .with_docs("Partition type (GUID or MBR hex value).\n")
                .with_attrs(&[::sidex::shape::Attr::List {
                    path: "json",
                    args: &[::sidex::shape::Attr::Assign {
                        path: "name",
                        value: ::sidex::shape::AttrValue::String("type"),
                    }],
                }])
                .optional()
                .json_name("type"),
                ::sidex::shape::FieldShape::new(
                    "root",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Root directory name in the layer.\n")
                .optional(),
                ::sidex::shape::FieldShape::new("bootable", ::sidex::shape::shape_of::<bool>)
                    .with_docs("Mark the partition bootable.\n")
                    .optional(),
            ])),
        )
        .with_docs("Partition in the BSP disk layout.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for BspPartition {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for BspPartition {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.name)),
                1usize => self
                    .size
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .filesystem
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                3usize => self.ty.as_ref().map(|value| ::sidex::Encode::encode(value)),
                4usize => self
                    .root
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                5usize => self
                    .bootable
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for BspPartition {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<super::foreign::NumBytes>::None;
            let mut __field_2 = ::std::option::Option::<super::images::Filesystem>::None;
            let mut __field_3 = ::std::option::Option::<super::images::PartitionType>::None;
            let mut __field_4 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_5 = ::std::option::Option::<bool>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <super::foreign::NumBytes as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <super::images::Filesystem as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <super::images::PartitionType as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    5usize => {
                        __field_5 =
                            ::std::option::Option::Some(<bool as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                name: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `name`"))?,
                size: __field_1,
                filesystem: __field_2,
                ty: __field_3,
                root: __field_4,
                bootable: __field_5,
            })
        }
    }
    impl BspPartition {
        #[doc = "Creates a new [`BspPartition`]."]
        pub fn new(name: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            Self {
                name: name.into(),
                size: ::std::default::Default::default(),
                filesystem: ::std::default::Default::default(),
                ty: ::std::default::Default::default(),
                root: ::std::default::Default::default(),
                bootable: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(
            &mut self,
            name: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.name = name.into();
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            self.name = name.into();
            self
        }
        #[doc = "Sets the value of `size`."]
        pub fn set_size(
            &mut self,
            size: ::std::option::Option<super::foreign::NumBytes>,
        ) -> &mut Self {
            self.size = size;
            self
        }
        #[doc = "Sets the value of `size`."]
        pub fn with_size(mut self, size: ::std::option::Option<super::foreign::NumBytes>) -> Self {
            self.size = size;
            self
        }
        #[doc = "Sets the value of `filesystem`."]
        pub fn set_filesystem(
            &mut self,
            filesystem: ::std::option::Option<super::images::Filesystem>,
        ) -> &mut Self {
            self.filesystem = filesystem;
            self
        }
        #[doc = "Sets the value of `filesystem`."]
        pub fn with_filesystem(
            mut self,
            filesystem: ::std::option::Option<super::images::Filesystem>,
        ) -> Self {
            self.filesystem = filesystem;
            self
        }
        #[doc = "Sets the value of `ty`."]
        pub fn set_ty(
            &mut self,
            ty: ::std::option::Option<super::images::PartitionType>,
        ) -> &mut Self {
            self.ty = ty;
            self
        }
        #[doc = "Sets the value of `ty`."]
        pub fn with_ty(mut self, ty: ::std::option::Option<super::images::PartitionType>) -> Self {
            self.ty = ty;
            self
        }
        #[doc = "Sets the value of `root`."]
        pub fn set_root(&mut self, root: ::std::option::Option<::sidex::types::Str>) -> &mut Self {
            self.root = root;
            self
        }
        #[doc = "Sets the value of `root`."]
        pub fn with_root(mut self, root: ::std::option::Option<::sidex::types::Str>) -> Self {
            self.root = root;
            self
        }
        #[doc = "Sets the value of `bootable`."]
        pub fn set_bootable(&mut self, bootable: ::std::option::Option<bool>) -> &mut Self {
            self.bootable = bootable;
            self
        }
        #[doc = "Sets the value of `bootable`."]
        pub fn with_bootable(mut self, bootable: ::std::option::Option<bool>) -> Self {
            self.bootable = bootable;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for BspPartition {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for BspPartition {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Bundle payload mapping.\n"]
    #[derive(Clone, Debug)]
    pub struct BspBundle {
        #[doc = "Payload definitions for update bundles.\n"]
        pub payloads: ::sidex::types::Seq<BspBundlePayload>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for BspBundle {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::bsp::BspBundle",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "payloads",
                    ::sidex::shape::shape_of::<::sidex::types::Seq<BspBundlePayload>>,
                )
                .with_docs("Payload definitions for update bundles.\n"),
            ])),
        )
        .with_docs("Bundle payload mapping.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for BspBundle {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for BspBundle {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.payloads)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for BspBundle {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 =
                ::std::option::Option::<::sidex::types::Seq<BspBundlePayload>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(<::sidex::types::Seq<
                            BspBundlePayload,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                payloads: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `payloads`"))?,
            })
        }
    }
    impl BspBundle {
        #[doc = "Creates a new [`BspBundle`]."]
        pub fn new(payloads: ::sidex::types::Seq<BspBundlePayload>) -> Self {
            Self { payloads: payloads }
        }
        #[doc = "Sets the value of `payloads`."]
        pub fn set_payloads(
            &mut self,
            payloads: ::sidex::types::Seq<BspBundlePayload>,
        ) -> &mut Self {
            self.payloads = payloads;
            self
        }
        #[doc = "Sets the value of `payloads`."]
        pub fn with_payloads(mut self, payloads: ::sidex::types::Seq<BspBundlePayload>) -> Self {
            self.payloads = payloads;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for BspBundle {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for BspBundle {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Bundle payload mapping entry.\n"]
    #[derive(Clone, Debug)]
    pub struct BspBundlePayload {
        #[doc = "Slot name in rugix-ctrl.\n"]
        pub slot: ::sidex::types::Str,
        #[doc = "Partition number (1-based) whose filesystem image is the payload.\n"]
        pub partition: ::std::option::Option<u32>,
        #[doc = "File path (relative to artifacts) for file-based payloads.\n"]
        pub file: ::std::option::Option<::sidex::types::Str>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for BspBundlePayload {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::bsp::BspBundlePayload",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "slot",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Slot name in rugix-ctrl.\n"),
                ::sidex::shape::FieldShape::new("partition", ::sidex::shape::shape_of::<u32>)
                    .with_docs(
                        "Partition number (1-based) whose filesystem image is the payload.\n",
                    )
                    .optional(),
                ::sidex::shape::FieldShape::new(
                    "file",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("File path (relative to artifacts) for file-based payloads.\n")
                .optional(),
            ])),
        )
        .with_docs("Bundle payload mapping entry.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for BspBundlePayload {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for BspBundlePayload {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.slot)),
                1usize => self
                    .partition
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .file
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for BspBundlePayload {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<u32>::None;
            let mut __field_2 = ::std::option::Option::<::sidex::types::Str>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 =
                            ::std::option::Option::Some(<u32 as ::sidex::Decode>::decode(decoder)?)
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                slot: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `slot`"))?,
                partition: __field_1,
                file: __field_2,
            })
        }
    }
    impl BspBundlePayload {
        #[doc = "Creates a new [`BspBundlePayload`]."]
        pub fn new(slot: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            Self {
                slot: slot.into(),
                partition: ::std::default::Default::default(),
                file: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `slot`."]
        pub fn set_slot(
            &mut self,
            slot: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.slot = slot.into();
            self
        }
        #[doc = "Sets the value of `slot`."]
        pub fn with_slot(mut self, slot: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            self.slot = slot.into();
            self
        }
        #[doc = "Sets the value of `partition`."]
        pub fn set_partition(&mut self, partition: ::std::option::Option<u32>) -> &mut Self {
            self.partition = partition;
            self
        }
        #[doc = "Sets the value of `partition`."]
        pub fn with_partition(mut self, partition: ::std::option::Option<u32>) -> Self {
            self.partition = partition;
            self
        }
        #[doc = "Sets the value of `file`."]
        pub fn set_file(&mut self, file: ::std::option::Option<::sidex::types::Str>) -> &mut Self {
            self.file = file;
            self
        }
        #[doc = "Sets the value of `file`."]
        pub fn with_file(mut self, file: ::std::option::Option<::sidex::types::Str>) -> Self {
            self.file = file;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for BspBundlePayload {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for BspBundlePayload {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
}
pub mod build_context {
    #![doc = "Metadata supplied to repository-defined system assembly.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "Release metadata exposed as the context artifact during image assembly.\n"]
    #[derive(Clone, Debug)]
    pub struct BuildContext {
        #[doc = ""]
        pub system: ::sidex::types::Str,
        #[doc = ""]
        pub architecture: super::systems::Architecture,
        #[doc = ""]
        pub source_date_epoch: u64,
        #[doc = ""]
        pub release_id: ::sidex::types::Str,
        #[doc = ""]
        pub release_version: ::sidex::types::Str,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for BuildContext {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::build_context::BuildContext",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "system",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                ),
                ::sidex::shape::FieldShape::new(
                    "architecture",
                    ::sidex::shape::shape_of::<super::systems::Architecture>,
                ),
                ::sidex::shape::FieldShape::new(
                    "source_date_epoch",
                    ::sidex::shape::shape_of::<u64>,
                )
                .json_name("source-date-epoch"),
                ::sidex::shape::FieldShape::new(
                    "release_id",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .json_name("release-id"),
                ::sidex::shape::FieldShape::new(
                    "release_version",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .json_name("release-version"),
            ])),
        )
        .with_docs("Release metadata exposed as the context artifact during image assembly.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[::sidex::shape::Attr::Assign {
                path: "rename_all",
                value: ::sidex::shape::AttrValue::String("kebab-case"),
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for BuildContext {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for BuildContext {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.system)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.architecture)),
                2usize => {
                    ::std::option::Option::Some(::sidex::Encode::encode(&self.source_date_epoch))
                }
                3usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.release_id)),
                4usize => {
                    ::std::option::Option::Some(::sidex::Encode::encode(&self.release_version))
                }
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for BuildContext {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<super::systems::Architecture>::None;
            let mut __field_2 = ::std::option::Option::<u64>::None;
            let mut __field_3 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_4 = ::std::option::Option::<::sidex::types::Str>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <super::systems::Architecture as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 =
                            ::std::option::Option::Some(<u64 as ::sidex::Decode>::decode(decoder)?)
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                system: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `system`"))?,
                architecture: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `architecture`"))?,
                source_date_epoch: __field_2.ok_or_else(|| {
                    ::sidex::DecodeError::new("missing field `source-date-epoch`")
                })?,
                release_id: __field_3
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `release-id`"))?,
                release_version: __field_4
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `release-version`"))?,
            })
        }
    }
    impl BuildContext {
        #[doc = "Creates a new [`BuildContext`]."]
        pub fn new(
            system: impl ::std::convert::Into<::sidex::types::Str>,
            architecture: super::systems::Architecture,
            source_date_epoch: u64,
            release_id: impl ::std::convert::Into<::sidex::types::Str>,
            release_version: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> Self {
            Self {
                system: system.into(),
                architecture: architecture,
                source_date_epoch: source_date_epoch,
                release_id: release_id.into(),
                release_version: release_version.into(),
            }
        }
        #[doc = "Sets the value of `system`."]
        pub fn set_system(
            &mut self,
            system: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.system = system.into();
            self
        }
        #[doc = "Sets the value of `system`."]
        pub fn with_system(
            mut self,
            system: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> Self {
            self.system = system.into();
            self
        }
        #[doc = "Sets the value of `architecture`."]
        pub fn set_architecture(
            &mut self,
            architecture: super::systems::Architecture,
        ) -> &mut Self {
            self.architecture = architecture;
            self
        }
        #[doc = "Sets the value of `architecture`."]
        pub fn with_architecture(mut self, architecture: super::systems::Architecture) -> Self {
            self.architecture = architecture;
            self
        }
        #[doc = "Sets the value of `source_date_epoch`."]
        pub fn set_source_date_epoch(&mut self, source_date_epoch: u64) -> &mut Self {
            self.source_date_epoch = source_date_epoch;
            self
        }
        #[doc = "Sets the value of `source_date_epoch`."]
        pub fn with_source_date_epoch(mut self, source_date_epoch: u64) -> Self {
            self.source_date_epoch = source_date_epoch;
            self
        }
        #[doc = "Sets the value of `release_id`."]
        pub fn set_release_id(
            &mut self,
            release_id: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.release_id = release_id.into();
            self
        }
        #[doc = "Sets the value of `release_id`."]
        pub fn with_release_id(
            mut self,
            release_id: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> Self {
            self.release_id = release_id.into();
            self
        }
        #[doc = "Sets the value of `release_version`."]
        pub fn set_release_version(
            &mut self,
            release_version: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.release_version = release_version.into();
            self
        }
        #[doc = "Sets the value of `release_version`."]
        pub fn with_release_version(
            mut self,
            release_version: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> Self {
            self.release_version = release_version.into();
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for BuildContext {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for BuildContext {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
}
pub mod catalog {
    #![doc = "Read-only repository definition catalogs for inspection and documentation.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "Kind of repository definition.\n"]
    #[derive(Clone, Debug, Copy, PartialEq, Eq)]
    pub enum DefinitionKind {
        #[doc = ""]
        Rule,
        #[doc = ""]
        Recipe,
        #[doc = ""]
        Layer,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for DefinitionKind {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::catalog::DefinitionKind",
            ::sidex::shape::ShapeKind::Variant(
                ::sidex::shape::VariantShape::new(&[
                    ::sidex::shape::CaseShape::new("Rule", ::std::option::Option::None)
                        .json_name("rule"),
                    ::sidex::shape::CaseShape::new("Recipe", ::std::option::Option::None)
                        .json_name("recipe"),
                    ::sidex::shape::CaseShape::new("Layer", ::std::option::Option::None)
                        .json_name("layer"),
                ])
                .json_tagging(::sidex::formats::json::Tagging::External),
            ),
        )
        .with_docs("Kind of repository definition.\n")
        .with_attrs(&[
            ::sidex::shape::Attr::List {
                path: "json",
                args: &[
                    ::sidex::shape::Attr::Assign {
                        path: "tagged",
                        value: ::sidex::shape::AttrValue::Path("externally"),
                    },
                    ::sidex::shape::Attr::Assign {
                        path: "rename_all",
                        value: ::sidex::shape::AttrValue::String("lowercase"),
                    },
                ],
            },
            ::sidex::shape::Attr::List {
                path: "rust",
                args: &[::sidex::shape::Attr::List {
                    path: "derive",
                    args: &[
                        ::sidex::shape::Attr::Path("Copy"),
                        ::sidex::shape::Attr::Path("PartialEq"),
                        ::sidex::shape::Attr::Path("Eq"),
                    ],
                }],
            },
        ]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for DefinitionKind {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for DefinitionKind {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Rule => 0usize,
                Self::Recipe => 1usize,
                Self::Layer => 2usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Rule => ::std::option::Option::None,
                Self::Recipe => ::std::option::Option::None,
                Self::Layer => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for DefinitionKind {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Rule,
                    1usize => Self::Recipe,
                    2usize => Self::Layer,
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for DefinitionKind {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for DefinitionKind {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "A typed definition returned by inspection.\n"]
    #[derive(Clone, Debug)]
    pub enum DefinitionInfo {
        #[doc = ""]
        Rule(RuleInfo),
        #[doc = ""]
        Recipe(RecipeInfo),
        #[doc = ""]
        Layer(LayerInfo),
    }
    #[automatically_derived]
    impl ::sidex::Shaped for DefinitionInfo {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::catalog::DefinitionInfo",
            ::sidex::shape::ShapeKind::Variant(
                ::sidex::shape::VariantShape::new(&[
                    ::sidex::shape::CaseShape::new(
                        "Rule",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<RuleInfo>),
                    )
                    .json_name("rule"),
                    ::sidex::shape::CaseShape::new(
                        "Recipe",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<RecipeInfo>),
                    )
                    .json_name("recipe"),
                    ::sidex::shape::CaseShape::new(
                        "Layer",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<LayerInfo>),
                    )
                    .json_name("layer"),
                ])
                .json_tagging(::sidex::formats::json::Tagging::Internal {
                    tag: "kind",
                    content: "content",
                }),
            ),
        )
        .with_docs("A typed definition returned by inspection.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[
                ::sidex::shape::Attr::Assign {
                    path: "tag",
                    value: ::sidex::shape::AttrValue::String("kind"),
                },
                ::sidex::shape::Attr::Assign {
                    path: "rename_all",
                    value: ::sidex::shape::AttrValue::String("lowercase"),
                },
            ],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for DefinitionInfo {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for DefinitionInfo {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Rule(_) => 0usize,
                Self::Recipe(_) => 1usize,
                Self::Layer(_) => 2usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Rule(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Recipe(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Layer(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for DefinitionInfo {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Rule(<RuleInfo as ::sidex::Decode>::decode(decoder)?),
                    1usize => Self::Recipe(<RecipeInfo as ::sidex::Decode>::decode(decoder)?),
                    2usize => Self::Layer(<LayerInfo as ::sidex::Decode>::decode(decoder)?),
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for DefinitionInfo {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for DefinitionInfo {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Sorted definitions visible from a project.\n"]
    #[derive(Clone, Debug)]
    pub enum DefinitionCatalog {
        #[doc = ""]
        Rule(::sidex::types::Seq<RuleInfo>),
        #[doc = ""]
        Recipe(::sidex::types::Seq<RecipeInfo>),
        #[doc = ""]
        Layer(::sidex::types::Seq<LayerInfo>),
    }
    #[automatically_derived]
    impl ::sidex::Shaped for DefinitionCatalog {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::catalog::DefinitionCatalog",
            ::sidex::shape::ShapeKind::Variant(
                ::sidex::shape::VariantShape::new(&[
                    ::sidex::shape::CaseShape::new(
                        "Rule",
                        ::std::option::Option::Some(
                            ::sidex::shape::shape_of::<::sidex::types::Seq<RuleInfo>>,
                        ),
                    )
                    .json_name("rule"),
                    ::sidex::shape::CaseShape::new(
                        "Recipe",
                        ::std::option::Option::Some(
                            ::sidex::shape::shape_of::<::sidex::types::Seq<RecipeInfo>>,
                        ),
                    )
                    .json_name("recipe"),
                    ::sidex::shape::CaseShape::new(
                        "Layer",
                        ::std::option::Option::Some(
                            ::sidex::shape::shape_of::<::sidex::types::Seq<LayerInfo>>,
                        ),
                    )
                    .json_name("layer"),
                ])
                .json_tagging(::sidex::formats::json::Tagging::Adjacent {
                    tag: "kind",
                    content: "definitions",
                }),
            ),
        )
        .with_docs("Sorted definitions visible from a project.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[
                ::sidex::shape::Attr::Assign {
                    path: "tagged",
                    value: ::sidex::shape::AttrValue::Path("adjacently"),
                },
                ::sidex::shape::Attr::Assign {
                    path: "tag",
                    value: ::sidex::shape::AttrValue::String("kind"),
                },
                ::sidex::shape::Attr::Assign {
                    path: "content",
                    value: ::sidex::shape::AttrValue::String("definitions"),
                },
                ::sidex::shape::Attr::Assign {
                    path: "rename_all",
                    value: ::sidex::shape::AttrValue::String("lowercase"),
                },
            ],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for DefinitionCatalog {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for DefinitionCatalog {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Rule(_) => 0usize,
                Self::Recipe(_) => 1usize,
                Self::Layer(_) => 2usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Rule(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Recipe(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Layer(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for DefinitionCatalog {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Rule(
                        <::sidex::types::Seq<RuleInfo> as ::sidex::Decode>::decode(decoder)?,
                    ),
                    1usize => Self::Recipe(
                        <::sidex::types::Seq<RecipeInfo> as ::sidex::Decode>::decode(decoder)?,
                    ),
                    2usize => Self::Layer(
                        <::sidex::types::Seq<LayerInfo> as ::sidex::Decode>::decode(decoder)?,
                    ),
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for DefinitionCatalog {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for DefinitionCatalog {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "A named rule and its optional README.md.\n"]
    #[derive(Clone, Debug)]
    pub struct RuleInfo {
        #[doc = ""]
        pub name: ::sidex::types::Str,
        #[doc = ""]
        pub definition: super::rules::RuleDefinition,
        #[doc = ""]
        pub documentation: ::std::option::Option<::sidex::types::Str>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for RuleInfo {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::catalog::RuleInfo",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "name",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                ),
                ::sidex::shape::FieldShape::new(
                    "definition",
                    ::sidex::shape::shape_of::<super::rules::RuleDefinition>,
                ),
                ::sidex::shape::FieldShape::new(
                    "documentation",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .optional(),
            ])),
        )
        .with_docs("A named rule and its optional README.md.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for RuleInfo {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for RuleInfo {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.name)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.definition)),
                2usize => self
                    .documentation
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for RuleInfo {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<super::rules::RuleDefinition>::None;
            let mut __field_2 = ::std::option::Option::<::sidex::types::Str>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <super::rules::RuleDefinition as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                name: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `name`"))?,
                definition: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `definition`"))?,
                documentation: __field_2,
            })
        }
    }
    impl RuleInfo {
        #[doc = "Creates a new [`RuleInfo`]."]
        pub fn new(
            name: impl ::std::convert::Into<::sidex::types::Str>,
            definition: super::rules::RuleDefinition,
        ) -> Self {
            Self {
                name: name.into(),
                definition: definition,
                documentation: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(
            &mut self,
            name: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.name = name.into();
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            self.name = name.into();
            self
        }
        #[doc = "Sets the value of `definition`."]
        pub fn set_definition(&mut self, definition: super::rules::RuleDefinition) -> &mut Self {
            self.definition = definition;
            self
        }
        #[doc = "Sets the value of `definition`."]
        pub fn with_definition(mut self, definition: super::rules::RuleDefinition) -> Self {
            self.definition = definition;
            self
        }
        #[doc = "Sets the value of `documentation`."]
        pub fn set_documentation(
            &mut self,
            documentation: ::std::option::Option<::sidex::types::Str>,
        ) -> &mut Self {
            self.documentation = documentation;
            self
        }
        #[doc = "Sets the value of `documentation`."]
        pub fn with_documentation(
            mut self,
            documentation: ::std::option::Option<::sidex::types::Str>,
        ) -> Self {
            self.documentation = documentation;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for RuleInfo {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for RuleInfo {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "A named recipe and its optional README.md.\n"]
    #[derive(Clone, Debug)]
    pub struct RecipeInfo {
        #[doc = ""]
        pub name: ::sidex::types::Str,
        #[doc = ""]
        pub definition: super::recipes::RecipeConfig,
        #[doc = ""]
        pub documentation: ::std::option::Option<::sidex::types::Str>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for RecipeInfo {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::catalog::RecipeInfo",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "name",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                ),
                ::sidex::shape::FieldShape::new(
                    "definition",
                    ::sidex::shape::shape_of::<super::recipes::RecipeConfig>,
                ),
                ::sidex::shape::FieldShape::new(
                    "documentation",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .optional(),
            ])),
        )
        .with_docs("A named recipe and its optional README.md.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for RecipeInfo {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for RecipeInfo {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.name)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.definition)),
                2usize => self
                    .documentation
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for RecipeInfo {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<super::recipes::RecipeConfig>::None;
            let mut __field_2 = ::std::option::Option::<::sidex::types::Str>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <super::recipes::RecipeConfig as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                name: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `name`"))?,
                definition: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `definition`"))?,
                documentation: __field_2,
            })
        }
    }
    impl RecipeInfo {
        #[doc = "Creates a new [`RecipeInfo`]."]
        pub fn new(
            name: impl ::std::convert::Into<::sidex::types::Str>,
            definition: super::recipes::RecipeConfig,
        ) -> Self {
            Self {
                name: name.into(),
                definition: definition,
                documentation: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(
            &mut self,
            name: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.name = name.into();
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            self.name = name.into();
            self
        }
        #[doc = "Sets the value of `definition`."]
        pub fn set_definition(&mut self, definition: super::recipes::RecipeConfig) -> &mut Self {
            self.definition = definition;
            self
        }
        #[doc = "Sets the value of `definition`."]
        pub fn with_definition(mut self, definition: super::recipes::RecipeConfig) -> Self {
            self.definition = definition;
            self
        }
        #[doc = "Sets the value of `documentation`."]
        pub fn set_documentation(
            &mut self,
            documentation: ::std::option::Option<::sidex::types::Str>,
        ) -> &mut Self {
            self.documentation = documentation;
            self
        }
        #[doc = "Sets the value of `documentation`."]
        pub fn with_documentation(
            mut self,
            documentation: ::std::option::Option<::sidex::types::Str>,
        ) -> Self {
            self.documentation = documentation;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for RecipeInfo {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for RecipeInfo {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "A named layer and its optional same-named .md sidecar.\n"]
    #[derive(Clone, Debug)]
    pub struct LayerInfo {
        #[doc = ""]
        pub name: ::sidex::types::Str,
        #[doc = ""]
        pub definition: LayerDefinition,
        #[doc = ""]
        pub documentation: ::std::option::Option<::sidex::types::Str>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for LayerInfo {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::catalog::LayerInfo",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "name",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                ),
                ::sidex::shape::FieldShape::new(
                    "definition",
                    ::sidex::shape::shape_of::<LayerDefinition>,
                ),
                ::sidex::shape::FieldShape::new(
                    "documentation",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .optional(),
            ])),
        )
        .with_docs("A named layer and its optional same-named .md sidecar.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for LayerInfo {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for LayerInfo {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.name)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.definition)),
                2usize => self
                    .documentation
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for LayerInfo {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<LayerDefinition>::None;
            let mut __field_2 = ::std::option::Option::<::sidex::types::Str>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <LayerDefinition as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                name: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `name`"))?,
                definition: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `definition`"))?,
                documentation: __field_2,
            })
        }
    }
    impl LayerInfo {
        #[doc = "Creates a new [`LayerInfo`]."]
        pub fn new(
            name: impl ::std::convert::Into<::sidex::types::Str>,
            definition: LayerDefinition,
        ) -> Self {
            Self {
                name: name.into(),
                definition: definition,
                documentation: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(
            &mut self,
            name: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.name = name.into();
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            self.name = name.into();
            self
        }
        #[doc = "Sets the value of `definition`."]
        pub fn set_definition(&mut self, definition: LayerDefinition) -> &mut Self {
            self.definition = definition;
            self
        }
        #[doc = "Sets the value of `definition`."]
        pub fn with_definition(mut self, definition: LayerDefinition) -> Self {
            self.definition = definition;
            self
        }
        #[doc = "Sets the value of `documentation`."]
        pub fn set_documentation(
            &mut self,
            documentation: ::std::option::Option<::sidex::types::Str>,
        ) -> &mut Self {
            self.documentation = documentation;
            self
        }
        #[doc = "Sets the value of `documentation`."]
        pub fn with_documentation(
            mut self,
            documentation: ::std::option::Option<::sidex::types::Str>,
        ) -> Self {
            self.documentation = documentation;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for LayerInfo {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for LayerInfo {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "A layer's architecture-specific definitions and optional default.\n"]
    #[derive(Clone, Debug)]
    pub struct LayerDefinition {
        #[doc = ""]
        pub variants: ::sidex::types::Seq<LayerVariant>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for LayerDefinition {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::catalog::LayerDefinition",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "variants",
                    ::sidex::shape::shape_of::<::sidex::types::Seq<LayerVariant>>,
                ),
            ])),
        )
        .with_docs("A layer's architecture-specific definitions and optional default.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for LayerDefinition {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for LayerDefinition {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.variants)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for LayerDefinition {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Seq<LayerVariant>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Seq<LayerVariant> as ::sidex::Decode>::decode(
                                decoder,
                            )?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                variants: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `variants`"))?,
            })
        }
    }
    impl LayerDefinition {
        #[doc = "Creates a new [`LayerDefinition`]."]
        pub fn new(variants: ::sidex::types::Seq<LayerVariant>) -> Self {
            Self { variants: variants }
        }
        #[doc = "Sets the value of `variants`."]
        pub fn set_variants(&mut self, variants: ::sidex::types::Seq<LayerVariant>) -> &mut Self {
            self.variants = variants;
            self
        }
        #[doc = "Sets the value of `variants`."]
        pub fn with_variants(mut self, variants: ::sidex::types::Seq<LayerVariant>) -> Self {
            self.variants = variants;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for LayerDefinition {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for LayerDefinition {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "One complete layer configuration. An architecture override replaces the default.\n"]
    #[derive(Clone, Debug)]
    pub struct LayerVariant {
        #[doc = "Absent for the fallback definition.\n"]
        pub architecture: ::std::option::Option<super::systems::Architecture>,
        #[doc = ""]
        pub config: super::layers::LayerConfig,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for LayerVariant {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::catalog::LayerVariant",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "architecture",
                    ::sidex::shape::shape_of::<super::systems::Architecture>,
                )
                .with_docs("Absent for the fallback definition.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "config",
                    ::sidex::shape::shape_of::<super::layers::LayerConfig>,
                ),
            ])),
        )
        .with_docs(
            "One complete layer configuration. An architecture override replaces the default.\n",
        );
    }
    #[automatically_derived]
    impl ::sidex::Encode for LayerVariant {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for LayerVariant {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .architecture
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.config)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for LayerVariant {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<super::systems::Architecture>::None;
            let mut __field_1 = ::std::option::Option::<super::layers::LayerConfig>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <super::systems::Architecture as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <super::layers::LayerConfig as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                architecture: __field_0,
                config: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `config`"))?,
            })
        }
    }
    impl LayerVariant {
        #[doc = "Creates a new [`LayerVariant`]."]
        pub fn new(config: super::layers::LayerConfig) -> Self {
            Self {
                config: config,
                architecture: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `architecture`."]
        pub fn set_architecture(
            &mut self,
            architecture: ::std::option::Option<super::systems::Architecture>,
        ) -> &mut Self {
            self.architecture = architecture;
            self
        }
        #[doc = "Sets the value of `architecture`."]
        pub fn with_architecture(
            mut self,
            architecture: ::std::option::Option<super::systems::Architecture>,
        ) -> Self {
            self.architecture = architecture;
            self
        }
        #[doc = "Sets the value of `config`."]
        pub fn set_config(&mut self, config: super::layers::LayerConfig) -> &mut Self {
            self.config = config;
            self
        }
        #[doc = "Sets the value of `config`."]
        pub fn with_config(mut self, config: super::layers::LayerConfig) -> Self {
            self.config = config;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for LayerVariant {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for LayerVariant {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
}
pub mod envs {
    #![doc = "Inspection results and storage selection for tooling environments.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "Storage backend for tooling environment images.\n"]
    #[derive(Clone, Debug, Copy, PartialEq, Eq, clap :: ValueEnum)]
    pub enum EnvStorage {
        #[doc = "Use native overlay when supported, otherwise directory copies.\n"]
        Auto,
        #[doc = "Use kernel OverlayFS.\n"]
        Native,
        #[doc = "Use fuse-overlayfs (requires /dev/fuse).\n"]
        Fuse,
        #[doc = "Use directory copies.\n"]
        Vfs,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for EnvStorage {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::envs::EnvStorage",
            ::sidex::shape::ShapeKind::Variant(
                ::sidex::shape::VariantShape::new(&[
                    ::sidex::shape::CaseShape::new("Auto", ::std::option::Option::None)
                        .with_docs(
                            "Use native overlay when supported, otherwise directory copies.\n",
                        )
                        .json_name("auto"),
                    ::sidex::shape::CaseShape::new("Native", ::std::option::Option::None)
                        .with_docs("Use kernel OverlayFS.\n")
                        .json_name("native"),
                    ::sidex::shape::CaseShape::new("Fuse", ::std::option::Option::None)
                        .with_docs("Use fuse-overlayfs (requires /dev/fuse).\n")
                        .json_name("fuse"),
                    ::sidex::shape::CaseShape::new("Vfs", ::std::option::Option::None)
                        .with_docs("Use directory copies.\n")
                        .json_name("vfs"),
                ])
                .json_tagging(::sidex::formats::json::Tagging::External),
            ),
        )
        .with_docs("Storage backend for tooling environment images.\n")
        .with_attrs(&[
            ::sidex::shape::Attr::List {
                path: "json",
                args: &[
                    ::sidex::shape::Attr::Assign {
                        path: "tagged",
                        value: ::sidex::shape::AttrValue::Path("externally"),
                    },
                    ::sidex::shape::Attr::Assign {
                        path: "rename_all",
                        value: ::sidex::shape::AttrValue::String("kebab-case"),
                    },
                ],
            },
            ::sidex::shape::Attr::List {
                path: "rust",
                args: &[::sidex::shape::Attr::List {
                    path: "derive",
                    args: &[
                        ::sidex::shape::Attr::Path("Copy"),
                        ::sidex::shape::Attr::Path("PartialEq"),
                        ::sidex::shape::Attr::Path("Eq"),
                        ::sidex::shape::Attr::Path("clap::ValueEnum"),
                    ],
                }],
            },
        ]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for EnvStorage {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for EnvStorage {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Auto => 0usize,
                Self::Native => 1usize,
                Self::Fuse => 2usize,
                Self::Vfs => 3usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Auto => ::std::option::Option::None,
                Self::Native => ::std::option::Option::None,
                Self::Fuse => ::std::option::Option::None,
                Self::Vfs => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for EnvStorage {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Auto,
                    1usize => Self::Native,
                    2usize => Self::Fuse,
                    3usize => Self::Vfs,
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for EnvStorage {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for EnvStorage {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "A discovered environment definition.\n"]
    #[derive(Clone, Debug)]
    pub struct EnvInfo {
        #[doc = "Environment reference relative to the project.\n"]
        pub name: ::sidex::types::Str,
        #[doc = "Absolute path to the Dockerfile.\n"]
        pub dockerfile: ::sidex::types::Str,
        #[doc = "Absolute path to the build context.\n"]
        pub context: ::sidex::types::Str,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for EnvInfo {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::envs::EnvInfo",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "name",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Environment reference relative to the project.\n"),
                ::sidex::shape::FieldShape::new(
                    "dockerfile",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Absolute path to the Dockerfile.\n"),
                ::sidex::shape::FieldShape::new(
                    "context",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Absolute path to the build context.\n"),
            ])),
        )
        .with_docs("A discovered environment definition.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for EnvInfo {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for EnvInfo {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.name)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.dockerfile)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.context)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for EnvInfo {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_2 = ::std::option::Option::<::sidex::types::Str>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                name: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `name`"))?,
                dockerfile: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `dockerfile`"))?,
                context: __field_2
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `context`"))?,
            })
        }
    }
    impl EnvInfo {
        #[doc = "Creates a new [`EnvInfo`]."]
        pub fn new(
            name: impl ::std::convert::Into<::sidex::types::Str>,
            dockerfile: impl ::std::convert::Into<::sidex::types::Str>,
            context: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> Self {
            Self {
                name: name.into(),
                dockerfile: dockerfile.into(),
                context: context.into(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(
            &mut self,
            name: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.name = name.into();
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            self.name = name.into();
            self
        }
        #[doc = "Sets the value of `dockerfile`."]
        pub fn set_dockerfile(
            &mut self,
            dockerfile: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.dockerfile = dockerfile.into();
            self
        }
        #[doc = "Sets the value of `dockerfile`."]
        pub fn with_dockerfile(
            mut self,
            dockerfile: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> Self {
            self.dockerfile = dockerfile.into();
            self
        }
        #[doc = "Sets the value of `context`."]
        pub fn set_context(
            &mut self,
            context: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.context = context.into();
            self
        }
        #[doc = "Sets the value of `context`."]
        pub fn with_context(
            mut self,
            context: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> Self {
            self.context = context.into();
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for EnvInfo {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for EnvInfo {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Result of building an environment.\n"]
    #[derive(Clone, Debug)]
    pub struct EnvBuildInfo {
        #[doc = "Resolved local OCI image identity.\n"]
        pub image_id: ::sidex::types::Str,
        #[doc = "Storage backend containing the image.\n"]
        pub storage: EnvStorage,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for EnvBuildInfo {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::envs::EnvBuildInfo",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "image_id",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Resolved local OCI image identity.\n")
                .json_name("imageId"),
                ::sidex::shape::FieldShape::new("storage", ::sidex::shape::shape_of::<EnvStorage>)
                    .with_docs("Storage backend containing the image.\n"),
            ])),
        )
        .with_docs("Result of building an environment.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for EnvBuildInfo {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for EnvBuildInfo {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.image_id)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.storage)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for EnvBuildInfo {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<EnvStorage>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <EnvStorage as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                image_id: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `imageId`"))?,
                storage: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `storage`"))?,
            })
        }
    }
    impl EnvBuildInfo {
        #[doc = "Creates a new [`EnvBuildInfo`]."]
        pub fn new(
            image_id: impl ::std::convert::Into<::sidex::types::Str>,
            storage: EnvStorage,
        ) -> Self {
            Self {
                image_id: image_id.into(),
                storage: storage,
            }
        }
        #[doc = "Sets the value of `image_id`."]
        pub fn set_image_id(
            &mut self,
            image_id: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.image_id = image_id.into();
            self
        }
        #[doc = "Sets the value of `image_id`."]
        pub fn with_image_id(
            mut self,
            image_id: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> Self {
            self.image_id = image_id.into();
            self
        }
        #[doc = "Sets the value of `storage`."]
        pub fn set_storage(&mut self, storage: EnvStorage) -> &mut Self {
            self.storage = storage;
            self
        }
        #[doc = "Sets the value of `storage`."]
        pub fn with_storage(mut self, storage: EnvStorage) -> Self {
            self.storage = storage;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for EnvBuildInfo {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for EnvBuildInfo {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
}
pub mod foreign {
    #![doc = "Foreign types.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "Number of bytes.\n"]
    pub type NumBytes = ::sidex::serde::Serde<::byte_calc::NumBytes>;
}
pub mod images {
    #![doc = "Image configuration.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "Timestamp.\n"]
    pub type Timestamp = ::sidex::types::Timestamp;
    #[doc = "Image configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct ImageConfig {
        #[doc = "Layer the image is based on.\n"]
        pub layer: ::sidex::types::Str,
        #[doc = "Architecture of the image.\n"]
        pub architecture: super::systems::Architecture,
        #[doc = "Rugix Bakery target.\n"]
        pub target: ::std::option::Option<super::systems::Target>,
        #[doc = "Size of the image.\n"]
        pub size: ::std::option::Option<super::foreign::NumBytes>,
        #[doc = "Layout of the image.\n"]
        pub layout: ::std::option::Option<ImageLayout>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ImageConfig {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::images::ImageConfig",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "layer",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Layer the image is based on.\n"),
                ::sidex::shape::FieldShape::new(
                    "architecture",
                    ::sidex::shape::shape_of::<super::systems::Architecture>,
                )
                .with_docs("Architecture of the image.\n"),
                ::sidex::shape::FieldShape::new(
                    "target",
                    ::sidex::shape::shape_of::<super::systems::Target>,
                )
                .with_docs("Rugix Bakery target.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "size",
                    ::sidex::shape::shape_of::<super::foreign::NumBytes>,
                )
                .with_docs("Size of the image.\n")
                .optional(),
                ::sidex::shape::FieldShape::new("layout", ::sidex::shape::shape_of::<ImageLayout>)
                    .with_docs("Layout of the image.\n")
                    .optional(),
            ])),
        )
        .with_docs("Image configuration.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for ImageConfig {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ImageConfig {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.layer)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.architecture)),
                2usize => self
                    .target
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                3usize => self
                    .size
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                4usize => self
                    .layout
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ImageConfig {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<super::systems::Architecture>::None;
            let mut __field_2 = ::std::option::Option::<super::systems::Target>::None;
            let mut __field_3 = ::std::option::Option::<super::foreign::NumBytes>::None;
            let mut __field_4 = ::std::option::Option::<ImageLayout>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <super::systems::Architecture as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <super::systems::Target as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <super::foreign::NumBytes as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(
                            <ImageLayout as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                layer: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `layer`"))?,
                architecture: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `architecture`"))?,
                target: __field_2,
                size: __field_3,
                layout: __field_4,
            })
        }
    }
    impl ImageConfig {
        #[doc = "Creates a new [`ImageConfig`]."]
        pub fn new(
            layer: impl ::std::convert::Into<::sidex::types::Str>,
            architecture: super::systems::Architecture,
        ) -> Self {
            Self {
                layer: layer.into(),
                architecture: architecture,
                target: ::std::default::Default::default(),
                size: ::std::default::Default::default(),
                layout: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `layer`."]
        pub fn set_layer(
            &mut self,
            layer: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.layer = layer.into();
            self
        }
        #[doc = "Sets the value of `layer`."]
        pub fn with_layer(mut self, layer: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            self.layer = layer.into();
            self
        }
        #[doc = "Sets the value of `architecture`."]
        pub fn set_architecture(
            &mut self,
            architecture: super::systems::Architecture,
        ) -> &mut Self {
            self.architecture = architecture;
            self
        }
        #[doc = "Sets the value of `architecture`."]
        pub fn with_architecture(mut self, architecture: super::systems::Architecture) -> Self {
            self.architecture = architecture;
            self
        }
        #[doc = "Sets the value of `target`."]
        pub fn set_target(
            &mut self,
            target: ::std::option::Option<super::systems::Target>,
        ) -> &mut Self {
            self.target = target;
            self
        }
        #[doc = "Sets the value of `target`."]
        pub fn with_target(
            mut self,
            target: ::std::option::Option<super::systems::Target>,
        ) -> Self {
            self.target = target;
            self
        }
        #[doc = "Sets the value of `size`."]
        pub fn set_size(
            &mut self,
            size: ::std::option::Option<super::foreign::NumBytes>,
        ) -> &mut Self {
            self.size = size;
            self
        }
        #[doc = "Sets the value of `size`."]
        pub fn with_size(mut self, size: ::std::option::Option<super::foreign::NumBytes>) -> Self {
            self.size = size;
            self
        }
        #[doc = "Sets the value of `layout`."]
        pub fn set_layout(&mut self, layout: ::std::option::Option<ImageLayout>) -> &mut Self {
            self.layout = layout;
            self
        }
        #[doc = "Sets the value of `layout`."]
        pub fn with_layout(mut self, layout: ::std::option::Option<ImageLayout>) -> Self {
            self.layout = layout;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ImageConfig {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ImageConfig {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Layout of an image.\n"]
    #[derive(Clone, Debug)]
    pub struct ImageLayout {
        #[doc = "Type of the partition table.\n"]
        pub ty: ::std::option::Option<PartitionTableType>,
        #[doc = "Raw blobs written at fixed byte offsets on the disk image.\n"]
        pub raw_blobs: ::std::option::Option<::sidex::types::Seq<RawBlob>>,
        #[doc = "Image partitions.\n"]
        pub partitions: ::std::option::Option<::sidex::types::Seq<ImagePartition>>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ImageLayout {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::images::ImageLayout",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "ty",
                    ::sidex::shape::shape_of::<PartitionTableType>,
                )
                .with_docs("Type of the partition table.\n")
                .with_attrs(&[::sidex::shape::Attr::List {
                    path: "json",
                    args: &[::sidex::shape::Attr::Assign {
                        path: "name",
                        value: ::sidex::shape::AttrValue::String("type"),
                    }],
                }])
                .optional()
                .json_name("type"),
                ::sidex::shape::FieldShape::new(
                    "raw_blobs",
                    ::sidex::shape::shape_of::<::sidex::types::Seq<RawBlob>>,
                )
                .with_docs("Raw blobs written at fixed byte offsets on the disk image.\n")
                .optional()
                .json_name("raw-blobs"),
                ::sidex::shape::FieldShape::new(
                    "partitions",
                    ::sidex::shape::shape_of::<::sidex::types::Seq<ImagePartition>>,
                )
                .with_docs("Image partitions.\n")
                .optional(),
            ])),
        )
        .with_docs("Layout of an image.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[::sidex::shape::Attr::Assign {
                path: "rename_all",
                value: ::sidex::shape::AttrValue::String("kebab-case"),
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for ImageLayout {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ImageLayout {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self.ty.as_ref().map(|value| ::sidex::Encode::encode(value)),
                1usize => self
                    .raw_blobs
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .partitions
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ImageLayout {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<PartitionTableType>::None;
            let mut __field_1 = ::std::option::Option::<::sidex::types::Seq<RawBlob>>::None;
            let mut __field_2 = ::std::option::Option::<::sidex::types::Seq<ImagePartition>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <PartitionTableType as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::sidex::types::Seq<RawBlob> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(<::sidex::types::Seq<
                            ImagePartition,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                ty: __field_0,
                raw_blobs: __field_1,
                partitions: __field_2,
            })
        }
    }
    impl ImageLayout {
        #[doc = "Creates a new [`ImageLayout`]."]
        pub fn new() -> Self {
            Self {
                ty: ::std::default::Default::default(),
                raw_blobs: ::std::default::Default::default(),
                partitions: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `ty`."]
        pub fn set_ty(&mut self, ty: ::std::option::Option<PartitionTableType>) -> &mut Self {
            self.ty = ty;
            self
        }
        #[doc = "Sets the value of `ty`."]
        pub fn with_ty(mut self, ty: ::std::option::Option<PartitionTableType>) -> Self {
            self.ty = ty;
            self
        }
        #[doc = "Sets the value of `raw_blobs`."]
        pub fn set_raw_blobs(
            &mut self,
            raw_blobs: ::std::option::Option<::sidex::types::Seq<RawBlob>>,
        ) -> &mut Self {
            self.raw_blobs = raw_blobs;
            self
        }
        #[doc = "Sets the value of `raw_blobs`."]
        pub fn with_raw_blobs(
            mut self,
            raw_blobs: ::std::option::Option<::sidex::types::Seq<RawBlob>>,
        ) -> Self {
            self.raw_blobs = raw_blobs;
            self
        }
        #[doc = "Sets the value of `partitions`."]
        pub fn set_partitions(
            &mut self,
            partitions: ::std::option::Option<::sidex::types::Seq<ImagePartition>>,
        ) -> &mut Self {
            self.partitions = partitions;
            self
        }
        #[doc = "Sets the value of `partitions`."]
        pub fn with_partitions(
            mut self,
            partitions: ::std::option::Option<::sidex::types::Seq<ImagePartition>>,
        ) -> Self {
            self.partitions = partitions;
            self
        }
    }
    impl ::std::default::Default for ImageLayout {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ImageLayout {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ImageLayout {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Raw firmware blob written at a fixed offset on the disk.\n"]
    #[derive(Clone, Debug)]
    pub struct RawBlob {
        #[doc = "Path to the blob file (relative to the layer root).\n"]
        pub file: ::sidex::types::Str,
        #[doc = "Byte offset from the start of the disk image.\n"]
        pub offset: super::foreign::NumBytes,
        #[doc = "Reserved size for the blob region. When set, partition offsets are\ncomputed from `offset + size` instead of `offset + file_size`. Must\nbe at least as large as the actual blob file.\n"]
        pub size: ::std::option::Option<super::foreign::NumBytes>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for RawBlob {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::rugix_bakery::images::RawBlob" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("file" , :: sidex :: shape :: shape_of :: < :: sidex :: types :: Str > ,) . with_docs ("Path to the blob file (relative to the layer root).\n") , :: sidex :: shape :: FieldShape :: new ("offset" , :: sidex :: shape :: shape_of :: < super :: foreign :: NumBytes > ,) . with_docs ("Byte offset from the start of the disk image.\n") , :: sidex :: shape :: FieldShape :: new ("size" , :: sidex :: shape :: shape_of :: < super :: foreign :: NumBytes > ,) . with_docs ("Reserved size for the blob region. When set, partition offsets are\ncomputed from `offset + size` instead of `offset + file_size`. Must\nbe at least as large as the actual blob file.\n") . optional ()]))) . with_docs ("Raw firmware blob written at a fixed offset on the disk.\n") ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for RawBlob {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for RawBlob {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.file)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.offset)),
                2usize => self
                    .size
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for RawBlob {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<super::foreign::NumBytes>::None;
            let mut __field_2 = ::std::option::Option::<super::foreign::NumBytes>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <super::foreign::NumBytes as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <super::foreign::NumBytes as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                file: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `file`"))?,
                offset: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `offset`"))?,
                size: __field_2,
            })
        }
    }
    impl RawBlob {
        #[doc = "Creates a new [`RawBlob`]."]
        pub fn new(
            file: impl ::std::convert::Into<::sidex::types::Str>,
            offset: super::foreign::NumBytes,
        ) -> Self {
            Self {
                file: file.into(),
                offset: offset,
                size: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `file`."]
        pub fn set_file(
            &mut self,
            file: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.file = file.into();
            self
        }
        #[doc = "Sets the value of `file`."]
        pub fn with_file(mut self, file: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            self.file = file.into();
            self
        }
        #[doc = "Sets the value of `offset`."]
        pub fn set_offset(&mut self, offset: super::foreign::NumBytes) -> &mut Self {
            self.offset = offset;
            self
        }
        #[doc = "Sets the value of `offset`."]
        pub fn with_offset(mut self, offset: super::foreign::NumBytes) -> Self {
            self.offset = offset;
            self
        }
        #[doc = "Sets the value of `size`."]
        pub fn set_size(
            &mut self,
            size: ::std::option::Option<super::foreign::NumBytes>,
        ) -> &mut Self {
            self.size = size;
            self
        }
        #[doc = "Sets the value of `size`."]
        pub fn with_size(mut self, size: ::std::option::Option<super::foreign::NumBytes>) -> Self {
            self.size = size;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for RawBlob {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for RawBlob {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Partition table type.\n"]
    #[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
    pub enum PartitionTableType {
        #[doc = "MBR partition.\n"]
        Mbr,
        #[doc = "GPT partition.\n"]
        Gpt,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for PartitionTableType {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::images::PartitionTableType",
            ::sidex::shape::ShapeKind::Variant(
                ::sidex::shape::VariantShape::new(&[
                    ::sidex::shape::CaseShape::new("Mbr", ::std::option::Option::None)
                        .with_docs("MBR partition.\n")
                        .json_name("mbr"),
                    ::sidex::shape::CaseShape::new("Gpt", ::std::option::Option::None)
                        .with_docs("GPT partition.\n")
                        .json_name("gpt"),
                ])
                .json_tagging(::sidex::formats::json::Tagging::External),
            ),
        )
        .with_docs("Partition table type.\n")
        .with_attrs(&[
            ::sidex::shape::Attr::List {
                path: "json",
                args: &[
                    ::sidex::shape::Attr::Assign {
                        path: "tagged",
                        value: ::sidex::shape::AttrValue::Path("externally"),
                    },
                    ::sidex::shape::Attr::Assign {
                        path: "rename_all",
                        value: ::sidex::shape::AttrValue::String("lowercase"),
                    },
                ],
            },
            ::sidex::shape::Attr::List {
                path: "rust",
                args: &[::sidex::shape::Attr::List {
                    path: "derive",
                    args: &[
                        ::sidex::shape::Attr::Path("Copy"),
                        ::sidex::shape::Attr::Path("PartialEq"),
                        ::sidex::shape::Attr::Path("Eq"),
                        ::sidex::shape::Attr::Path("Hash"),
                    ],
                }],
            },
        ]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for PartitionTableType {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for PartitionTableType {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Mbr => 0usize,
                Self::Gpt => 1usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Mbr => ::std::option::Option::None,
                Self::Gpt => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for PartitionTableType {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Mbr,
                    1usize => Self::Gpt,
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for PartitionTableType {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for PartitionTableType {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Partition of an image.\n"]
    #[derive(Clone, Debug)]
    pub struct ImagePartition {
        #[doc = "Size of the partition.\n"]
        pub size: ::std::option::Option<super::foreign::NumBytes>,
        #[doc = "Filesystem of the partition.\n"]
        pub filesystem: ::std::option::Option<Filesystem>,
        #[doc = "Root directory to copy into the filesystem.\n"]
        pub root: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Type of the partition (GUID or MBR hex value).\n"]
        pub ty: ::std::option::Option<PartitionType>,
        #[doc = "Mark the partition bootable.\n"]
        pub bootable: ::std::option::Option<bool>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ImagePartition {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::images::ImagePartition",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "size",
                    ::sidex::shape::shape_of::<super::foreign::NumBytes>,
                )
                .with_docs("Size of the partition.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "filesystem",
                    ::sidex::shape::shape_of::<Filesystem>,
                )
                .with_docs("Filesystem of the partition.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "root",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Root directory to copy into the filesystem.\n")
                .optional(),
                ::sidex::shape::FieldShape::new("ty", ::sidex::shape::shape_of::<PartitionType>)
                    .with_docs("Type of the partition (GUID or MBR hex value).\n")
                    .with_attrs(&[::sidex::shape::Attr::List {
                        path: "json",
                        args: &[::sidex::shape::Attr::Assign {
                            path: "name",
                            value: ::sidex::shape::AttrValue::String("type"),
                        }],
                    }])
                    .optional()
                    .json_name("type"),
                ::sidex::shape::FieldShape::new("bootable", ::sidex::shape::shape_of::<bool>)
                    .with_docs("Mark the partition bootable.\n")
                    .optional(),
            ])),
        )
        .with_docs("Partition of an image.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for ImagePartition {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ImagePartition {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .size
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => self
                    .filesystem
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .root
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                3usize => self.ty.as_ref().map(|value| ::sidex::Encode::encode(value)),
                4usize => self
                    .bootable
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ImagePartition {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<super::foreign::NumBytes>::None;
            let mut __field_1 = ::std::option::Option::<Filesystem>::None;
            let mut __field_2 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_3 = ::std::option::Option::<PartitionType>::None;
            let mut __field_4 = ::std::option::Option::<bool>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <super::foreign::NumBytes as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <Filesystem as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <PartitionType as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 =
                            ::std::option::Option::Some(<bool as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                size: __field_0,
                filesystem: __field_1,
                root: __field_2,
                ty: __field_3,
                bootable: __field_4,
            })
        }
    }
    impl ImagePartition {
        #[doc = "Creates a new [`ImagePartition`]."]
        pub fn new() -> Self {
            Self {
                size: ::std::default::Default::default(),
                filesystem: ::std::default::Default::default(),
                root: ::std::default::Default::default(),
                ty: ::std::default::Default::default(),
                bootable: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `size`."]
        pub fn set_size(
            &mut self,
            size: ::std::option::Option<super::foreign::NumBytes>,
        ) -> &mut Self {
            self.size = size;
            self
        }
        #[doc = "Sets the value of `size`."]
        pub fn with_size(mut self, size: ::std::option::Option<super::foreign::NumBytes>) -> Self {
            self.size = size;
            self
        }
        #[doc = "Sets the value of `filesystem`."]
        pub fn set_filesystem(
            &mut self,
            filesystem: ::std::option::Option<Filesystem>,
        ) -> &mut Self {
            self.filesystem = filesystem;
            self
        }
        #[doc = "Sets the value of `filesystem`."]
        pub fn with_filesystem(mut self, filesystem: ::std::option::Option<Filesystem>) -> Self {
            self.filesystem = filesystem;
            self
        }
        #[doc = "Sets the value of `root`."]
        pub fn set_root(&mut self, root: ::std::option::Option<::sidex::types::Str>) -> &mut Self {
            self.root = root;
            self
        }
        #[doc = "Sets the value of `root`."]
        pub fn with_root(mut self, root: ::std::option::Option<::sidex::types::Str>) -> Self {
            self.root = root;
            self
        }
        #[doc = "Sets the value of `ty`."]
        pub fn set_ty(&mut self, ty: ::std::option::Option<PartitionType>) -> &mut Self {
            self.ty = ty;
            self
        }
        #[doc = "Sets the value of `ty`."]
        pub fn with_ty(mut self, ty: ::std::option::Option<PartitionType>) -> Self {
            self.ty = ty;
            self
        }
        #[doc = "Sets the value of `bootable`."]
        pub fn set_bootable(&mut self, bootable: ::std::option::Option<bool>) -> &mut Self {
            self.bootable = bootable;
            self
        }
        #[doc = "Sets the value of `bootable`."]
        pub fn with_bootable(mut self, bootable: ::std::option::Option<bool>) -> Self {
            self.bootable = bootable;
            self
        }
    }
    impl ::std::default::Default for ImagePartition {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ImagePartition {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ImagePartition {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Filesystem.\n"]
    #[derive(Clone, Debug)]
    pub enum Filesystem {
        #[doc = "EXT4\n"]
        Ext4(Ext4Options),
        #[doc = "Fat32\n"]
        Fat32,
        #[doc = "Squashfs\n"]
        Squashfs(SquashfsOptions),
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Filesystem {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::images::Filesystem",
            ::sidex::shape::ShapeKind::Variant(
                ::sidex::shape::VariantShape::new(&[
                    ::sidex::shape::CaseShape::new(
                        "Ext4",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<Ext4Options>),
                    )
                    .with_docs("EXT4\n")
                    .json_name("ext4"),
                    ::sidex::shape::CaseShape::new("Fat32", ::std::option::Option::None)
                        .with_docs("Fat32\n")
                        .json_name("fat32"),
                    ::sidex::shape::CaseShape::new(
                        "Squashfs",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<SquashfsOptions>),
                    )
                    .with_docs("Squashfs\n")
                    .json_name("squashfs"),
                ])
                .json_tagging(::sidex::formats::json::Tagging::Internal {
                    tag: "type",
                    content: "content",
                }),
            ),
        )
        .with_docs("Filesystem.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[
                ::sidex::shape::Attr::Assign {
                    path: "tag",
                    value: ::sidex::shape::AttrValue::String("type"),
                },
                ::sidex::shape::Attr::Assign {
                    path: "rename_all",
                    value: ::sidex::shape::AttrValue::String("lowercase"),
                },
            ],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for Filesystem {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for Filesystem {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Ext4(_) => 0usize,
                Self::Fat32 => 1usize,
                Self::Squashfs(_) => 2usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Ext4(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Fat32 => ::std::option::Option::None,
                Self::Squashfs(value) => {
                    ::std::option::Option::Some(::sidex::Encode::encode(value))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Filesystem {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Ext4(<Ext4Options as ::sidex::Decode>::decode(decoder)?),
                    1usize => Self::Fat32,
                    2usize => {
                        Self::Squashfs(<SquashfsOptions as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for Filesystem {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for Filesystem {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct Ext4Options {
        #[doc = ""]
        pub additional_options: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        #[doc = ""]
        pub clamp_mtime: ::std::option::Option<Timestamp>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Ext4Options {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::images::Ext4Options",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "additional_options",
                    ::sidex::shape::shape_of::<::sidex::types::Seq<::sidex::types::Str>>,
                )
                .optional()
                .json_name("additional-options"),
                ::sidex::shape::FieldShape::new(
                    "clamp_mtime",
                    ::sidex::shape::shape_of::<Timestamp>,
                )
                .optional()
                .json_name("clamp-mtime"),
            ])),
        )
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[::sidex::shape::Attr::Assign {
                path: "rename_all",
                value: ::sidex::shape::AttrValue::String("kebab-case"),
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for Ext4Options {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for Ext4Options {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .additional_options
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => self
                    .clamp_mtime
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Ext4Options {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 =
                ::std::option::Option::<::sidex::types::Seq<::sidex::types::Str>>::None;
            let mut __field_1 = ::std::option::Option::<Timestamp>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(<::sidex::types::Seq<
                            ::sidex::types::Str,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <Timestamp as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                additional_options: __field_0,
                clamp_mtime: __field_1,
            })
        }
    }
    impl Ext4Options {
        #[doc = "Creates a new [`Ext4Options`]."]
        pub fn new() -> Self {
            Self {
                additional_options: ::std::default::Default::default(),
                clamp_mtime: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `additional_options`."]
        pub fn set_additional_options(
            &mut self,
            additional_options: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        ) -> &mut Self {
            self.additional_options = additional_options;
            self
        }
        #[doc = "Sets the value of `additional_options`."]
        pub fn with_additional_options(
            mut self,
            additional_options: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        ) -> Self {
            self.additional_options = additional_options;
            self
        }
        #[doc = "Sets the value of `clamp_mtime`."]
        pub fn set_clamp_mtime(
            &mut self,
            clamp_mtime: ::std::option::Option<Timestamp>,
        ) -> &mut Self {
            self.clamp_mtime = clamp_mtime;
            self
        }
        #[doc = "Sets the value of `clamp_mtime`."]
        pub fn with_clamp_mtime(mut self, clamp_mtime: ::std::option::Option<Timestamp>) -> Self {
            self.clamp_mtime = clamp_mtime;
            self
        }
    }
    impl ::std::default::Default for Ext4Options {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for Ext4Options {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for Ext4Options {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct SquashfsOptions {
        #[doc = ""]
        pub additional_options: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        #[doc = ""]
        pub no_compression: ::std::option::Option<bool>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for SquashfsOptions {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::images::SquashfsOptions",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "additional_options",
                    ::sidex::shape::shape_of::<::sidex::types::Seq<::sidex::types::Str>>,
                )
                .optional()
                .json_name("additional-options"),
                ::sidex::shape::FieldShape::new("no_compression", ::sidex::shape::shape_of::<bool>)
                    .optional()
                    .json_name("no-compression"),
            ])),
        )
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[::sidex::shape::Attr::Assign {
                path: "rename_all",
                value: ::sidex::shape::AttrValue::String("kebab-case"),
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for SquashfsOptions {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for SquashfsOptions {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .additional_options
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => self
                    .no_compression
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for SquashfsOptions {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 =
                ::std::option::Option::<::sidex::types::Seq<::sidex::types::Str>>::None;
            let mut __field_1 = ::std::option::Option::<bool>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(<::sidex::types::Seq<
                            ::sidex::types::Str,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    1usize => {
                        __field_1 =
                            ::std::option::Option::Some(<bool as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                additional_options: __field_0,
                no_compression: __field_1,
            })
        }
    }
    impl SquashfsOptions {
        #[doc = "Creates a new [`SquashfsOptions`]."]
        pub fn new() -> Self {
            Self {
                additional_options: ::std::default::Default::default(),
                no_compression: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `additional_options`."]
        pub fn set_additional_options(
            &mut self,
            additional_options: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        ) -> &mut Self {
            self.additional_options = additional_options;
            self
        }
        #[doc = "Sets the value of `additional_options`."]
        pub fn with_additional_options(
            mut self,
            additional_options: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        ) -> Self {
            self.additional_options = additional_options;
            self
        }
        #[doc = "Sets the value of `no_compression`."]
        pub fn set_no_compression(
            &mut self,
            no_compression: ::std::option::Option<bool>,
        ) -> &mut Self {
            self.no_compression = no_compression;
            self
        }
        #[doc = "Sets the value of `no_compression`."]
        pub fn with_no_compression(mut self, no_compression: ::std::option::Option<bool>) -> Self {
            self.no_compression = no_compression;
            self
        }
    }
    impl ::std::default::Default for SquashfsOptions {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for SquashfsOptions {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for SquashfsOptions {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Partition type.\n"]
    pub type PartitionType = ::sidex::serde::Serde<::rugix_common::disk::PartitionType>;
}
pub mod layers {
    #![doc = "Layer configuration.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "Layer configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct LayerConfig {
        #[doc = "Human-friendly name of the layer.\n"]
        pub name: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Description of the layer.\n"]
        pub description: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Optional group used when presenting a layer catalog.\n"]
        pub category: ::std::option::Option<::sidex::types::Str>,
        #[doc = "URL for importing the layer.\n"]
        pub url: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Parent layer.\n"]
        pub parent: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Indicates whether the layer is a root layer.\n"]
        pub root: ::std::option::Option<bool>,
        #[doc = "Recipes to build the layer with.\n"]
        pub recipes: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        #[doc = "Recipes to specifically exclude.\n"]
        pub exclude: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        #[doc = "Prerequisite rules available to recipe input bindings.\n"]
        pub rules: ::std::option::Option<
            ::sidex::types::Map<::sidex::types::Str, super::rules::RuleInvocation>,
        >,
        #[doc = "Artifact bindings by recipe reference and input name.\n"]
        pub inputs: ::std::option::Option<
            ::sidex::types::Map<
                ::sidex::types::Str,
                ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactSource>,
            >,
        >,
        #[doc = "Recipe parameters.\n"]
        pub parameters: ::std::option::Option<
            ::sidex::types::Map<
                ::sidex::types::Str,
                ::sidex::types::Map<::sidex::types::Str, super::recipes::ParameterValue>,
            >,
        >,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for LayerConfig {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::layers::LayerConfig",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "name",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Human-friendly name of the layer.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "description",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Description of the layer.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "category",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Optional group used when presenting a layer catalog.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "url",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("URL for importing the layer.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "parent",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Parent layer.\n")
                .optional(),
                ::sidex::shape::FieldShape::new("root", ::sidex::shape::shape_of::<bool>)
                    .with_docs("Indicates whether the layer is a root layer.\n")
                    .optional(),
                ::sidex::shape::FieldShape::new(
                    "recipes",
                    ::sidex::shape::shape_of::<::sidex::types::Seq<::sidex::types::Str>>,
                )
                .with_docs("Recipes to build the layer with.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "exclude",
                    ::sidex::shape::shape_of::<::sidex::types::Seq<::sidex::types::Str>>,
                )
                .with_docs("Recipes to specifically exclude.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "rules",
                    ::sidex::shape::shape_of::<
                        ::sidex::types::Map<::sidex::types::Str, super::rules::RuleInvocation>,
                    >,
                )
                .with_docs("Prerequisite rules available to recipe input bindings.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "inputs",
                    ::sidex::shape::shape_of::<
                        ::sidex::types::Map<
                            ::sidex::types::Str,
                            ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactSource>,
                        >,
                    >,
                )
                .with_docs("Artifact bindings by recipe reference and input name.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "parameters",
                    ::sidex::shape::shape_of::<
                        ::sidex::types::Map<
                            ::sidex::types::Str,
                            ::sidex::types::Map<
                                ::sidex::types::Str,
                                super::recipes::ParameterValue,
                            >,
                        >,
                    >,
                )
                .with_docs("Recipe parameters.\n")
                .optional(),
            ])),
        )
        .with_docs("Layer configuration.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for LayerConfig {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for LayerConfig {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .name
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => self
                    .description
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .category
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                3usize => self
                    .url
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                4usize => self
                    .parent
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                5usize => self
                    .root
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                6usize => self
                    .recipes
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                7usize => self
                    .exclude
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                8usize => self
                    .rules
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                9usize => self
                    .inputs
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                10usize => self
                    .parameters
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for LayerConfig {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_2 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_3 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_4 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_5 = ::std::option::Option::<bool>::None;
            let mut __field_6 =
                ::std::option::Option::<::sidex::types::Seq<::sidex::types::Str>>::None;
            let mut __field_7 =
                ::std::option::Option::<::sidex::types::Seq<::sidex::types::Str>>::None;
            let mut __field_8 = ::std::option::Option::<
                ::sidex::types::Map<::sidex::types::Str, super::rules::RuleInvocation>,
            >::None;
            let mut __field_9 = ::std::option::Option::<
                ::sidex::types::Map<
                    ::sidex::types::Str,
                    ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactSource>,
                >,
            >::None;
            let mut __field_10 = ::std::option::Option::<
                ::sidex::types::Map<
                    ::sidex::types::Str,
                    ::sidex::types::Map<::sidex::types::Str, super::recipes::ParameterValue>,
                >,
            >::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    5usize => {
                        __field_5 =
                            ::std::option::Option::Some(<bool as ::sidex::Decode>::decode(decoder)?)
                    }
                    6usize => {
                        __field_6 = ::std::option::Option::Some(<::sidex::types::Seq<
                            ::sidex::types::Str,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    7usize => {
                        __field_7 = ::std::option::Option::Some(<::sidex::types::Seq<
                            ::sidex::types::Str,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    8usize => {
                        __field_8 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            super::rules::RuleInvocation,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    9usize => {
                        __field_9 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactSource>,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    10usize => {
                        __field_10 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            ::sidex::types::Map<
                                ::sidex::types::Str,
                                super::recipes::ParameterValue,
                            >,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                name: __field_0,
                description: __field_1,
                category: __field_2,
                url: __field_3,
                parent: __field_4,
                root: __field_5,
                recipes: __field_6,
                exclude: __field_7,
                rules: __field_8,
                inputs: __field_9,
                parameters: __field_10,
            })
        }
    }
    impl LayerConfig {
        #[doc = "Creates a new [`LayerConfig`]."]
        pub fn new() -> Self {
            Self {
                name: ::std::default::Default::default(),
                description: ::std::default::Default::default(),
                category: ::std::default::Default::default(),
                url: ::std::default::Default::default(),
                parent: ::std::default::Default::default(),
                root: ::std::default::Default::default(),
                recipes: ::std::default::Default::default(),
                exclude: ::std::default::Default::default(),
                rules: ::std::default::Default::default(),
                inputs: ::std::default::Default::default(),
                parameters: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(&mut self, name: ::std::option::Option<::sidex::types::Str>) -> &mut Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: ::std::option::Option<::sidex::types::Str>) -> Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<::sidex::types::Str>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(
            mut self,
            description: ::std::option::Option<::sidex::types::Str>,
        ) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `category`."]
        pub fn set_category(
            &mut self,
            category: ::std::option::Option<::sidex::types::Str>,
        ) -> &mut Self {
            self.category = category;
            self
        }
        #[doc = "Sets the value of `category`."]
        pub fn with_category(
            mut self,
            category: ::std::option::Option<::sidex::types::Str>,
        ) -> Self {
            self.category = category;
            self
        }
        #[doc = "Sets the value of `url`."]
        pub fn set_url(&mut self, url: ::std::option::Option<::sidex::types::Str>) -> &mut Self {
            self.url = url;
            self
        }
        #[doc = "Sets the value of `url`."]
        pub fn with_url(mut self, url: ::std::option::Option<::sidex::types::Str>) -> Self {
            self.url = url;
            self
        }
        #[doc = "Sets the value of `parent`."]
        pub fn set_parent(
            &mut self,
            parent: ::std::option::Option<::sidex::types::Str>,
        ) -> &mut Self {
            self.parent = parent;
            self
        }
        #[doc = "Sets the value of `parent`."]
        pub fn with_parent(mut self, parent: ::std::option::Option<::sidex::types::Str>) -> Self {
            self.parent = parent;
            self
        }
        #[doc = "Sets the value of `root`."]
        pub fn set_root(&mut self, root: ::std::option::Option<bool>) -> &mut Self {
            self.root = root;
            self
        }
        #[doc = "Sets the value of `root`."]
        pub fn with_root(mut self, root: ::std::option::Option<bool>) -> Self {
            self.root = root;
            self
        }
        #[doc = "Sets the value of `recipes`."]
        pub fn set_recipes(
            &mut self,
            recipes: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        ) -> &mut Self {
            self.recipes = recipes;
            self
        }
        #[doc = "Sets the value of `recipes`."]
        pub fn with_recipes(
            mut self,
            recipes: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        ) -> Self {
            self.recipes = recipes;
            self
        }
        #[doc = "Sets the value of `exclude`."]
        pub fn set_exclude(
            &mut self,
            exclude: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        ) -> &mut Self {
            self.exclude = exclude;
            self
        }
        #[doc = "Sets the value of `exclude`."]
        pub fn with_exclude(
            mut self,
            exclude: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        ) -> Self {
            self.exclude = exclude;
            self
        }
        #[doc = "Sets the value of `rules`."]
        pub fn set_rules(
            &mut self,
            rules: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, super::rules::RuleInvocation>,
            >,
        ) -> &mut Self {
            self.rules = rules;
            self
        }
        #[doc = "Sets the value of `rules`."]
        pub fn with_rules(
            mut self,
            rules: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, super::rules::RuleInvocation>,
            >,
        ) -> Self {
            self.rules = rules;
            self
        }
        #[doc = "Sets the value of `inputs`."]
        pub fn set_inputs(
            &mut self,
            inputs: ::std::option::Option<
                ::sidex::types::Map<
                    ::sidex::types::Str,
                    ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactSource>,
                >,
            >,
        ) -> &mut Self {
            self.inputs = inputs;
            self
        }
        #[doc = "Sets the value of `inputs`."]
        pub fn with_inputs(
            mut self,
            inputs: ::std::option::Option<
                ::sidex::types::Map<
                    ::sidex::types::Str,
                    ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactSource>,
                >,
            >,
        ) -> Self {
            self.inputs = inputs;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn set_parameters(
            &mut self,
            parameters: ::std::option::Option<
                ::sidex::types::Map<
                    ::sidex::types::Str,
                    ::sidex::types::Map<::sidex::types::Str, super::recipes::ParameterValue>,
                >,
            >,
        ) -> &mut Self {
            self.parameters = parameters;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn with_parameters(
            mut self,
            parameters: ::std::option::Option<
                ::sidex::types::Map<
                    ::sidex::types::Str,
                    ::sidex::types::Map<::sidex::types::Str, super::recipes::ParameterValue>,
                >,
            >,
        ) -> Self {
            self.parameters = parameters;
            self
        }
    }
    impl ::std::default::Default for LayerConfig {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for LayerConfig {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for LayerConfig {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
}
pub mod mixins {
    #![doc = "Mixin configuration.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "Mixin configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct MixinConfig {
        #[doc = "Human-friendly name of the mixin.\n"]
        pub name: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Description of the mixin.\n"]
        pub description: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Other mixins required by this mixin.\n"]
        pub dependencies: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        #[doc = "Recipes to apply with the mixin.\n"]
        pub recipes: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        #[doc = "Recipes to specifically exclude from this mixin.\n"]
        pub exclude: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        #[doc = "Prerequisite rules available to recipe input bindings.\n"]
        pub rules: ::std::option::Option<
            ::sidex::types::Map<::sidex::types::Str, super::rules::RuleInvocation>,
        >,
        #[doc = "Artifact bindings by recipe reference and input name.\n"]
        pub inputs: ::std::option::Option<
            ::sidex::types::Map<
                ::sidex::types::Str,
                ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactSource>,
            >,
        >,
        #[doc = "Recipe parameters.\n"]
        pub parameters: ::std::option::Option<
            ::sidex::types::Map<
                ::sidex::types::Str,
                ::sidex::types::Map<::sidex::types::Str, super::recipes::ParameterValue>,
            >,
        >,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for MixinConfig {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::mixins::MixinConfig",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "name",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Human-friendly name of the mixin.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "description",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Description of the mixin.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "dependencies",
                    ::sidex::shape::shape_of::<::sidex::types::Seq<::sidex::types::Str>>,
                )
                .with_docs("Other mixins required by this mixin.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "recipes",
                    ::sidex::shape::shape_of::<::sidex::types::Seq<::sidex::types::Str>>,
                )
                .with_docs("Recipes to apply with the mixin.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "exclude",
                    ::sidex::shape::shape_of::<::sidex::types::Seq<::sidex::types::Str>>,
                )
                .with_docs("Recipes to specifically exclude from this mixin.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "rules",
                    ::sidex::shape::shape_of::<
                        ::sidex::types::Map<::sidex::types::Str, super::rules::RuleInvocation>,
                    >,
                )
                .with_docs("Prerequisite rules available to recipe input bindings.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "inputs",
                    ::sidex::shape::shape_of::<
                        ::sidex::types::Map<
                            ::sidex::types::Str,
                            ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactSource>,
                        >,
                    >,
                )
                .with_docs("Artifact bindings by recipe reference and input name.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "parameters",
                    ::sidex::shape::shape_of::<
                        ::sidex::types::Map<
                            ::sidex::types::Str,
                            ::sidex::types::Map<
                                ::sidex::types::Str,
                                super::recipes::ParameterValue,
                            >,
                        >,
                    >,
                )
                .with_docs("Recipe parameters.\n")
                .optional(),
            ])),
        )
        .with_docs("Mixin configuration.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for MixinConfig {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for MixinConfig {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .name
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => self
                    .description
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .dependencies
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                3usize => self
                    .recipes
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                4usize => self
                    .exclude
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                5usize => self
                    .rules
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                6usize => self
                    .inputs
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                7usize => self
                    .parameters
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for MixinConfig {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_2 =
                ::std::option::Option::<::sidex::types::Seq<::sidex::types::Str>>::None;
            let mut __field_3 =
                ::std::option::Option::<::sidex::types::Seq<::sidex::types::Str>>::None;
            let mut __field_4 =
                ::std::option::Option::<::sidex::types::Seq<::sidex::types::Str>>::None;
            let mut __field_5 = ::std::option::Option::<
                ::sidex::types::Map<::sidex::types::Str, super::rules::RuleInvocation>,
            >::None;
            let mut __field_6 = ::std::option::Option::<
                ::sidex::types::Map<
                    ::sidex::types::Str,
                    ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactSource>,
                >,
            >::None;
            let mut __field_7 = ::std::option::Option::<
                ::sidex::types::Map<
                    ::sidex::types::Str,
                    ::sidex::types::Map<::sidex::types::Str, super::recipes::ParameterValue>,
                >,
            >::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(<::sidex::types::Seq<
                            ::sidex::types::Str,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(<::sidex::types::Seq<
                            ::sidex::types::Str,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(<::sidex::types::Seq<
                            ::sidex::types::Str,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    5usize => {
                        __field_5 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            super::rules::RuleInvocation,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    6usize => {
                        __field_6 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactSource>,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    7usize => {
                        __field_7 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            ::sidex::types::Map<
                                ::sidex::types::Str,
                                super::recipes::ParameterValue,
                            >,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                name: __field_0,
                description: __field_1,
                dependencies: __field_2,
                recipes: __field_3,
                exclude: __field_4,
                rules: __field_5,
                inputs: __field_6,
                parameters: __field_7,
            })
        }
    }
    impl MixinConfig {
        #[doc = "Creates a new [`MixinConfig`]."]
        pub fn new() -> Self {
            Self {
                name: ::std::default::Default::default(),
                description: ::std::default::Default::default(),
                dependencies: ::std::default::Default::default(),
                recipes: ::std::default::Default::default(),
                exclude: ::std::default::Default::default(),
                rules: ::std::default::Default::default(),
                inputs: ::std::default::Default::default(),
                parameters: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(&mut self, name: ::std::option::Option<::sidex::types::Str>) -> &mut Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: ::std::option::Option<::sidex::types::Str>) -> Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<::sidex::types::Str>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(
            mut self,
            description: ::std::option::Option<::sidex::types::Str>,
        ) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `dependencies`."]
        pub fn set_dependencies(
            &mut self,
            dependencies: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        ) -> &mut Self {
            self.dependencies = dependencies;
            self
        }
        #[doc = "Sets the value of `dependencies`."]
        pub fn with_dependencies(
            mut self,
            dependencies: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        ) -> Self {
            self.dependencies = dependencies;
            self
        }
        #[doc = "Sets the value of `recipes`."]
        pub fn set_recipes(
            &mut self,
            recipes: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        ) -> &mut Self {
            self.recipes = recipes;
            self
        }
        #[doc = "Sets the value of `recipes`."]
        pub fn with_recipes(
            mut self,
            recipes: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        ) -> Self {
            self.recipes = recipes;
            self
        }
        #[doc = "Sets the value of `exclude`."]
        pub fn set_exclude(
            &mut self,
            exclude: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        ) -> &mut Self {
            self.exclude = exclude;
            self
        }
        #[doc = "Sets the value of `exclude`."]
        pub fn with_exclude(
            mut self,
            exclude: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        ) -> Self {
            self.exclude = exclude;
            self
        }
        #[doc = "Sets the value of `rules`."]
        pub fn set_rules(
            &mut self,
            rules: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, super::rules::RuleInvocation>,
            >,
        ) -> &mut Self {
            self.rules = rules;
            self
        }
        #[doc = "Sets the value of `rules`."]
        pub fn with_rules(
            mut self,
            rules: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, super::rules::RuleInvocation>,
            >,
        ) -> Self {
            self.rules = rules;
            self
        }
        #[doc = "Sets the value of `inputs`."]
        pub fn set_inputs(
            &mut self,
            inputs: ::std::option::Option<
                ::sidex::types::Map<
                    ::sidex::types::Str,
                    ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactSource>,
                >,
            >,
        ) -> &mut Self {
            self.inputs = inputs;
            self
        }
        #[doc = "Sets the value of `inputs`."]
        pub fn with_inputs(
            mut self,
            inputs: ::std::option::Option<
                ::sidex::types::Map<
                    ::sidex::types::Str,
                    ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactSource>,
                >,
            >,
        ) -> Self {
            self.inputs = inputs;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn set_parameters(
            &mut self,
            parameters: ::std::option::Option<
                ::sidex::types::Map<
                    ::sidex::types::Str,
                    ::sidex::types::Map<::sidex::types::Str, super::recipes::ParameterValue>,
                >,
            >,
        ) -> &mut Self {
            self.parameters = parameters;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn with_parameters(
            mut self,
            parameters: ::std::option::Option<
                ::sidex::types::Map<
                    ::sidex::types::Str,
                    ::sidex::types::Map<::sidex::types::Str, super::recipes::ParameterValue>,
                >,
            >,
        ) -> Self {
            self.parameters = parameters;
            self
        }
    }
    impl ::std::default::Default for MixinConfig {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for MixinConfig {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for MixinConfig {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
}
pub mod projects {
    #![doc = "Project configuration.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "Project configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct ProjectConfig {
        #[doc = "Repositories imported into the project.\n"]
        pub repositories: ::std::option::Option<
            ::sidex::types::Map<::sidex::types::Str, super::repositories::SourceConfig>,
        >,
        #[doc = "System declarations.\n"]
        pub systems: ::std::option::Option<
            ::sidex::types::Map<::sidex::types::Str, super::systems::SystemConfig>,
        >,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ProjectConfig {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::projects::ProjectConfig",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "repositories",
                    ::sidex::shape::shape_of::<
                        ::sidex::types::Map<::sidex::types::Str, super::repositories::SourceConfig>,
                    >,
                )
                .with_docs("Repositories imported into the project.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "systems",
                    ::sidex::shape::shape_of::<
                        ::sidex::types::Map<::sidex::types::Str, super::systems::SystemConfig>,
                    >,
                )
                .with_docs("System declarations.\n")
                .optional(),
            ])),
        )
        .with_docs("Project configuration.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for ProjectConfig {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ProjectConfig {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .repositories
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => self
                    .systems
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ProjectConfig {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<
                ::sidex::types::Map<::sidex::types::Str, super::repositories::SourceConfig>,
            >::None;
            let mut __field_1 = ::std::option::Option::<
                ::sidex::types::Map<::sidex::types::Str, super::systems::SystemConfig>,
            >::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            super::repositories::SourceConfig,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            super::systems::SystemConfig,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                repositories: __field_0,
                systems: __field_1,
            })
        }
    }
    impl ProjectConfig {
        #[doc = "Creates a new [`ProjectConfig`]."]
        pub fn new() -> Self {
            Self {
                repositories: ::std::default::Default::default(),
                systems: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `repositories`."]
        pub fn set_repositories(
            &mut self,
            repositories: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, super::repositories::SourceConfig>,
            >,
        ) -> &mut Self {
            self.repositories = repositories;
            self
        }
        #[doc = "Sets the value of `repositories`."]
        pub fn with_repositories(
            mut self,
            repositories: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, super::repositories::SourceConfig>,
            >,
        ) -> Self {
            self.repositories = repositories;
            self
        }
        #[doc = "Sets the value of `systems`."]
        pub fn set_systems(
            &mut self,
            systems: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, super::systems::SystemConfig>,
            >,
        ) -> &mut Self {
            self.systems = systems;
            self
        }
        #[doc = "Sets the value of `systems`."]
        pub fn with_systems(
            mut self,
            systems: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, super::systems::SystemConfig>,
            >,
        ) -> Self {
            self.systems = systems;
            self
        }
    }
    impl ::std::default::Default for ProjectConfig {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ProjectConfig {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ProjectConfig {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
}
pub mod recipes {
    #![doc = "Recipe configuration.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "Recipe configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct RecipeConfig {
        #[doc = "Description of the recipe.\n"]
        pub description: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Optional group used when presenting a recipe catalog.\n"]
        pub category: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Priority of the recipe.\n"]
        pub priority: ::std::option::Option<i64>,
        #[doc = "Dependencies of the recipe.\n"]
        pub dependencies: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        #[doc = "Required artifact inputs.\n"]
        pub inputs: ::std::option::Option<
            ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactInput>,
        >,
        #[doc = "Prerequisite rules available to this recipe's default input bindings.\n"]
        pub rules: ::std::option::Option<
            ::sidex::types::Map<::sidex::types::Str, super::rules::RuleInvocation>,
        >,
        #[doc = "Input bindings used when the enclosing layer supplies no artifact.\n"]
        pub input_defaults: ::std::option::Option<
            ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactSource>,
        >,
        #[doc = "Parameter definitions of the recipe.\n"]
        pub parameters:
            ::std::option::Option<::sidex::types::Map<::sidex::types::Str, ParameterDef>>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for RecipeConfig {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::recipes::RecipeConfig",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "description",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Description of the recipe.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "category",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Optional group used when presenting a recipe catalog.\n")
                .optional(),
                ::sidex::shape::FieldShape::new("priority", ::sidex::shape::shape_of::<i64>)
                    .with_docs("Priority of the recipe.\n")
                    .optional(),
                ::sidex::shape::FieldShape::new(
                    "dependencies",
                    ::sidex::shape::shape_of::<::sidex::types::Seq<::sidex::types::Str>>,
                )
                .with_docs("Dependencies of the recipe.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "inputs",
                    ::sidex::shape::shape_of::<
                        ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactInput>,
                    >,
                )
                .with_docs("Required artifact inputs.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "rules",
                    ::sidex::shape::shape_of::<
                        ::sidex::types::Map<::sidex::types::Str, super::rules::RuleInvocation>,
                    >,
                )
                .with_docs(
                    "Prerequisite rules available to this recipe's default input bindings.\n",
                )
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "input_defaults",
                    ::sidex::shape::shape_of::<
                        ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactSource>,
                    >,
                )
                .with_docs("Input bindings used when the enclosing layer supplies no artifact.\n")
                .optional()
                .json_name("input-defaults"),
                ::sidex::shape::FieldShape::new(
                    "parameters",
                    ::sidex::shape::shape_of::<
                        ::sidex::types::Map<::sidex::types::Str, ParameterDef>,
                    >,
                )
                .with_docs("Parameter definitions of the recipe.\n")
                .optional(),
            ])),
        )
        .with_docs("Recipe configuration.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[::sidex::shape::Attr::Assign {
                path: "rename_all",
                value: ::sidex::shape::AttrValue::String("kebab-case"),
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for RecipeConfig {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for RecipeConfig {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .description
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => self
                    .category
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .priority
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                3usize => self
                    .dependencies
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                4usize => self
                    .inputs
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                5usize => self
                    .rules
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                6usize => self
                    .input_defaults
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                7usize => self
                    .parameters
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for RecipeConfig {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_2 = ::std::option::Option::<i64>::None;
            let mut __field_3 =
                ::std::option::Option::<::sidex::types::Seq<::sidex::types::Str>>::None;
            let mut __field_4 = ::std::option::Option::<
                ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactInput>,
            >::None;
            let mut __field_5 = ::std::option::Option::<
                ::sidex::types::Map<::sidex::types::Str, super::rules::RuleInvocation>,
            >::None;
            let mut __field_6 = ::std::option::Option::<
                ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactSource>,
            >::None;
            let mut __field_7 = ::std::option::Option::<
                ::sidex::types::Map<::sidex::types::Str, ParameterDef>,
            >::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 =
                            ::std::option::Option::Some(<i64 as ::sidex::Decode>::decode(decoder)?)
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(<::sidex::types::Seq<
                            ::sidex::types::Str,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            super::rules::ArtifactInput,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    5usize => {
                        __field_5 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            super::rules::RuleInvocation,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    6usize => {
                        __field_6 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            super::rules::ArtifactSource,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    7usize => {
                        __field_7 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            ParameterDef,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                description: __field_0,
                category: __field_1,
                priority: __field_2,
                dependencies: __field_3,
                inputs: __field_4,
                rules: __field_5,
                input_defaults: __field_6,
                parameters: __field_7,
            })
        }
    }
    impl RecipeConfig {
        #[doc = "Creates a new [`RecipeConfig`]."]
        pub fn new() -> Self {
            Self {
                description: ::std::default::Default::default(),
                category: ::std::default::Default::default(),
                priority: ::std::default::Default::default(),
                dependencies: ::std::default::Default::default(),
                inputs: ::std::default::Default::default(),
                rules: ::std::default::Default::default(),
                input_defaults: ::std::default::Default::default(),
                parameters: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<::sidex::types::Str>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(
            mut self,
            description: ::std::option::Option<::sidex::types::Str>,
        ) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `category`."]
        pub fn set_category(
            &mut self,
            category: ::std::option::Option<::sidex::types::Str>,
        ) -> &mut Self {
            self.category = category;
            self
        }
        #[doc = "Sets the value of `category`."]
        pub fn with_category(
            mut self,
            category: ::std::option::Option<::sidex::types::Str>,
        ) -> Self {
            self.category = category;
            self
        }
        #[doc = "Sets the value of `priority`."]
        pub fn set_priority(&mut self, priority: ::std::option::Option<i64>) -> &mut Self {
            self.priority = priority;
            self
        }
        #[doc = "Sets the value of `priority`."]
        pub fn with_priority(mut self, priority: ::std::option::Option<i64>) -> Self {
            self.priority = priority;
            self
        }
        #[doc = "Sets the value of `dependencies`."]
        pub fn set_dependencies(
            &mut self,
            dependencies: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        ) -> &mut Self {
            self.dependencies = dependencies;
            self
        }
        #[doc = "Sets the value of `dependencies`."]
        pub fn with_dependencies(
            mut self,
            dependencies: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        ) -> Self {
            self.dependencies = dependencies;
            self
        }
        #[doc = "Sets the value of `inputs`."]
        pub fn set_inputs(
            &mut self,
            inputs: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactInput>,
            >,
        ) -> &mut Self {
            self.inputs = inputs;
            self
        }
        #[doc = "Sets the value of `inputs`."]
        pub fn with_inputs(
            mut self,
            inputs: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactInput>,
            >,
        ) -> Self {
            self.inputs = inputs;
            self
        }
        #[doc = "Sets the value of `rules`."]
        pub fn set_rules(
            &mut self,
            rules: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, super::rules::RuleInvocation>,
            >,
        ) -> &mut Self {
            self.rules = rules;
            self
        }
        #[doc = "Sets the value of `rules`."]
        pub fn with_rules(
            mut self,
            rules: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, super::rules::RuleInvocation>,
            >,
        ) -> Self {
            self.rules = rules;
            self
        }
        #[doc = "Sets the value of `input_defaults`."]
        pub fn set_input_defaults(
            &mut self,
            input_defaults: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactSource>,
            >,
        ) -> &mut Self {
            self.input_defaults = input_defaults;
            self
        }
        #[doc = "Sets the value of `input_defaults`."]
        pub fn with_input_defaults(
            mut self,
            input_defaults: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, super::rules::ArtifactSource>,
            >,
        ) -> Self {
            self.input_defaults = input_defaults;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn set_parameters(
            &mut self,
            parameters: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, ParameterDef>,
            >,
        ) -> &mut Self {
            self.parameters = parameters;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn with_parameters(
            mut self,
            parameters: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, ParameterDef>,
            >,
        ) -> Self {
            self.parameters = parameters;
            self
        }
    }
    impl ::std::default::Default for RecipeConfig {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for RecipeConfig {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for RecipeConfig {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Recipe parameter definition.\n"]
    #[derive(Clone, Debug)]
    pub struct ParameterDef {
        #[doc = "Description, including units and special values.\n"]
        pub description: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Optional default value of the parameter.\n"]
        pub default: ::std::option::Option<ParameterValue>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ParameterDef {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::recipes::ParameterDef",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "description",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Description, including units and special values.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "default",
                    ::sidex::shape::shape_of::<ParameterValue>,
                )
                .with_docs("Optional default value of the parameter.\n")
                .optional(),
            ])),
        )
        .with_docs("Recipe parameter definition.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for ParameterDef {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ParameterDef {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .description
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => self
                    .default
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ParameterDef {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<ParameterValue>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <ParameterValue as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                description: __field_0,
                default: __field_1,
            })
        }
    }
    impl ParameterDef {
        #[doc = "Creates a new [`ParameterDef`]."]
        pub fn new() -> Self {
            Self {
                description: ::std::default::Default::default(),
                default: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<::sidex::types::Str>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(
            mut self,
            description: ::std::option::Option<::sidex::types::Str>,
        ) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `default`."]
        pub fn set_default(&mut self, default: ::std::option::Option<ParameterValue>) -> &mut Self {
            self.default = default;
            self
        }
        #[doc = "Sets the value of `default`."]
        pub fn with_default(mut self, default: ::std::option::Option<ParameterValue>) -> Self {
            self.default = default;
            self
        }
    }
    impl ::std::default::Default for ParameterDef {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ParameterDef {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ParameterDef {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Value of a parameter.\n"]
    #[derive(Clone, Debug)]
    pub enum ParameterValue {
        #[doc = "String.\n"]
        String(::sidex::types::Str),
        #[doc = "Boolean.\n"]
        Boolean(bool),
        #[doc = "Integer.\n"]
        Integer(i64),
        #[doc = "Float.\n"]
        Float(f64),
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ParameterValue {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::recipes::ParameterValue",
            ::sidex::shape::ShapeKind::Variant(
                ::sidex::shape::VariantShape::new(&[
                    ::sidex::shape::CaseShape::new(
                        "String",
                        ::std::option::Option::Some(
                            ::sidex::shape::shape_of::<::sidex::types::Str>,
                        ),
                    )
                    .with_docs("String.\n"),
                    ::sidex::shape::CaseShape::new(
                        "Boolean",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<bool>),
                    )
                    .with_docs("Boolean.\n"),
                    ::sidex::shape::CaseShape::new(
                        "Integer",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<i64>),
                    )
                    .with_docs("Integer.\n"),
                    ::sidex::shape::CaseShape::new(
                        "Float",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<f64>),
                    )
                    .with_docs("Float.\n"),
                ])
                .json_tagging(::sidex::formats::json::Tagging::Implicit),
            ),
        )
        .with_docs("Value of a parameter.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[::sidex::shape::Attr::Assign {
                path: "tagged",
                value: ::sidex::shape::AttrValue::Path("implicitly"),
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for ParameterValue {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for ParameterValue {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::String(_) => 0usize,
                Self::Boolean(_) => 1usize,
                Self::Integer(_) => 2usize,
                Self::Float(_) => 3usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::String(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Boolean(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Integer(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Float(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ParameterValue {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => {
                        Self::String(<::sidex::types::Str as ::sidex::Decode>::decode(decoder)?)
                    }
                    1usize => Self::Boolean(<bool as ::sidex::Decode>::decode(decoder)?),
                    2usize => Self::Integer(<i64 as ::sidex::Decode>::decode(decoder)?),
                    3usize => Self::Float(<f64 as ::sidex::Decode>::decode(decoder)?),
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ParameterValue {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ParameterValue {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
}
pub mod repositories {
    #![doc = "Repository configuration.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "Repository configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct RepositoryConfig {
        #[doc = "Human-friendly name of the repository.\n"]
        pub name: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Description of the repository.\n"]
        pub description: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Repositories used by this repository.\n"]
        pub repositories:
            ::std::option::Option<::sidex::types::Map<::sidex::types::Str, SourceConfig>>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for RepositoryConfig {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::repositories::RepositoryConfig",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "name",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Human-friendly name of the repository.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "description",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Description of the repository.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "repositories",
                    ::sidex::shape::shape_of::<
                        ::sidex::types::Map<::sidex::types::Str, SourceConfig>,
                    >,
                )
                .with_docs("Repositories used by this repository.\n")
                .optional(),
            ])),
        )
        .with_docs("Repository configuration.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for RepositoryConfig {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for RepositoryConfig {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .name
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => self
                    .description
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .repositories
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for RepositoryConfig {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_2 = ::std::option::Option::<
                ::sidex::types::Map<::sidex::types::Str, SourceConfig>,
            >::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            SourceConfig,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                name: __field_0,
                description: __field_1,
                repositories: __field_2,
            })
        }
    }
    impl RepositoryConfig {
        #[doc = "Creates a new [`RepositoryConfig`]."]
        pub fn new() -> Self {
            Self {
                name: ::std::default::Default::default(),
                description: ::std::default::Default::default(),
                repositories: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(&mut self, name: ::std::option::Option<::sidex::types::Str>) -> &mut Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: ::std::option::Option<::sidex::types::Str>) -> Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<::sidex::types::Str>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(
            mut self,
            description: ::std::option::Option<::sidex::types::Str>,
        ) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `repositories`."]
        pub fn set_repositories(
            &mut self,
            repositories: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, SourceConfig>,
            >,
        ) -> &mut Self {
            self.repositories = repositories;
            self
        }
        #[doc = "Sets the value of `repositories`."]
        pub fn with_repositories(
            mut self,
            repositories: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, SourceConfig>,
            >,
        ) -> Self {
            self.repositories = repositories;
            self
        }
    }
    impl ::std::default::Default for RepositoryConfig {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for RepositoryConfig {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for RepositoryConfig {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Repository source.\n"]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum SourceConfig {
        #[doc = "Repository is an external Git repository.\n"]
        Git(GitSourceConfig),
        #[doc = "Repository is a path in the project directory.\n"]
        Path(PathSourceConfig),
    }
    #[automatically_derived]
    impl ::sidex::Shaped for SourceConfig {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::repositories::SourceConfig",
            ::sidex::shape::ShapeKind::Variant(
                ::sidex::shape::VariantShape::new(&[
                    ::sidex::shape::CaseShape::new(
                        "Git",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<GitSourceConfig>),
                    )
                    .with_docs("Repository is an external Git repository.\n"),
                    ::sidex::shape::CaseShape::new(
                        "Path",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<PathSourceConfig>),
                    )
                    .with_docs("Repository is a path in the project directory.\n"),
                ])
                .json_tagging(::sidex::formats::json::Tagging::Implicit),
            ),
        )
        .with_docs("Repository source.\n")
        .with_attrs(&[
            ::sidex::shape::Attr::List {
                path: "rust",
                args: &[::sidex::shape::Attr::List {
                    path: "derive",
                    args: &[
                        ::sidex::shape::Attr::Path("PartialEq"),
                        ::sidex::shape::Attr::Path("Eq"),
                    ],
                }],
            },
            ::sidex::shape::Attr::List {
                path: "json",
                args: &[::sidex::shape::Attr::Assign {
                    path: "tagged",
                    value: ::sidex::shape::AttrValue::Path("implicitly"),
                }],
            },
        ]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for SourceConfig {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for SourceConfig {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Git(_) => 0usize,
                Self::Path(_) => 1usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Git(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Path(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for SourceConfig {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Git(<GitSourceConfig as ::sidex::Decode>::decode(decoder)?),
                    1usize => Self::Path(<PathSourceConfig as ::sidex::Decode>::decode(decoder)?),
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for SourceConfig {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for SourceConfig {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Git repository source.\n"]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct GitSourceConfig {
        #[doc = "URL of the Git repository.\n"]
        pub url: ::sidex::types::Str,
        #[doc = "Specific tag of the Git repository.\n"]
        pub tag: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Specific branch of the Git repository.\n"]
        pub branch: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Specific revision of the Git repository.\n"]
        pub rev: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Subdirectory in which the repository is located.\n"]
        pub dir: ::std::option::Option<::sidex::types::Str>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for GitSourceConfig {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::repositories::GitSourceConfig",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "url",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("URL of the Git repository.\n")
                .with_attrs(&[::sidex::shape::Attr::List {
                    path: "json",
                    args: &[::sidex::shape::Attr::Assign {
                        path: "name",
                        value: ::sidex::shape::AttrValue::String("git"),
                    }],
                }])
                .json_name("git"),
                ::sidex::shape::FieldShape::new(
                    "tag",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Specific tag of the Git repository.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "branch",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Specific branch of the Git repository.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "rev",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Specific revision of the Git repository.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "dir",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Subdirectory in which the repository is located.\n")
                .optional(),
            ])),
        )
        .with_docs("Git repository source.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "derive",
                args: &[
                    ::sidex::shape::Attr::Path("PartialEq"),
                    ::sidex::shape::Attr::Path("Eq"),
                ],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for GitSourceConfig {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for GitSourceConfig {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.url)),
                1usize => self
                    .tag
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .branch
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                3usize => self
                    .rev
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                4usize => self
                    .dir
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for GitSourceConfig {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_2 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_3 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_4 = ::std::option::Option::<::sidex::types::Str>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                url: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `git`"))?,
                tag: __field_1,
                branch: __field_2,
                rev: __field_3,
                dir: __field_4,
            })
        }
    }
    impl GitSourceConfig {
        #[doc = "Creates a new [`GitSourceConfig`]."]
        pub fn new(url: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            Self {
                url: url.into(),
                tag: ::std::default::Default::default(),
                branch: ::std::default::Default::default(),
                rev: ::std::default::Default::default(),
                dir: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `url`."]
        pub fn set_url(
            &mut self,
            url: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.url = url.into();
            self
        }
        #[doc = "Sets the value of `url`."]
        pub fn with_url(mut self, url: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            self.url = url.into();
            self
        }
        #[doc = "Sets the value of `tag`."]
        pub fn set_tag(&mut self, tag: ::std::option::Option<::sidex::types::Str>) -> &mut Self {
            self.tag = tag;
            self
        }
        #[doc = "Sets the value of `tag`."]
        pub fn with_tag(mut self, tag: ::std::option::Option<::sidex::types::Str>) -> Self {
            self.tag = tag;
            self
        }
        #[doc = "Sets the value of `branch`."]
        pub fn set_branch(
            &mut self,
            branch: ::std::option::Option<::sidex::types::Str>,
        ) -> &mut Self {
            self.branch = branch;
            self
        }
        #[doc = "Sets the value of `branch`."]
        pub fn with_branch(mut self, branch: ::std::option::Option<::sidex::types::Str>) -> Self {
            self.branch = branch;
            self
        }
        #[doc = "Sets the value of `rev`."]
        pub fn set_rev(&mut self, rev: ::std::option::Option<::sidex::types::Str>) -> &mut Self {
            self.rev = rev;
            self
        }
        #[doc = "Sets the value of `rev`."]
        pub fn with_rev(mut self, rev: ::std::option::Option<::sidex::types::Str>) -> Self {
            self.rev = rev;
            self
        }
        #[doc = "Sets the value of `dir`."]
        pub fn set_dir(&mut self, dir: ::std::option::Option<::sidex::types::Str>) -> &mut Self {
            self.dir = dir;
            self
        }
        #[doc = "Sets the value of `dir`."]
        pub fn with_dir(mut self, dir: ::std::option::Option<::sidex::types::Str>) -> Self {
            self.dir = dir;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for GitSourceConfig {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for GitSourceConfig {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Local repository source.\n"]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct PathSourceConfig {
        #[doc = "Path of the repository relative to the project directory.\n"]
        pub path: ::sidex::types::Str,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for PathSourceConfig {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::repositories::PathSourceConfig",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "path",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Path of the repository relative to the project directory.\n"),
            ])),
        )
        .with_docs("Local repository source.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "rust",
            args: &[::sidex::shape::Attr::List {
                path: "derive",
                args: &[
                    ::sidex::shape::Attr::Path("PartialEq"),
                    ::sidex::shape::Attr::Path("Eq"),
                ],
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for PathSourceConfig {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for PathSourceConfig {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.path)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for PathSourceConfig {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                path: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `path`"))?,
            })
        }
    }
    impl PathSourceConfig {
        #[doc = "Creates a new [`PathSourceConfig`]."]
        pub fn new(path: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            Self { path: path.into() }
        }
        #[doc = "Sets the value of `path`."]
        pub fn set_path(
            &mut self,
            path: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.path = path.into();
            self
        }
        #[doc = "Sets the value of `path`."]
        pub fn with_path(mut self, path: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            self.path = path.into();
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for PathSourceConfig {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for PathSourceConfig {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
}
pub mod rules {
    #![doc = "Repository-defined artifact builds.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "A reusable operation or composition.\n"]
    #[derive(Clone, Debug)]
    pub struct RuleDefinition {
        #[doc = "Description for users of this rule.\n"]
        pub description: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Optional group used when presenting a rule catalog.\n"]
        pub category: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Required artifact inputs.\n"]
        pub inputs: ::std::option::Option<::sidex::types::Map<::sidex::types::Str, ArtifactInput>>,
        #[doc = "Configurable parameters.\n"]
        pub parameters: ::std::option::Option<
            ::sidex::types::Map<::sidex::types::Str, super::recipes::ParameterDef>,
        >,
        #[doc = "Required sensitive input bindings. Secret changes invalidate cached outputs.\n"]
        pub secrets: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        #[doc = "Implementation of the rule.\n"]
        pub execution: RuleExecution,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for RuleDefinition {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::rugix_bakery::rules::RuleDefinition" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("description" , :: sidex :: shape :: shape_of :: < :: sidex :: types :: Str > ,) . with_docs ("Description for users of this rule.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("category" , :: sidex :: shape :: shape_of :: < :: sidex :: types :: Str > ,) . with_docs ("Optional group used when presenting a rule catalog.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("inputs" , :: sidex :: shape :: shape_of :: < :: sidex :: types :: Map < :: sidex :: types :: Str , ArtifactInput , > > ,) . with_docs ("Required artifact inputs.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("parameters" , :: sidex :: shape :: shape_of :: < :: sidex :: types :: Map < :: sidex :: types :: Str , super :: recipes :: ParameterDef , > > ,) . with_docs ("Configurable parameters.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("secrets" , :: sidex :: shape :: shape_of :: < :: sidex :: types :: Seq < :: sidex :: types :: Str , > > ,) . with_docs ("Required sensitive input bindings. Secret changes invalidate cached outputs.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("execution" , :: sidex :: shape :: shape_of :: < RuleExecution > ,) . with_docs ("Implementation of the rule.\n")]))) . with_docs ("A reusable operation or composition.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "rename_all" , value : :: sidex :: shape :: AttrValue :: String ("kebab-case") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for RuleDefinition {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for RuleDefinition {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .description
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => self
                    .category
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .inputs
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                3usize => self
                    .parameters
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                4usize => self
                    .secrets
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                5usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.execution)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for RuleDefinition {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_2 = ::std::option::Option::<
                ::sidex::types::Map<::sidex::types::Str, ArtifactInput>,
            >::None;
            let mut __field_3 = ::std::option::Option::<
                ::sidex::types::Map<::sidex::types::Str, super::recipes::ParameterDef>,
            >::None;
            let mut __field_4 =
                ::std::option::Option::<::sidex::types::Seq<::sidex::types::Str>>::None;
            let mut __field_5 = ::std::option::Option::<RuleExecution>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            ArtifactInput,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            super::recipes::ParameterDef,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(<::sidex::types::Seq<
                            ::sidex::types::Str,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    5usize => {
                        __field_5 = ::std::option::Option::Some(
                            <RuleExecution as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                description: __field_0,
                category: __field_1,
                inputs: __field_2,
                parameters: __field_3,
                secrets: __field_4,
                execution: __field_5
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `execution`"))?,
            })
        }
    }
    impl RuleDefinition {
        #[doc = "Creates a new [`RuleDefinition`]."]
        pub fn new(execution: RuleExecution) -> Self {
            Self {
                execution: execution,
                description: ::std::default::Default::default(),
                category: ::std::default::Default::default(),
                inputs: ::std::default::Default::default(),
                parameters: ::std::default::Default::default(),
                secrets: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<::sidex::types::Str>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(
            mut self,
            description: ::std::option::Option<::sidex::types::Str>,
        ) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `category`."]
        pub fn set_category(
            &mut self,
            category: ::std::option::Option<::sidex::types::Str>,
        ) -> &mut Self {
            self.category = category;
            self
        }
        #[doc = "Sets the value of `category`."]
        pub fn with_category(
            mut self,
            category: ::std::option::Option<::sidex::types::Str>,
        ) -> Self {
            self.category = category;
            self
        }
        #[doc = "Sets the value of `inputs`."]
        pub fn set_inputs(
            &mut self,
            inputs: ::std::option::Option<::sidex::types::Map<::sidex::types::Str, ArtifactInput>>,
        ) -> &mut Self {
            self.inputs = inputs;
            self
        }
        #[doc = "Sets the value of `inputs`."]
        pub fn with_inputs(
            mut self,
            inputs: ::std::option::Option<::sidex::types::Map<::sidex::types::Str, ArtifactInput>>,
        ) -> Self {
            self.inputs = inputs;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn set_parameters(
            &mut self,
            parameters: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, super::recipes::ParameterDef>,
            >,
        ) -> &mut Self {
            self.parameters = parameters;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn with_parameters(
            mut self,
            parameters: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, super::recipes::ParameterDef>,
            >,
        ) -> Self {
            self.parameters = parameters;
            self
        }
        #[doc = "Sets the value of `secrets`."]
        pub fn set_secrets(
            &mut self,
            secrets: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        ) -> &mut Self {
            self.secrets = secrets;
            self
        }
        #[doc = "Sets the value of `secrets`."]
        pub fn with_secrets(
            mut self,
            secrets: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        ) -> Self {
            self.secrets = secrets;
            self
        }
        #[doc = "Sets the value of `execution`."]
        pub fn set_execution(&mut self, execution: RuleExecution) -> &mut Self {
            self.execution = execution;
            self
        }
        #[doc = "Sets the value of `execution`."]
        pub fn with_execution(mut self, execution: RuleExecution) -> Self {
            self.execution = execution;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for RuleDefinition {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for RuleDefinition {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "A compact artifact kind or a documented input.\n"]
    #[derive(Clone, Debug)]
    pub enum ArtifactInput {
        #[doc = ""]
        Kind(ArtifactKind),
        #[doc = ""]
        Definition(ArtifactInputDefinition),
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ArtifactInput {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::rules::ArtifactInput",
            ::sidex::shape::ShapeKind::Variant(
                ::sidex::shape::VariantShape::new(&[
                    ::sidex::shape::CaseShape::new(
                        "Kind",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<ArtifactKind>),
                    ),
                    ::sidex::shape::CaseShape::new(
                        "Definition",
                        ::std::option::Option::Some(
                            ::sidex::shape::shape_of::<ArtifactInputDefinition>,
                        ),
                    ),
                ])
                .json_tagging(::sidex::formats::json::Tagging::Implicit),
            ),
        )
        .with_docs("A compact artifact kind or a documented input.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[::sidex::shape::Attr::Assign {
                path: "tagged",
                value: ::sidex::shape::AttrValue::Path("implicitly"),
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for ArtifactInput {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for ArtifactInput {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Kind(_) => 0usize,
                Self::Definition(_) => 1usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Kind(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Definition(value) => {
                    ::std::option::Option::Some(::sidex::Encode::encode(value))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ArtifactInput {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Kind(<ArtifactKind as ::sidex::Decode>::decode(decoder)?),
                    1usize => Self::Definition(
                        <ArtifactInputDefinition as ::sidex::Decode>::decode(decoder)?,
                    ),
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ArtifactInput {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ArtifactInput {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "An artifact input with documentation.\n"]
    #[derive(Clone, Debug)]
    pub struct ArtifactInputDefinition {
        #[doc = ""]
        pub kind: ArtifactKind,
        #[doc = ""]
        pub description: ::std::option::Option<::sidex::types::Str>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ArtifactInputDefinition {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::rules::ArtifactInputDefinition",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new("kind", ::sidex::shape::shape_of::<ArtifactKind>),
                ::sidex::shape::FieldShape::new(
                    "description",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .optional(),
            ])),
        )
        .with_docs("An artifact input with documentation.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for ArtifactInputDefinition {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ArtifactInputDefinition {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.kind)),
                1usize => self
                    .description
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ArtifactInputDefinition {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<ArtifactKind>::None;
            let mut __field_1 = ::std::option::Option::<::sidex::types::Str>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <ArtifactKind as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                kind: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `kind`"))?,
                description: __field_1,
            })
        }
    }
    impl ArtifactInputDefinition {
        #[doc = "Creates a new [`ArtifactInputDefinition`]."]
        pub fn new(kind: ArtifactKind) -> Self {
            Self {
                kind: kind,
                description: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `kind`."]
        pub fn set_kind(&mut self, kind: ArtifactKind) -> &mut Self {
            self.kind = kind;
            self
        }
        #[doc = "Sets the value of `kind`."]
        pub fn with_kind(mut self, kind: ArtifactKind) -> Self {
            self.kind = kind;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<::sidex::types::Str>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(
            mut self,
            description: ::std::option::Option<::sidex::types::Str>,
        ) -> Self {
            self.description = description;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ArtifactInputDefinition {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ArtifactInputDefinition {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "An invocation of a repository-defined rule.\n"]
    #[derive(Clone, Debug)]
    pub struct RuleInvocation {
        #[doc = "Local or repository-qualified rule name.\n"]
        pub uses: ::sidex::types::Str,
        #[doc = "Artifact input bindings.\n"]
        pub inputs: ::std::option::Option<::sidex::types::Map<::sidex::types::Str, ArtifactSource>>,
        #[doc = "Parameter bindings.\n"]
        pub parameters:
            ::std::option::Option<::sidex::types::Map<::sidex::types::Str, ParameterBinding>>,
        #[doc = "Secret input bindings.\n"]
        pub secrets:
            ::std::option::Option<::sidex::types::Map<::sidex::types::Str, ArtifactSource>>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for RuleInvocation {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::rules::RuleInvocation",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "uses",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Local or repository-qualified rule name.\n"),
                ::sidex::shape::FieldShape::new(
                    "inputs",
                    ::sidex::shape::shape_of::<
                        ::sidex::types::Map<::sidex::types::Str, ArtifactSource>,
                    >,
                )
                .with_docs("Artifact input bindings.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "parameters",
                    ::sidex::shape::shape_of::<
                        ::sidex::types::Map<::sidex::types::Str, ParameterBinding>,
                    >,
                )
                .with_docs("Parameter bindings.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "secrets",
                    ::sidex::shape::shape_of::<
                        ::sidex::types::Map<::sidex::types::Str, ArtifactSource>,
                    >,
                )
                .with_docs("Secret input bindings.\n")
                .optional(),
            ])),
        )
        .with_docs("An invocation of a repository-defined rule.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[::sidex::shape::Attr::Assign {
                path: "rename_all",
                value: ::sidex::shape::AttrValue::String("kebab-case"),
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for RuleInvocation {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for RuleInvocation {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.uses)),
                1usize => self
                    .inputs
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .parameters
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                3usize => self
                    .secrets
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for RuleInvocation {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<
                ::sidex::types::Map<::sidex::types::Str, ArtifactSource>,
            >::None;
            let mut __field_2 = ::std::option::Option::<
                ::sidex::types::Map<::sidex::types::Str, ParameterBinding>,
            >::None;
            let mut __field_3 = ::std::option::Option::<
                ::sidex::types::Map<::sidex::types::Str, ArtifactSource>,
            >::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            ArtifactSource,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            ParameterBinding,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            ArtifactSource,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                uses: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `uses`"))?,
                inputs: __field_1,
                parameters: __field_2,
                secrets: __field_3,
            })
        }
    }
    impl RuleInvocation {
        #[doc = "Creates a new [`RuleInvocation`]."]
        pub fn new(uses: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            Self {
                uses: uses.into(),
                inputs: ::std::default::Default::default(),
                parameters: ::std::default::Default::default(),
                secrets: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `uses`."]
        pub fn set_uses(
            &mut self,
            uses: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.uses = uses.into();
            self
        }
        #[doc = "Sets the value of `uses`."]
        pub fn with_uses(mut self, uses: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            self.uses = uses.into();
            self
        }
        #[doc = "Sets the value of `inputs`."]
        pub fn set_inputs(
            &mut self,
            inputs: ::std::option::Option<::sidex::types::Map<::sidex::types::Str, ArtifactSource>>,
        ) -> &mut Self {
            self.inputs = inputs;
            self
        }
        #[doc = "Sets the value of `inputs`."]
        pub fn with_inputs(
            mut self,
            inputs: ::std::option::Option<::sidex::types::Map<::sidex::types::Str, ArtifactSource>>,
        ) -> Self {
            self.inputs = inputs;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn set_parameters(
            &mut self,
            parameters: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, ParameterBinding>,
            >,
        ) -> &mut Self {
            self.parameters = parameters;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn with_parameters(
            mut self,
            parameters: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, ParameterBinding>,
            >,
        ) -> Self {
            self.parameters = parameters;
            self
        }
        #[doc = "Sets the value of `secrets`."]
        pub fn set_secrets(
            &mut self,
            secrets: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, ArtifactSource>,
            >,
        ) -> &mut Self {
            self.secrets = secrets;
            self
        }
        #[doc = "Sets the value of `secrets`."]
        pub fn with_secrets(
            mut self,
            secrets: ::std::option::Option<
                ::sidex::types::Map<::sidex::types::Str, ArtifactSource>,
            >,
        ) -> Self {
            self.secrets = secrets;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for RuleInvocation {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for RuleInvocation {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Filesystem object accepted by an input or produced by an output.\n"]
    #[derive(Clone, Debug, Copy, PartialEq, Eq)]
    pub enum ArtifactKind {
        #[doc = ""]
        File,
        #[doc = ""]
        Directory,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ArtifactKind {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::rules::ArtifactKind",
            ::sidex::shape::ShapeKind::Variant(
                ::sidex::shape::VariantShape::new(&[
                    ::sidex::shape::CaseShape::new("File", ::std::option::Option::None)
                        .json_name("file"),
                    ::sidex::shape::CaseShape::new("Directory", ::std::option::Option::None)
                        .json_name("directory"),
                ])
                .json_tagging(::sidex::formats::json::Tagging::External),
            ),
        )
        .with_docs("Filesystem object accepted by an input or produced by an output.\n")
        .with_attrs(&[
            ::sidex::shape::Attr::List {
                path: "json",
                args: &[
                    ::sidex::shape::Attr::Assign {
                        path: "tagged",
                        value: ::sidex::shape::AttrValue::Path("externally"),
                    },
                    ::sidex::shape::Attr::Assign {
                        path: "rename_all",
                        value: ::sidex::shape::AttrValue::String("kebab-case"),
                    },
                ],
            },
            ::sidex::shape::Attr::List {
                path: "rust",
                args: &[::sidex::shape::Attr::List {
                    path: "derive",
                    args: &[
                        ::sidex::shape::Attr::Path("Copy"),
                        ::sidex::shape::Attr::Path("PartialEq"),
                        ::sidex::shape::Attr::Path("Eq"),
                    ],
                }],
            },
        ]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for ArtifactKind {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for ArtifactKind {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::File => 0usize,
                Self::Directory => 1usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::File => ::std::option::Option::None,
                Self::Directory => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ArtifactKind {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::File,
                    1usize => Self::Directory,
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ArtifactKind {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ArtifactKind {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "An artifact reference within a composition.\n"]
    #[derive(Clone, Debug)]
    pub enum ArtifactSource {
        #[doc = "An input of the enclosing composition.\n"]
        Input(::sidex::types::Str),
        #[doc = "A file or directory relative to the repository containing this reference.\n"]
        File(::sidex::types::Str),
        #[doc = "An output of another node in this composition.\n"]
        Rule(RuleOutputSource),
        #[doc = "Select an existing file or directory within a directory artifact.\n"]
        Select(ArtifactSelection),
        #[doc = "Assemble artifacts at explicit, non-overlapping relative paths.\n"]
        Tree(::sidex::types::Map<::sidex::types::Str, ArtifactSource>),
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ArtifactSource {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::rugix_bakery::rules::ArtifactSource" , :: sidex :: shape :: ShapeKind :: Variant (:: sidex :: shape :: VariantShape :: new (& [:: sidex :: shape :: CaseShape :: new ("Input" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < :: sidex :: types :: Str >) ,) . with_docs ("An input of the enclosing composition.\n") . json_name ("input") , :: sidex :: shape :: CaseShape :: new ("File" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < :: sidex :: types :: Str >) ,) . with_docs ("A file or directory relative to the repository containing this reference.\n") . json_name ("file") , :: sidex :: shape :: CaseShape :: new ("Rule" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < RuleOutputSource >) ,) . with_docs ("An output of another node in this composition.\n") . json_name ("rule") , :: sidex :: shape :: CaseShape :: new ("Select" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < ArtifactSelection >) ,) . with_docs ("Select an existing file or directory within a directory artifact.\n") . json_name ("select") , :: sidex :: shape :: CaseShape :: new ("Tree" , :: std :: option :: Option :: Some (:: sidex :: shape :: shape_of :: < :: sidex :: types :: Map < :: sidex :: types :: Str , ArtifactSource , > >) ,) . with_docs ("Assemble artifacts at explicit, non-overlapping relative paths.\n") . json_name ("tree")]) . json_tagging (:: sidex :: formats :: json :: Tagging :: External))) . with_docs ("An artifact reference within a composition.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "tagged" , value : :: sidex :: shape :: AttrValue :: Path ("externally") } , :: sidex :: shape :: Attr :: Assign { path : "rename_all" , value : :: sidex :: shape :: AttrValue :: String ("kebab-case") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for ArtifactSource {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for ArtifactSource {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Input(_) => 0usize,
                Self::File(_) => 1usize,
                Self::Rule(_) => 2usize,
                Self::Select(_) => 3usize,
                Self::Tree(_) => 4usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Input(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::File(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Rule(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Select(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Tree(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ArtifactSource {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out =
                    ::std::option::Option::Some(match case {
                        0usize => {
                            Self::Input(<::sidex::types::Str as ::sidex::Decode>::decode(decoder)?)
                        }
                        1usize => {
                            Self::File(<::sidex::types::Str as ::sidex::Decode>::decode(decoder)?)
                        }
                        2usize => {
                            Self::Rule(<RuleOutputSource as ::sidex::Decode>::decode(decoder)?)
                        }
                        3usize => {
                            Self::Select(<ArtifactSelection as ::sidex::Decode>::decode(decoder)?)
                        }
                        4usize => Self::Tree(<::sidex::types::Map<
                            ::sidex::types::Str,
                            ArtifactSource,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?),
                        _ => {
                            return ::std::result::Result::Err(::sidex::DecodeError::new(
                                "unknown case",
                            ));
                        }
                    });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ArtifactSource {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ArtifactSource {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "A path within a directory artifact. Symlinks must resolve inside that directory.\n"]
    #[derive(Clone, Debug)]
    pub struct ArtifactSelection {
        #[doc = ""]
        pub source: ::std::boxed::Box<ArtifactSource>,
        #[doc = ""]
        pub path: ::sidex::types::Str,
        #[doc = ""]
        pub kind: ArtifactKind,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ArtifactSelection {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::rules::ArtifactSelection",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "source",
                    ::sidex::shape::shape_of::<::std::boxed::Box<ArtifactSource>>,
                )
                .with_attrs(&[::sidex::shape::Attr::List {
                    path: "rust",
                    args: &[::sidex::shape::Attr::Path("box")],
                }]),
                ::sidex::shape::FieldShape::new(
                    "path",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                ),
                ::sidex::shape::FieldShape::new("kind", ::sidex::shape::shape_of::<ArtifactKind>),
            ])),
        )
        .with_docs(
            "A path within a directory artifact. Symlinks must resolve inside that directory.\n",
        );
    }
    #[automatically_derived]
    impl ::sidex::Encode for ArtifactSelection {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ArtifactSelection {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.source)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.path)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.kind)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ArtifactSelection {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::std::boxed::Box<ArtifactSource>>::None;
            let mut __field_1 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_2 = ::std::option::Option::<ArtifactKind>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::std::boxed::Box<ArtifactSource> as ::sidex::Decode>::decode(
                                decoder,
                            )?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <ArtifactKind as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                source: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `source`"))?,
                path: __field_1.ok_or_else(|| ::sidex::DecodeError::new("missing field `path`"))?,
                kind: __field_2.ok_or_else(|| ::sidex::DecodeError::new("missing field `kind`"))?,
            })
        }
    }
    impl ArtifactSelection {
        #[doc = "Creates a new [`ArtifactSelection`]."]
        pub fn new(
            source: ::std::boxed::Box<ArtifactSource>,
            path: impl ::std::convert::Into<::sidex::types::Str>,
            kind: ArtifactKind,
        ) -> Self {
            Self {
                source: source,
                path: path.into(),
                kind: kind,
            }
        }
        #[doc = "Sets the value of `source`."]
        pub fn set_source(&mut self, source: ::std::boxed::Box<ArtifactSource>) -> &mut Self {
            self.source = source;
            self
        }
        #[doc = "Sets the value of `source`."]
        pub fn with_source(mut self, source: ::std::boxed::Box<ArtifactSource>) -> Self {
            self.source = source;
            self
        }
        #[doc = "Sets the value of `path`."]
        pub fn set_path(
            &mut self,
            path: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.path = path.into();
            self
        }
        #[doc = "Sets the value of `path`."]
        pub fn with_path(mut self, path: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            self.path = path.into();
            self
        }
        #[doc = "Sets the value of `kind`."]
        pub fn set_kind(&mut self, kind: ArtifactKind) -> &mut Self {
            self.kind = kind;
            self
        }
        #[doc = "Sets the value of `kind`."]
        pub fn with_kind(mut self, kind: ArtifactKind) -> Self {
            self.kind = kind;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ArtifactSelection {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ArtifactSelection {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Reference to a node output.\n"]
    #[derive(Clone, Debug)]
    pub struct RuleOutputSource {
        #[doc = ""]
        pub node: ::sidex::types::Str,
        #[doc = ""]
        pub output: ::sidex::types::Str,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for RuleOutputSource {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::rules::RuleOutputSource",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "node",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                ),
                ::sidex::shape::FieldShape::new(
                    "output",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                ),
            ])),
        )
        .with_docs("Reference to a node output.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for RuleOutputSource {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for RuleOutputSource {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.node)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.output)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for RuleOutputSource {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<::sidex::types::Str>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                node: __field_0.ok_or_else(|| ::sidex::DecodeError::new("missing field `node`"))?,
                output: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `output`"))?,
            })
        }
    }
    impl RuleOutputSource {
        #[doc = "Creates a new [`RuleOutputSource`]."]
        pub fn new(
            node: impl ::std::convert::Into<::sidex::types::Str>,
            output: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> Self {
            Self {
                node: node.into(),
                output: output.into(),
            }
        }
        #[doc = "Sets the value of `node`."]
        pub fn set_node(
            &mut self,
            node: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.node = node.into();
            self
        }
        #[doc = "Sets the value of `node`."]
        pub fn with_node(mut self, node: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            self.node = node.into();
            self
        }
        #[doc = "Sets the value of `output`."]
        pub fn set_output(
            &mut self,
            output: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.output = output.into();
            self
        }
        #[doc = "Sets the value of `output`."]
        pub fn with_output(
            mut self,
            output: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> Self {
            self.output = output.into();
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for RuleOutputSource {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for RuleOutputSource {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "A literal parameter value or a reference to an enclosing parameter.\n"]
    #[derive(Clone, Debug)]
    pub enum ParameterBinding {
        #[doc = ""]
        Reference(ParameterReference),
        #[doc = ""]
        Value(super::recipes::ParameterValue),
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ParameterBinding {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::rules::ParameterBinding",
            ::sidex::shape::ShapeKind::Variant(
                ::sidex::shape::VariantShape::new(&[
                    ::sidex::shape::CaseShape::new(
                        "Reference",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<ParameterReference>),
                    ),
                    ::sidex::shape::CaseShape::new(
                        "Value",
                        ::std::option::Option::Some(
                            ::sidex::shape::shape_of::<super::recipes::ParameterValue>,
                        ),
                    ),
                ])
                .json_tagging(::sidex::formats::json::Tagging::Implicit),
            ),
        )
        .with_docs("A literal parameter value or a reference to an enclosing parameter.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[::sidex::shape::Attr::Assign {
                path: "tagged",
                value: ::sidex::shape::AttrValue::Path("implicitly"),
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for ParameterBinding {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for ParameterBinding {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Reference(_) => 0usize,
                Self::Value(_) => 1usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Reference(value) => {
                    ::std::option::Option::Some(::sidex::Encode::encode(value))
                }
                Self::Value(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ParameterBinding {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => {
                        Self::Reference(<ParameterReference as ::sidex::Decode>::decode(decoder)?)
                    }
                    1usize => Self::Value(
                        <super::recipes::ParameterValue as ::sidex::Decode>::decode(decoder)?,
                    ),
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ParameterBinding {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ParameterBinding {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Reference to an enclosing parameter.\n"]
    #[derive(Clone, Debug)]
    pub struct ParameterReference {
        #[doc = ""]
        pub parameter: ::sidex::types::Str,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for ParameterReference {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::rules::ParameterReference",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "parameter",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                ),
            ])),
        )
        .with_docs("Reference to an enclosing parameter.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for ParameterReference {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for ParameterReference {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.parameter)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for ParameterReference {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                parameter: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `parameter`"))?,
            })
        }
    }
    impl ParameterReference {
        #[doc = "Creates a new [`ParameterReference`]."]
        pub fn new(parameter: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            Self {
                parameter: parameter.into(),
            }
        }
        #[doc = "Sets the value of `parameter`."]
        pub fn set_parameter(
            &mut self,
            parameter: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.parameter = parameter.into();
            self
        }
        #[doc = "Sets the value of `parameter`."]
        pub fn with_parameter(
            mut self,
            parameter: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> Self {
            self.parameter = parameter.into();
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for ParameterReference {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for ParameterReference {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Implementation of a reusable rule.\n"]
    #[derive(Clone, Debug)]
    pub enum RuleExecution {
        #[doc = ""]
        Run(RunRule),
        #[doc = ""]
        Graph(GraphRule),
        #[doc = "Build an existing layer with its recipes.\n"]
        Layer(LayerRule),
    }
    #[automatically_derived]
    impl ::sidex::Shaped for RuleExecution {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::rules::RuleExecution",
            ::sidex::shape::ShapeKind::Variant(
                ::sidex::shape::VariantShape::new(&[
                    ::sidex::shape::CaseShape::new(
                        "Run",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<RunRule>),
                    )
                    .json_name("run"),
                    ::sidex::shape::CaseShape::new(
                        "Graph",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<GraphRule>),
                    )
                    .json_name("graph"),
                    ::sidex::shape::CaseShape::new(
                        "Layer",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<LayerRule>),
                    )
                    .with_docs("Build an existing layer with its recipes.\n")
                    .json_name("layer"),
                ])
                .json_tagging(::sidex::formats::json::Tagging::Internal {
                    tag: "type",
                    content: "content",
                }),
            ),
        )
        .with_docs("Implementation of a reusable rule.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[
                ::sidex::shape::Attr::Assign {
                    path: "tag",
                    value: ::sidex::shape::AttrValue::String("type"),
                },
                ::sidex::shape::Attr::Assign {
                    path: "rename_all",
                    value: ::sidex::shape::AttrValue::String("kebab-case"),
                },
            ],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for RuleExecution {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for RuleExecution {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Run(_) => 0usize,
                Self::Graph(_) => 1usize,
                Self::Layer(_) => 2usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Run(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Graph(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Layer(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for RuleExecution {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Run(<RunRule as ::sidex::Decode>::decode(decoder)?),
                    1usize => Self::Graph(<GraphRule as ::sidex::Decode>::decode(decoder)?),
                    2usize => Self::Layer(<LayerRule as ::sidex::Decode>::decode(decoder)?),
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for RuleExecution {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for RuleExecution {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "A command executed in Bakery or a fresh tooling environment.\n"]
    #[derive(Clone, Debug)]
    pub struct RunRule {
        #[doc = "Optional environment resolved relative to the defining repository. Defaults to Bakery.\n"]
        pub env: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Command and literal arguments. No shell expansion is performed by Bakery.\n"]
        pub command: ::sidex::types::Seq<::sidex::types::Str>,
        #[doc = "Files and directories to publish from /run/rugix/bakery/work/outputs.\n"]
        pub outputs: ::sidex::types::Map<::sidex::types::Str, RuleOutput>,
        #[doc = "Reuse successful outputs. Defaults to true.\n"]
        pub cache: ::std::option::Option<bool>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for RunRule {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::rugix_bakery::rules::RunRule" , :: sidex :: shape :: ShapeKind :: Record (:: sidex :: shape :: RecordShape :: new (& [:: sidex :: shape :: FieldShape :: new ("env" , :: sidex :: shape :: shape_of :: < :: sidex :: types :: Str > ,) . with_docs ("Optional environment resolved relative to the defining repository. Defaults to Bakery.\n") . optional () , :: sidex :: shape :: FieldShape :: new ("command" , :: sidex :: shape :: shape_of :: < :: sidex :: types :: Seq < :: sidex :: types :: Str , > > ,) . with_docs ("Command and literal arguments. No shell expansion is performed by Bakery.\n") , :: sidex :: shape :: FieldShape :: new ("outputs" , :: sidex :: shape :: shape_of :: < :: sidex :: types :: Map < :: sidex :: types :: Str , RuleOutput , > > ,) . with_docs ("Files and directories to publish from /run/rugix/bakery/work/outputs.\n") , :: sidex :: shape :: FieldShape :: new ("cache" , :: sidex :: shape :: shape_of :: < bool > ,) . with_docs ("Reuse successful outputs. Defaults to true.\n") . optional ()]))) . with_docs ("A command executed in Bakery or a fresh tooling environment.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "rename_all" , value : :: sidex :: shape :: AttrValue :: String ("kebab-case") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for RunRule {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for RunRule {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .env
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.command)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.outputs)),
                3usize => self
                    .cache
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for RunRule {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 =
                ::std::option::Option::<::sidex::types::Seq<::sidex::types::Str>>::None;
            let mut __field_2 =
                ::std::option::Option::<::sidex::types::Map<::sidex::types::Str, RuleOutput>>::None;
            let mut __field_3 = ::std::option::Option::<bool>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(<::sidex::types::Seq<
                            ::sidex::types::Str,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            RuleOutput,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    3usize => {
                        __field_3 =
                            ::std::option::Option::Some(<bool as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                env: __field_0,
                command: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `command`"))?,
                outputs: __field_2
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `outputs`"))?,
                cache: __field_3,
            })
        }
    }
    impl RunRule {
        #[doc = "Creates a new [`RunRule`]."]
        pub fn new(
            command: ::sidex::types::Seq<::sidex::types::Str>,
            outputs: ::sidex::types::Map<::sidex::types::Str, RuleOutput>,
        ) -> Self {
            Self {
                command: command,
                outputs: outputs,
                env: ::std::default::Default::default(),
                cache: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `env`."]
        pub fn set_env(&mut self, env: ::std::option::Option<::sidex::types::Str>) -> &mut Self {
            self.env = env;
            self
        }
        #[doc = "Sets the value of `env`."]
        pub fn with_env(mut self, env: ::std::option::Option<::sidex::types::Str>) -> Self {
            self.env = env;
            self
        }
        #[doc = "Sets the value of `command`."]
        pub fn set_command(
            &mut self,
            command: ::sidex::types::Seq<::sidex::types::Str>,
        ) -> &mut Self {
            self.command = command;
            self
        }
        #[doc = "Sets the value of `command`."]
        pub fn with_command(mut self, command: ::sidex::types::Seq<::sidex::types::Str>) -> Self {
            self.command = command;
            self
        }
        #[doc = "Sets the value of `outputs`."]
        pub fn set_outputs(
            &mut self,
            outputs: ::sidex::types::Map<::sidex::types::Str, RuleOutput>,
        ) -> &mut Self {
            self.outputs = outputs;
            self
        }
        #[doc = "Sets the value of `outputs`."]
        pub fn with_outputs(
            mut self,
            outputs: ::sidex::types::Map<::sidex::types::Str, RuleOutput>,
        ) -> Self {
            self.outputs = outputs;
            self
        }
        #[doc = "Sets the value of `cache`."]
        pub fn set_cache(&mut self, cache: ::std::option::Option<bool>) -> &mut Self {
            self.cache = cache;
            self
        }
        #[doc = "Sets the value of `cache`."]
        pub fn with_cache(mut self, cache: ::std::option::Option<bool>) -> Self {
            self.cache = cache;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for RunRule {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for RunRule {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "A declared command output.\n"]
    #[derive(Clone, Debug)]
    pub struct RuleOutput {
        #[doc = "Description for users of the output artifact.\n"]
        pub description: ::std::option::Option<::sidex::types::Str>,
        #[doc = ""]
        pub kind: ArtifactKind,
        #[doc = "Relative path within /run/rugix/bakery/work/outputs.\n"]
        pub path: ::sidex::types::Str,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for RuleOutput {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::rules::RuleOutput",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "description",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Description for users of the output artifact.\n")
                .optional(),
                ::sidex::shape::FieldShape::new("kind", ::sidex::shape::shape_of::<ArtifactKind>),
                ::sidex::shape::FieldShape::new(
                    "path",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Relative path within /run/rugix/bakery/work/outputs.\n"),
            ])),
        )
        .with_docs("A declared command output.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for RuleOutput {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for RuleOutput {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .description
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.kind)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.path)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for RuleOutput {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<ArtifactKind>::None;
            let mut __field_2 = ::std::option::Option::<::sidex::types::Str>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <ArtifactKind as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                description: __field_0,
                kind: __field_1.ok_or_else(|| ::sidex::DecodeError::new("missing field `kind`"))?,
                path: __field_2.ok_or_else(|| ::sidex::DecodeError::new("missing field `path`"))?,
            })
        }
    }
    impl RuleOutput {
        #[doc = "Creates a new [`RuleOutput`]."]
        pub fn new(
            kind: ArtifactKind,
            path: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> Self {
            Self {
                kind: kind,
                path: path.into(),
                description: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<::sidex::types::Str>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(
            mut self,
            description: ::std::option::Option<::sidex::types::Str>,
        ) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `kind`."]
        pub fn set_kind(&mut self, kind: ArtifactKind) -> &mut Self {
            self.kind = kind;
            self
        }
        #[doc = "Sets the value of `kind`."]
        pub fn with_kind(mut self, kind: ArtifactKind) -> Self {
            self.kind = kind;
            self
        }
        #[doc = "Sets the value of `path`."]
        pub fn set_path(
            &mut self,
            path: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.path = path.into();
            self
        }
        #[doc = "Sets the value of `path`."]
        pub fn with_path(mut self, path: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            self.path = path.into();
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for RuleOutput {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for RuleOutput {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "A composition with the same interface as an executable rule.\n"]
    #[derive(Clone, Debug)]
    pub struct GraphRule {
        #[doc = ""]
        pub rules: ::sidex::types::Map<::sidex::types::Str, RuleInvocation>,
        #[doc = ""]
        pub outputs: ::sidex::types::Map<::sidex::types::Str, ArtifactSource>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for GraphRule {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::rules::GraphRule",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "rules",
                    ::sidex::shape::shape_of::<
                        ::sidex::types::Map<::sidex::types::Str, RuleInvocation>,
                    >,
                ),
                ::sidex::shape::FieldShape::new(
                    "outputs",
                    ::sidex::shape::shape_of::<
                        ::sidex::types::Map<::sidex::types::Str, ArtifactSource>,
                    >,
                ),
            ])),
        )
        .with_docs("A composition with the same interface as an executable rule.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for GraphRule {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for GraphRule {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.rules)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.outputs)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for GraphRule {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<
                ::sidex::types::Map<::sidex::types::Str, RuleInvocation>,
            >::None;
            let mut __field_1 = ::std::option::Option::<
                ::sidex::types::Map<::sidex::types::Str, ArtifactSource>,
            >::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            RuleInvocation,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(<::sidex::types::Map<
                            ::sidex::types::Str,
                            ArtifactSource,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                rules: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `rules`"))?,
                outputs: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `outputs`"))?,
            })
        }
    }
    impl GraphRule {
        #[doc = "Creates a new [`GraphRule`]."]
        pub fn new(
            rules: ::sidex::types::Map<::sidex::types::Str, RuleInvocation>,
            outputs: ::sidex::types::Map<::sidex::types::Str, ArtifactSource>,
        ) -> Self {
            Self {
                rules: rules,
                outputs: outputs,
            }
        }
        #[doc = "Sets the value of `rules`."]
        pub fn set_rules(
            &mut self,
            rules: ::sidex::types::Map<::sidex::types::Str, RuleInvocation>,
        ) -> &mut Self {
            self.rules = rules;
            self
        }
        #[doc = "Sets the value of `rules`."]
        pub fn with_rules(
            mut self,
            rules: ::sidex::types::Map<::sidex::types::Str, RuleInvocation>,
        ) -> Self {
            self.rules = rules;
            self
        }
        #[doc = "Sets the value of `outputs`."]
        pub fn set_outputs(
            &mut self,
            outputs: ::sidex::types::Map<::sidex::types::Str, ArtifactSource>,
        ) -> &mut Self {
            self.outputs = outputs;
            self
        }
        #[doc = "Sets the value of `outputs`."]
        pub fn with_outputs(
            mut self,
            outputs: ::sidex::types::Map<::sidex::types::Str, ArtifactSource>,
        ) -> Self {
            self.outputs = outputs;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for GraphRule {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for GraphRule {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "A layer executed as part of an artifact graph.\n"]
    #[derive(Clone, Debug)]
    pub struct LayerRule {
        #[doc = "Layer name resolved from the rule's repository.\n"]
        pub layer: ::sidex::types::Str,
        #[doc = ""]
        pub architecture: super::systems::Architecture,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for LayerRule {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::rules::LayerRule",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "layer",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Layer name resolved from the rule's repository.\n"),
                ::sidex::shape::FieldShape::new(
                    "architecture",
                    ::sidex::shape::shape_of::<super::systems::Architecture>,
                ),
            ])),
        )
        .with_docs("A layer executed as part of an artifact graph.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for LayerRule {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for LayerRule {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.layer)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.architecture)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for LayerRule {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<super::systems::Architecture>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <super::systems::Architecture as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                layer: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `layer`"))?,
                architecture: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `architecture`"))?,
            })
        }
    }
    impl LayerRule {
        #[doc = "Creates a new [`LayerRule`]."]
        pub fn new(
            layer: impl ::std::convert::Into<::sidex::types::Str>,
            architecture: super::systems::Architecture,
        ) -> Self {
            Self {
                layer: layer.into(),
                architecture: architecture,
            }
        }
        #[doc = "Sets the value of `layer`."]
        pub fn set_layer(
            &mut self,
            layer: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.layer = layer.into();
            self
        }
        #[doc = "Sets the value of `layer`."]
        pub fn with_layer(mut self, layer: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            self.layer = layer.into();
            self
        }
        #[doc = "Sets the value of `architecture`."]
        pub fn set_architecture(
            &mut self,
            architecture: super::systems::Architecture,
        ) -> &mut Self {
            self.architecture = architecture;
            self
        }
        #[doc = "Sets the value of `architecture`."]
        pub fn with_architecture(mut self, architecture: super::systems::Architecture) -> Self {
            self.architecture = architecture;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for LayerRule {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for LayerRule {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
}
pub mod systems {
    #![doc = "System configuration.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct SystemConfig {
        #[doc = "Layer the image is based on.\n"]
        pub layer: ::sidex::types::Str,
        #[doc = "Mixins enabled by default for the system.\n"]
        pub mixins: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        #[doc = "Architecture of the image.\n"]
        pub architecture: Architecture,
        #[doc = "Rugix Bakery target.\n"]
        pub target: ::std::option::Option<Target>,
        #[doc = "System image configuration.\n"]
        pub image: ::std::option::Option<SystemImageConfig>,
        #[doc = "Repository-defined image assembly, receiving the existing layer build.\n"]
        pub build: ::std::option::Option<super::rules::RuleInvocation>,
        #[doc = "Additional options.\n"]
        pub options: ::std::option::Option<SystemOptions>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for SystemConfig {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::systems::SystemConfig",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "layer",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Layer the image is based on.\n"),
                ::sidex::shape::FieldShape::new(
                    "mixins",
                    ::sidex::shape::shape_of::<::sidex::types::Seq<::sidex::types::Str>>,
                )
                .with_docs("Mixins enabled by default for the system.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "architecture",
                    ::sidex::shape::shape_of::<Architecture>,
                )
                .with_docs("Architecture of the image.\n"),
                ::sidex::shape::FieldShape::new("target", ::sidex::shape::shape_of::<Target>)
                    .with_docs("Rugix Bakery target.\n")
                    .optional(),
                ::sidex::shape::FieldShape::new(
                    "image",
                    ::sidex::shape::shape_of::<SystemImageConfig>,
                )
                .with_docs("System image configuration.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "build",
                    ::sidex::shape::shape_of::<super::rules::RuleInvocation>,
                )
                .with_docs(
                    "Repository-defined image assembly, receiving the existing layer build.\n",
                )
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "options",
                    ::sidex::shape::shape_of::<SystemOptions>,
                )
                .with_docs("Additional options.\n")
                .optional(),
            ])),
        );
    }
    #[automatically_derived]
    impl ::sidex::Encode for SystemConfig {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for SystemConfig {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.layer)),
                1usize => self
                    .mixins
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.architecture)),
                3usize => self
                    .target
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                4usize => self
                    .image
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                5usize => self
                    .build
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                6usize => self
                    .options
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for SystemConfig {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 =
                ::std::option::Option::<::sidex::types::Seq<::sidex::types::Str>>::None;
            let mut __field_2 = ::std::option::Option::<Architecture>::None;
            let mut __field_3 = ::std::option::Option::<Target>::None;
            let mut __field_4 = ::std::option::Option::<SystemImageConfig>::None;
            let mut __field_5 = ::std::option::Option::<super::rules::RuleInvocation>::None;
            let mut __field_6 = ::std::option::Option::<SystemOptions>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(<::sidex::types::Seq<
                            ::sidex::types::Str,
                        > as ::sidex::Decode>::decode(
                            decoder
                        )?)
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <Architecture as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 = ::std::option::Option::Some(
                            <Target as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    4usize => {
                        __field_4 = ::std::option::Option::Some(
                            <SystemImageConfig as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    5usize => {
                        __field_5 = ::std::option::Option::Some(
                            <super::rules::RuleInvocation as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    6usize => {
                        __field_6 = ::std::option::Option::Some(
                            <SystemOptions as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                layer: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `layer`"))?,
                mixins: __field_1,
                architecture: __field_2
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `architecture`"))?,
                target: __field_3,
                image: __field_4,
                build: __field_5,
                options: __field_6,
            })
        }
    }
    impl SystemConfig {
        #[doc = "Creates a new [`SystemConfig`]."]
        pub fn new(
            layer: impl ::std::convert::Into<::sidex::types::Str>,
            architecture: Architecture,
        ) -> Self {
            Self {
                layer: layer.into(),
                architecture: architecture,
                mixins: ::std::default::Default::default(),
                target: ::std::default::Default::default(),
                image: ::std::default::Default::default(),
                build: ::std::default::Default::default(),
                options: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `layer`."]
        pub fn set_layer(
            &mut self,
            layer: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.layer = layer.into();
            self
        }
        #[doc = "Sets the value of `layer`."]
        pub fn with_layer(mut self, layer: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            self.layer = layer.into();
            self
        }
        #[doc = "Sets the value of `mixins`."]
        pub fn set_mixins(
            &mut self,
            mixins: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        ) -> &mut Self {
            self.mixins = mixins;
            self
        }
        #[doc = "Sets the value of `mixins`."]
        pub fn with_mixins(
            mut self,
            mixins: ::std::option::Option<::sidex::types::Seq<::sidex::types::Str>>,
        ) -> Self {
            self.mixins = mixins;
            self
        }
        #[doc = "Sets the value of `architecture`."]
        pub fn set_architecture(&mut self, architecture: Architecture) -> &mut Self {
            self.architecture = architecture;
            self
        }
        #[doc = "Sets the value of `architecture`."]
        pub fn with_architecture(mut self, architecture: Architecture) -> Self {
            self.architecture = architecture;
            self
        }
        #[doc = "Sets the value of `target`."]
        pub fn set_target(&mut self, target: ::std::option::Option<Target>) -> &mut Self {
            self.target = target;
            self
        }
        #[doc = "Sets the value of `target`."]
        pub fn with_target(mut self, target: ::std::option::Option<Target>) -> Self {
            self.target = target;
            self
        }
        #[doc = "Sets the value of `image`."]
        pub fn set_image(&mut self, image: ::std::option::Option<SystemImageConfig>) -> &mut Self {
            self.image = image;
            self
        }
        #[doc = "Sets the value of `image`."]
        pub fn with_image(mut self, image: ::std::option::Option<SystemImageConfig>) -> Self {
            self.image = image;
            self
        }
        #[doc = "Sets the value of `build`."]
        pub fn set_build(
            &mut self,
            build: ::std::option::Option<super::rules::RuleInvocation>,
        ) -> &mut Self {
            self.build = build;
            self
        }
        #[doc = "Sets the value of `build`."]
        pub fn with_build(
            mut self,
            build: ::std::option::Option<super::rules::RuleInvocation>,
        ) -> Self {
            self.build = build;
            self
        }
        #[doc = "Sets the value of `options`."]
        pub fn set_options(&mut self, options: ::std::option::Option<SystemOptions>) -> &mut Self {
            self.options = options;
            self
        }
        #[doc = "Sets the value of `options`."]
        pub fn with_options(mut self, options: ::std::option::Option<SystemOptions>) -> Self {
            self.options = options;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for SystemConfig {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for SystemConfig {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct SystemOptions {
        #[doc = "Use squashfs for system filesystems by default.\n"]
        pub use_squashfs: ::std::option::Option<super::images::SquashfsOptions>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for SystemOptions {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::systems::SystemOptions",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "use_squashfs",
                    ::sidex::shape::shape_of::<super::images::SquashfsOptions>,
                )
                .with_docs("Use squashfs for system filesystems by default.\n")
                .optional()
                .json_name("use-squashfs"),
            ])),
        )
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[::sidex::shape::Attr::Assign {
                path: "rename_all",
                value: ::sidex::shape::AttrValue::String("kebab-case"),
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for SystemOptions {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for SystemOptions {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .use_squashfs
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for SystemOptions {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<super::images::SquashfsOptions>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <super::images::SquashfsOptions as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                use_squashfs: __field_0,
            })
        }
    }
    impl SystemOptions {
        #[doc = "Creates a new [`SystemOptions`]."]
        pub fn new() -> Self {
            Self {
                use_squashfs: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `use_squashfs`."]
        pub fn set_use_squashfs(
            &mut self,
            use_squashfs: ::std::option::Option<super::images::SquashfsOptions>,
        ) -> &mut Self {
            self.use_squashfs = use_squashfs;
            self
        }
        #[doc = "Sets the value of `use_squashfs`."]
        pub fn with_use_squashfs(
            mut self,
            use_squashfs: ::std::option::Option<super::images::SquashfsOptions>,
        ) -> Self {
            self.use_squashfs = use_squashfs;
            self
        }
    }
    impl ::std::default::Default for SystemOptions {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for SystemOptions {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for SystemOptions {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Architecture.\n"]
    #[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
    pub enum Architecture {
        #[doc = "64-bit x86.\n"]
        Amd64,
        #[doc = "64-bit ARMv8.\n"]
        Arm64,
        #[doc = "32-bit ARMv7.\n"]
        Armv7,
        #[doc = "32-bit ARMv6 (Hard-Float).\n"]
        Armhf,
        #[doc = "32-bit ARMv6.\n"]
        Arm,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Architecture {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::systems::Architecture",
            ::sidex::shape::ShapeKind::Variant(
                ::sidex::shape::VariantShape::new(&[
                    ::sidex::shape::CaseShape::new("Amd64", ::std::option::Option::None)
                        .with_docs("64-bit x86.\n")
                        .json_name("amd64"),
                    ::sidex::shape::CaseShape::new("Arm64", ::std::option::Option::None)
                        .with_docs("64-bit ARMv8.\n")
                        .json_name("arm64"),
                    ::sidex::shape::CaseShape::new("Armv7", ::std::option::Option::None)
                        .with_docs("32-bit ARMv7.\n")
                        .json_name("armv7"),
                    ::sidex::shape::CaseShape::new("Armhf", ::std::option::Option::None)
                        .with_docs("32-bit ARMv6 (Hard-Float).\n")
                        .json_name("armhf"),
                    ::sidex::shape::CaseShape::new("Arm", ::std::option::Option::None)
                        .with_docs("32-bit ARMv6.\n")
                        .json_name("arm"),
                ])
                .json_tagging(::sidex::formats::json::Tagging::External),
            ),
        )
        .with_docs("Architecture.\n")
        .with_attrs(&[
            ::sidex::shape::Attr::List {
                path: "json",
                args: &[::sidex::shape::Attr::Assign {
                    path: "rename_all",
                    value: ::sidex::shape::AttrValue::String("lowercase"),
                }],
            },
            ::sidex::shape::Attr::List {
                path: "json",
                args: &[::sidex::shape::Attr::Assign {
                    path: "tagged",
                    value: ::sidex::shape::AttrValue::Path("externally"),
                }],
            },
            ::sidex::shape::Attr::List {
                path: "rust",
                args: &[::sidex::shape::Attr::List {
                    path: "derive",
                    args: &[
                        ::sidex::shape::Attr::Path("Copy"),
                        ::sidex::shape::Attr::Path("PartialEq"),
                        ::sidex::shape::Attr::Path("Eq"),
                        ::sidex::shape::Attr::Path("Hash"),
                    ],
                }],
            },
        ]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for Architecture {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for Architecture {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Amd64 => 0usize,
                Self::Arm64 => 1usize,
                Self::Armv7 => 2usize,
                Self::Armhf => 3usize,
                Self::Arm => 4usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Amd64 => ::std::option::Option::None,
                Self::Arm64 => ::std::option::Option::None,
                Self::Armv7 => ::std::option::Option::None,
                Self::Armhf => ::std::option::Option::None,
                Self::Arm => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Architecture {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Amd64,
                    1usize => Self::Arm64,
                    2usize => Self::Armv7,
                    3usize => Self::Armhf,
                    4usize => Self::Arm,
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for Architecture {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for Architecture {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Target.\n"]
    #[derive(Clone, Debug)]
    pub enum Target {
        #[doc = "Generic target for EFI-compatible systems.\n"]
        GenericGrubEfi,
        #[doc = "Raspberry Pi-specific target using the `tryboot` mechanism.\n"]
        RpiTryboot,
        #[doc = "Raspberry Pi-specific target using U-Boot.\n"]
        RpiUboot,
        #[doc = "Custom BSP target — reads disk layout from `bsp/rugix-bsp.toml` in the layer.\n"]
        Bsp,
        #[doc = "Target for unsupported devices.\n"]
        Unknown,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for Target {
        const SHAPE : & 'static :: sidex :: Shape = & :: sidex :: Shape :: new ("::rugix_bakery::systems::Target" , :: sidex :: shape :: ShapeKind :: Variant (:: sidex :: shape :: VariantShape :: new (& [:: sidex :: shape :: CaseShape :: new ("GenericGrubEfi" , :: std :: option :: Option :: None) . with_docs ("Generic target for EFI-compatible systems.\n") . json_name ("generic-grub-efi") , :: sidex :: shape :: CaseShape :: new ("RpiTryboot" , :: std :: option :: Option :: None) . with_docs ("Raspberry Pi-specific target using the `tryboot` mechanism.\n") . json_name ("rpi-tryboot") , :: sidex :: shape :: CaseShape :: new ("RpiUboot" , :: std :: option :: Option :: None) . with_docs ("Raspberry Pi-specific target using U-Boot.\n") . json_name ("rpi-uboot") , :: sidex :: shape :: CaseShape :: new ("Bsp" , :: std :: option :: Option :: None) . with_docs ("Custom BSP target — reads disk layout from `bsp/rugix-bsp.toml` in the layer.\n") . json_name ("bsp") , :: sidex :: shape :: CaseShape :: new ("Unknown" , :: std :: option :: Option :: None) . with_docs ("Target for unsupported devices.\n") . json_name ("unknown")]) . json_tagging (:: sidex :: formats :: json :: Tagging :: External))) . with_docs ("Target.\n") . with_attrs (& [:: sidex :: shape :: Attr :: List { path : "json" , args : & [:: sidex :: shape :: Attr :: Assign { path : "tagged" , value : :: sidex :: shape :: AttrValue :: Path ("externally") } , :: sidex :: shape :: Attr :: Assign { path : "rename_all" , value : :: sidex :: shape :: AttrValue :: String ("kebab-case") }] }]) ;
    }
    #[automatically_derived]
    impl ::sidex::Encode for Target {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for Target {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::GenericGrubEfi => 0usize,
                Self::RpiTryboot => 1usize,
                Self::RpiUboot => 2usize,
                Self::Bsp => 3usize,
                Self::Unknown => 4usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::GenericGrubEfi => ::std::option::Option::None,
                Self::RpiTryboot => ::std::option::Option::None,
                Self::RpiUboot => ::std::option::Option::None,
                Self::Bsp => ::std::option::Option::None,
                Self::Unknown => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for Target {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::GenericGrubEfi,
                    1usize => Self::RpiTryboot,
                    2usize => Self::RpiUboot,
                    3usize => Self::Bsp,
                    4usize => Self::Unknown,
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for Target {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for Target {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct SystemImageConfig {
        #[doc = "Size of the image.\n"]
        pub size: ::std::option::Option<super::foreign::NumBytes>,
        #[doc = "Layout of the image.\n"]
        pub layout: ::std::option::Option<super::images::ImageLayout>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for SystemImageConfig {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::systems::SystemImageConfig",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "size",
                    ::sidex::shape::shape_of::<super::foreign::NumBytes>,
                )
                .with_docs("Size of the image.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "layout",
                    ::sidex::shape::shape_of::<super::images::ImageLayout>,
                )
                .with_docs("Layout of the image.\n")
                .optional(),
            ])),
        );
    }
    #[automatically_derived]
    impl ::sidex::Encode for SystemImageConfig {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for SystemImageConfig {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .size
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => self
                    .layout
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for SystemImageConfig {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<super::foreign::NumBytes>::None;
            let mut __field_1 = ::std::option::Option::<super::images::ImageLayout>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <super::foreign::NumBytes as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <super::images::ImageLayout as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                size: __field_0,
                layout: __field_1,
            })
        }
    }
    impl SystemImageConfig {
        #[doc = "Creates a new [`SystemImageConfig`]."]
        pub fn new() -> Self {
            Self {
                size: ::std::default::Default::default(),
                layout: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `size`."]
        pub fn set_size(
            &mut self,
            size: ::std::option::Option<super::foreign::NumBytes>,
        ) -> &mut Self {
            self.size = size;
            self
        }
        #[doc = "Sets the value of `size`."]
        pub fn with_size(mut self, size: ::std::option::Option<super::foreign::NumBytes>) -> Self {
            self.size = size;
            self
        }
        #[doc = "Sets the value of `layout`."]
        pub fn set_layout(
            &mut self,
            layout: ::std::option::Option<super::images::ImageLayout>,
        ) -> &mut Self {
            self.layout = layout;
            self
        }
        #[doc = "Sets the value of `layout`."]
        pub fn with_layout(
            mut self,
            layout: ::std::option::Option<super::images::ImageLayout>,
        ) -> Self {
            self.layout = layout;
            self
        }
    }
    impl ::std::default::Default for SystemImageConfig {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for SystemImageConfig {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for SystemImageConfig {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
}
pub mod tests {
    #![doc = "Test configuration.\n"]
    #![allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        clippy::cargo,
        dead_code
    )]
    #[doc = "Test configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct TestConfig {
        #[doc = "Systems to run the test on.\n"]
        pub systems: ::sidex::types::Seq<SystemConfig>,
        #[doc = "Steps of the test.\n"]
        pub steps: ::sidex::types::Seq<TestStep>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for TestConfig {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::tests::TestConfig",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "systems",
                    ::sidex::shape::shape_of::<::sidex::types::Seq<SystemConfig>>,
                )
                .with_docs("Systems to run the test on.\n"),
                ::sidex::shape::FieldShape::new(
                    "steps",
                    ::sidex::shape::shape_of::<::sidex::types::Seq<TestStep>>,
                )
                .with_docs("Steps of the test.\n"),
            ])),
        )
        .with_docs("Test configuration.\n");
    }
    #[automatically_derived]
    impl ::sidex::Encode for TestConfig {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for TestConfig {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.systems)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.steps)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for TestConfig {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Seq<SystemConfig>>::None;
            let mut __field_1 = ::std::option::Option::<::sidex::types::Seq<TestStep>>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Seq<SystemConfig> as ::sidex::Decode>::decode(
                                decoder,
                            )?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::sidex::types::Seq<TestStep> as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                systems: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `systems`"))?,
                steps: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `steps`"))?,
            })
        }
    }
    impl TestConfig {
        #[doc = "Creates a new [`TestConfig`]."]
        pub fn new(
            systems: ::sidex::types::Seq<SystemConfig>,
            steps: ::sidex::types::Seq<TestStep>,
        ) -> Self {
            Self {
                systems: systems,
                steps: steps,
            }
        }
        #[doc = "Sets the value of `systems`."]
        pub fn set_systems(&mut self, systems: ::sidex::types::Seq<SystemConfig>) -> &mut Self {
            self.systems = systems;
            self
        }
        #[doc = "Sets the value of `systems`."]
        pub fn with_systems(mut self, systems: ::sidex::types::Seq<SystemConfig>) -> Self {
            self.systems = systems;
            self
        }
        #[doc = "Sets the value of `steps`."]
        pub fn set_steps(&mut self, steps: ::sidex::types::Seq<TestStep>) -> &mut Self {
            self.steps = steps;
            self
        }
        #[doc = "Sets the value of `steps`."]
        pub fn with_steps(mut self, steps: ::sidex::types::Seq<TestStep>) -> Self {
            self.steps = steps;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for TestConfig {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for TestConfig {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "System configuration for testing.\n"]
    #[derive(Clone, Debug)]
    pub struct SystemConfig {
        #[doc = "Image to use.\n"]
        pub system: ::sidex::types::Str,
        #[doc = "Size of the disk.\n"]
        pub disk_size: ::std::option::Option<super::foreign::NumBytes>,
        #[doc = "SSH configuration.\n"]
        pub ssh: ::std::option::Option<SshConfig>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for SystemConfig {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::tests::SystemConfig",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "system",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Image to use.\n"),
                ::sidex::shape::FieldShape::new(
                    "disk_size",
                    ::sidex::shape::shape_of::<super::foreign::NumBytes>,
                )
                .with_docs("Size of the disk.\n")
                .optional()
                .json_name("disk-size"),
                ::sidex::shape::FieldShape::new("ssh", ::sidex::shape::shape_of::<SshConfig>)
                    .with_docs("SSH configuration.\n")
                    .optional(),
            ])),
        )
        .with_docs("System configuration for testing.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[::sidex::shape::Attr::Assign {
                path: "rename_all",
                value: ::sidex::shape::AttrValue::String("kebab-case"),
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for SystemConfig {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for SystemConfig {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.system)),
                1usize => self
                    .disk_size
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                2usize => self
                    .ssh
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for SystemConfig {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<super::foreign::NumBytes>::None;
            let mut __field_2 = ::std::option::Option::<SshConfig>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <super::foreign::NumBytes as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <SshConfig as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                system: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `system`"))?,
                disk_size: __field_1,
                ssh: __field_2,
            })
        }
    }
    impl SystemConfig {
        #[doc = "Creates a new [`SystemConfig`]."]
        pub fn new(system: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            Self {
                system: system.into(),
                disk_size: ::std::default::Default::default(),
                ssh: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `system`."]
        pub fn set_system(
            &mut self,
            system: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.system = system.into();
            self
        }
        #[doc = "Sets the value of `system`."]
        pub fn with_system(
            mut self,
            system: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> Self {
            self.system = system.into();
            self
        }
        #[doc = "Sets the value of `disk_size`."]
        pub fn set_disk_size(
            &mut self,
            disk_size: ::std::option::Option<super::foreign::NumBytes>,
        ) -> &mut Self {
            self.disk_size = disk_size;
            self
        }
        #[doc = "Sets the value of `disk_size`."]
        pub fn with_disk_size(
            mut self,
            disk_size: ::std::option::Option<super::foreign::NumBytes>,
        ) -> Self {
            self.disk_size = disk_size;
            self
        }
        #[doc = "Sets the value of `ssh`."]
        pub fn set_ssh(&mut self, ssh: ::std::option::Option<SshConfig>) -> &mut Self {
            self.ssh = ssh;
            self
        }
        #[doc = "Sets the value of `ssh`."]
        pub fn with_ssh(mut self, ssh: ::std::option::Option<SshConfig>) -> Self {
            self.ssh = ssh;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for SystemConfig {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for SystemConfig {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "SSH configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct SshConfig {
        #[doc = "Path to the private key.\n"]
        pub private_key: ::sidex::types::Str,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for SshConfig {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::tests::SshConfig",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "private_key",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Path to the private key.\n")
                .json_name("private-key"),
            ])),
        )
        .with_docs("SSH configuration.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[::sidex::shape::Attr::Assign {
                path: "rename_all",
                value: ::sidex::shape::AttrValue::String("kebab-case"),
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for SshConfig {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for SshConfig {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.private_key)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for SshConfig {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                private_key: __field_0
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `private-key`"))?,
            })
        }
    }
    impl SshConfig {
        #[doc = "Creates a new [`SshConfig`]."]
        pub fn new(private_key: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            Self {
                private_key: private_key.into(),
            }
        }
        #[doc = "Sets the value of `private_key`."]
        pub fn set_private_key(
            &mut self,
            private_key: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.private_key = private_key.into();
            self
        }
        #[doc = "Sets the value of `private_key`."]
        pub fn with_private_key(
            mut self,
            private_key: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> Self {
            self.private_key = private_key.into();
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for SshConfig {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for SshConfig {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Test step.\n"]
    #[derive(Clone, Debug)]
    pub enum TestStep {
        #[doc = "Run a script on the SUT.\n"]
        Run(RunStep),
        #[doc = "Wait for a given amount of time.\n"]
        Wait(WaitStep),
    }
    #[automatically_derived]
    impl ::sidex::Shaped for TestStep {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::tests::TestStep",
            ::sidex::shape::ShapeKind::Variant(
                ::sidex::shape::VariantShape::new(&[
                    ::sidex::shape::CaseShape::new(
                        "Run",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<RunStep>),
                    )
                    .with_docs("Run a script on the SUT.\n")
                    .json_name("run"),
                    ::sidex::shape::CaseShape::new(
                        "Wait",
                        ::std::option::Option::Some(::sidex::shape::shape_of::<WaitStep>),
                    )
                    .with_docs("Wait for a given amount of time.\n")
                    .json_name("wait"),
                ])
                .json_tagging(::sidex::formats::json::Tagging::Internal {
                    tag: "action",
                    content: "content",
                }),
            ),
        )
        .with_docs("Test step.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[
                ::sidex::shape::Attr::Assign {
                    path: "tag",
                    value: ::sidex::shape::AttrValue::String("action"),
                },
                ::sidex::shape::Attr::Assign {
                    path: "rename_all",
                    value: ::sidex::shape::AttrValue::String("kebab-case"),
                },
            ],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for TestStep {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Variant(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::VariantView for TestStep {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn case(&self) -> usize {
            match *self {
                Self::Run(_) => 0usize,
                Self::Wait(_) => 1usize,
            }
        }
        fn payload(&self) -> ::std::option::Option<::sidex::View<'_>> {
            match self {
                Self::Run(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
                Self::Wait(value) => ::std::option::Option::Some(::sidex::Encode::encode(value)),
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for TestStep {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut out = ::std::option::Option::<Self>::None;
            decoder.variant(<Self as ::sidex::Shaped>::SHAPE, &mut |case, decoder| {
                out = ::std::option::Option::Some(match case {
                    0usize => Self::Run(<RunStep as ::sidex::Decode>::decode(decoder)?),
                    1usize => Self::Wait(<WaitStep as ::sidex::Decode>::decode(decoder)?),
                    _ => {
                        return ::std::result::Result::Err(::sidex::DecodeError::new(
                            "unknown case",
                        ));
                    }
                });
                ::std::result::Result::Ok(())
            })?;
            out.ok_or_else(|| ::sidex::DecodeError::new("missing case"))
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for TestStep {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for TestStep {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Run step.\n"]
    #[derive(Clone, Debug)]
    pub struct RunStep {
        #[doc = "Description of the step.\n"]
        pub description: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Script to run.\n"]
        pub script: ::sidex::types::Str,
        #[doc = "File to provide on the standard input to the script.\n"]
        pub stdin_file: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Do no treat SSH disconnects as failures.\n"]
        pub may_disconnect: ::std::option::Option<bool>,
        #[doc = "Do not treat non-zero exit code as failures.\n"]
        pub may_fail: ::std::option::Option<bool>,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for RunStep {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::tests::RunStep",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "description",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Description of the step.\n")
                .optional(),
                ::sidex::shape::FieldShape::new(
                    "script",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Script to run.\n"),
                ::sidex::shape::FieldShape::new(
                    "stdin_file",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("File to provide on the standard input to the script.\n")
                .optional()
                .json_name("stdin-file"),
                ::sidex::shape::FieldShape::new("may_disconnect", ::sidex::shape::shape_of::<bool>)
                    .with_docs("Do no treat SSH disconnects as failures.\n")
                    .optional()
                    .json_name("may-disconnect"),
                ::sidex::shape::FieldShape::new("may_fail", ::sidex::shape::shape_of::<bool>)
                    .with_docs("Do not treat non-zero exit code as failures.\n")
                    .optional()
                    .json_name("may-fail"),
            ])),
        )
        .with_docs("Run step.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[::sidex::shape::Attr::Assign {
                path: "rename_all",
                value: ::sidex::shape::AttrValue::String("kebab-case"),
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for RunStep {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for RunStep {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .description
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.script)),
                2usize => self
                    .stdin_file
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                3usize => self
                    .may_disconnect
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                4usize => self
                    .may_fail
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for RunStep {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_2 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_3 = ::std::option::Option::<bool>::None;
            let mut __field_4 = ::std::option::Option::<bool>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    2usize => {
                        __field_2 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    3usize => {
                        __field_3 =
                            ::std::option::Option::Some(<bool as ::sidex::Decode>::decode(decoder)?)
                    }
                    4usize => {
                        __field_4 =
                            ::std::option::Option::Some(<bool as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                description: __field_0,
                script: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `script`"))?,
                stdin_file: __field_2,
                may_disconnect: __field_3,
                may_fail: __field_4,
            })
        }
    }
    impl RunStep {
        #[doc = "Creates a new [`RunStep`]."]
        pub fn new(script: impl ::std::convert::Into<::sidex::types::Str>) -> Self {
            Self {
                script: script.into(),
                description: ::std::default::Default::default(),
                stdin_file: ::std::default::Default::default(),
                may_disconnect: ::std::default::Default::default(),
                may_fail: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<::sidex::types::Str>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(
            mut self,
            description: ::std::option::Option<::sidex::types::Str>,
        ) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `script`."]
        pub fn set_script(
            &mut self,
            script: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> &mut Self {
            self.script = script.into();
            self
        }
        #[doc = "Sets the value of `script`."]
        pub fn with_script(
            mut self,
            script: impl ::std::convert::Into<::sidex::types::Str>,
        ) -> Self {
            self.script = script.into();
            self
        }
        #[doc = "Sets the value of `stdin_file`."]
        pub fn set_stdin_file(
            &mut self,
            stdin_file: ::std::option::Option<::sidex::types::Str>,
        ) -> &mut Self {
            self.stdin_file = stdin_file;
            self
        }
        #[doc = "Sets the value of `stdin_file`."]
        pub fn with_stdin_file(
            mut self,
            stdin_file: ::std::option::Option<::sidex::types::Str>,
        ) -> Self {
            self.stdin_file = stdin_file;
            self
        }
        #[doc = "Sets the value of `may_disconnect`."]
        pub fn set_may_disconnect(
            &mut self,
            may_disconnect: ::std::option::Option<bool>,
        ) -> &mut Self {
            self.may_disconnect = may_disconnect;
            self
        }
        #[doc = "Sets the value of `may_disconnect`."]
        pub fn with_may_disconnect(mut self, may_disconnect: ::std::option::Option<bool>) -> Self {
            self.may_disconnect = may_disconnect;
            self
        }
        #[doc = "Sets the value of `may_fail`."]
        pub fn set_may_fail(&mut self, may_fail: ::std::option::Option<bool>) -> &mut Self {
            self.may_fail = may_fail;
            self
        }
        #[doc = "Sets the value of `may_fail`."]
        pub fn with_may_fail(mut self, may_fail: ::std::option::Option<bool>) -> Self {
            self.may_fail = may_fail;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for RunStep {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for RunStep {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
    #[doc = "Wait step.\n"]
    #[derive(Clone, Debug)]
    pub struct WaitStep {
        #[doc = "Description of the step.\n"]
        pub description: ::std::option::Option<::sidex::types::Str>,
        #[doc = "Time to wait in seconds.\n"]
        pub duration: f64,
    }
    #[automatically_derived]
    impl ::sidex::Shaped for WaitStep {
        const SHAPE: &'static ::sidex::Shape = &::sidex::Shape::new(
            "::rugix_bakery::tests::WaitStep",
            ::sidex::shape::ShapeKind::Record(::sidex::shape::RecordShape::new(&[
                ::sidex::shape::FieldShape::new(
                    "description",
                    ::sidex::shape::shape_of::<::sidex::types::Str>,
                )
                .with_docs("Description of the step.\n")
                .optional(),
                ::sidex::shape::FieldShape::new("duration", ::sidex::shape::shape_of::<f64>)
                    .with_docs("Time to wait in seconds.\n"),
            ])),
        )
        .with_docs("Wait step.\n")
        .with_attrs(&[::sidex::shape::Attr::List {
            path: "json",
            args: &[::sidex::shape::Attr::Assign {
                path: "rename_all",
                value: ::sidex::shape::AttrValue::String("kebab-case"),
            }],
        }]);
    }
    #[automatically_derived]
    impl ::sidex::Encode for WaitStep {
        fn encode(&self) -> ::sidex::View<'_> {
            ::sidex::View::Record(self)
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::encode::RecordView for WaitStep {
        fn shape(&self) -> &'static ::sidex::Shape {
            <Self as ::sidex::Shaped>::SHAPE
        }
        fn field(&self, index: usize) -> ::std::option::Option<::sidex::View<'_>> {
            match index {
                0usize => self
                    .description
                    .as_ref()
                    .map(|value| ::sidex::Encode::encode(value)),
                1usize => ::std::option::Option::Some(::sidex::Encode::encode(&self.duration)),
                _ => ::std::option::Option::None,
            }
        }
    }
    #[automatically_derived]
    #[allow(deprecated)]
    impl ::sidex::Decode for WaitStep {
        fn decode(
            decoder: &mut dyn ::sidex::Decoder,
        ) -> ::std::result::Result<Self, ::sidex::DecodeError> {
            let mut __field_0 = ::std::option::Option::<::sidex::types::Str>::None;
            let mut __field_1 = ::std::option::Option::<f64>::None;
            decoder.record(<Self as ::sidex::Shaped>::SHAPE, &mut |index, decoder| {
                match index {
                    0usize => {
                        __field_0 = ::std::option::Option::Some(
                            <::sidex::types::Str as ::sidex::Decode>::decode(decoder)?,
                        )
                    }
                    1usize => {
                        __field_1 =
                            ::std::option::Option::Some(<f64 as ::sidex::Decode>::decode(decoder)?)
                    }
                    _ => {}
                }
                ::std::result::Result::Ok(())
            })?;
            ::std::result::Result::Ok(Self {
                description: __field_0,
                duration: __field_1
                    .ok_or_else(|| ::sidex::DecodeError::new("missing field `duration`"))?,
            })
        }
    }
    impl WaitStep {
        #[doc = "Creates a new [`WaitStep`]."]
        pub fn new(duration: f64) -> Self {
            Self {
                duration: duration,
                description: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<::sidex::types::Str>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(
            mut self,
            description: ::std::option::Option<::sidex::types::Str>,
        ) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `duration`."]
        pub fn set_duration(&mut self, duration: f64) -> &mut Self {
            self.duration = duration;
            self
        }
        #[doc = "Sets the value of `duration`."]
        pub fn with_duration(mut self, duration: f64) -> Self {
            self.duration = duration;
            self
        }
    }
    #[automatically_derived]
    impl ::serde::Serialize for WaitStep {
        fn serialize<__S: ::serde::Serializer>(
            &self,
            serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            ::sidex::serde::serialize(self, serializer)
        }
    }
    #[automatically_derived]
    impl<'de> ::serde::Deserialize<'de> for WaitStep {
        fn deserialize<__D: ::serde::Deserializer<'de>>(
            deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            ::sidex::serde::deserialize(deserializer)
        }
    }
}
