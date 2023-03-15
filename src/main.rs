use std::collections::{HashMap, HashSet};
use std::env;
use regex::Regex;
use evalexpr::*;

// (!(A and B or A) and (A or B (A or B)) and (A or B)) or (A and B)
//    ^     ^    ^       ^    ^  ^    ^        ^    ^       ^     ^
fn find_tokens(input: &str) -> Vec<String> {
    let regex_token = Regex::new(r"[A-Z]+").unwrap();
    let mut token_set = HashSet::new();
    for value in regex_token.find_iter(input) {
        let token = value.as_str().to_string();
        if !token_set.contains(&token) {
            token_set.insert(token);
        }
    }
    let mut token_vec = Vec::new();
    for token in token_set {
        token_vec.push(token);
    }
    token_vec.sort();
    token_vec
}

fn substitute_tokens(input: &str, token: &str, value: usize) -> String {
    let mut output = input.to_string();
    output = output.replace(&token, &*(value & 1 == 1).to_string());
    output
}

fn main() {
    let input_string = env::args().nth(1).unwrap();
    println!("Input: {}", input_string);
    let tokens = find_tokens(&input_string);
    let mut token_map = HashMap::new();
    let mut longest_token: usize = 0;
    for token in &tokens {
        token_map.insert(token, false);
        if token.len() > longest_token {
            longest_token = token.len();
        }
    }
    //┌──┬─────────┐
    let top_bar = {
        let mut top_bar = String::new();
        top_bar.push_str("┌");
        for _ in 0..tokens.len() {
            top_bar.push_str("─".repeat(longest_token).as_str());
            top_bar.push_str("──┬");
        }
        top_bar.push_str("─".repeat(longest_token).as_str());
        top_bar.push_str("──┐");
        top_bar
    };
    //├──┼─────────┤
    let middle_bar = {
        let mut middle_bar = String::new();
        middle_bar.push_str("├");
        for _ in 0..tokens.len() {
            middle_bar.push_str("─".repeat(longest_token).as_str());
            middle_bar.push_str("──┼");
        }
        middle_bar.push_str("─".repeat(longest_token).as_str());
        middle_bar.push_str("──┤");
        middle_bar
    };
    //└──┴─────────┘
    let bottom_bar = {
        let mut bottom_bar = String::new();
        bottom_bar.push_str("└");
        for _ in 0..tokens.len() {
            bottom_bar.push_str("─".repeat(longest_token).as_str());
            bottom_bar.push_str("──┴");
        }
        bottom_bar.push_str("─".repeat(longest_token).as_str());
        bottom_bar.push_str("──┘");
        bottom_bar
    };
    println!("{}", top_bar);
    for token in &tokens {
        print!("│");
        print!(" {:^1$} ", token, longest_token);
        for _ in 0..token.len()-longest_token {
            print!(" ");
        }
    }
    print!("│");
    print!(" ={} │\n", " ".repeat(longest_token-1));

    for i in 0..tokens.len()*tokens.len()-1 {
        let mut expression = input_string.clone();
        let mut j = 0;
        for token in &tokens {
            expression = substitute_tokens(&expression, token, i >> j);
            j += 1;
        }
        let res = eval_boolean(&expression);
        let res = res.unwrap();
        println!("{}", middle_bar);
        j = 0;
        for token in &tokens {
            print!("│");
            print!(" {:^1$} ", if (i >> j) & 1 == 1 {"1"} else {"0"}, longest_token);
            for _ in 0..token.len()-longest_token {
                print!(" ");
            }
            j = j + 1;
        }
        print!("│");
        print!(" {} │\n", if res {"1"} else {"0"});
    }

    println!("{}", bottom_bar);
}
