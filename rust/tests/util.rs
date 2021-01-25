use core::convert::TryFrom;
use das_types::util::is_entity_eq;
use das_types::{
    constants::{DataType, WITNESS_HEADER},
    packed::*,
    prelude::*,
    util,
};
use hex;

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
fn test_wrap_witness() {
    let witness = util::wrap_witness(DataType::ActionData, Bytes::default());

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
    let data = Bytes::new_unchecked(raw.to_vec().into());
    assert!(is_entity_eq(&data, &Bytes::default()))
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
fn test_wrap_data_witness() {
    let config_cell_data = ConfigCellData::new_builder()
        .reserved_account_filter(Bytes::default())
        .proposal_min_confirm_require(Uint8::from(4u8))
        .proposal_min_extend_interval(Uint8::from(2u8))
        .proposal_max_account_affect(Uint32::from(50))
        .proposal_max_pre_account_contain(Uint32::from(50))
        .apply_min_waiting_time(Uint32::from(60))
        .apply_max_waiting_time(Uint32::from(86400))
        .account_max_length(Uint32::from(1000))
        // .price_configs()
        // .char_sets()
        .min_ttl(Uint32::from(300))
        .closing_limit_of_primary_market_auction(Uint32::from(86400))
        .closing_limit_of_secondary_market_auction(Uint32::from(86400))
        .build();
    // eprintln!("config_cell_data = {:#?}", config_cell_data);

    let witness = util::wrap_data_witness(
        DataType::ConfigCellData,
        Some((1, 0, config_cell_data.clone())),
        None,
        None,
    );
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
        DataType::ConfigCellData as u32,
        "The wrapped bytes should have DAS data type."
    );

    let raw = witness.as_slice().get(11..).unwrap();
    let data = Data::new_unchecked(raw.to_vec().into());
    let result = ConfigCellData::new_unchecked(
        data.new()
            .to_opt()
            .unwrap()
            .entity()
            .as_reader()
            .raw_data()
            .to_vec()
            .into(),
    );
    assert!(
        util::is_entity_eq(&result, &config_cell_data),
        "The wrapped bytes should have original entity data."
    );
}
