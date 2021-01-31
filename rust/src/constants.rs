#[repr(u32)]
pub enum DataType {
    ActionData,
    ConfigCellData,
    AccountCellData,
    OnSaleCellData,
    BiddingCellData,
    ProposalCellData,
    PreAccountCellData,
}

#[repr(u32)]
pub enum CharSet {
    Emoji,
    En,
    ZhCn,
}

// [100, 97, 115] equals "das".as_bytes()
pub const WITNESS_HEADER: [u8; 3] = [100, 97, 115];
