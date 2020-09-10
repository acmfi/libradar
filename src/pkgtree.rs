use dex::class::Class;
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

// FIXME: This algorithm is wrong. Check the comment in your first commit.
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

// FIXME: Specify the return type here
fn calc_hash(lst: Vec<String>) {
    let ret = Digest::new();
    for s in lst {
        ret.update(Bytes::from(s));
    }
    return ret.finalize();
}

impl Tree {

    // FIXME: You have to return the `Tree` thingy
    // The fix for `Dex` is that a `Dex` is parametrized on some reference to
    // a buffer of bytes (`u8`s). This syntax I think is the most generic AFAIK to
    // represent this.
    pub fn new<R: AsRef<[u8]>>(dex: Dex<R>, api_set: Vec<String>) {
        let branches = HashMap::new();
        let name = String::from("L");
        let tree = Tree::Root { branches, name };

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
}
