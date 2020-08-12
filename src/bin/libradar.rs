use libradar::apk::*;
use libradar::disass::disassemble;
use std::convert::TryInto;

fn print_info_of_apk(apk: Apk) {
    for class in apk.classes() {
        let class = class.expect("Failed to load class");
        let class_name = class.jtype().type_descriptor().to_string();
        println!("class {}", class_name);
        for method in class.methods() {
            println!("  method {}", method.name().to_string());
            if let Some(code) = method.code() {
                for ins in disassemble(code) {
                    println!("    {}", ins.mnemonic());
                    if ins.is_invoke() {
                        let target = ins.invoke_target();
                        let target_method = apk.get_method_item(target.try_into().unwrap()).unwrap();
                        println!("      {} | {:?}", target, target_method);
                    }
                }
            } else {
                println!("    This method has no code");
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
