use crate::disass::disassemble;
use dex;
use dex::code::CodeItem;
use dex::method::MethodIdItem;
use dex::Dex;
use std::convert::TryInto;

pub fn get_invoked_methods<'a>(
    code: &'a CodeItem,
    dex: &'a Dex<Vec<u8>>,
) -> impl Iterator<Item = MethodIdItem> + 'a {
    disassemble(code).filter_map(move |ins| {
        if ins.is_invoke() {
            let target = dex
                .get_method_item(ins.invoke_target().try_into().unwrap())
                .unwrap();
            Some(target)
        } else {
            None
        }
    })
}

pub fn get_invoked_methods_names<'a>(
    code: &'a CodeItem,
    dex: &'a Dex<Vec<u8>>,
) -> impl Iterator<Item = String> + 'a {
    get_invoked_methods(code, dex).map(move |target| {
        let method_name = dex.get_string(target.name_idx()).unwrap().to_string();
        let class_name = dex
            .get_type(target.class_idx().into())
            .unwrap()
            .type_descriptor()
            .to_string();
        format!("{}->{}", class_name, method_name)
    })
}
