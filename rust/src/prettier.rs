use std::prelude::v1::*;
use super::schemas::packed::*;

pub trait Prettier {
    fn as_prettier(&self) -> String;
}

impl Prettier for Uint8 {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for Uint8Reader<'a> {
    fn as_prettier(&self) -> String {
        use core::convert::TryInto;
        let number = self.raw_data().try_into().map(u8::from_le_bytes).expect("Decoding Uint8 failed.");
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
        let number = self.raw_data().try_into().map(u32::from_le_bytes).expect("Decoding Uint32 failed.");
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
        let number = self.raw_data().try_into().map(u64::from_le_bytes).expect("Decoding Uint64 failed.");
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
        let number = self.raw_data().try_into().map(u64::from_le_bytes).expect("Decoding Uint64 failed.");
        format!("Uint64({})", number)
    }
}

impl Prettier for Script {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for ScriptReader<'a> {
    fn as_prettier(&self) -> String {

        String::from("Script")
            + " { "
            + &format!("{}: {}, ", "code_hash", self.code_hash())
            + &format!("{}: {}, ", "hash_type", self.hash_type())
            + &format!("{}: {}, ", "args", self.args())
            + "}"
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

impl Prettier for PriceConfig {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for PriceConfigReader<'a> {
    fn as_prettier(&self) -> String {

        String::from("PriceConfig")
            + " { "
            + &format!("{}: {}, ", "length", self.length().as_prettier())
            + &format!("{}: {}, ", "new", self.new().as_prettier())
            + &format!("{}: {}, ", "renew", self.renew().as_prettier())
            + "}"
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

        format!("AccountChars({})", String::from_utf8(ret).expect("AccountChars should contains only utf8 chars."))
    }
}

impl Prettier for PreAccountCellData {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for PreAccountCellDataReader<'a> {
    fn as_prettier(&self) -> String {
        String::from("PreAccountCellData")
            + " { "
            + &format!("{}: {}, ", "account", self.account().as_prettier())
            + &format!("{}: {}, ", "refund_lock", self.refund_lock().as_prettier())
            + &format!("{}: {}, ", "owner_lock_args", self.owner_lock_args())
            + &format!("{}: {}, ", "inviter_id", self.inviter_id())
            + &format!("{}: {}, ", "inviter_lock", self.inviter_lock().as_prettier())
            + &format!("{}: {}, ", "channel_lock", self.channel_lock().as_prettier())
            + &format!("{}: {}, ", "price", self.price().as_prettier())
            + &format!("{}: {}, ", "quote", self.quote().as_prettier())
            + &format!("{}: {}, ", "invited_discount", self.invited_discount().as_prettier())
            + &format!("{}: {}, ", "created_at", self.created_at().as_prettier())
            + "}"

    }
}

impl Prettier for Records {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for RecordsReader<'a> {
    fn as_prettier(&self) -> String {
        let mut ret = String::from("Records") + " [ ";

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

        String::from("Record")
            + " { "
            + &format!("{}: {}, ", "record_type", self.record_type())
            + &format!("{}: {}, ", "record_key", self.record_key())
            + &format!("{}: {}, ", "record_label", self.record_label())
            + &format!("{}: {}, ", "record_value", self.record_value())
            + &format!("{}: {}, ", "record_ttl", self.record_ttl().as_prettier())
            + "}"
    }
}

impl Prettier for AccountCellData {
    fn as_prettier(&self) -> String {
        self.as_reader().as_prettier()
    }
}

impl<'a> Prettier for AccountCellDataReader<'a> {
    fn as_prettier(&self) -> String {
        String::from("AccountCellData")
            + " { "
            + &format!("{}: {}, ", "id", self.id().as_prettier())
            + &format!("{}: {}, ", "account", self.account().as_prettier())
            + &format!("{}: {}, ", "registered_at", self.registered_at().as_prettier())
            + &format!("{}: {}, ", "last_transfer_account_at", self.last_transfer_account_at().as_prettier())
            + &format!("{}: {}, ", "last_edit_manager_at", self.last_edit_manager_at().as_prettier())
            + &format!("{}: {}, ", "last_edit_records_at", self.last_edit_records_at().as_prettier())
            + &format!("{}: {}, ", "status", self.status().as_prettier())
            + &format!("{}: {}, ", "records", self.records().as_prettier())
            + "}"
    }
}
