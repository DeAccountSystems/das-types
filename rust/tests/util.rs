use core::convert::TryFrom;
use das_types::{
    constants::{DataType, WITNESS_HEADER},
    packed::*,
    prelude::*,
    util,
};
use hex;
use std::convert::TryInto;

#[test]
fn test_is_entity_eq() {
    let a = Bytes::from("aaa".as_bytes());
    let b = Bytes::from("aaa".as_bytes());
    assert!(
        util::is_entity_eq(&a, &b),
        "Function is_entity_eq should return true if bytes are the same."
    );

    let a = Bytes::from("aaa".as_bytes());
    let b = Bytes::from("bbb".as_bytes());
    assert!(
        !util::is_entity_eq(&a, &b),
        "Function is_entity_eq should return false if bytes are not the same."
    );
}

#[test]
fn test_is_reader_eq() {
    let a = Bytes::from("aaa".as_bytes());
    let b = Bytes::from("aaa".as_bytes());
    assert!(
        util::is_reader_eq(a.as_reader(), b.as_reader()),
        "Function is_reader_eq should return true if bytes are the same."
    );

    let a = Bytes::from("aaa".as_bytes());
    let b = Bytes::from("bbb".as_bytes());
    assert!(
        !util::is_reader_eq(a.as_reader(), b.as_reader()),
        "Function is_reader_eq should return false if bytes are not the same."
    );
}

#[test]
fn test_wrap_as_data_entity() {
    let code_hash = Hash::try_from(
        hex::decode("e683b04139344768348499c23eb1326d5a52d6db006c0d2fece00a831f3660d7").unwrap(),
    )
    .unwrap();
    let raw = Script::new_builder()
        .code_hash(code_hash)
        .hash_type(Byte::new(0))
        .build();
    let data = util::wrap_data_entity(1, 0, raw.clone());

    assert!(util::is_entity_eq(&data.version(), &Uint32::from(1)));
    assert!(util::is_entity_eq(&data.index(), &Uint32::from(0)));
    assert!(util::is_entity_eq(
        &data.entity(),
        &Bytes::from(raw.as_slice())
    ));
}

#[test]
fn test_wrap_as_data_entity_opt() {
    let code_hash = Hash::try_from(
        hex::decode("e683b04139344768348499c23eb1326d5a52d6db006c0d2fece00a831f3660d7").unwrap(),
    )
    .unwrap();
    let raw = Script::new_builder()
        .code_hash(code_hash)
        .hash_type(Byte::new(0))
        .build();
    let data_opt = util::wrap_data_entity_opt(1, 0, raw.clone());

    assert!(data_opt.is_some());

    let data = data_opt.to_opt().unwrap();

    assert!(util::is_entity_eq(&data.version(), &Uint32::from(1)));
    assert!(util::is_entity_eq(&data.index(), &Uint32::from(0)));
    assert!(util::is_entity_eq(
        &data.entity(),
        &Bytes::from(raw.as_slice())
    ));
}

#[test]
fn test_wrap_action_witness() {
    let params = Bytes::from(&[1, 0, 1][..]);
    let witness = util::wrap_action_witness("config", Some(params));
    // eprintln!("witness = {:#?}", witness);

    let header = witness.as_slice().get(4..7).unwrap();
    assert_eq!(
        header, &WITNESS_HEADER,
        "The wrapped bytes should have DAS header."
    );

    let raw = witness.as_slice().get(7..11).unwrap();
    let data_type = u32::from(Uint32::new_unchecked(raw.to_vec().into()));
    assert_eq!(
        data_type,
        DataType::ActionData as u32,
        "The wrapped bytes should have DAS data type."
    );

    let raw = witness.as_slice().get(11..).unwrap();
    let action_data = ActionData::new_unchecked(raw.to_vec().into());
    assert!(util::is_reader_eq(
        action_data.as_reader().action(),
        Bytes::from("config".as_bytes()).as_reader()
    ));
    assert!(util::is_reader_eq(
        action_data.as_reader().params(),
        Bytes::from(&[1, 0, 1][..]).as_reader()
    ));
}

#[test]
fn test_wrap_raw_witness() {
    let raw_bytes = vec![1, 0, 0, 0, 0, 0, 0, 1];
    // println!("raw_bytes = {:?}", raw_bytes);

    let witness = util::wrap_raw_witness(DataType::ConfigCellRecordKeyNamespace, raw_bytes.clone());
    // println!("witness = {:#?}", witness);

    let header = witness.as_slice().get(4..7).unwrap();
    assert_eq!(
        header, &WITNESS_HEADER,
        "The wrapped bytes should have DAS header."
    );

    let raw = witness.as_slice().get(7..11).unwrap();
    let data_type = u32::from_le_bytes(raw.try_into().unwrap());
    assert_eq!(
        data_type,
        DataType::ConfigCellRecordKeyNamespace as u32,
        "The wrapped bytes should be DataType::ConfigCellRecordKeyNamespace ."
    );

    let raw = witness.as_slice().get(11..).unwrap();
    assert!(raw == raw_bytes, "The wrapped bytes should be raw bytes.")
}

#[test]
fn test_wrap_entity_witness() {
    let entity = ConfigCellMain::default();
    // println!("entity = {:#?}", entity);

    let witness = util::wrap_entity_witness(DataType::ConfigCellMain, entity);
    // println!("witness = {:#?}", witness);

    let header = witness.as_slice().get(4..7).unwrap();
    assert_eq!(
        header, &WITNESS_HEADER,
        "The wrapped bytes should have DAS header."
    );

    let raw = witness.as_slice().get(7..11).unwrap();
    let data_type = u32::from_le_bytes(raw.try_into().unwrap());
    assert_eq!(
        data_type,
        DataType::ConfigCellMain as u32,
        "The wrapped bytes should be DataType::ConfigCellMain ."
    );

    let raw = witness.as_slice().get(11..).unwrap();
    let ret = ConfigCellMain::from_slice(raw);
    assert!(ret.is_ok(), "The wrapped bytes should be an entity.")
}

#[test]
fn test_wrap_data_witness() {
    let entity = AccountCellData::default();
    // println!("entity = {:#?}", entity);

    let witness = util::wrap_data_witness(
        DataType::AccountCellData,
        Some((1, 0, entity.clone())),
        None,
        None,
    );
    // println!("witness = {:#?}", witness);

    let header = witness.as_slice().get(4..7).unwrap();
    assert_eq!(
        header, &WITNESS_HEADER,
        "The wrapped bytes should have DAS header."
    );

    let raw = witness.as_slice().get(7..11).unwrap();
    let data_type = u32::from_le_bytes(raw.try_into().unwrap());
    assert_eq!(
        data_type,
        DataType::AccountCellData as u32,
        "The wrapped bytes should have DAS data type."
    );

    let raw = witness.as_slice().get(11..).unwrap();
    let ret = Data::from_slice(raw);
    assert!(
        ret.is_ok(),
        "The wrapped bytes should have original entity data."
    );
}
