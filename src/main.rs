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

#[macro_use]
extern crate log;

use env_logger;
use git2::{Config as GitConfig, Repository};
use std::env;
use std::path::Path;
mod config;
mod errors;
use crate::errors::SetGitConfigError;

/// Set the correct username and email based on the giturl
///
/// The location of default config.toml can be changed by a variable
/// export SETGITCONFIG_CONF=/my/custom/path/to/config.toml
/// set-git-config
///
fn set_gitconfig() -> Result<(), SetGitConfigError> {
	// We assume that we are in a valid directory.
	let path = env::current_dir()?;
	let absolute_path = path
		.to_str()
		.ok_or_else(|| errors::SetGitConfigError::PathError)?
		.to_lowercase();
	let repo = Repository::open(&Path::new(&absolute_path))?;
	let remote = repo.find_remote("origin")?;
	let url = remote.url().ok_or(git2::Error::from_str("failed"))?;
	let mut gitconfig = GitConfig::open_default()?;

	let configuration = config::read().expect("Unable to read the config file for setgitconfig");
	for conf in configuration.repositories {
		gitconfig.set_str("user.name", &conf.username)?;
		if url.contains(&conf.giturl) {
			gitconfig.set_str("user.email", &conf.email)?;
		}
	}
	Ok(())
}

fn main() -> Result<(), SetGitConfigError> {
	env_logger::init();
	debug!("starting up");
	match set_gitconfig() {
		Ok(_) => Ok(()),
		Err(error) => {
			debug!("{}", error);
			Ok(())
		}
	}
}
