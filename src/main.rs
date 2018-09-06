use std::env;
use std::process::Command;

fn execute_cmd<T>(cmd: T, args: &[&str])
where
    T: Into<String>,
{
    Command::new(cmd.into())
        .args(args)
        .spawn()
        .expect("failed to execute process");
}

fn set_gitconfig() {
    let username = &vec!["config", "user.name", "Ankur Srivastava"];
    let reco_email = &vec!["config", "user.email", "ankur.srivastava@recogizer.de"];
    let personal_email = &vec!["config", "user.email", "best.ankur@gmail.com"];
    let git_cmd = "git";

    // We assume that we are in a valid directory.
    let path = env::current_dir().expect("This is my current working directory");
    let absolute_path = path
        .to_str()
        .expect("Failed to get current working dir")
        .to_lowercase();

    if absolute_path.contains("reco") {
        println!("Recogizer workspace");
        execute_cmd(git_cmd, &username);
        execute_cmd(git_cmd, &reco_email);
    }

    if absolute_path.contains("ankur") {
        println!("Ankur workspace");
        execute_cmd(git_cmd, &personal_email);
        execute_cmd(git_cmd, &username);
    }
}

fn main() {
    set_gitconfig()
}
