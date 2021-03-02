use core::convert::TryFrom;
use molecule::error::VerificationError;

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum DataType {
    ActionData,
    ConfigCellData, // Deprecated
    AccountCellData,
    OnSaleCellData,
    BiddingCellData,
    ProposalCellData,
    PreAccountCellData,
    ConfigCellMain = 7,
    ConfigCellRegister,
    ConfigCellBloomFilter,
    ConfigCellMarket,
}

impl TryFrom<u32> for DataType {
    type Error = VerificationError;

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == DataType::ActionData as u32 => Ok(DataType::ActionData),
            x if x == DataType::AccountCellData as u32 => Ok(DataType::AccountCellData),
            x if x == DataType::OnSaleCellData as u32 => Ok(DataType::OnSaleCellData),
            x if x == DataType::BiddingCellData as u32 => Ok(DataType::BiddingCellData),
            x if x == DataType::ProposalCellData as u32 => Ok(DataType::ProposalCellData),
            x if x == DataType::PreAccountCellData as u32 => Ok(DataType::PreAccountCellData),
            x if x == DataType::ConfigCellMain as u32 => Ok(DataType::ConfigCellMain),
            x if x == DataType::ConfigCellRegister as u32 => Ok(DataType::ConfigCellRegister),
            x if x == DataType::ConfigCellBloomFilter as u32 => Ok(DataType::ConfigCellBloomFilter),
            x if x == DataType::ConfigCellMarket as u32 => Ok(DataType::ConfigCellMarket),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum ConfigID {
    ConfigCellMain,
    ConfigCellRegister,
    ConfigCellBloomFilter,
    ConfigCellMarket,
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

// [100, 97, 115] equals "das".as_bytes()
pub const WITNESS_HEADER: [u8; 3] = [100, 97, 115];
