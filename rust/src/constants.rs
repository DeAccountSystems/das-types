use core::convert::TryFrom;

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
    ConfigCellRecord,
    ConfigCellMarket,
    ConfigCellBloomFilter,
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
            x if x == DataType::ConfigCellMain as u32 => Ok(DataType::ConfigCellMain),
            x if x == DataType::ConfigCellRegister as u32 => Ok(DataType::ConfigCellRegister),
            x if x == DataType::ConfigCellRecord as u32 => Ok(DataType::ConfigCellRecord),
            x if x == DataType::ConfigCellMarket as u32 => Ok(DataType::ConfigCellMarket),
            x if x == DataType::ConfigCellBloomFilter as u32 => Ok(DataType::ConfigCellBloomFilter),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum ConfigID {
    ConfigCellMain,
    ConfigCellRegister,
    ConfigCellRecord,
    ConfigCellMarket,
    ConfigCellBloomFilter = 10,
}

impl TryFrom<u32> for ConfigID {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == ConfigID::ConfigCellMain as u32 => Ok(ConfigID::ConfigCellMain),
            x if x == ConfigID::ConfigCellRegister as u32 => Ok(ConfigID::ConfigCellRegister),
            x if x == ConfigID::ConfigCellRecord as u32 => Ok(ConfigID::ConfigCellRecord),
            x if x == ConfigID::ConfigCellMarket as u32 => Ok(ConfigID::ConfigCellMarket),
            x if x == ConfigID::ConfigCellBloomFilter as u32 => Ok(ConfigID::ConfigCellBloomFilter),
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
