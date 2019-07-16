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

use clap::{App, Arg};
use flexi_logger::Logger;
use log::*;

mod bb;

use bb::exp;
use bb::config;

fn main() {
    let matches = App::new("BigBang")
        .about("A Fediverse Exploder")
        .version("0.1.0")
        .author("@rcode3@masto.rootdc.xyz")
        .arg(
            Arg::with_name("BB_DIR")
                .help("BigBang configuration and working directory")
                .required(true)
                .index(1),
        )
        .get_matches();

    let bb_dir = matches.value_of("BB_DIR").unwrap();

    let mut log_dir = PathBuf::from(bb_dir);
    log_dir.push("_logs");

    Logger::with_env_or_str("bigbang=info")
        .log_to_file()
        .directory(log_dir.as_os_str())
        .start()
        .unwrap();

    info!("Starting Big Bang using directory {}", bb_dir);
    let config = config::read_config(bb_dir);
    println!("repeat = {}", config.repeat);
    exp::find_exploders(bb_dir);
}

