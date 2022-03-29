use super::schemas::packed::*;

macro_rules! print_fields {
    ($self:expr, $struct_name:expr, {$( $tt:tt ),+}) => {
        String::from($struct_name) + "{"
        $(+ &print_fields!(@field $self, $tt) + ", ")+
        + "}"
    };
    (@field $self:expr, $field:ident) => {
        String::from(stringify!($field)) + ": " + &$self.$field().as_prettier()
    };
    (@field $self:expr, ($field:ident -> $value:expr)) => {
        String::from(stringify!($field)) + ": " + $value
    }
}

pub trait Prettier {
    fn as_prettier(&self) -> String;
}

/// For compatible with returning a string instead of any type.
impl Prettier for String {
    fn as_prettier(&self) -> String {
        self.clone()
    }
}

impl Prettier for Uint8 {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for Uint8Reader<'a> {
    fn as_prettier(&self) -> String {
        use core::convert::TryInto;
        let number = self
            .raw_data()
            .try_into()
            .map(u8::from_le_bytes)
            .expect("Decoding Uint8 failed.");
        format!("Uint8({})", number)
    }
}

impl Prettier for Uint32 {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for Uint32Reader<'a> {
    fn as_prettier(&self) -> String {
        use core::convert::TryInto;
        let number = self
            .raw_data()
            .try_into()
            .map(u32::from_le_bytes)
            .expect("Decoding Uint32 failed.");
        format!("Uint32({})", number)
    }
}

impl Prettier for Uint64 {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for Uint64Reader<'a> {
    fn as_prettier(&self) -> String {
        use core::convert::TryInto;
        let number = self
            .raw_data()
            .try_into()
            .map(u64::from_le_bytes)
            .expect("Decoding Uint64 failed.");
        format!("Uint64({})", number)
    }
}

impl Prettier for Timestamp {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for TimestampReader<'a> {
    fn as_prettier(&self) -> String {
        use core::convert::TryInto;
        let number = self
            .raw_data()
            .try_into()
            .map(u64::from_le_bytes)
            .expect("Decoding Uint64 failed.");
        format!("Uint64({})", number)
    }
}

impl Prettier for Byte {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for ByteReader<'a> {
    fn as_prettier(&self) -> String {
        use molecule::hex_string;
        let raw_data = hex_string(&self.as_slice());

        format!("Bytes(0x{})", raw_data)
    }
}

impl Prettier for Bytes {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for BytesReader<'a> {
    fn as_prettier(&self) -> String {
        use molecule::hex_string;
        let raw_data = hex_string(&self.raw_data());

        format!("Bytes(0x{})", raw_data)
    }
}

impl Prettier for Hash {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for HashReader<'a> {
    fn as_prettier(&self) -> String {
        use molecule::hex_string;
        let raw_data = hex_string(&self.raw_data());

        format!("Bytes(0x{})", raw_data)
    }
}

impl Prettier for Script {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for ScriptReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "Script", {
            code_hash,
            hash_type,
            args
        })
    }
}

impl Prettier for ScriptOpt {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for ScriptOptReader<'a> {
    fn as_prettier(&self) -> String {
        if let Some(v) = self.to_opt() {
            format!("ScriptOpt(Some({}))", v.as_prettier())
        } else {
            String::from("ScriptOpt(None)")
        }
    }
}

impl Prettier for OutPoint {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for OutPointReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "OutPoint", {
            tx_hash,
            index
        })
    }
}

impl Prettier for AccountId {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for AccountIdReader<'a> {
    fn as_prettier(&self) -> String {
        use molecule::hex_string;
        let raw_data = hex_string(&self.raw_data());

        format!("AccountId(0x{})", raw_data)
    }
}

impl Prettier for AccountChars {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for AccountCharsReader<'a> {
    fn as_prettier(&self) -> String {
        let mut ret = Vec::new();
        for reader in self.iter() {
            ret.append(&mut reader.bytes().raw_data().to_owned());
        }

        format!(
            "AccountChars({})",
            String::from_utf8(ret).expect("AccountChars should contains only utf8 chars.")
        )
    }
}

impl Prettier for PreAccountCellData {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for PreAccountCellDataReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "PreAccountCellData", {
            account,
            refund_lock,
            owner_lock_args,
            inviter_id,
            inviter_lock,
            channel_lock,
            price,
            quote,
            invited_discount,
            created_at
        })
    }
}

