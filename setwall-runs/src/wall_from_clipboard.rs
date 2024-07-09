use std::{
    env,
    error::Error,
    path::Path,
    process::{self, Command},
};

use cli_clipboard as clip;

fn main() -> Result<(), Box<dyn Error>> {
    let path = if env::args().len() > 1 {
        env::args().nth(1).unwrap()
    } else {
        clip::get_contents()?
    };

    if Path::new(&path).exists() {
        set_wall_nitrogen(path);
    } else {
        notify("unknown clip!", format!("Unknown Path! {}", path))
    }

    std::process::exit(0);
}

fn set_wall_nitrogen(original: String) {
    process::Command::new("nitrogen")
        .arg("--set-auto")
        .arg("--save")
        .arg(original)
        .spawn()
        .expect("feh failed.");
}

fn notify(s: &str, path: String) {
    Command::new("notify-send").arg(s).spawn().unwrap();
    eprintln!("{:?}", path);
}
