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
    let mut action_hash    = HashMap::new();

    for (i, x) in args.iter().enumerate() {
        if is_option(x) {
            let next_value = pop_next(&args, i);
            action_hash.insert(x, next_value);
        }
        if is_stand_alone_option(x) {
            action_hash.insert(x, "".to_string());
        }
    }
    
    let mut host         = safe_get(&action_hash, &"-h".to_string());
    let mut port         = safe_get(&action_hash, &"-p".to_string());
    let mut api_key_file = safe_get(&action_hash, &"-f".to_string());

    for (k, v) in action_hash {
        println!("{}: {}", k, v)
    }
}

fn safe_get(action_hash: &HashMap<&String, String>, key: &String) -> String {
    if ! &action_hash.contains_key(key) {
        return "".to_string();
    }
    return (&action_hash[&*key]).to_string();

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