impl Prettier for Records {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for RecordsReader<'a> {
    fn as_prettier(&self) -> String {
        let mut ret = String::from("[ ");

        for item in self.iter() {
            ret = ret + &item.as_prettier() + ", ";
        }

        ret + "]"
    }
}

impl Prettier for Record {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for RecordReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "Record", {
            record_type,
            record_key,
            record_label,
            record_value,
            record_ttl
        })
    }
}

impl Prettier for AccountCellDataV2 {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for AccountCellDataV2Reader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "AccountCellDataV2", {
            id,
            account,
            registered_at,
            last_transfer_account_at,
            last_edit_manager_at,
            last_edit_records_at,
            status,
            records
        })
    }
}

impl Prettier for AccountCellData {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for AccountCellDataReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "AccountCellData", {
            id,
            account,
            registered_at,
            last_transfer_account_at,
            last_edit_manager_at,
            last_edit_records_at,
            status,
            records,
            enable_sub_account,
            renew_sub_account_price
        })
    }
}

impl Prettier for AccountSaleCellDataV1 {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for AccountSaleCellDataV1Reader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "AccountSaleCellDataV1", {
            account_id,
            account,
            price,
            description,
            started_at
        })
    }
}

impl Prettier for AccountSaleCellData {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for AccountSaleCellDataReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "AccountSaleCellData", {
            account_id,
            account,
            price,
            description,
            started_at,
            buyer_inviter_profit_rate
        })
    }
}

impl Prettier for OfferCellData {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for OfferCellDataReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "AccountSaleCellData", {
            account,
            price,
            message,
            inviter_lock,
            channel_lock
        })
    }
}

impl Prettier for IncomeCellData {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for IncomeCellDataReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "IncomeCellData", {
            creator,
            records
        })
    }
}

impl Prettier for IncomeRecords {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for IncomeRecordsReader<'a> {
    fn as_prettier(&self) -> String {
        let mut ret = String::from("[ ");

        for item in self.iter() {
            ret = ret + &item.as_prettier() + ", ";
        }

        ret + "]"
    }
}

impl Prettier for IncomeRecord {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for IncomeRecordReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "IncomeRecord", {
            belong_to,
            capacity
        })
    }
}

impl Prettier for ProposalCellData {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for ProposalCellDataReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "ProposalCellData", {
            proposer_lock,
            created_at_height,
            slices
        })
    }
}

impl Prettier for SliceList {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for SliceListReader<'a> {
    fn as_prettier(&self) -> String {
        let mut ret = String::from("[ ");

        for item in self.iter() {
            ret = ret + &item.as_prettier() + ", ";
        }

        ret + "]"
    }
}

impl Prettier for SL {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for SLReader<'a> {
    fn as_prettier(&self) -> String {
        let mut ret = String::from("[ ");

        for item in self.iter() {
            ret = ret + &item.as_prettier() + ", ";
        }

        ret + "]"
    }
}

impl Prettier for ProposalItem {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for ProposalItemReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "ProposalItem", {
            account_id,
            item_type,
            next
        })
    }
}

impl Prettier for ActionData {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for ActionDataReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "ActionData", {
            action,
            params
        })
    }
}

impl Prettier for ConfigCellMain {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for ConfigCellMainReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "ConfigCellMain", {
            status,
            type_id_table,
            das_lock_out_point_table
        })
    }
}

impl Prettier for TypeIdTable {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for TypeIdTableReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "TypeIdTable", {
            account_cell,
            apply_register_cell,
            balance_cell,
            income_cell,
            pre_account_cell,
            proposal_cell,
            account_sale_cell,
            account_auction_cell,
            offer_cell,
            reverse_record_cell,
            sub_account_cell
        })
    }
}

impl Prettier for DasLockOutPointTable {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for DasLockOutPointTableReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "TypeIdTable", {
            ckb_signall,
            ckb_multisign,
            ckb_anyone_can_pay,
            eth,
            tron
        })
    }
}

