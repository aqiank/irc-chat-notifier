#[macro_use]
extern crate log;

use clap::{Arg,App};
use irc::client::prelude::*;

fn main() {
    env_logger::init();

    let matches = App::new("Desktop Chat Notifier")
                        .version("0.1.0")
                        .author("Jacky Boen <jacky@veand.co>")
                        .about("Sends desktop notification when someone chats on the channel")
                        .arg(Arg::with_name("nick")
                            .short("n")
                            .long("nick")
                            .help("Sets the IRC nickname (e.g. john_doe)")
                            .takes_value(true)
                            .required(true))
                        .arg(Arg::with_name("server")
                            .short("s")
                            .long("server")
                            .help("Sets the IRC server (e.g. irc.freenode.org)")
                            .takes_value(true)
                            .required(true))
                        .arg(Arg::with_name("password")
                            .short("p")
                            .long("password")
                            .help("Sets the IRC password (e.g. abcd1234)")
                            .takes_value(true)
                            .required(true))
                        .arg(Arg::with_name("channel")
                            .short("c")
                            .long("channel")
                            .help("Sets the IRC channel (e.g. #linux)")
                            .takes_value(true)
                            .required(true))
                        .arg(Arg::with_name("ignore-nick")
                            .long("ignore-nick")
                            .help("Ignore messages from specified nickname")
                            .multiple(true)
                            .takes_value(true))
                        .get_matches();

    let config = Config{
        nickname: Some(matches.value_of("nick").unwrap().to_owned()),
        server: Some(matches.value_of("server").unwrap().to_owned()),
        password: Some(matches.value_of("password").unwrap().to_owned()),
        ..Default::default()
    };
    let client = IrcClient::from_config(config).unwrap();
    client.identify().unwrap();

    info!("Joining channel m4ch");
    client.send(Command::JOIN(matches.value_of("channel").unwrap().to_owned(), None, None)).unwrap();

    let ignored_nicks: Vec<&str> = match matches.values_of("ignore-nick") {
        Some(values) => values.map(|v| v).collect(),
        None => vec![],
    };

    info!("Ignoring {:?}", ignored_nicks);

    info!("Listening for incoming messages");
    client.for_each_incoming(|irc_msg| {
        if let Command::PRIVMSG(_channel, message) = &irc_msg.command {
            let source_nickname = match irc_msg.source_nickname() {
                Some(nickname) => nickname,
                None => "unknown"
            };

            if ignored_nicks.contains(&source_nickname) {
                return;
            }

            let formatted_message = format!("{}: {}", source_nickname, &message);
            info!("{}", &formatted_message);
            notifica::notify("IRC", &formatted_message);
        }
    }).unwrap();
}