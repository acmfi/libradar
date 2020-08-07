pub mod disass;

pub mod radar {
	use std::fs;
	use std::io::{BufReader,Read};
	use std::collections::HashMap;
	use dex::{method,Dex,DexReader};
	use super::disass::*;
	
	type ApkArchive = zip::read::ZipArchive<BufReader<fs::File>>;

	pub struct APK <'a> {
		path: &'a str,
		apk: ApkArchive
	}

	impl <'a> APK <'a> {
		pub fn new (path: &'a str, reader: BufReader<fs::File>) -> Result<Self,Box<dyn std::error::Error>> {
			let apk = zip::ZipArchive::new(reader)?;
			Ok(
				APK {
					path: path,
					apk: apk
				}
			)

		}

		pub fn set_path(&mut self, path: &'static str) {
			self.path = path;
		}

		pub fn get_path(&self) -> &'a str {
			self.path
		}

		pub fn get_apk(&mut self) -> &mut ApkArchive {
			&mut self.apk
		}

	}

	pub fn find_apk(name: &str) -> String {
		let fname = std::path::Path::new(name);
		assert!(fname.exists());
		name.to_string()
	}

	pub fn open_apk(name: &str) -> Result<APK,Box<dyn std::error::Error>> {
		let path = find_apk(&name);
		let file = fs::File::open(&path)?;
		let reader = BufReader::new(file);
		println!("APK file {} opened succesfully",&name);

		APK::new(name,reader)
	}

	pub fn show_apk_contents(apk: &mut APK) {
		let list = apk.get_apk().file_names();
		for name in list {
			println!("{}",name)
		}
	}

	pub fn show_dex_files(apk: &mut APK) {
		let list = apk.get_apk().file_names();
		for name in list {
			if name.contains(".dex") {
				println!("{}",name)
			}
		}
	}

	pub fn get_dex_list<'a>(apk: &'a mut APK<'a>) -> Vec<&'a str> {
		let list = apk.get_apk().file_names();
		let mut vec: Vec<&'a str> = Vec::new();
		for name in list {
			if name.contains(".dex") {
				vec.push(name)
			}
		}
		vec
	}

	fn read_dex_file(apk: &mut APK, file: &str, mut buf: &mut Vec<u8>) -> Result<(),Box<dyn std::error::Error>>{
		apk.get_apk().by_name(file).unwrap().read_to_end(&mut buf)?;
		Ok(())
	}
	
	pub fn get_dex_files<'a>(apk: &mut APK, files: Vec<&'a str>) -> Result<HashMap<&'a str, Dex<Vec<u8>>>,Box<dyn std::error::Error>> {
		let mut filemap: HashMap<&str, Dex<Vec<u8>>> = HashMap::new();
		for file in files.iter() {
			let mut bytearray: Vec<u8> = Vec::new();
			read_dex_file(apk, file, &mut bytearray)?;
			let dex_array = DexReader::from_vec(bytearray)?;
			filemap.insert(file, dex_array);
		}		
		Ok(filemap)
	}
	
	pub fn print_dex_methods<'a>(dexmap: HashMap<&'a str, Dex<Vec<u8>>>) {
		let mut mapcopy = dexmap;
		let dex = mapcopy.remove("classes.dex").unwrap();
		for class_def in dex.class_defs() {
			let class_def = class_def.unwrap();
			let javatype = dex.get_type(class_def.class_idx()).unwrap();
			//if type.to_java_type().contains("android") {
				let class = dex.find_class_by_name(&javatype.type_descriptor().to_string()).unwrap().unwrap();
				println!("Java Type: {}",javatype.to_java_type());
				println!("Methods: <");
				for method in class.methods() {
					println!("\t{} {},",javatype.to_java_type(),method.name());
					if let Some(code) = method.code() {
						println!("\tIntructions for {}", method.name());
						//let mut found = false;
						for (idx, ins) in disassemble(code).enumerate() {
							println!("\t\tInstruction: {}, Is invoke?: {}",ins.mnemonic(),ins.is_invoke());
							// if ins.is_invoke() && !found {
							// 	found = true;
							// 	println!("\t{} {},",jtype.to_java_type(),method.name());
							// }
						}
						println!();
					}
					
				}
				println!(">");
				//println!("Result: {:#?}",result.unwrap().unwrap());
			//}	
		}
	}
	
}

#[cfg(test)]
mod tests {
    use super::radar::APK;
    use super::radar::*;
    use std::fs;
    use std::io::BufReader;
	//use std::io::Read;

    #[test]
    fn find_file() {
        assert_eq!(find_apk("resources/test01.apk"), "resources/test01.apk");
    }

    #[test]
    fn create_struct() -> Result<(), Box<dyn std::error::Error>> {
        let file = fs::File::open("resources/test01.apk")?;
        let reader = BufReader::new(file);
        let _apk = APK::new("resources/test01.apk", reader)?;
        Ok(())
    }
    #[test]
    fn get_dex() -> Result<(), Box<dyn std::error::Error>> {
		let mut apk = open_apk("resources/test01.apk")?;
		let files = get_dex_list(&mut apk);
		let mut apk = open_apk("resources/test01.apk")?;
		let map = get_dex_files(&mut apk, files)?;
		// let mut dex = fs::File::open("resources/classes.dex")?;
		// let mut buf: Vec<u8> = Vec::new();
		// dex.read_to_end(&mut buf)?;
		assert!(map.contains_key("classes.dex"));
		for (k, _v) in map {
			assert_eq!(k,"classes.dex");
			// assert_eq!(v.as_slice(),buf.as_slice());
		}
        Ok(())
	}

    #[test]
    fn show_content() {}

    #[test]
    fn show_dex() {}
}
