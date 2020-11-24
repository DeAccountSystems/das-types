#[cfg(not(feature = "std"))]
use alloc::borrow::ToOwned;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use ckb_std::ckb_types::packed;
#[cfg(feature = "std")]
use ckb_types::packed;

use crate::schemas::basic::{Bytes, Uint8, Uint16, Uint32, Uint32Reader, Uint64, Hash, OutPoint, Script, Uint8Reader};
use core::convert::TryFrom;
use molecule::{
    error::VerificationError,
    prelude::{Builder, Byte, Entity},
};

macro_rules! impl_uint_convert {
    // `()` indicates that the macro takes no argument.
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

impl From<Vec<u8>> for Bytes {
    fn from(v: Vec<u8>) -> Self {
        Bytes::new_builder()
            .set(v.into_iter().map(Byte::new).collect())
            .build()
    }
}

impl From<u32> for Uint32 {
    fn from(v: u32) -> Self {
        let mut inner = [Byte::new(0); 4];
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

impl From<Uint32> for u32 {
    fn from(v: Uint32) -> Self {
        let mut buf = [0u8; 4];
        buf.copy_from_slice(v.raw_data().as_ref());
        u32::from_le_bytes(buf)
    }
}

impl From<Uint32Reader<'_>> for u32 {
    fn from(v: Uint32Reader<'_>) -> Self {
        let mut buf = [0u8; 4];
        buf.copy_from_slice(v.raw_data());
        u32::from_le_bytes(buf)
    }
}

impl_uint_convert!(u8, Uint8, Uint8Reader, 1);

impl TryFrom<Vec<u8>> for Hash {
    type Error = VerificationError;
    fn try_from(v: Vec<u8>) -> Result<Self, VerificationError> {
        if v.len() != 32 {
            return Err(VerificationError::TotalSizeNotMatch(
                "Byte32".to_owned(),
                32,
                v.len(),
            ));
        }
        let mut inner = [Byte::new(0); 32];
        let v = v.into_iter().map(Byte::new).collect::<Vec<_>>();
        inner.copy_from_slice(&v);
        Ok(Self::new_builder().set(inner).build())
    }
}

impl From<packed::Script> for Script {
    fn from(v: packed::Script) -> Self {
        Self::new_unchecked(v.as_bytes())
    }
}

impl From<packed::OutPoint> for OutPoint {
    fn from(v: packed::OutPoint) -> Self {
        Self::new_unchecked(v.as_bytes())
    }
}
