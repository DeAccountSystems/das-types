use super::constants::*;
use super::schemas::packed::*;
use core::convert::TryFrom;
use molecule::prelude::*;

pub use molecule::hex_string;

pub fn is_entity_eq<T: Entity>(a: &T, b: &T) -> bool {
    a.as_slice() == b.as_slice()
}

pub fn is_reader_eq<'a, T: Reader<'a>>(a: T, b: T) -> bool {
    a.as_slice() == b.as_slice()
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

pub fn wrap_data_witness<T: Entity>(
    data_type: DataType,
    output_opt: Option<(u32, u32, T)>,
    input_opt: Option<(u32, u32, T)>,
    dep_opt: Option<(u32, u32, T)>,
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

pub fn data_type_to_char_set(data_type: DataType) -> CharSetType {
    CharSetType::try_from(data_type as u32 - 100000).unwrap()
}

pub fn char_set_to_data_type(char_set: CharSetType) -> DataType {
    DataType::try_from(char_set as u32 + 100000).unwrap()
}
