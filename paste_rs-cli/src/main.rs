// Copyright 2022 Canvas02.
// SPDX-License-Identifier: MIT

// ! Used in development never turn on for a production build
// #![allow(dead_code)]
// #![allow(unused_variables)]
// ! enable for production build
#![deny(dead_code)]
#![deny(unused_variables)]

mod cli;

// TODO: create -o(--output) flag for `get` to output to a file

use crate::cli::{Cli, Commands};
use clap::Parser;
use paste_rs_api::Url;
use std::fs;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Get { val } => {
            let url = match Url::new(&*val) {
                Ok(val) => val,
                Err(err) => {
                    eprintln!("ERROR: {:#?}", err);
                    std::process::exit(-1);
                }
            };

            let res = match url.make_request().await {
                Ok(val) => val,
                Err(err) => {
                    eprintln!("ERROR: {:#?}", err);
                    std::process::exit(-1);
                }
            };

            // Cache work goes here

            println!(
                "====\t{}\t======================================================",
                url
            );
            println!("");
            println!("{}", res);
            println!("");
            println!("======================================================================================");

            std::process::exit(0);
        }
        Commands::New { file } => {
            let data = match fs::read_to_string(file) {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("ERROR: {:#?}", err);
                    std::process::exit(-1);
                }
            };

            let url = match paste_rs_api::new_paste(data).await {
                Ok(res) => res,
                Err(err) => {
                    eprintln!("ERROR: {:#?}", err);
                    std::process::exit(-1);
                }
            };

            println!("Successfully created new paste at {}", url);
        }
    }
}

// => A main for testing
// #[tokio::main]
// async fn main() {
//     let mut url = match crate::paste::Url::new("osx") {
//         Ok(val) => val,
//         Err(err) => {
//             eprintln!("Error: {:#?}", err);
//             std::process::exit(-1);
//         }
//     };

//     let id = url.get_id();
//     dbg!(id);
// }
