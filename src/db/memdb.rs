use crate::db::*;
use std::collections::BTreeMap;
use std::io::{BufReader,BufRead,Read};

pub struct MemDB<'a> {
	db_pkgs: BTreeMap<Vec<u8>,BTreeMap<&'a str, i32>>,
	db_libs: BTreeMap<Vec<u8>,Vec<&'a str>>,
	db_weight: BTreeMap<Vec<u8>, i32>,
	api_set: Vec<String>,
	lib_set: Vec<String>	
}

impl <'a> MemDB<'a> {
	pub fn init() -> Result<Self,Box<dyn std::error::Error>> {
		let apis = BufReader::new(std::fs::File::open("apis.txt")?).lines();
		let mut api_set: Vec<String> = Vec::new();
		for api in apis {
			api_set.push(api.unwrap());
		}
			
		let libs = BufReader::new(std::fs::File::open("libs.txt")?).lines();
		let mut lib_set: Vec<String> = Vec::new();
		for lib in libs {
			lib_set.push(lib.unwrap());
		}

		let db_pkgs: BTreeMap<Vec<u8>,BTreeMap<&str,i32>>
			= BTreeMap::new();
		let db_libs: BTreeMap<Vec<u8>,Vec<&str>>
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

impl DexDB for MemDB<'_> {
	fn add_pkgs(mut db: &MemDB, pkgs: Vec<PkgInfo>) {
		for pkg in pkgs {
			// lo que se hace en memdb.py es que se modifica el peso que se encuentra
			// en 'db_pkgs' (un diccionario que tiene por valor otro diccionario,
			// y el valor de este es el peso) y en 'db_weight'
			// de estar trasteando al final me he quedado con esto y me suelta este erorr:
			// lifetime mismatch ...but data from 'pkgs' flows into 'db'
			// en 55:52
			*db.db_pkgs.get_mut(&pkg.hash).unwrap().get_mut(&pkg.name).unwrap() += 1;
			*db.db_weight.get_mut(&pkg.hash).unwrap() = pkg.weight;
		}			
	}

	fn add_libs(mut db: &MemDB, libs: Vec<LibInfo>) {
		for lib in libs {
			db.db_libs.get(&lib.hash).unwrap().push(&lib.name);			
		}
	}

	fn remove_pkgs(mut db: &MemDB, pkgs: Vec<PkgInfo>) {
		for pkg in pkgs {
			// lo mismo que en add_pkgs()
			*db.db_pkgs.get_mut(&pkg.hash).unwrap().get_mut(&pkg.name).unwrap() -= 1;
		}					
	}

	fn get_pkgs<'a>(db: &'a MemDB, threshold: i32) -> Vec<PkgInfo<'a>> {
		let mut ret: Vec<PkgInfo> = Vec::new();
		for (hash, pkgs) in db.db_pkgs {
			let w = *db.db_weight.get(&hash).unwrap();
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

	fn match_libs<'a>(db: &'a MemDB, hash_list: Vec<Vec<u8>>) -> Vec<LibInfo<'a>> {
		let mut ret: Vec<LibInfo> = Vec::new();
		for hash in hash_list {
			for pkg in db.db_libs.get(&hash).unwrap() {
				ret.push(
					LibInfo{
						hash: hash.clone(),
						name: pkg
					}
				)
			}
		}
		ret
	}

	fn load(db: &MemDB) {
		
	}

	fn preload() {
		
	}

	fn dump() {
		
	}
}
