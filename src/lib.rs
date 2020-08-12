pub mod disass;

pub mod radar {
	use std::{
		fs,
		io::Read,
		collections::HashMap,
	};
	use dex::{Dex,DexReader};
	use super::disass::*;
	use rc_zip::{prelude::*,EntryContents};
	
	type ApkArchive = rc_zip::Archive;	

	pub struct APK<'a> {
		path: &'a str,
		file: fs::File,
		apk: ApkArchive
	}
	
	
	impl <'a> APK<'a> {
		pub fn new (path: &'a str, file: fs::File) -> Result<Self,Box<dyn std::error::Error>> {
			let apk = file.read_zip()?;
			Ok(
				APK {
					path: path,
					file: file,
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

		pub fn get_file(&self)  -> &fs::File {
			&self.file
		}
		
		pub fn get_apk(&self) -> &ApkArchive {
			&self.apk
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
		println!("APK file {} opened succesfully",&name);

		APK::new(name,file)
	}

	pub fn show_apk_contents(apk: &APK) {
		let list = apk.get_apk().entries();
		for name in list {			
			println!("{}",name.name())
		}
	}

	pub fn show_dex_files(apk: &mut APK) {
		let list = apk.get_apk().entries();
		for name in list {
			if name.name().contains(".dex") {
				println!("{}",name.name())
			}
		}
	}

	pub fn get_dex_list<'a>(apk: &'a APK<'a>) -> Vec<&'a str> {
		let list = apk.get_apk().entries();
		let mut vec: Vec<&'a str> = Vec::new();
		for name in list {
			if name.name().contains(".dex") {
				vec.push(name.name())
			}
		}
		vec
	}
	
	fn read_dex_file(apk: &APK, file: &str, buf: &mut Vec<u8>) -> Result<(),Box<dyn std::error::Error>>{
		if let EntryContents::File(f) = apk.get_apk().by_name(file).unwrap().contents() {
			let mut r = f.entry.reader(|offset| positioned_io::Cursor::new_pos(apk.get_file(),offset));			
			r.read_to_end(buf)?;
		}
		Ok(())
	}
	
	pub fn get_dex_files<'a>(apk: &APK, files: Vec<&'a str>) -> Result<HashMap<&'a str, Dex<Vec<u8>>>,Box<dyn std::error::Error>> {
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
						for (_, ins) in disassemble(code).enumerate() {
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
    use super::radar::{APK,*};
    use std::fs;
	use dex::DexReader;
	
    #[test]
    fn find_file() {
        assert_eq!(find_apk("resources/test01.apk"), "resources/test01.apk");
    }

    #[test]
    fn create_struct() -> Result<(), Box<dyn std::error::Error>> {
        let file = fs::File::open("resources/test01.apk")?;
        //let reader = BufReader::new(file);
        let _apk = APK::new("resources/test01.apk", file)?;
        Ok(())
    }
    // #[test]
    // fn get_dex() -> Result<(), Box<dyn std::error::Error>> {
	// 	let mut apk = open_apk("resources/test01.apk")?;
	// 	let files = get_dex_list(&mut apk);
	// 	let mut apk = open_apk("resources/test01.apk")?;
	// 	let map = get_dex_files(&mut apk, files)?;
	// 	let vv = DexReader::from_file("resources/testapk/classes.dex")?;
	// 	assert!(map.contains_key("classes.dex"));
	// 	for (k, v) in map {
	//     	assert_eq!(k,"classes.dex");
	// 		assert_eq!(v.header().checksum(),vv.header().checksum());
	// 	}
    //     Ok(())
	// }

	#[test]
    fn get_dex() -> Result<(), Box<dyn std::error::Error>> {
		let apk = open_apk("resources/test01.apk")?;
		let files = get_dex_list(&apk);
		let map = get_dex_files(&apk, files)?;
		let dex = DexReader::from_file("resources/testapk/classes.dex")?;
		assert!(map.contains_key("classes.dex"));
		for (k, v) in map {
	    	assert_eq!(k,"classes.dex");
			assert_eq!(v.header().checksum(),dex.header().checksum());
		}
        Ok(())
	}

}
