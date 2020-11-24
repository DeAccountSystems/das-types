use das_types::basic;
use das_types::convert;
use molecule::prelude::Entity;

#[test]
fn should_support_u8() {
    let num: u8 = 255;
    let data = Uint8::from(num);
    println!("{:?}", data);

    assert_eq!(true, false);
}
