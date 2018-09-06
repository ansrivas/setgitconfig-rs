
[![Build Status](https://travis-ci.com/ansrivas/setgitconfig-rs.svg?token=hM6V8mr5fwQPseiFSYVi&branch=master)](https://travis-ci.com/ansrivas/setgitconfig-rs)
[![Tag](https://img.shields.io/github/tag/ansrivas/setgitconfig-rs.svg)](https://github.com/ansrivas/setgitconfig-rs/releases/latest)

### setgitconfig-rs

Sets correct username and useremail for your git-url based on a configuration file.

### Use case:

We work on several repositories on several repositories. Sometime our username and emails are different
and we need to set the git config for each of them otherwise the repositories do not detect it.

This binary solves exactly that, based on a simple toml file.

### Configuration:

- Install the binary using
  ``` cargo install setgitconfig-rs```
- Put this in your `~/.bashrc`

  `PROMPT_COMMAND="/usr/local/bin/setgitconfig; $PROMPT_COMMAND"`

- Just create a directory in your user's home

  `mkdir -p ~/.config/setgitconfig`

- Create a file like this:

  `touch ~/.config/setgitconfig/config.toml`

- Content of the file can look like this

  ```
  $ cat ~/.config/setgitconfig/config.toml

  [[repositories]]
  username= "Ankur Srivastava"
  email = "myuser@xmail.com"
  giturl = "github.com"
  [[repositories]]
  username= "Ankur Srivastava"
  email = "myuserfor-github@xmail.com"
  giturl = "gitlab.com"
  ```
