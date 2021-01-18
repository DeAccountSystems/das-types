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
