use ckb_hash::blake2b_256;
use ckb_std::ckb_types::prelude::Entity;
use clap::Clap;
use das_types::{constants::*, packed::*, prelude::*, VerificationError};
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt::Display;
use std::process;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Link Xie. <xieaolin@gmail.com>")]
struct Options {
    #[clap(short = 'd', long = "data", about = "Hex data to virtualize.")]
    data: Option<String>,
    #[clap(short = 't', long = "data-type", about = "A file path to the configuration file.")]
    data_type: Option<String>,
    #[clap(short = 'w', long = "witness", about = "Hex witness data to virtualize.")]
    witness: Option<String>,
}

fn main() {
    // Parse options
    let options: Options = Options::parse();
    // println!("{:?}", options);

    if options.data.is_some() {
        if options.data_type.is_none() {
            eprintln!("When --data option is provided, --data-type is required.");
            process::exit(1);
        }

        match hex_to_bytes(&options.data.unwrap()) {
            Ok(bytes) => {
                if let Err(e) = virtualize_data(&options.data_type.unwrap(), bytes.as_slice()) {
                    println!("Parse data to actual data type failed: {}", e.to_string());
                    process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Decode hex witness data failed: {}", e.to_string());
                process::exit(1);
            }
        }
    } else if options.witness.is_some() {
        match hex_to_bytes(&options.witness.unwrap()) {
            Ok(bytes) => {
                let raw_data_type = bytes.get(3..7).unwrap();
                let data_type = DataType::try_from(u32::from_le_bytes(raw_data_type.try_into().unwrap())).unwrap();
                let raw = bytes.get(7..).unwrap();

                println!("expected_data_type: {:?}", data_type);
                if let Err(e) = virtualize_witness(data_type, raw) {
                    println!("Parse witness to actual data type failed: {}", e.to_string());
                    process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Decode hex witness data failed: {}", e.to_string());
                process::exit(1);
            }
        }
    } else {
        eprintln!("One of --data or --witness must be provided.");
        process::exit(1);
    }
}

pub fn hex_to_bytes(input: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let hex = input.trim_start_matches("0x");
    if hex == "" {
        Ok(Vec::new())
    } else {
        Ok(hex::decode(hex)?)
    }
}

pub fn virtualize_witness(data_type: DataType, raw: &[u8]) -> Result<(), Box<dyn Error>> {
    match data_type as u32 {
        data_type_in_int if data_type_in_int == 0 || (data_type_in_int >= 100 && data_type_in_int < 1000) => {
            let (hash, entity) = virtualize_entity(data_type, 1, raw)?;
            println!("hash: 0x{}\nentity: {}", hex::encode(hash), entity);
        }
        data_type_in_int if data_type_in_int >= 10000 && data_type_in_int <= 20000 => {
            let hash = blake2b_256(raw);
            let length = u32::from_le_bytes(raw.get(..4).unwrap().try_into().unwrap());
            println!("hash: 0x{}\nlength: {}", hex::encode(hash), length);
        }
        data_type_in_int if data_type_in_int >= 100000 && data_type_in_int <= 110000 => {
            let hash = blake2b_256(raw);
            let length = u32::from_le_bytes(raw.get(..4).unwrap().try_into().unwrap());
            println!("hash: 0x{}\nlength: {}", hex::encode(hash), length);
        }
        _ => {
            let data = Data::from_slice(raw).map_err(error_to_string)?;
            println!("witness: {{");
            if let Some(dep_data_entity) = data.dep().to_opt() {
                let (hash, entity) = virtualize_entity(
                    data_type,
                    u32::from(dep_data_entity.version()),
                    dep_data_entity.entity().as_reader().raw_data(),
                )?;
                println!(
                    "  dep: {{\n    version: {}\n    index: {}\n    entity(0x{}): {} \n  }}",
                    dep_data_entity.version(),
                    dep_data_entity.index(),
                    hex::encode(hash),
                    entity,
                );
            }
            if let Some(old_data_entity) = data.old().to_opt() {
                let (hash, entity) = virtualize_entity(
                    data_type,
                    u32::from(old_data_entity.version()),
                    old_data_entity.entity().as_reader().raw_data(),
                )?;
                println!(
                    "  old: {{\n    version: {}\n    index: {}\n    entity(0x{}): {} \n  }}",
                    old_data_entity.version(),
                    old_data_entity.index(),
                    hex::encode(hash),
                    entity
                );
            }
            if let Some(new_data_entity) = data.new().to_opt() {
                let (hash, entity) = virtualize_entity(
                    data_type,
                    u32::from(new_data_entity.version()),
                    new_data_entity.entity().as_reader().raw_data(),
                )?;
                println!(
                    "  new: {{\n    version: {}\n    index: {}\n    entity(0x{}): {} \n  }}",
                    new_data_entity.version(),
                    new_data_entity.index(),
                    hex::encode(hash),
                    entity
                );
            }
            println!("}}");
        }
    }

    Ok(())
}

pub fn virtualize_entity(
    data_type: DataType,
    version: u32,
    raw: &[u8],
) -> Result<([u8; 32], Box<dyn Display>), Box<dyn Error>> {
    let entity: Box<dyn Display>;

    match data_type {
        DataType::ActionData => {
            entity = Box::new(ActionData::from_slice(raw).map_err(error_to_string)?);
        }
        DataType::AccountCellData => {
            if version == 1 {
                entity = Box::new(AccountCellDataV1::from_slice(raw).map_err(error_to_string)?);
            } else {
                entity = Box::new(AccountCellData::from_slice(raw).map_err(error_to_string)?);
            }
        }
        DataType::OnSaleCellData => {
            entity = Box::new(OnSaleCellData::from_slice(raw).map_err(error_to_string)?);
        }
        DataType::BiddingCellData => {
            entity = Box::new(BiddingCellData::from_slice(raw).map_err(error_to_string)?);
        }
        DataType::ProposalCellData => {
            entity = Box::new(ProposalCellData::from_slice(raw).map_err(error_to_string)?);
        }
        DataType::PreAccountCellData => {
            entity = Box::new(PreAccountCellData::from_slice(raw).map_err(error_to_string)?);
        }
        DataType::IncomeCellData => {
            entity = Box::new(IncomeCellData::from_slice(raw).map_err(error_to_string)?);
        }
        DataType::ConfigCellAccount => {
            entity = Box::new(ConfigCellAccount::from_slice(raw).map_err(error_to_string)?);
        }
        DataType::ConfigCellApply => {
            entity = Box::new(ConfigCellApply::from_slice(raw).map_err(error_to_string)?);
        }
        DataType::ConfigCellIncome => {
            entity = Box::new(ConfigCellIncome::from_slice(raw).map_err(error_to_string)?);
        }
        DataType::ConfigCellMain => {
            entity = Box::new(ConfigCellMain::from_slice(raw).map_err(error_to_string)?);
        }
        DataType::ConfigCellProposal => {
            entity = Box::new(ConfigCellProposal::from_slice(raw).map_err(error_to_string)?);
        }
        DataType::ConfigCellProfitRate => {
            entity = Box::new(ConfigCellProfitRate::from_slice(raw).map_err(error_to_string)?);
        }
        DataType::ConfigCellRelease => {
            entity = Box::new(ConfigCellRelease::from_slice(raw).map_err(error_to_string)?);
        }
        DataType::ConfigCellPrice => {
            entity = Box::new(ConfigCellPrice::from_slice(raw).map_err(error_to_string)?);
        }
        _ => return Err(format!("unsupported DataType for virtualization: {:?}", data_type).into()),
    }

    Ok((blake2b_256(raw), entity))
}

pub fn virtualize_data(data_type: &str, raw: &[u8]) -> Result<(), Box<dyn Error>> {
    let data: Box<dyn Display>;

    match data_type {
        "Uint8" => data = Box::new(u8::from(Uint8::from_slice(raw).map_err(error_to_string)?)),
        "Uint32" => data = Box::new(u32::from(Uint32::from_slice(raw).map_err(error_to_string)?)),
        "Uint64" => data = Box::new(u64::from(Uint64::from_slice(raw).map_err(error_to_string)?)),
        "Script" => data = Box::new(Script::from_slice(raw).map_err(error_to_string)?),
        "OutPoint" => data = Box::new(OutPoint::from_slice(raw).map_err(error_to_string)?),
        _ => return Err(format!("unsupported DataType for virtualization: {}", data_type).into()),
    }

    println!("{}: {}", data_type, data);

    Ok(())
}

fn error_to_string(e: VerificationError) -> String {
    e.to_string()
}
