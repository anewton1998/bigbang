// Copyright (C) 2019 Andrew Newton

extern crate clap;
extern crate serde;
extern crate toml;

use clap::{App, Arg};
use flexi_logger::Logger;
use log::*;
use serde::Deserialize;

use std::fs;
use std::path::PathBuf;
use std::process;

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

    info!("Starting Big Bang!");
    let config = read_config(bb_dir);
    println!("repeat = {}", config.repeat);
}

#[derive(Deserialize)]
struct Config {
    repeat: bool,
}

fn read_config(bb_dir: &str) -> Config {
    let mut config_path = PathBuf::from(bb_dir);
    config_path.push("bigbang.toml");
    let file_contents = fs::read_to_string(config_path).unwrap_or_else(|_err| {
        eprintln!("Unable to read bigbang.toml file in {}", bb_dir);
        process::exit(1);
    });

    let config: Config = toml::from_str(&*file_contents).unwrap_or_else(|_err| {
        eprintln!("bigbang.toml in {} cannot be parsed as TOML", bb_dir);
        process::exit(1)
    });

    config
}
