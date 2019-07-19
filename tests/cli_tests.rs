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

use std::io::*;
use std::process::Command;

use assert_cmd::prelude::*;
use tempfile::Builder;
use std::path::{Path, PathBuf};

mod test_utils;

#[test]
fn test_test() {

    let d = test_utils::project_path();
    eprintln!("project directory {}",d.display());

    let d2 = test_utils::project_test_resources();
    eprintln!("test resources in {}", d2.display());
    assert_eq!(4,4);
}

#[test]
fn new_directory_test() -> Result<()> {
    let d = Builder::new().prefix("new_directory_test").tempdir()?;

    let mut cmd = Command::cargo_bin("bigbang").unwrap();
    cmd.arg("-d")
        .arg(d.path())
        .arg("new-directory");
    let status = cmd.status().unwrap();
    assert!(status.success());

    let mut p = PathBuf::from( d.path() );
    p.push("bigbang.toml");
    assert!(p.is_file());

    Ok(())
}