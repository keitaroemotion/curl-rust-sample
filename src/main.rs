extern crate regex;

use regex::Regex;
use std::env;
use std::collections::HashMap;

// --dry-run
// -h host
// -p port
// -f api-key-file
//
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut actionHash    = HashMap::new();

    for (i, x) in args.iter().enumerate() {
        if is_option(x) {
            let nextValue = pop_next(&args, i);
            actionHash.insert(x, nextValue);
        }
        if is_stand_alone_option(x) {
            actionHash.insert(x, "".to_string());
        }
    }

    for (k, v) in actionHash {
        println!("{}: {}", k, v)
    }
}

fn pop_next(args: &Vec<String>, i: usize) -> String {
    let next_elem = &args[i + 1];
    if starts_with_hyphen(next_elem.to_string()) {
        return "".to_string();
    }
    return next_elem.to_string();
}

fn is_stand_alone_option(text: &str) -> bool {
    return matches(r"^\-\-[^\-]+", text);
}

fn is_option(text: &str) -> bool {
    return matches(r"^\-[^\-]+", text);
}

fn starts_with_hyphen(text: String) -> bool {
    return matches(r"^\-", &*text);
}

fn matches(regex: &str, text: &str) -> bool {
    return Regex::new   (regex)
               .unwrap  ()
               .captures(text)
               .is_some ();
}
