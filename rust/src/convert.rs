#[cfg(not(feature = "std"))]
use alloc::borrow::ToOwned;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::schemas::basic::*;
use core::convert::TryFrom;
use core::str;
use core::str::Utf8Error;

use molecule::{
    error::VerificationError,
    prelude::{Builder, Byte, Entity},
};

/// Implement convert between primitive type and molecule types
macro_rules! impl_uint_convert {
    ($uint_type:ty, $mol_type:ty, $mol_reader_typer:ident, $length: expr) => {
        impl From<$uint_type> for $mol_type {
            fn from(v: $uint_type) -> Self {
                let mut inner = [Byte::new(0); $length];
                let v = v
                    .to_le_bytes()
                    .to_vec()
                    .into_iter()
                    .map(Byte::new)
                    .collect::<Vec<_>>();
                inner.copy_from_slice(&v);
                Self::new_builder().set(inner).build()
            }
        }

        impl From<$mol_type> for $uint_type {
            fn from(v: $mol_type) -> Self {
                let mut buf = [0u8; $length];
                buf.copy_from_slice(v.raw_data().as_ref());
                <$uint_type>::from_le_bytes(buf)
            }
        }

        impl From<$mol_reader_typer<'_>> for $uint_type {
            fn from(v: $mol_reader_typer<'_>) -> Self {
                let mut buf = [0u8; $length];
                buf.copy_from_slice(v.raw_data());
                <$uint_type>::from_le_bytes(buf)
            }
        }
    };
}

impl_uint_convert!(u8, Uint8, Uint8Reader, 1);
impl_uint_convert!(u16, Uint16, Uint16Reader, 2);
impl_uint_convert!(u32, Uint32, Uint32Reader, 4);
impl_uint_convert!(u64, Uint64, Uint64Reader, 8);
impl_uint_convert!(u128, Uint128, Uint128Reader, 16);

impl_uint_convert!(u64, Timestamp, TimestampReader, 8);

/// Convert &[u8] to schemas::basic::Bytes
///
/// The difference with from_slice is that it does not require a dynvec header.
impl From<&[u8]> for Bytes {
    fn from(v: &[u8]) -> Self {
        Bytes::new_builder()
            .set(v.to_owned().into_iter().map(Byte::new).collect())
            .build()
    }
}

/// Convert Vec<u8> to schemas::basic::Bytes
///
/// The difference with from_slice is that it does not require a dynvec header.
impl From<Vec<u8>> for Bytes {
    fn from(v: Vec<u8>) -> Self {
        Bytes::from(v.as_slice())
    }
}

/// Convert schemas::basic::Bytes to Vec<u8>
///
/// The main thing here is to remove the Header from the Molecule data.
impl From<Bytes> for Vec<u8> {
    fn from(v: Bytes) -> Self {
        v.as_slice()
            .get(4..)
            .map(|v| Vec::from(v))
            .unwrap_or(Vec::new())
    }
}

/// Convert schemas::basic::Bytes to String
///
/// The main thing here is to remove the Header from the Molecule data.
#[cfg(feature = "std")]
impl TryFrom<Bytes> for String {
    type Error = Utf8Error;
    fn try_from(v: Bytes) -> Result<Self, Utf8Error> {
        v.as_slice()
            .get(4..)
            .map(|v| str::from_utf8(v).map(|v| String::from(v)))
            .unwrap_or(Ok(String::from("")))
    }
}

/// Convert &[u8] to schemas::basic::Hash
///
/// The difference with from_slice is that it does not require a dynvec header.
impl TryFrom<&[u8]> for Hash {
    type Error = VerificationError;
    fn try_from(v: &[u8]) -> Result<Self, VerificationError> {
        if v.len() != 32 {
            return Err(VerificationError::TotalSizeNotMatch(
                "Byte32".to_owned(),
                32,
                v.len(),
            ));
        }
        let mut inner = [Byte::new(0); 32];
        let v = v.to_owned().into_iter().map(Byte::new).collect::<Vec<_>>();
        inner.copy_from_slice(&v);
        Ok(Self::new_builder().set(inner).build())
    }
}

impl TryFrom<Vec<u8>> for Hash {
    type Error = VerificationError;
    fn try_from(v: Vec<u8>) -> Result<Self, VerificationError> {
        Hash::try_from(v.as_slice())
    }
}

/// Convert schemas::basic::Hash to Vec<u8>
impl From<Hash> for Vec<u8> {
    fn from(v: Hash) -> Self {
        v.as_slice().to_vec()
    }
}
