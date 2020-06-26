extern crate regex;

use curl::easy::Easy;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::io::{stdin, Read};
use std::io::{stdout, Write};

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
    
    let mut action       = safe_get(&action_hash, &"-a".to_string());
    let mut host         = safe_get(&action_hash, &"-h".to_string());
    let mut port         = safe_get(&action_hash, &"-p".to_string());
    let mut api_key_file = safe_get(&action_hash, &"-f".to_string());
    let mut dry_run      = &action_hash.contains_key(&"--dry-run".to_string());
    let mut help         = &action_hash.contains_key(&"--help"   .to_string());
    //
    // TODO need to set json data body set too...
    //
    if help == &true {
        println!("kogeki --help ... help");
        println!("kogeki -a [action] -h [URL] -p [PORT] -f [API-KEY-FILE-PATH]");
        println!("\nactions:");
        println!("ce: create employee");
        println!("le: list employees");
        println!("");
        return;
    }

    // if host missing http:// it should be complemented
    // if it starts from https:// needs to replace it with http 
    println!("url:          {}:{}", host, port);
    println!("api-key-file: {}", api_key_file);
    request(format!("http://{}:{}/api/auth/token", host, port));
}

//
// using curl module, request to the target with parameters...
//
fn request(url: String) {
    let mut handle = Easy::new();
    let mut data   = Vec::new();
    handle.url(&*url).unwrap();
    handle.post(true).unwrap();
    handle.perform().unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|into| {
            data.extend_from_slice(into);
            Ok(stdout().write(into).unwrap())
        }).unwrap();
        transfer.perform().unwrap();
    }
    println!("{:?}", String::from_utf8(data).unwrap());
    //println!("---> {:?}", x);
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
