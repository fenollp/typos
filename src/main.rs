extern crate bk_tree;
use bk_tree::{BKTree, metrics};

use std::collections::HashMap;

fn main() {
    let mut bkt: BKTree<&str> = BKTree::new(metrics::levenshtein);
    let mut frq: HashMap<String,u64> = HashMap::new();

    bump("foo", &mut bkt, &mut frq);
    bump("bar", &mut bkt, &mut frq);
    bump("bar", &mut bkt, &mut frq);
    bump("baz", &mut bkt, &mut frq);
    bump("quux", &mut bkt, &mut frq);
    bump("fooo", &mut bkt, &mut frq);

    println!(">>> frq {}: {}", "bar", frq.get("bar").unwrap());
    println!(">>> 0 {:?}", bkt.find("bar", 0));
    println!(">>> 1 {:?}", bkt.find("bar", 1));
    println!(">>> 2 {:?}", bkt.find("bar", 2));
}


fn bump<'a>(s: &'a str, bkt: &mut BKTree<&'a str>, frq: &mut HashMap<String,u64>) -> () {
    let ss = s.to_string();
    match frq.get(&ss) {
        Some(&freq) => {
            frq.insert(ss, freq + 1);
        }
        _ => {
            frq.insert(ss, 1);
            bkt.add(s);
        }
    }
}
