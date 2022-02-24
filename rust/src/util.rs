use super::constants::*;
use core::convert::TryFrom;
use molecule::prelude::*;

pub use molecule::hex_string;

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
