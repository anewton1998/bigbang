/*
Copyright (C) 2019 Andrew Newton

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

extern crate clap;
extern crate serde;
extern crate toml;

use std::path::PathBuf;
use std::env;
use std::process;

use clap::{App, Arg, SubCommand};
use flexi_logger::Logger;
use log::*;

use elefren::prelude::Registration;

mod bb;

use bb::exp;
use bb::config;

fn main() {
    let matches = App::new("BigBang")
        .about("A Fediverse Exploder")
        .version("0.1.0")
        .author("@rcode3@masto.rootdc.xyz")
        .arg(
            Arg::with_name("directory")
                .short("d")
                .long("directory")
                .value_name("DIRECTORY")
                .help("Configuration and Working directory")
                .takes_value(true)
        )
        .subcommand(
            SubCommand::with_name("new-directory")
                .about("Sets up a new configuration and working directory")
        )
        .subcommand(
            SubCommand::with_name("new-exploder")
                .about("Sets up a new exploder")
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .value_name("NAME")
                        .help("Name of the exploder")
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("instance url")
                        .short("i")
                        .long("instance")
                        .value_name("INSTANCE URL")
                        .help("Mastodon Instance")
                        .required(true)
                        .takes_value(true)
                )
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs exploders")
        )
        .get_matches();

    let bb_dir = match matches.value_of("directory") {
        Some(value) => PathBuf::from(value),
        None => env::current_dir().unwrap(),
    };

    match matches.subcommand() {
        ("new-directory", Some(sub_m)) => new_directory(bb_dir),
        ("new-exploder", Some(sub_m)) => {
            new_exploder(bb_dir, sub_m.value_of("instance url").unwrap());
        },
        ("run", Some(sub_m)) => run(bb_dir),
        _ => {
            eprintln!("Not a valid subcommand");
            process::exit(1);
        }
    }
}

fn new_directory(bb_dir: PathBuf) {
    config::write_config_file(&bb_dir).unwrap();
}

fn new_exploder(bb_dir: PathBuf, instance_url: &str) {
    let registration = Registration::new(instance_url)
        .client_name("bigbang")
        .build();
    let url = registration.unwrap().authorize_url().unwrap();
    println!("\n\
To register BigBank with your Mastodon account, open the URL below in a browser: ");
    println!("\n{}\n", url );
    print!("\
Enter the authentication code to complete the registration: ");
}

fn run(bb_dir: PathBuf) {
    let mut log_dir = bb_dir.clone();
    log_dir.push("_logs");

    Logger::with_env_or_str("bigbang=info")
        .log_to_file()
        .directory(log_dir.as_os_str())
        .start()
        .unwrap();

    info!("Starting Big Bang using directory {}", bb_dir.to_string_lossy());
    let config = config::read_config(&bb_dir);
    println!("repeat = {}", config.repeat);
    exp::find_exploders(&bb_dir);
}

