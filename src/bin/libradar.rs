use libradar::radar::*;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("An argument is needed in order to work.");
        println!("Usage: {} <apk filename>", &*args[0]);
        panic!();
    }

    println!("Finding APK file...");
    let apk_path = find_apk(&*args[1]);
    println!("{}\n", apk_path);

	println!("Opening APK file...");
	let mut apk_file = &mut open_apk(&*args[1]).unwrap();
	println!();

	println!("Listing all contents in APK file...");
	//show_apk_contents(apk_file);
	println!();

	println!("Listing .dex files in APK file...");
	show_dex_files(apk_file);
	println!();

	println!("Getting list of .dex files...");
	let list = get_dex_list(&mut apk_file);
	//let list
	println!();

	let apk_file = &mut open_apk(&*args[1]).unwrap();
	println!();

	println!("Mapping all fetched .dex files...");
	let _map = get_dex_files(apk_file,list);
	println!();



}
