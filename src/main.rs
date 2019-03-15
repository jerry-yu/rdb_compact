extern crate rocksdb;

use clap::App;
use rocksdb::{ColumnFamilyDescriptor, Options, DB};

const COL_NUM: u32 = 7;

fn main() {
    let matches = App::new("cita-recover")
        .author("yubo")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage("-d, --data=[PATH] 'Set DB data dir'")
        .args_from_usage("-t, --thread=[NUMBER] 'Set compaction threads number'")
        .args_from_usage("-o, --openfile=[NUMBER] 'Set max open file number'")
        .get_matches();

    let data_path = matches
        .value_of("data")
        .expect("please specify the db data dir");

    let th = matches
        .value_of("thread")
        .unwrap_or("2")
        .parse::<i32>()
        .unwrap_or(2);

    let fnum = matches
        .value_of("openfile")
        .unwrap_or("512")
        .parse::<i32>()
        .unwrap_or(512);

    let mut cfs = Vec::new();
    let mut cfname = Vec::new();

    for i in 0..COL_NUM {
        cfname.push("col".to_owned() + &i.to_string());
    }

    for name in cfname.clone() {
        cfs.push(ColumnFamilyDescriptor::new(name, Options::default()));
    }

    let mut db_opts = Options::default();
    db_opts.create_missing_column_families(false);
    db_opts.create_if_missing(false);
    db_opts.set_max_background_compactions(th);
    db_opts.set_max_open_files(fnum);

    let db = DB::open_cf_descriptors(&db_opts, data_path, cfs).unwrap();
    for col in cfname {
        println!("compact column begin");
        let cf = db.cf_handle(&*col).expect("column not found");
        db.compact_range_cf(cf, None, None);
        println!("compact column end");
    }
    //let _ = DB::destroy(&db_opts, data_path);
}
