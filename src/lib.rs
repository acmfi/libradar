pub mod radar {
    use std::fs;
    use std::io::BufReader;

    pub struct APK<'a> {
        path: &'a str,
        apk: zip::read::ZipArchive<BufReader<fs::File>>,
    }

    impl<'a> APK<'a> {
        pub fn new(
            path: &'a str,
            reader: BufReader<fs::File>,
        ) -> Result<Self, Box<std::error::Error>> {
            let apk = zip::ZipArchive::new(reader)?;
            Ok(APK {
                path: &path,
                apk: apk,
            })
        }

        pub fn set_path(&mut self, path: &'a str) {
            self.path = path;
        }

        pub fn get_path(&self) -> &'a str {
            self.path
        }

        pub fn get_apk(&self) -> &zip::ZipArchive<BufReader<fs::File>> {
            let ret = &self.apk;
            &ret
        }
    }

    pub fn find_apk(name: &str) -> String {
        let fname = std::path::Path::new(name);
        assert!(fname.exists());
        name.to_string()
    }

    pub fn open_apk(name: &str) -> Result<APK, Box<std::error::Error>> {
        let path = find_apk(&name);
        let file = fs::File::open(&path)?;
        let reader = BufReader::new(file);
        println!("APK file {} opened succesfully", &name);

        APK::new(&name, reader)
    }

    pub fn show_apk_contents(apk: &APK) {
        let list = apk.get_apk().file_names();
        for name in list {
            println!("{}", name)
        }
    }

    pub fn show_dex_files(apk: &APK) {
        let list = apk.get_apk().file_names();
        for name in list {
            if name.contains("classes") {
                println!("{}", name)
            }
        }
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
