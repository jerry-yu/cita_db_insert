extern crate bincode;
extern crate cita_types;
extern crate clap;
extern crate common_types as types;
extern crate kvdb_rocksdb as cita_db;
extern crate libproto;
extern crate log;
extern crate proof;
extern crate rlp;

use cita_db::{Database, DatabaseConfig};
use cita_types::H256;
use clap::App;
//use types::db::{Readable, Writable};
//use types::header::*;
use std::time::Duration;
use types::db;
//use types::db::Key;

fn write_transaction(db: &Database, height: u64) {
    let ch = H256::from("7cabfb7709b29c16d9e876e876c9988d03f9c3414e1d3ff77ec1de2d0ee59f66");
    let cp = H256::from("7cabfb7709b29c16d9e876e876c9988d03f9c3414e1d3ff77ec1de2d0ee59f67");

    let ch = ch.to_vec();;
    let cp = cp.to_vec();

    let mut batch = db.transaction();
    let mut hash = H256::zero();
    hash.randomize();
    batch.put_vec(db::COL_EXTRA, &ch.as_slice(), hash.to_vec());

    /*let mut hdr = Header::default();
    hdr.set_number(height);*/
    let tmp = (height % 256) as u8;
    let tmps = vec![tmp; 1024];
    batch.put_vec(db::COL_HEADERS, &height.to_be_bytes(), tmps);

    batch.put(db::COL_BODIES, &height.to_be_bytes(), &[0xc8]);

    //let proof = hdr.proof();
    //batch.put(db::COL_EXTRA, &cp.as_slice(), proof.get_content());
    let proof = vec![1; 100];
    batch.put_vec(db::COL_EXTRA, &cp.as_slice(), proof);
    db.write(batch).unwrap();
}

fn main() {
    let matches = App::new("cita-recover")
        //.version(get_build_info_str(true))
        .author("yubo")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage("-p, --previos=[NUMBER] 'Sets the previous height for insert'")
        .args_from_usage("-t, --time=[NUMBER] 'delay insert in ms' ")
        .args_from_usage("-d, --data=[PATH] 'Set data dir'")
        .args_from_usage("-g, --target=[NUMBER] 'Set rocksdb memory_budget'")
        .get_matches();

    let hi = matches
        .value_of("previos")
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
    let mem_limit = matches
        .value_of("target")
        .unwrap_or("1024")
        .to_string()
        .parse::<usize>()
        .unwrap_or(1024);

    let data_path = matches.value_of("data").unwrap_or("./data");

    let mut database_config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
    database_config.memory_budget = Some(mem_limit);
    let chain_db = Database::open(&database_config, &*data_path).expect("DB dir not right");

    for i in 0..hi {
        if i % 100000 == 0 {
            println!("speed mode,now height {:?}", i);
        }
        write_transaction(&chain_db, i as u64);
    }

    let mut hi = hi as u64;
    loop {
        hi += 1;
        write_transaction(&chain_db, hi);
        ::std::thread::sleep(Duration::from_millis(delay));
        if hi % 100 == 0 {
            println!("relax mode,now height {:?}", hi);
        }
    }
}
