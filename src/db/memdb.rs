use crate::db::*;
use std::collections::BTreeMap;
use std::io::{BufReader,BufRead,Write};

pub struct MemDB {
	db_pkgs: BTreeMap<Vec<u8>,BTreeMap<String, i32>>,
	db_libs: BTreeMap<Vec<u8>,Vec<String>>,
	db_weight: BTreeMap<Vec<u8>, i32>,
	api_set: Vec<String>,
	lib_set: Vec<String>	
}

impl MemDB {
	pub fn init() -> Result<Self,Box<dyn std::error::Error>> {
		let apis = BufReader::new(std::fs::File::open("resources/apis.txt")?).lines();
		let mut api_set: Vec<String> = Vec::new();
		for api in apis {
			api_set.push(api.unwrap());
		}
			
		let libs = BufReader::new(std::fs::File::open("resources/libs.txt")?).lines();
		let mut lib_set: Vec<String> = Vec::new();
		for lib in libs {
			lib_set.push(lib.unwrap());
		}

		let db_pkgs: BTreeMap<Vec<u8>,BTreeMap<String,i32>>
			= BTreeMap::new();
		let db_libs: BTreeMap<Vec<u8>,Vec<String>>
			= BTreeMap::new();
		let db_weight: BTreeMap<Vec<u8>,i32>
			= BTreeMap::new();	
		
		Ok(
			MemDB {
				db_pkgs: db_pkgs,
				db_libs: db_libs,
				db_weight: db_weight,
				api_set: api_set,
				lib_set: lib_set
			}
		)
	}
}

impl DexDB for MemDB {
	fn add_pkgs(&mut self, pkgs: Vec<PkgInfo>) {
		for pkg in pkgs {
			*self.db_pkgs.get_mut(&pkg.hash)
				.expect("could not find entry in db_pkgs")
				.get_mut(&pkg.name)
				.expect("could not find pkg in db_pkgs") += 1;
			*self.db_weight.get_mut(&pkg.hash).unwrap() = pkg.weight;
		}			
	}

	fn add_libs(&mut self, libs: Vec<LibInfo>) {
		for lib in libs {
			self.db_libs.get(&lib.hash)
				.expect("could not find entry in db_libs")
				.clone().push(lib.name);			
		}
	}

	fn remove_pkgs(&mut self, pkgs: Vec<PkgInfo>) {
		for pkg in pkgs {
			*self.db_pkgs.get_mut(&pkg.hash)
				.expect("could not find entry in db_pkgs")
				.get_mut(&pkg.name)
				.expect("could not find pkg in db_pkgs") -= 1;
		}					
	}

	fn get_pkgs(&mut self, threshold: i32) -> Vec<PkgInfo> {
		let mut ret: Vec<PkgInfo> = Vec::new();
		for (hash, pkgs) in self.db_pkgs.clone() {
			let w = *self.db_weight.get(&hash)
				.expect("could not find entry in db_weight");
			for (pkg, count) in pkgs {
				if count >= threshold {
					ret.push(
						PkgInfo{
							hash: hash.clone(),
							name: pkg,
							weight: w
						}
					)
				}
			}
		}
		ret
	}

	fn match_libs(&mut self, hash_list: Vec<Vec<u8>>) -> Vec<LibInfo> {
		let mut ret: Vec<LibInfo> = Vec::new();
		for hash in hash_list {
			for pkg in self.db_libs.get(&hash).unwrap() {
				ret.push(
					LibInfo{
						hash: hash.clone(),
						name: pkg.to_string()
					}
				)
			}
		}
		ret
	}

	fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		if let Ok(f) = std::fs::File::open("resources/db_pkgs.txt") {
			let pkgs = BufReader::new(f).lines();
			for line in pkgs {
				let l = line?;
				let mut pkg: BTreeMap<String, i32> = BTreeMap::new();
				let elems: Vec<&str> = l.split_whitespace().collect();
				pkg.insert(elems[1].to_string(),
						   elems[2].parse::<i32>().unwrap());
				self.db_pkgs.insert(elems[0].as_bytes().to_vec(), pkg);
			}
		}
		else{ println!("db_pkgs.txt not found") }
		
		if let Ok(f) = std::fs::File::open("resources/db_libs.txt") {
			let libs = BufReader::new(f).lines();
			let mut map: BTreeMap<Vec<u8>,Vec<String>> = BTreeMap::new();
			for line in libs {
				let l = line.expect("could not read line");
				let elems: Vec<&str> = l.split_whitespace().collect();
				map.entry(elems[0].as_bytes().to_vec())
					.or_default()
					.push(elems[1].to_string());
				//println!("{:?} - {}",elems[0].as_bytes().to_vec(),elems[1].to_string())
			}
			self.db_libs.append(&mut map);
		}
		else { println!("db_libs.txt not found") }
		
		if let Ok(f) = std::fs::File::open("resources/db_weights.txt") {
			let weights = BufReader::new(f).lines();
			for line in weights {
				let l = line.expect("could not read line");
				let elems: Vec<&str> = l.split_whitespace().collect();
				self.db_weight.insert(elems[0].as_bytes().to_vec(),
									  elems[1].parse::<i32>().unwrap());
			}
		}
		else{ println!("db_weights.txt not found") }
		
		Ok(())
	}

	fn preload(&mut self) {
		println!("A memory database doesn't have to preload anything");
	}

	fn dump(&mut self) -> Result<(),Box<dyn std::error::Error>> {
		if let Ok(mut f) = std::fs::File::create("resources/db_pkgs_dump.txt") {
			let pkgs = &self.db_pkgs;
			for (hash, pkg) in pkgs {
				for (name, count) in pkg {
					f.write_fmt(
						format_args!("{:?} {} {}\n",
									 hash,
									 name,
									 count)
					).expect("couldn't write db_pkgs.txt");
				}
			}
		}

		if let Ok(mut f) = std::fs::File::create("resources/db_libs_dump.txt") {
			let libs = &self.db_libs;
			for (hash, pkg) in libs {
				for name in pkg {
					f.write_fmt(
						format_args!(
							"{} {}\n",
							&String::from_utf8(hash.to_vec())
								.expect("couldn't convert hash"),
							name
						)
					).expect("couldn't write db_libs.txt");
				}
			}
		}

		if let Ok(mut f) = std::fs::File::create("resources/db_weights_dump.txt") {
			let weights = &self.db_weight;
			for (hash, weight) in weights {
				f.write_fmt(
					format_args!("{:?} {}\n",
								 hash,
								 weight)
				).expect("couldn't write db_weights.txt");
			}
		}
		Ok(())
	}
}

