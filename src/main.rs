/* Copyright 2016, Stephen Pearson

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.  */

mod common;
mod p1;
mod p2;
mod p3;

use std::env;

fn usage(args: Vec<String>) {
    println!("{} <problem> [arg1] ..", args[0]);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        match (&args[1]).parse::<i32>() {
            Ok(n) => {
                match n {
                    1 => { p1::init::run(args) }
                    2 => { p2::init::run(args) }
                    3 => { p3::init::run(args) }
                    _ => {
                        println!("Unknown problem number!");
                        usage(args);
                    }
                }
            }
            Err(_) => {
                println!("Not a valid problem number!");
                usage(args);
            }
        }
    } else {
        println!("Must specify a problem number!");
        usage(args);
    }
}
