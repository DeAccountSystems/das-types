use super::constants::*;
use super::schemas::packed::*;
use core::convert::TryFrom;
use molecule::prelude::*;

pub fn is_entity_eq<T: Entity>(a: &T, b: &T) -> bool {
    a.as_slice() == b.as_slice()
}

pub fn is_reader_eq<'a, T: Reader<'a>>(a: T, b: T) -> bool {
    a.as_slice() == b.as_slice()
}

pub fn data_type_to_char_set(data_type: DataType) -> CharSetType {
    CharSetType::try_from(data_type as u32 - 100000).unwrap()
}

pub fn char_set_to_data_type(char_set: CharSetType) -> DataType {
    DataType::try_from(char_set as u32 + 100000).unwrap()
}

pub fn data_type_to_preserved_accounts_group(data_type: DataType) -> usize {
    data_type as u32 as usize - 10000
}

pub fn preserved_accounts_group_to_data_type(group: usize) -> DataType {
    DataType::try_from(group as u32 + 10000).unwrap()
}

pub fn wrap_data_entity(version: u32, index: u32, entity: impl Entity) -> DataEntity {
    DataEntity::new_builder()
        .version(Uint32::from(version))
        .index(Uint32::from(index))
        .entity(Bytes::from(entity.as_slice()))
        .build()
}

pub fn wrap_data_entity_opt(version: u32, index: u32, entity: impl Entity) -> DataEntityOpt {
    DataEntityOpt::new_builder()
        .set(Some(wrap_data_entity(version, index, entity)))
        .build()
}

pub fn wrap_raw_witness(data_type: DataType, mut bytes: Vec<u8>) -> Bytes {
    let mut data = Vec::new();
    let mut data_type_bytes = (data_type as u32).to_le_bytes().to_vec();
    data.append(&mut WITNESS_HEADER.to_vec());
    data.append(&mut data_type_bytes);
    data.append(&mut bytes);

    Bytes::new_builder()
        .set(data.into_iter().map(Byte::new).collect())
        .build()
}

pub fn wrap_entity_witness(data_type: DataType, entity: impl Entity) -> Bytes {
    let mut data = Vec::new();
    let mut data_type_bytes = (data_type as u32).to_le_bytes().to_vec();
    data.append(&mut WITNESS_HEADER.to_vec());
    data.append(&mut data_type_bytes);
    data.append(&mut entity.as_slice().to_vec());

    Bytes::new_builder()
        .set(data.into_iter().map(Byte::new).collect())
        .build()
}

pub fn wrap_action_witness(action: &str, params_opt: Option<Bytes>) -> Bytes {
    let mut builder = ActionData::new_builder().action(Bytes::from(action.as_bytes()));

    if let Some(params) = params_opt {
        builder = builder.params(params);
    }

    wrap_entity_witness(DataType::ActionData, builder.build())
}

#[deprecated(since = "1.3.0", note = "Please use `wrap_data_witness_v3` instead.")]
pub fn wrap_data_witness<A: Entity, B: Entity, C: Entity>(
    data_type: DataType,
    output_opt: Option<(u32, u32, A)>,
    input_opt: Option<(u32, u32, B)>,
    dep_opt: Option<(u32, u32, C)>,
) -> Bytes {
    let mut new = DataEntityOpt::default();
    if let Some((version, index, output)) = output_opt {
        new = wrap_data_entity_opt(version, index, output);
    }

    let mut old = DataEntityOpt::default();
    if let Some((version, index, input)) = input_opt {
        old = wrap_data_entity_opt(version, index, input);
    }

    let mut dep = DataEntityOpt::default();
    if let Some((version, index, dep_)) = dep_opt {
        dep = wrap_data_entity_opt(version, index, dep_);
    }

    let builder = Data::new_builder().dep(dep).new(new).old(old);

    wrap_entity_witness(data_type, builder.build())
}

pub enum EntityWrapper {
    ActionData(ActionData),
    PreAccountCellData(PreAccountCellData),
    ProposalCellData(ProposalCellData),
    AccountCellData(AccountCellData),
    AccountCellDataV2(AccountCellDataV2),
    AccountSaleCellData(AccountSaleCellData),
    AccountSaleCellDataV1(AccountSaleCellDataV1),
    AccountAuctionCellData(AccountAuctionCellData),
    IncomeCellData(IncomeCellData),
    OfferCellData(OfferCellData),
    SubAccount(SubAccount),
    ConfigCellAccount(ConfigCellAccount),
    ConfigCellApply(ConfigCellApply),
    ConfigCellIncome(ConfigCellIncome),
    ConfigCellMain(ConfigCellMain),
    ConfigCellPrice(ConfigCellPrice),
    ConfigCellProposal(ConfigCellProposal),
    ConfigCellProfitRate(ConfigCellProfitRate),
    ConfigCellRelease(ConfigCellRelease),
    ConfigCellSecondaryMarket(ConfigCellSecondaryMarket),
    ConfigCellReverseResolution(ConfigCellReverseResolution),
    ConfigCellSubAccount(ConfigCellSubAccount),
}

