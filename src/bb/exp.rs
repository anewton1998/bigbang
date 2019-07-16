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
use std::fs::DirEntry;
use std::path::PathBuf;

use serde::Deserialize;

pub fn find_exploders(bb_dir: &PathBuf) {
    fs::read_dir(bb_dir).unwrap()
        .filter_map(Result::ok)
        .filter(|p: &DirEntry| p.path().is_dir())
        .filter(|p: &DirEntry| ! p.file_name().to_string_lossy().starts_with("_"))
        .filter(|p: &DirEntry| {
            let mut exp_path = PathBuf::from( p.path() );
            exp_path.push("exp.toml");
            exp_path.is_file()
        })
        .for_each(|p| println!("{:?}",p));
}

static EXP_CONFIG_EXAMPLE : &str = "\
# An example exploder config (exp.toml)
mastodon_url = \"http://foo.example.com\"
enabled = false
";

#[derive(Deserialize)]
struct ExpConfig {
    enabled: bool,
    mastodon_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialization() {
        let exp_config : ExpConfig = toml::from_str(EXP_CONFIG_EXAMPLE).unwrap();
        assert_eq!(exp_config.enabled,false);
    }
}
