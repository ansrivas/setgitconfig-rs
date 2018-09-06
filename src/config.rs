// MIT License
//
// Copyright (c) 2019 Ankur Srivastava
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use dirs;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use toml;

use crate::errors::SetGitConfigError;

const SETGITCONFIG_CONF: &str = "SETGITCONFIG_CONF";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RepositoryMap {
	pub email: String,
	pub username: String,
	pub giturl: String,
}

/// Structure to store the contents of config.toml
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
	pub repositories: Vec<RepositoryMap>,
}

/// Read the configuration from ~/.config/setgitconfig/config.toml
///
/// The location of default config.toml can be changed by a variable
/// export SETGITCONFIG_CONF=/my/custom/path/to/config.toml
/// set-git-config
///
pub fn read() -> Result<Configuration, SetGitConfigError> {
	let path_to_config = match env::var(SETGITCONFIG_CONF) {
		Ok(val) => val,
		Err(_) => {
			let config_dir = dirs::config_dir().expect("Please create ~/.config directory");
			let pathconf = config_dir.join("setgitconfig").join("config.toml");
			pathconf.to_str().unwrap().to_owned()
		}
	};

	let content = fs::read_to_string(path_to_config)?;
	let configuration: Configuration = toml::from_str(&content)?;
	Ok(configuration)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_config() {
		let toml_str = r#"
        [[repositories]]
        username= "Hakuna Matata"
        email = "hakunamatata@gmail.com"
        giturl = "github.com"
    "#;
		let decoded: Configuration = toml::from_str(toml_str).unwrap();
		for conf in decoded.repositories {
			assert_eq!(conf.email, "hakunamatata@gmail.com");
			assert_eq!(conf.giturl, "github.com");
			assert_eq!(conf.username, "Hakuna Matata");
		}
	}
}
