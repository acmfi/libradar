pub mod memdb;

use dex::Dex;
use crate::apk::Apk;

pub struct PkgInfo {
	hash: Vec<u8>,
	name: String,
	weight: i32
}

pub struct LibInfo {
	hash: Vec<u8>,
	name: String
}

pub struct PkgResult {
	hash: Vec<u8>,
	name: String,
	lib_name: String,
	similarity: Option<f32>
}

pub struct Thresholds {
	lib_match_rate: f32,
	min_api_weight: i32,
	min_lib_count: i32,
	pkg_name_blacklist: Vec<String>	
}

pub trait DexDB {	
	fn add_pkgs(&mut self, _pkgs: Vec<PkgInfo>);
	
	fn add_libs(&mut self, _libs: Vec<LibInfo>);
	
	fn remove_pkgs(&mut self, _pkgs: Vec<PkgInfo>);
	
	fn get_pkgs(&mut self, _threshold: i32) -> Vec<PkgInfo>;

	fn match_libs(&mut self, _hash_list: Vec<Vec<u8>>) -> Vec<LibInfo>;

	fn load(&mut self) -> Result<(), Box<dyn std::error::Error>>;
	
	fn preload(&mut self);

	fn dump(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}


