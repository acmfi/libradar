pub mod memdb;

use dex::Dex;
use crate::apk::Apk;

struct PkgInfo {
	hash: Vec<u8>,
	name: String,
	weight: i32
}

struct LibInfo {
	hash: Vec<u8>,
	name: String
}

struct PkgResult {
	hash: Vec<u8>,
	name: String,
	lib_name: String,
	similarity: Option<f32>
}

struct Thresholds {
	lib_match_rate: f32,
	min_api_weight: i32,
	min_lib_count: i32,
	pkg_name_blacklist: Vec<String>	
}

trait DexDB {
	fn detect_dex_libs(&mut self, dex: Dex<Vec<u8>>) -> Vec<PkgResult>;

	fn detect_apk_libs(&mut self, apk: Apk) -> Vec<PkgResult>;
	
	fn add_dex_to_db(&mut self, dex: Dex<Vec<u8>>);
	
	fn add_apk_to_db(&mut self, apk: Apk);
	
	fn remove_dex_from_db(&mut self, dex: Dex<Vec<u8>>);
	
	fn remove_apk_from_db(&mut self, apk: Apk);
	
	fn get_pkgtree(&mut self, dex: Dex<Vec<u8>>) -> Vec<PkgInfo>;
	
	fn update_lib_db(&mut self);
	
	fn dump_db(&mut self);
	
	fn load_db(&mut self);
	
	fn preload_db(&mut self);
	
	fn add_pkgs(&mut self, _pkgs: Vec<PkgInfo>);
	
	fn add_libs(&mut self, _libs: Vec<LibInfo>);
	
	fn remove_pkgs(&mut self, _pkgs: Vec<PkgInfo>);
	
	fn get_pkgs(&mut self, _threshold: i32) -> Vec<PkgInfo>;

	fn match_libs(&mut self, _hash_list: Vec<Vec<u8>>) -> Vec<LibInfo>;

	fn load(&mut self) -> Result<(), Box<dyn std::error::Error>>;
	
	fn preload(&mut self);

	fn dump(&mut self);
}