impl Prettier for ConfigCellAccount {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for ConfigCellAccountReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "ConfigCellAccount", {
            max_length,
            basic_capacity,
            prepared_fee_capacity,
            expiration_grace_period,
            record_min_ttl,
            record_size_limit,
            transfer_account_fee,
            edit_manager_fee,
            edit_records_fee,
            common_fee,
            transfer_account_throttle,
            edit_manager_throttle,
            edit_records_throttle,
            common_throttle
        })
    }
}

impl Prettier for ConfigCellApply {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for ConfigCellApplyReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "ConfigCellApply", {
            apply_min_waiting_block_number,
            apply_max_waiting_block_number
        })
    }
}

impl Prettier for ConfigCellPrice {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for ConfigCellPriceReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "ConfigCellApply", {
            discount,
            prices
        })
    }
}

impl Prettier for DiscountConfig {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for DiscountConfigReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "DiscountConfig", { invited_discount })
    }
}

impl Prettier for PriceConfigList {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for PriceConfigListReader<'a> {
    fn as_prettier(&self) -> String {
        let mut ret = String::from("[ ");

        for item in self.iter() {
            ret = ret + &item.as_prettier() + ", ";
        }

        ret + "]"
    }
}

impl Prettier for PriceConfig {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for PriceConfigReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "DiscountConfig", {
            length,
            new,
            renew
        })
    }
}

impl Prettier for ConfigCellProposal {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for ConfigCellProposalReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "ConfigCellProposal", {
            proposal_min_confirm_interval,
            proposal_min_extend_interval,
            proposal_min_recycle_interval,
            proposal_max_account_affect,
            proposal_max_pre_account_contain
        })
    }
}

impl Prettier for ConfigCellProfitRate {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for ConfigCellProfitRateReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "ConfigCellProfitRate", {
            inviter,
            channel,
            proposal_create,
            proposal_confirm,
            income_consolidate,
            sale_buyer_inviter,
            sale_buyer_channel,
            sale_das,
            auction_bidder_inviter,
            auction_bidder_channel,
            auction_das,
            auction_prev_bidder
        })
    }
}

impl Prettier for ConfigCellIncome {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for ConfigCellIncomeReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "ConfigCellIncome", {
            basic_capacity,
            max_records,
            min_transfer_capacity
        })
    }
}

impl Prettier for ConfigCellSecondaryMarket {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for ConfigCellSecondaryMarketReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "ConfigCellSecondaryMarket", {
            common_fee,
            sale_min_price,
            sale_expiration_limit,
            sale_description_bytes_limit,
            sale_cell_basic_capacity,
            sale_cell_prepared_fee_capacity,
            auction_max_extendable_duration,
            auction_duration_increment_each_bid,
            auction_min_opening_price,
            auction_min_increment_rate_each_bid,
            auction_description_bytes_limit,
            auction_cell_basic_capacity,
            auction_cell_prepared_fee_capacity,
            offer_min_price,
            offer_cell_basic_capacity,
            offer_cell_prepared_fee_capacity,
            offer_message_bytes_limit
        })
    }
}

impl Prettier for ConfigCellReverseResolution {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for ConfigCellReverseResolutionReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "ConfigCellReverseResolution", {
            record_basic_capacity,
            record_prepared_fee_capacity,
            common_fee
        })
    }
}

impl Prettier for ConfigCellSubAccount {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for ConfigCellSubAccountReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "ConfigCellSubAccount", {
            basic_capacity,
            prepared_fee_capacity,
            new_sub_account_price,
            renew_sub_account_price,
            common_fee,
            create_fee,
            edit_fee,
            renew_fee,
            recycle_fee
        })
    }
}

impl Prettier for ConfigCellRelease {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for ConfigCellReleaseReader<'a> {
    fn as_prettier(&self) -> String {
        print_fields!(self, "ConfigCellRelease", {
            lucky_number
        })
    }
}

impl Prettier for SubAccount {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for SubAccountReader<'a> {
    fn as_prettier(&self) -> String {
        let fmt_suffix = String::from_utf8(self.suffix().raw_data().to_vec()).expect("Encoding utf-8 failed.");
        print_fields!(self, "SubAccount", {
            lock,
            id,
            account,
            (suffix -> &fmt_suffix),
            registered_at,
            expired_at,
            status,
            records,
            nonce,
            enable_sub_account,
            renew_sub_account_price
        })
    }
}
