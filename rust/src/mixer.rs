use super::schemas::packed::*;
use molecule::error::{VerificationError, VerificationResult};
use std::prelude::v1::*;

pub trait AccountCellDataMixer {
    fn version(&self) -> u32;
    fn as_reader(&self) -> Box<dyn AccountCellDataReaderMixer + '_>;
}

impl AccountCellDataMixer for AccountCellDataV1 {
    fn version(&self) -> u32 {
        1
    }

    fn as_reader(&self) -> Box<dyn AccountCellDataReaderMixer + '_> {
        Box::new(self.as_reader())
    }
}

impl AccountCellDataMixer for AccountCellData {
    fn version(&self) -> u32 {
        2
    }

    fn as_reader(&self) -> Box<dyn AccountCellDataReaderMixer + '_> {
        Box::new(self.as_reader())
    }
}

pub trait AccountCellDataReaderMixer<'r> {
    fn version(&self) -> u32;
    fn try_into_v1(&self) -> VerificationResult<AccountCellDataV1Reader<'r>>;
    fn try_into_latest(&self) -> VerificationResult<AccountCellDataReader<'r>>;
}

impl<'r> AccountCellDataReaderMixer<'r> for AccountCellDataV1Reader<'r> {
    fn version(&self) -> u32 {
        1
    }

    fn try_into_v1(&self) -> VerificationResult<AccountCellDataV1Reader<'r>> {
        AccountCellDataV1Reader::from_slice(self.as_slice())
    }

    fn try_into_latest(&self) -> VerificationResult<AccountCellDataReader<'r>> {
        Err(VerificationError::OffsetsNotMatch("AccountCellDataReader".to_string()))
    }
}

impl<'r> AccountCellDataReaderMixer<'r> for AccountCellDataReader<'r> {
    fn version(&self) -> u32 {
        2
    }

    fn try_into_v1(&self) -> VerificationResult<AccountCellDataV1Reader<'r>> {
        Err(VerificationError::OffsetsNotMatch(
            "AccountCellDataV1Reader".to_string(),
        ))
    }

    fn try_into_latest(&self) -> VerificationResult<AccountCellDataReader<'r>> {
        AccountCellDataReader::from_slice(self.as_slice())
    }
}
