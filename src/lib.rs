pub mod disass;

pub mod radar {
	use std::fs;
	use std::io::BufReader;
	use std::io::Read;
	use std::collections::HashMap;
	use std::borrow::ToOwned;

	type ApkArchive = zip::read::ZipArchive<BufReader<fs::File>>;

	pub struct APK <'a> {
		path: &'a str,
		apk: ApkArchive
	}

	impl <'a> APK <'a> {
		pub fn new (path: &'a str, reader: BufReader<fs::File>) -> Result<Self,Box<std::error::Error>> {
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

	pub fn open_apk(name: &str) -> Result<APK,Box<std::error::Error>> {
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

	fn read_dex_file(apk: &mut APK, file: &str, mut buf: &mut Vec<u8>) {
		apk.get_apk().by_name(file).unwrap().read_to_end(&mut buf);
	}

	pub fn get_dex_files<'a>(apk: &mut APK, files: Vec<&'a str>) -> HashMap<&'a str, Vec<u8>> {
		let mut filemap: HashMap<&str, Vec<u8>> = HashMap::new();
		for file in files.iter() {
			let mut bytearray: Vec<u8> = Vec::new();
			read_dex_file(apk, file, &mut bytearray);
			filemap.insert(file, bytearray);
		}
		filemap
	}
}

#[cfg(test)]
mod tests {
    use super::radar::APK;
    use super::radar::*;
    use std::fs;
    use std::io::BufReader;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn find_file() {
        assert_eq!(find_apk("resources/test01.apk"), "resources/test01.apk");
    }

    #[test]
    fn create_struct() -> Result<(), Box<std::error::Error>> {
        let file = fs::File::open("resources/test01.apk")?;
        let reader = BufReader::new(file);
        let _apk = APK::new("resources/test01.apk", reader)?;
        Ok(())
    }
    #[test]
    fn open_file() {}

    #[test]
    fn show_content() {}

    #[test]
    fn show_dex() {}
}
