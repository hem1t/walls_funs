mod settings;
mod wall_fetcher;

use settings::Settings;
use wall_fetcher::wallhaven::Wallhaven;
use wall_fetcher::ImageFetcher;
use resolve_path::PathResolveExt;

use std::sync::Arc;
use std::thread;
use websocket::OwnedMessage;

static WEBSOCKET_ADDRESS: &str = "127.0.0.1:7878";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Arc::new(
        Settings::from_file("~/.config/wall_funs/config.toml".resolve().to_path_buf())
            .expect("Couldn't read config file"),
    );
    let server = websocket::sync::Server::bind(WEBSOCKET_ADDRESS).unwrap();

    // websocket handler spawner
    for connection in server.filter_map(Result::ok) {
        let settings = settings.clone();

        thread::spawn(move || {
            let client = connection.accept().unwrap();
            println!("{:?}", client.peer_addr());
            let (mut recv, mut sender) = client.split().unwrap();

            for msg in recv.incoming_messages() {
                match msg.unwrap() {
                    OwnedMessage::Text(text) => {
                        println!("{}", text);
                        if text.starts_with("url by setwall:") {
                            println!("{:?}", text.split_once(":").unwrap_or_default().1);
                        } else if text.starts_with("wallhaven url:") {
                            Wallhaven::new(&settings, text.split_once(':').unwrap_or_default().1).unwrap().download_image(settings.dir_path().to_owned());
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

