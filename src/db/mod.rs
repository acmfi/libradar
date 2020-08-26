pub mod memdb;

struct PkgInfo {
	hash: Vec<u8>,
	name: String,
	weight: i32
}

struct LibInfo {
	hash: Vec<u8>,
	name: String
}

struct PkgResutl {
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
	fn add_pkgs(&mut self, _pkgs: Vec<PkgInfo>);

	fn add_libs(&mut self, _libs: Vec<LibInfo>);

	fn remove_pkgs(&mut self, _pkgs: Vec<PkgInfo>);

	fn get_pkgs(&mut self, _threshold: i32) -> Vec<PkgInfo>;

	fn match_libs(&mut self, _hash_list: Vec<Vec<u8>>) -> Vec<LibInfo>;

	fn load(&mut self) -> Result<(), Box<dyn std::error::Error>>;
	
	fn preload();

	fn dump();
}
