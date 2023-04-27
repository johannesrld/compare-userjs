// Copyright (c) 2023 Johannes Richard Levi Dickenson
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, env, fs, hash::Hash};
fn main()
{
    let args: Vec<String> = env::args().collect();
    let userjs_file_1 = read_jsvars(&args[1].to_string());
    let userjs_file_2 = read_jsvars(&args[2].to_string());
    let mut userjs_1_diff: HashMap<String, String> = HashMap::new();
    let mut userjs_2_diff: HashMap<String, String> = HashMap::new();

    for (confvar, value) in &userjs_file_1
    {
        if !userjs_file_2.contains_key(confvar) || userjs_file_2.get(confvar) != Some(value)
        {
            userjs_1_diff.insert(confvar.clone(), value.clone());
        }
    }

    for (confvar, value) in &userjs_file_2
    {
        if !userjs_file_1.contains_key(confvar) || userjs_file_1.get(confvar) != Some(value)
        {
            userjs_2_diff.insert(confvar.clone(), value.clone());
        }
    }

    println!("----------------------------------");
    for (confvar, value) in &userjs_1_diff
    {
        if userjs_2_diff.contains_key(confvar)
        {
            let userjs_2_val = userjs_2_diff.get(confvar).unwrap();
            println!("File 1: {confvar} = {value}");
            println!("File 2: {confvar} = {userjs_2_val}");
            println!("----------------------------------");
        }
    }
    // I would much prefer to use the drain_filter method in the previous statement but filter will do for now until drain_filter is considered stable
    println!("File 1's Unique confvars:");
    for (confvar, value) in userjs_1_diff.iter().filter(|(confvar, _)| !userjs_2_diff.contains_key(&**confvar))
    {
        println!("{confvar} = {value}")
    }

    println!("\nFile 2's Unique confvars");
    for (confvar, value) in userjs_2_diff.iter().filter(|(confvar, _)| !userjs_1_diff.contains_key(&**confvar))
    {
        println!("{confvar} = {value}");
    }
}

fn read_jsvars(file_name: &String) -> HashMap<String, String>
{
    lazy_static!
    {
        static ref RE: Regex = Regex::new(r#"(?m)^user_pref\("(.+)",(?:\s"?)(.*?)"?\)"#).unwrap();
    }
    let file = fs::read_to_string(file_name).expect("Failed to read file");
    let conf_vec: HashMap<String, String> = RE
        .captures_iter(&file)
        .filter_map(|regex_capture| Some((regex_capture[1].to_string(), regex_capture[2].to_string())))
        .collect();
    return conf_vec;
}
