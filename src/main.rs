#[macro_use]
extern crate nom;

extern crate bk_tree;
use bk_tree::{BKTree, metrics};

use std::collections::HashMap;

use nom::{IResult,alpha,space};

named!(words<&str>,
       chain!(
           space? ~
           word: map_res!(alpha, std::str::from_utf8) ~
           space?
           ,|| word
       )
      );


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
    match words("  yo!".as_bytes()) {
        IResult::Done(rest, val) => {
            println!("{:?} rest: {:?}", val, rest);
        },
        _ => println!("error!")
    }
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


#[test]
fn test_words_parser() {
    let empty = "".as_bytes();
    assert_eq!(words("hey".as_bytes()), IResult::Done(empty, "hey"));
    assert_eq!(words(" hey".as_bytes()), IResult::Done(empty, "hey"));
}
