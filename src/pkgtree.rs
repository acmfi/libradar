use dex::Class;
use dex::Dex;
use md5::Digest;
use std::collections::HashMap;

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

pub fn build_tree(dex: Dex, api_set: Vec<String>) {
    Tree::Root.name = "";
    Tree::Root.branches = HashMap::new();

    for class in dex.classes() {
        let name = class.name();
        assert!(name.starts_with('L'));
        let apis = get_invoked_apis(class, api_set);
        if apis.len() == 0 {
            continue;
        }
        leaf = Tree::Leaf {
            name: name,
            hash: calc_hash(apis),
            weight: apis.len(),
        };
        Tree::Root::branches::insert(name, leaf);
    }

    fn get_invoked_apis(class: Class, api_set: Vec<String>) -> Vec<String> {
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
            ret.update(Byte::from(s));
        }
        return ret.finalize();
    }
}
