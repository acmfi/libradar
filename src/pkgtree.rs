use crate::callgraph::get_invoked_methods_names;
use dex::class::Class;
use dex::Dex;
use hex;
use md5::{Digest, Md5};
use std::collections::HashMap;

#[derive(Copy)]
enum Tree {
    Root {
        tree_name: String,
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
        let branches = HashMap::new();
        let tree_name = String::from("L");
        let tree = Tree::Root {
            branches,
            tree_name,
        };

        for class in dex.classes() {
            let class = class.expect("Failed to load class");
            let class_name = class.jtype().type_descriptor().to_string();
            assert!(class_name.starts_with('L'));
            let apis = get_invoked_apis(&dex, class, &api_set);
            let apis_len = apis.len();
            if apis_len == 0 {
                continue;
            }
            let leaf = Tree::Leaf {
                name: class_name,
                hash: calc_hash(apis),
                weight: apis_len,
            };
            tree.add_leaf(leaf);
        }
        return tree;
    }

    fn add_leaf(self, leaf: Tree) {
        match leaf {
            Tree::Leaf { name, hash, weight } => match self {
                Tree::Root {
                    tree_name,
                    mut branches,
                } => {
                    let suffix = name
                        .as_str()
                        .get(tree_name.as_str().len() + 1..)
                        .expect("Failed to load");
                    let elems: Vec<&str> = suffix.split('/').collect();
                    let next_name = elems.get(0).expect("Failed to get element");
                    if suffix.contains('/') {
                        if !branches.contains_key::<str>(next_name) {
                            branches.insert(
                                next_name.to_string(),
                                Tree::Leaf {
                                    name: tree_name + "/" + next_name,
                                    hash,
                                    weight,
                                },
                            );
                            branches
                                .get::<str>(next_name)
                                .expect("Something went wrong")
                                .add_leaf(leaf);
                        }
                    } else {
                        branches.insert(next_name.to_string(), leaf);
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}
