use core::convert::TryFrom;

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum DataType {
    ActionData = 0,
    AccountCellData,
    OnSaleCellData,
    BiddingCellData,
    ProposalCellData,
    PreAccountCellData,
    IncomeCellData,
    ConfigCellAccount = 100,
    ConfigCellApply,
    ConfigCellCharSet,
    ConfigCellIncome,
    ConfigCellMain,
    ConfigCellPrice,
    ConfigCellProposal,
    ConfigCellProfitRate,
    ConfigCellRecordKeyNamespace,
    ConfigCellPreservedAccount00 = 150,
    // ConfigCellPreservedAccount01,
    // ConfigCellPreservedAccount02,
    // ConfigCellPreservedAccount03,
    // ConfigCellPreservedAccount04,
    // ConfigCellPreservedAccount05,
    // ConfigCellPreservedAccount06,
    // ConfigCellPreservedAccount07,
}

impl TryFrom<u32> for DataType {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == DataType::ActionData as u32 => Ok(DataType::ActionData),
            x if x == DataType::AccountCellData as u32 => Ok(DataType::AccountCellData),
            x if x == DataType::OnSaleCellData as u32 => Ok(DataType::OnSaleCellData),
            x if x == DataType::BiddingCellData as u32 => Ok(DataType::BiddingCellData),
            x if x == DataType::ProposalCellData as u32 => Ok(DataType::ProposalCellData),
            x if x == DataType::PreAccountCellData as u32 => Ok(DataType::PreAccountCellData),
            x if x == DataType::IncomeCellData as u32 => Ok(DataType::IncomeCellData),
            x if x == DataType::ConfigCellAccount as u32 => Ok(DataType::ConfigCellAccount),
            x if x == DataType::ConfigCellApply as u32 => Ok(DataType::ConfigCellApply),
            x if x == DataType::ConfigCellCharSet as u32 => Ok(DataType::ConfigCellCharSet),
            x if x == DataType::ConfigCellIncome as u32 => Ok(DataType::ConfigCellIncome),
            x if x == DataType::ConfigCellMain as u32 => Ok(DataType::ConfigCellMain),
            x if x == DataType::ConfigCellPrice as u32 => Ok(DataType::ConfigCellPrice),
            x if x == DataType::ConfigCellProposal as u32 => Ok(DataType::ConfigCellProposal),
            x if x == DataType::ConfigCellProfitRate as u32 => Ok(DataType::ConfigCellProfitRate),
            x if x == DataType::ConfigCellRecordKeyNamespace as u32 => {
                Ok(DataType::ConfigCellRecordKeyNamespace)
            }
            x if x == DataType::ConfigCellPreservedAccount00 as u32 => {
                Ok(DataType::ConfigCellPreservedAccount00)
            }
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum CharSetType {
    Emoji,
    Digit,
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

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum AccountStatus {
    Normal,
    Selling,
    Auction,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum LockRole {
    Owner,
    Manager,
}

// [100, 97, 115] equals "das".as_bytes()
pub const WITNESS_HEADER: [u8; 3] = [100, 97, 115];
