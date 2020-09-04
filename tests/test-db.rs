use dex::DexReader;
use libradar::{
	db::{
		DexDB,
		memdb::*
	},
	apk::*
};

#[test]
fn test_load_dump() {
	let db = &mut MemDB::init().unwrap();
	db.load().expect("couldn't load databases");
	db.dump().expect("couldn't export databases");
}

// #[test]
// fn test_add_pkgs() {
// 	let db = &mut MemDB::init().unwrap();
// 	db.load().expect("couldn't load databases");
// 	let apk = Apk::from_path("resources/test01.apk")
// 		.expect("couldn't open apk file");
// 	for dex in apk.dex_files {
// 		dex.
// 	}
	
// }