pub fn wrap_entity_witness_v3(data_type: DataType, entity: EntityWrapper) -> Bytes {
    let mut data = Vec::new();
    let mut data_type_bytes = (data_type as u32).to_le_bytes().to_vec();
    data.append(&mut WITNESS_HEADER.to_vec());
    data.append(&mut data_type_bytes);

    let mut entity_bytes = match entity {
        EntityWrapper::ActionData(entity) => entity.as_slice().to_vec(),
        EntityWrapper::ConfigCellAccount(entity) => entity.as_slice().to_vec(),
        EntityWrapper::ConfigCellApply(entity) => entity.as_slice().to_vec(),
        EntityWrapper::ConfigCellIncome(entity) => entity.as_slice().to_vec(),
        EntityWrapper::ConfigCellMain(entity) => entity.as_slice().to_vec(),
        EntityWrapper::ConfigCellPrice(entity) => entity.as_slice().to_vec(),
        EntityWrapper::ConfigCellProposal(entity) => entity.as_slice().to_vec(),
        EntityWrapper::ConfigCellProfitRate(entity) => entity.as_slice().to_vec(),
        EntityWrapper::ConfigCellRelease(entity) => entity.as_slice().to_vec(),
        EntityWrapper::ConfigCellSecondaryMarket(entity) => entity.as_slice().to_vec(),
        EntityWrapper::ConfigCellReverseResolution(entity) => entity.as_slice().to_vec(),
        EntityWrapper::ConfigCellSubAccount(entity) => entity.as_slice().to_vec(),
        _ => unreachable!(),
    };
    data.append(&mut entity_bytes);

    Bytes::new_builder()
        .set(data.into_iter().map(Byte::new).collect())
        .build()
}

pub fn wrap_data_entity_v3(version: u32, index: usize, entity: EntityWrapper) -> DataEntity {
    fn wrap_data_entity(version: u32, index: usize, entity: impl Entity) -> DataEntity {
        DataEntity::new_builder()
            .version(Uint32::from(version))
            .index(Uint32::from(index as u32))
            .entity(Bytes::from(entity.as_slice()))
            .build()
    }

    match entity {
        EntityWrapper::PreAccountCellData(entity) => wrap_data_entity(version, index, entity),
        EntityWrapper::ProposalCellData(entity) => wrap_data_entity(version, index, entity),
        EntityWrapper::AccountCellData(entity) => wrap_data_entity(version, index, entity),
        EntityWrapper::AccountCellDataV2(entity) => wrap_data_entity(version, index, entity),
        EntityWrapper::AccountSaleCellData(entity) => wrap_data_entity(version, index, entity),
        EntityWrapper::AccountSaleCellDataV1(entity) => wrap_data_entity(version, index, entity),
        EntityWrapper::AccountAuctionCellData(entity) => wrap_data_entity(version, index, entity),
        EntityWrapper::IncomeCellData(entity) => wrap_data_entity(version, index, entity),
        EntityWrapper::OfferCellData(entity) => wrap_data_entity(version, index, entity),
        EntityWrapper::SubAccount(entity) => wrap_data_entity(version, index, entity),
        EntityWrapper::ConfigCellAccount(entity) => wrap_data_entity(version, index, entity),
        EntityWrapper::ConfigCellApply(entity) => wrap_data_entity(version, index, entity),
        EntityWrapper::ConfigCellIncome(entity) => wrap_data_entity(version, index, entity),
        EntityWrapper::ConfigCellMain(entity) => wrap_data_entity(version, index, entity),
        EntityWrapper::ConfigCellPrice(entity) => wrap_data_entity(version, index, entity),
        EntityWrapper::ConfigCellProposal(entity) => wrap_data_entity(version, index, entity),
        EntityWrapper::ConfigCellProfitRate(entity) => wrap_data_entity(version, index, entity),
        EntityWrapper::ConfigCellRelease(entity) => wrap_data_entity(version, index, entity),
        EntityWrapper::ConfigCellSecondaryMarket(entity) => wrap_data_entity(version, index, entity),
        EntityWrapper::ConfigCellReverseResolution(entity) => wrap_data_entity(version, index, entity),
        EntityWrapper::ConfigCellSubAccount(entity) => wrap_data_entity(version, index, entity),
        _ => unreachable!(),
    }
}

pub fn wrap_data_entity_opt_v3(version: u32, index: usize, entity: EntityWrapper) -> DataEntityOpt {
    DataEntityOpt::new_builder()
        .set(Some(wrap_data_entity_v3(version, index, entity)))
        .build()
}

pub fn wrap_data_witness_v3(
    data_type: DataType,
    version: u32,
    index: usize,
    entity: EntityWrapper,
    source: Source,
) -> Bytes {
    let data = match source {
        Source::CellDep => {
            let data_entity = wrap_data_entity_opt_v3(version, index, entity);
            Data::new_builder().dep(data_entity).build()
        }
        Source::Input => {
            let data_entity = wrap_data_entity_opt_v3(version, index, entity);
            Data::new_builder().old(data_entity).build()
        }
        Source::Output => {
            let data_entity = wrap_data_entity_opt_v3(version, index, entity);
            Data::new_builder().new(data_entity).build()
        }
        _ => panic!("Only CellDep, Input and Output is supported."),
    };

    wrap_entity_witness(data_type, data)
}

pub fn wrap_sub_account_witness(mut sub_account_data: Vec<u8>) -> Vec<u8> {
    let mut data = Vec::new();
    let mut data_type_bytes = (DataType::SubAccount as u32).to_le_bytes().to_vec();
    data.append(&mut WITNESS_HEADER.to_vec());
    data.append(&mut data_type_bytes);
    data.append(&mut sub_account_data);

    data
}
