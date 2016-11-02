extern crate bk_tree;

use bk_tree::{BKTree, metrics};

fn main() {
    let mut bkt: BKTree<&str> = BKTree::new(metrics::levenshtein);

    bkt.add("foo");
    bkt.add("bar");
    bkt.add("baz");
    bkt.add("quux");
    bkt.add("fooo");

    println!(">>> 0 {:?}", bkt.find("bar", 0));
    println!(">>> 1 {:?}", bkt.find("bar", 1));
    println!(">>> 2 {:?}", bkt.find("bar", 2));
}
