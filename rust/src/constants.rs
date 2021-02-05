#[derive(Debug, PartialEq, Copy, Clone)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum CharSetType {
    Emoji,
    En,
    ZhCn,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum ProposalSliceItemType {
    Exist,
    Proposed,
    New,
}

// [100, 97, 115] equals "das".as_bytes()
pub const WITNESS_HEADER: [u8; 3] = [100, 97, 115];
