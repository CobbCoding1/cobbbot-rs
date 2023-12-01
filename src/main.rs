use rustirc::{Client, ClientErrors};

fn read_file(filename: &str) -> String { 
    let data = std::fs::read_to_string(filename).unwrap();
    data
}

fn main() {
    let password = read_file(".client");
    if let Ok(mut client) = Client::new("irc.twitch.tv", "6667", "cobbcoding") {
        client.auth("cobbbot", &password);
        client.join();

        loop {
            match client.read_message() {
                Ok(msg) => {
                    let commands: Vec<&str> = msg.message.split_whitespace().collect();
                    match commands[0] {
                        "!help" => {
                            client.private_message("helped");
                        },
                        "!say" => {
                            client.private_message(&commands[1..commands.len()].join(" "));
                        },
                        _ => {
                            if commands[0].chars().nth(0) == Some('!') {
                                client.private_message(&format!("Unknown command {}", commands[0]));
                            } 
                        },
                    }
                },
                Err(ClientErrors::ReadError) => panic!("Could not read from stream"),
                Err(_) => {},
            } 
        }
    } else {
        panic!("could not connect");
    }
}
