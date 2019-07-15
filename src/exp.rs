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

pub fn find_exploders( bb_dir: &str ) {
    let path = PathBuf::from(bb_dir);

    fs::read_dir(path).unwrap()
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
