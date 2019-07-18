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

use std::fs;
use std::path::PathBuf;
use std::process;

use serde::Deserialize;

static CONFIG_EXAMPLE : &str = "\
#An example config file
repeat = false";

#[derive(Deserialize)]
pub struct Config {
    pub repeat: bool,
}

pub fn read_config(bb_dir: &PathBuf) -> Config {
    let config_path = config_path(bb_dir);
    let file_contents = fs::read_to_string(config_path).unwrap_or_else(|_err| {
        eprintln!("Unable to read bigbang.toml file in {}", bb_dir.to_string_lossy());
        process::exit(1);
    });

    let config: Config = toml::from_str(&*file_contents).unwrap_or_else(|_err| {
        eprintln!("bigbang.toml in {} cannot be parsed as TOML", bb_dir.to_string_lossy());
        process::exit(1)
    });

    config
}

pub fn write_config_file(bb_dir: &PathBuf) -> std::io::Result<()> {
    let config_path = config_path(bb_dir);
    fs::write(config_path,CONFIG_EXAMPLE)?;
    println!("Configuration file bigbang.toml written to {}", bb_dir.to_string_lossy());
    Ok(())
}

fn config_path(bb_dir: &PathBuf) -> PathBuf {
    let mut config_path = bb_dir.clone();
    config_path.push("bigbang.toml");
    config_path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialization() {
        let config: Config = toml::from_str(CONFIG_EXAMPLE).unwrap();
        assert_eq!(config.repeat,false);
    }
}
