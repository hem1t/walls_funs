mod settings;
mod wall_fetcher;

use settings::Settings;

use std::ffi::OsStr;
use std::fs;
use std::os;
use std::path::Path;
use std::process;
use std::thread;
use websocket::OwnedMessage;

static WEBSOCKET_ADDRESS: &str = "127.0.0.1:7878";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::from_file("~/.config/wall_funs/config.toml".into());
    let server = websocket::sync::Server::bind(WEBSOCKET_ADDRESS).unwrap();

    // websocket handler spawner
    for connection in server.filter_map(Result::ok) {
        thread::spawn(|| {
            let client = connection.accept().unwrap();
            println!("{:?}", client.peer_addr());
            let (mut recv, mut sender) = client.split().unwrap();

            for msg in recv.incoming_messages() {
                match msg.unwrap() {
                    OwnedMessage::Text(text) => {
                        println!("{}", text);
                        if text.starts_with("url by setwall") {
                            handle_connection(text.split_whitespace().last().unwrap().to_string());
                        } else if text.starts_with("url by wallhere") {
                            // handle this later.
                        }
                    }
                    OwnedMessage::Close(_) => {
                        sender.send_message(&OwnedMessage::Close(None)).ok();
                        return;
                    }
                    _ => (),
                }
            }
        });
    }
    Ok(())
}


fn get_image(url: String) {
    println!("in get image {:?}", url);
    let name = url.split('/').last().unwrap();
    let original = format!("/home/hem1t/Pictures/{}", name);

    // check if image is in ~/Pictures/ if not then download or set the wall.
    if !Path::new(&original).exists() {
        notify("Downloading the wall");
        let res = reqwest::blocking::get(url.clone());

        if res.as_ref().is_ok_and(|data| data.status() == 200) {
            let data = res.unwrap().bytes().unwrap();
            fs::write(&original, data).expect("not sure original");
        } else {
            // notify(dbg!("failed to reqwest!"));
            return;
        }
    }

    // Setting WAll
    set_wall(&original);
    notify(dbg!(format!("Should be done!, {}", name)));
}

fn set_wall(original: &String) {
    let wall_daemon = set_wall_nitrogen(original);
    set_wall_ww(original);
}

fn set_wall_nitrogen(original: &String) {
    process::Command::new("nitrogen")
        .arg("--set-auto")
        .arg("--save")
        .arg(original)
        .spawn()
        .expect("feh failed.");
}

fn set_wall_ww(original: &String) {
    let file = "/home/hem1t/.config/awesome/themes/night/ww.";
    fs::remove_file(format!("{file}jpg")).expect("JPG remove");
    fs::remove_file(format!("{file}png")).expect("PNG remove");
    os::unix::fs::symlink(original, format!("{file}jpg")).expect("link1");
    os::unix::fs::symlink(original, format!("{file}png")).expect("link2");
}

fn notify<S: AsRef<OsStr>>(msg: S) {
    process::Command::new("notify-send")
        .arg(msg)
        .spawn()
        .expect("notify");
}
