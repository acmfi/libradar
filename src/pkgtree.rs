use crate::callgraph::get_invoked_methods_names;
use dex::class::Class;
use dex::Dex;
use hex;
use md5::{Digest, Md5};
use std::collections::HashMap;

enum Tree {
    Root {
        name: String,
        branches: HashMap<String, Tree>,
    },
    Leaf {
        name: String,
        hash: String,
        weight: usize,
    },
}

// FIXME: This algorithm is wrong. Check the comment in your first commit.
fn get_invoked_apis<R: AsRef<[u8]>>(
    dex: &Dex<R>,
    class: Class,
    api_set: &Vec<String>,
) -> Vec<String> {
    let ret = Vec::new();
    for method in class.methods() {
        if let Some(code) = method.code() {
            for target in get_invoked_methods_names(&code, &dex) {
                if api_set.contains(&target) {}
            }
        }
    }
    return ret;
}

// FIXME: Specify the return type here
fn calc_hash(lst: Vec<String>) -> String {
    let mut ret = Md5::new();
    for s in lst {
        ret.update(s.as_bytes());
    }
    return hex::encode(ret.finalize());
}

impl Tree {
    // FIXME: You have to return the `Tree` thingy
    // The fix for `Dex` is that a `Dex` is parametrized on some reference to
    // a buffer of bytes (`u8`s). This syntax I think is the most generic AFAIK to
    // represent this.
    pub fn new<R: AsRef<[u8]>>(dex: Dex<R>, api_set: Vec<String>) -> Tree {
        let mut branches = HashMap::new();
        let name = String::from("L");
        let tree = Tree::Root { branches, name };

        for class in dex.classes() {
            let class = class.expect("Failed to load class");
            let class_name = class.jtype().type_descriptor().to_string();
            assert!(class_name.starts_with('L'));
            let apis = get_invoked_apis(&dex, class, &api_set);
            if apis.len() == 0 {
                continue;
            }
            let leaf = Tree::Leaf {
                name: class_name,
                hash: calc_hash(apis),
                weight: apis.clone().len(),
            };
	    
        }
        return tree;
    }
}
