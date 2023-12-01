use rustirc::{Client, ClientErrors};

const TODAY: &str = "/home/ic/.config/cobbbot/todaydata";
const CLIENT: &str = "/home/ic/.config/cobbbot/.client";


fn read_file(filename: &str) -> String { 
    let data = std::fs::read_to_string(filename).unwrap();
    data
}

fn handle_commands(client: &mut Client, commands: Vec<&str>, help_string: &str) {
    match commands[0] {
        "!help" => {
            client.private_message(help_string);
        },
        "!ping" => {
            client.private_message("PONG!");
        },
        "!today" => {
            let data = read_file(TODAY);
            client.private_message(&data);
        },
        "!socials" => {
            client.private_message(
            "X (Twitter): https://x.com/cobbcoding, YouTube: https://youtube.com/@cobbcoding, GitHub: https://github.com/CobbCoding1"
            );
        },
        "!yt" => {
            client.private_message("https://youtube.com/@cobbcoding");
        },
        "!discord" => {
            client.private_message("https://discord.gg/3SkpwrRxpA");
        },
        "!sub" => {
            client.private_message("https://www.twitch.tv/subs/cobbcoding");
        },
        "!69" => {
            client.private_message("nice");
        },
        "!nice" => {
            client.private_message("69");
        },
        "!specs" => {
            client.private_message("specs: i5 2400, 12GB RAM, Radeon R7 260X. Arch Linux BTW");
        },
        "!surprise" => {
            client.private_message("Enjoy your surpise: https://www.youtube.com/watch?v=xvFZjo5PgG0");
        },
        "!time" => {
            let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
            let time_str = time.to_string();
            let actual_time = &time_str[0..(time_str.len() - 3)];
            let output = std::process::Command::new("date").arg(format!("-d @{}", actual_time)).output().expect("Could not execute command");
            client.private_message(&format!("{}", String::from_utf8_lossy(&output.stdout)));
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
}

fn main() {
    let help: Vec<(&str, &str)> = Vec::from([
        ("help", "Display help message"),
        ("ping", "PONG"),
        ("today", "List the current project for today"),
        ("socials", "List all my social media accounts"),
        ("yt", "Get URL for my YouTube channel"),
        ("surprise", "Run for a fun surpise"),
        ("specs", "List PC specs and OS information"),
        ("say", "Make the bot say something"),
        ("time", "List the current time in the streamers' area"),
        ("sub", "to subscribe"),
        ("69", "nice"),
        ("nice", "69"),
        ("discord", "discord"),
    ]);

    let mut help_string = String::new();
    for (command, desc) in &help {
        help_string.push_str(&format!("{}: {} KAPOW ", command, desc));
    }

    let password = read_file(CLIENT);
    if let Ok(mut client) = Client::new("irc.twitch.tv", "6667", "cobbcoding") {
        client.auth("cobbbot", &password);
        client.join();

        loop {
            match client.read_message() {
                Ok(msg) => {
                    let commands: Vec<&str> = msg.message.split_whitespace().collect();
                    handle_commands(&mut client, commands, &help_string);
                },
                Err(ClientErrors::ReadError) => panic!("Could not read from stream"),
                Err(_) => {},
            } 
        }
    } else {
        panic!("could not connect");
    }
}
