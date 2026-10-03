//! Provides traits [`FromStream`], [`Decoder`], [`ToStream`] and [`Encoder`], which are
//! streaming/async analogues of [`serde`]'s `Deserialize`, `Deserializer`, `Serialize`,
//! and `Serializer`.
//!
//! The visitor model and portions of this crate's implementation are adapted from [`serde`].
//! Implement `destream` traits directly; this crate provides no derive macros.
//!
//! [`serde`] is dual-licensed under the MIT and Apache-2.0 licenses, which are available at
//! [https://github.com/serde-rs/serde/blob/master/LICENSE-MIT](https://github.com/serde-rs/serde/blob/master/LICENSE-MIT)
//! and
//! [https://github.com/serde-rs/serde/blob/master/LICENSE-APACHE](https://github.com/serde-rs/serde/blob/master/LICENSE-APACHE)
//! respectively.
//!
//! Ordinary typed values use [`FromStream`], [`ToStream`], and [`IntoStream`].
//! Decoding constructs values without borrowing input buffers; encoding can borrow values.
//!
//! For input-dependent nesting, value owners use iterative traversal with an explicit
//! frame stack: [`de::Container`] provides a decoding cursor, and [`en::Event`]
//! provides a pull-based structural encoding stream. Events alone do not prevent
//! call-stack exhaustion. Owners must also make destruction and other structural
//! operations stack-safe, and retain depth/allocation limits to bound heap usage and work.
//! Each codec owns event validation, byte framing, and leaf-stream consumption.
//!
//! `destream` itself does not implement support for any specific serialization format.
//! [`destream_json`] provides support for streaming JSON.
//!
//! [`destream_json`]: http://docs.rs/destream_json/
//! [`serde`]: http://docs.rs/serde

pub mod de;
pub mod en;

pub use de::{ArrayAccess, Decoder, FromStream, IgnoredAny, MapAccess, SeqAccess, Visitor};
pub use en::{
    EncodeMap, EncodeSeq, EncodeTuple, Encoder, IntoStream, MapStream, SeqStream, ToStream,
};
