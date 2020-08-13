use libradar::apk::*;
use libradar::callgraph::get_invoked_methods_names;

fn print_info_of_apk(apk: Apk) {
    for dex in apk.dex_files {
        for class in dex.classes() {
            let class = class.expect("Failed to load class");
            let class_name = class.jtype().type_descriptor().to_string();
            println!("class {}", class_name);
            for method in class.methods() {
                println!("  method {}", method.name().to_string());
                if let Some(code) = method.code() {
                    for target in get_invoked_methods_names(&code, &dex) {
                        println!("    {}", target);
                    }
                }
            }
        }
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("An argument is needed in order to work.");
        println!("Usage: {} <apk filename>", &*args[0]);
        return;
    }

    let apk = Apk::from_path(&*args[1]).expect("Failed to open APK");

    print_info_of_apk(apk);
}
