extern crate bincode;
extern crate cita_types;
extern crate clap;
extern crate common_types as types;
extern crate db as cita_db;
extern crate libproto;
extern crate log;
extern crate proof;
extern crate rlp;

use cita_db::{DBTransaction, Database, DatabaseConfig};
use cita_types::H256;
use clap::App;
/*use libproto::blockchain::{
    AccountGasLimit as ProtoAccountGasLimit, Proof as ProtoProof, ProofType,
};
use rlp::{decode, encode, Decodable, Encodable};
*/
use std::path::Path;
use types::db::{Readable, Writable};
use types::header::*;
use types::{db, extras, BlockNumber};

use bincode::{serialize, Infinite};
use proof::BftProof;
use std::fs::{read_dir, remove_file, OpenOptions};
use std::io::{self, Read, Seek, Write};
use std::mem::transmute;
use std::time::Duration;
//use types::db::Key;

const PTYPE: u8 = 5;
const HTYPE: u8 = 4;
const WAL: &str = "/wal";
const NOSQL: &str = "/nosql";
const STATEDB: &str = "/statedb";


fn write_transaction(height : u64) -> DBTransaction{
    let hash = H256::new();
    hash.randomize();
    let mut batch = DBTransaction::new();
    batch.write(db::COL_EXTRA, &extras::CurrentHash, &hash);

    let mut hdr = Header::default();
    hdr.set_number(height);
    batch.write(db::COL_HEADERS, &height, &hdr);

    let proof = hdr.proof();
    batch.write(db::COL_EXTRA, &extras::CurrentProof, &proof);
    batch
}

fn main() {
    let matches = App::new("cita-recover")
        //.version(get_build_info_str(true))
        .author("yubo")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage("-p, --previos=[NUMBER] 'Sets the previous height for insert'")
        .args_from_usage("-t, --time=[NUMBER] 'delay insert in ms' ")
        .args_from_usage("-d, --data=[PATH] 'Set data dir'")
        .get_matches();

    let hi = matches.value_of("previos")
        .unwrap_or("0")
        .to_string()
        .parse::<u64>()
        .unwrap_or(0);
    let delay = matches
        .value_of("time")
        .unwrap_or("3000")
        .to_string()
        .parse::<u64>()
        .unwrap_or(3000);
    let data_path = matches.value_of("data").unwrap_or("./data");

    let database_config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
    let chain_db = Database::open(&database_config, &*data_path).expect("DB dir not right");

    for i in 0..hi {
        let batch = write_transaction(i as u64);
        chain_db.write(batch).unwrap();
    }

    let mut hi = hi as u64;
    loop {
        hi += 1;
        let batch = write_transaction(hi);
        chain_db.write(batch).unwrap();
        ::std::thread::sleep(Duration::from_millis(delay));
    }







}
