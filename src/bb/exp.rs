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
use std::fs::{DirEntry, File};
use std::path::PathBuf;

use serde::Deserialize;
use elefren::Data;
use std::io::Write;

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

static EXP_CONFIG_EXAMPLE : &str = r#"
# An example exploder config (exp.toml)
enabled = false

[mastodon]
base = "http://instance_url"
client_id = "123"
client_secret = "secret"
redirect = "https://instance_redirect"
token = "ABCD1234"
"#;

#[derive(Deserialize)]
struct ExpConfig {
    enabled: bool,
    mastodon: Data,
}

pub fn write_exp_file(
        bb_dir: &PathBuf,
        exp_name: &str,
        mastodon_data: &Data) -> std::io::Result<()> {
    let exp_path = exp_path(bb_dir,exp_name);
    let mut file = File::create(&exp_path).unwrap();
    for line in EXP_CONFIG_EXAMPLE.lines() {
        let strings: Vec<&str> = line.split("=").collect();
        let new_line = match strings[0].trim() {
            "base" => format!("base = \"{}\"", mastodon_data.base),
            "client_id" => format!("client_id = \"{}\"", mastodon_data.client_id),
            "client_secret" => format!("client_secret = \"{}\"", mastodon_data.client_secret),
            "redirect" => format!("redirect = \"{}\"", mastodon_data.redirect),
            "token" => format!("token = \"{}\"", mastodon_data.token),
            _ => String::from(line)
        };
        file.write(format!("{}\n", new_line).as_ref());
    }
    println!("Exploder configuration file {} created", &exp_path.to_string_lossy());
    Ok(())
}

fn exp_path(bb_dir: &PathBuf, exp_name: &str) -> PathBuf {
    let mut exp_path = bb_dir.clone();
    exp_path.push(&exp_name);
    exp_path.push( "exp.toml");
    exp_path
}

#[cfg(test)]
mod tests {
    use super::*;
    use elefren::Mastodon;
    use std::borrow::{Cow, Borrow};
    use tempfile::Builder;

    #[test]
    fn test_deserialization() {
        let exp_config : ExpConfig = toml::from_str(EXP_CONFIG_EXAMPLE).unwrap();
        assert_eq!(exp_config.enabled,false);
    }

    #[test]
    fn test_write_exp_file() {
        let data = Data{
            base: Cow::from("https://foo"),
            client_id: Cow::from("bar"),
            client_secret: Cow::from("secret"),
            redirect: Cow::from("https://redirect"),
            token: Cow::from("123ABC")
        };
        let d = Builder::new().prefix("write_exp_file_test").tempdir().unwrap();
        let exp_path = exp_path(PathBuf::from(d.path()).borrow(), "test" );
        let parent = exp_path.parent().unwrap();
        fs::create_dir_all(parent);

        write_exp_file(PathBuf::from(d.path()).borrow(), "test", &data);

        let exp_file = fs::read_to_string( &exp_path ).unwrap();
        let exp_config : ExpConfig = toml::from_str(exp_file.as_ref()).unwrap();
        assert_eq!(exp_config.enabled,false);
        assert_eq!(exp_config.mastodon.base,"https://foo");
        assert_eq!(exp_config.mastodon.client_id,"bar");
        assert_eq!(exp_config.mastodon.client_secret,"secret");
        assert_eq!(exp_config.mastodon.redirect,"https://redirect");
        assert_eq!(exp_config.mastodon.token,"123ABC");
    }
}
