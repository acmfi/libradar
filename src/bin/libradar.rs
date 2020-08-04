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
    let apk_file = &open_apk(&*args[1]).unwrap();
    println!();

    println!("Listing all contents in APK file...");
    show_apk_contents(apk_file);
    println!();

    println!("Listing .dex files in APK file...");
    show_dex_files(apk_file);
}
