use dex::DexReader;
use libradar::disass::disassemble;

#[test]
fn test_disassembly() {
    let mnemonics = [
        "add-int/lit8",
        "invoke-super",
        "move-result-object",
        "invoke-virtual",
        "move-result",
        "if-eqz",
        "sget-object",
        "invoke-static",
        "return-object",
    ];
    let is_call = [false, true, false, true, false, false, false, true, false];

    let dex = DexReader::from_file("resources/classes.dex").expect("Can't open test dex file");
    let class = dex
        .find_class_by_name("Lcom/devoteam/quickaction/QuickActionItem;")
        .expect("Failed to load class")
        .expect("class not found");
    let mut found = false;
    for method in class.methods() {
        if method.name() == "onCreateDrawableState" {
            if let Some(code) = method.code() {
                found = true;
                for (idx, ins) in disassemble(code).enumerate() {
                    assert_eq!(ins.mnemonic(), mnemonics[idx]);
                    assert_eq!(ins.is_invoke(), is_call[idx]);
                }
            }
        }
    }

    assert!(
        found,
        "The test method to disassemble could not be found. Test missed!"
    );
}
