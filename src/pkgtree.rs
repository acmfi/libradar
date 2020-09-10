use dex::class;
use dex::Dex;
use md5::Digest;
use std::collections::HashMap;
use std::io::Bytes;

enum Tree {
    Root {
        name: String,
        branches: HashMap<String, Tree>,
    },
    Leaf {
        name: String,
        hash: String,
        weight: u16,
    },
}

impl Tree {
    pub fn new(dex: Dex<>, api_set: Vec<String>) {
	let branches = HashMap::new();
	let name = String::from("L");
        let tree = Tree::Root {branches, name};

        for class in dex.classes() {
            let name = class.name();
            assert!(name.starts_with('L'));
            let apis = get_invoked_apis(class, api_set);
            if apis.len() == 0 {
                continue;
            }
            let leaf = Tree::Leaf {
                name: name,
                hash: calc_hash(apis),
                weight: apis.len(),
            };
            tree.branches.insert(name, leaf);
        }
    }
    fn get_invoked_apis(class: class, api_set: Vec<String>) -> Vec<String> {
        let ret = Vec::new();
        for methods in class.methods() {
            for invoked_method in methods {
                if api_set.contains(&methods) {
                    ret.append(methods);
                }
            }
        }
        return ret;
    }
    fn calc_hash(lst: Vec<String>) {
        let ret = Digest::new();
        for s in lst {
            ret.update(Bytes::from(s));
        }
        return ret.finalize();
    }
}
