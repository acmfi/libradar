use crate::db::memdb::MemDB;

pub mod memdb;

struct PkgInfo<'a> {
	hash: Vec<u8>,
	name: &'a str,
	weight: i32
}

struct LibInfo<'a> {
	hash: Vec<u8>,
	name: &'a str	
}

struct PkgResutl<'a> {
	hash: Vec<u8>,
	name: &'a str,
	lib_name: &'a str,
	similarity: Option<f32>
}

struct Thresholds<'a> {
	lib_match_rate: f32,
	min_api_weight: i32,
	min_lib_count: i32,
	pkg_name_blacklist: Vec<&'a str>	
}

trait DexDB {
	fn add_pkgs(mut _db: &MemDB, _pkgs: Vec<PkgInfo>) { todo!() }

	fn add_libs(mut _db: &MemDB, _libs: Vec<LibInfo>) { todo!() }

	fn remove_pkgs(mut _db: &MemDB, _pkgs: Vec<PkgInfo>) { todo!() }

	fn get_pkgs<'a>(_db: &'a MemDB, _threshold: i32) -> Vec<PkgInfo<'a>> { todo!() }

	fn match_libs<'a>(_db: &'a MemDB, _hash_list: Vec<Vec<u8>>) -> Vec<LibInfo<'a>> { todo!() }

	fn load(_db: &MemDB) { todo!() }
	
	fn preload() { todo!() }

	fn dump() { todo!() }
}
