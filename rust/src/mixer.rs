use super::schemas::packed::*;
use molecule::error::{VerificationError, VerificationResult};

macro_rules! gen_trait_common_fns {
    ({$( $fn_name:ident -> $fn_return:ty ),+}) => {
        $(fn $fn_name(&self) -> $fn_return;)+
    };
}

macro_rules! gen_trait_field_fns {
    ({$( $field:ident -> $field_type:ty ),+}) => {
        $(fn $field(&self) -> $field_type;)+
    };
}

macro_rules! gen_impl_field_fns {
    ({$( $field:ident -> $field_type:ty ),+}) => {
        $(
            fn $field(&self) -> $field_type {
                self.$field()
            }
        )+
    };
}

pub trait AccountCellDataMixer {
    gen_trait_common_fns!({
        version -> u32,
        as_reader -> Box<dyn AccountCellDataReaderMixer + '_>
    });
}

impl AccountCellDataMixer for AccountCellDataV2 {
    fn version(&self) -> u32 {
        2
    }

    fn as_reader(&self) -> Box<dyn AccountCellDataReaderMixer + '_> {
        Box::new(self.as_reader())
    }
}

impl AccountCellDataMixer for AccountCellData {
    fn version(&self) -> u32 {
        3
    }

    fn as_reader(&self) -> Box<dyn AccountCellDataReaderMixer + '_> {
        Box::new(self.as_reader())
    }
}

pub trait AccountCellDataReaderMixer<'r> {
    gen_trait_common_fns!({
        version -> u32,
        try_into_v2 -> VerificationResult<AccountCellDataV2Reader<'r>>,
        try_into_latest -> VerificationResult<AccountCellDataReader<'r>>
    });

    gen_trait_field_fns!({
        id -> AccountIdReader<'r>,
        account -> AccountCharsReader<'r>,
        registered_at -> Uint64Reader<'r>,
        last_transfer_account_at -> Uint64Reader<'r>,
        last_edit_manager_at -> Uint64Reader<'r>,
        last_edit_records_at -> Uint64Reader<'r>,
        status -> Uint8Reader<'r>,
        records -> RecordsReader<'r>
    });
}

impl<'r> AccountCellDataReaderMixer<'r> for AccountCellDataV2Reader<'r> {
    fn version(&self) -> u32 {
        2
    }

    fn try_into_v2(&self) -> VerificationResult<AccountCellDataV2Reader<'r>> {
        AccountCellDataV2Reader::from_slice(self.as_slice())
    }

    fn try_into_latest(&self) -> VerificationResult<AccountCellDataReader<'r>> {
        Err(VerificationError::OffsetsNotMatch("AccountCellDataReader".to_string()))
    }

    gen_impl_field_fns!({
        id -> AccountIdReader<'r>,
        account -> AccountCharsReader<'r>,
        registered_at -> Uint64Reader<'r>,
        last_transfer_account_at -> Uint64Reader<'r>,
        last_edit_manager_at -> Uint64Reader<'r>,
        last_edit_records_at -> Uint64Reader<'r>,
        status -> Uint8Reader<'r>,
        records -> RecordsReader<'r>
    });
}

impl<'r> AccountCellDataReaderMixer<'r> for AccountCellDataReader<'r> {
    fn version(&self) -> u32 {
        3
    }

    fn try_into_v2(&self) -> VerificationResult<AccountCellDataV2Reader<'r>> {
        Err(VerificationError::OffsetsNotMatch(
            "AccountCellDataV2Reader".to_string(),
        ))
    }

    fn try_into_latest(&self) -> VerificationResult<AccountCellDataReader<'r>> {
        AccountCellDataReader::from_slice(self.as_slice())
    }

    gen_impl_field_fns!({
        id -> AccountIdReader<'r>,
        account -> AccountCharsReader<'r>,
        registered_at -> Uint64Reader<'r>,
        last_transfer_account_at -> Uint64Reader<'r>,
        last_edit_manager_at -> Uint64Reader<'r>,
        last_edit_records_at -> Uint64Reader<'r>,
        status -> Uint8Reader<'r>,
        records -> RecordsReader<'r>
    });
}

pub trait AccountSaleCellDataMixer {
    gen_trait_common_fns!({
        version -> u32,
        as_reader -> Box<dyn AccountSaleCellDataReaderMixer + '_>
    });
}

impl AccountSaleCellDataMixer for AccountSaleCellDataV1 {
    fn version(&self) -> u32 {
        1
    }

    fn as_reader(&self) -> Box<dyn AccountSaleCellDataReaderMixer + '_> {
        Box::new(self.as_reader())
    }
}

impl AccountSaleCellDataMixer for AccountSaleCellData {
    fn version(&self) -> u32 {
        2
    }

    fn as_reader(&self) -> Box<dyn AccountSaleCellDataReaderMixer + '_> {
        Box::new(self.as_reader())
    }
}

pub trait AccountSaleCellDataReaderMixer<'r> {
    gen_trait_common_fns!({
        version -> u32,
        try_into_v1 -> VerificationResult<AccountSaleCellDataV1Reader<'r>>,
        try_into_latest -> VerificationResult<AccountSaleCellDataReader<'r>>
    });

    gen_trait_field_fns!({
        account_id -> AccountIdReader<'r>,
        account -> BytesReader<'r>,
        price -> Uint64Reader<'r>,
        description -> BytesReader<'r>,
        started_at -> Uint64Reader<'r>
    });
}

impl<'r> AccountSaleCellDataReaderMixer<'r> for AccountSaleCellDataV1Reader<'r> {
    fn version(&self) -> u32 {
        1
    }

    fn try_into_v1(&self) -> VerificationResult<AccountSaleCellDataV1Reader<'r>> {
        AccountSaleCellDataV1Reader::from_slice(self.as_slice())
    }

    fn try_into_latest(&self) -> VerificationResult<AccountSaleCellDataReader<'r>> {
        Err(VerificationError::OffsetsNotMatch("AccountCellDataReader".to_string()))
    }

    gen_impl_field_fns!({
        account_id -> AccountIdReader<'r>,
        account -> BytesReader<'r>,
        price -> Uint64Reader<'r>,
        description -> BytesReader<'r>,
        started_at -> Uint64Reader<'r>
    });
}

impl<'r> AccountSaleCellDataReaderMixer<'r> for AccountSaleCellDataReader<'r> {
    fn version(&self) -> u32 {
        2
    }

    fn try_into_v1(&self) -> VerificationResult<AccountSaleCellDataV1Reader<'r>> {
        Err(VerificationError::OffsetsNotMatch(
            "AccountCellDataV2Reader".to_string(),
        ))
    }

    fn try_into_latest(&self) -> VerificationResult<AccountSaleCellDataReader<'r>> {
        AccountSaleCellDataReader::from_slice(self.as_slice())
    }

    gen_impl_field_fns!({
        account_id -> AccountIdReader<'r>,
        account -> BytesReader<'r>,
        price -> Uint64Reader<'r>,
        description -> BytesReader<'r>,
        started_at -> Uint64Reader<'r>
    });
}
