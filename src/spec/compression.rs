// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::error::{Result, ZipError};

/// A compression method supported by this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compression {
    Stored,
    Deflate,
    Bz,
    Lzma,
    Zstd,
    Xz,
}

impl Compression {
    // Convert a supported compression method into its relevant u16 stored with little endianness.
    // https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#445
    pub fn to_u16(&self) -> u16 {
        match self {
            Compression::Stored => 0,
            Compression::Deflate => 8,
            Compression::Bz => 12,
            Compression::Lzma => 14,
            Compression::Zstd => 93,
            Compression::Xz => 95,
        }
    }

    // Convert a u16 stored with little endianness into a supported compression method.
    // https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#445
    pub fn from_u16(value: u16) -> Result<Compression> {
        match value {
            0 => Ok(Compression::Stored),
            8 => Ok(Compression::Deflate),
            12 => Ok(Compression::Bz),
            14 => Ok(Compression::Lzma),
            93 => Ok(Compression::Zstd),
            95 => Ok(Compression::Xz),
            _ => Err(ZipError::UnsupportedCompressionError(value)),
        }
    }
}
