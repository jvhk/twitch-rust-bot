use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::TwitchIRCClient;
use twitch_irc::SecureTCPTransport;
use dotenv;
use std::fmt;
use twitch_irc::message::{IRCMessage, ServerMessage};
use std::convert::TryFrom;


#[tokio::main]
pub async fn main() {
    // default configuration is to join chat as anonymous.
    dotenv::from_filename("enviroment.env");

    let login_name = dotenv::var("USERNAME").unwrap();
    let oauth_token = dotenv::var("TWITCH_OAUTH_TOKEN").unwrap();

    let config = ClientConfig::new_simple(
        StaticLoginCredentials::new(login_name,Some(oauth_token)));

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.

    client.join("hk________".to_owned());

    
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
//            println!("Received message: {:?}", message:PrivmsgMessage);
//
   //           if PrivmsgMessage::message_text == "!ping" {
    //            client.say("hk________".to_owned(), "hello".to_owned()).await.unwrap();
      //      }
            let irc_message = IRCMessage::parse(":tmi.twitch.tv PING").unwrap();
            let server_message = ServerMessage::try_from(irc_message).unwrap();
            
            match server_message {
                //trata o PING da api da twitch
                ServerMessage::Ping{ .. } => println!("Got pinged"),
                rest => {
                    let irc_message = IRCMessage::from(rest);
                    if irc_message.command == "!hello"{
                        client.say("hk________".to_owned(), "Hello world!".to_owned()).await.unwrap();
                    }
                },
            }
        }
    });



    // join a channel
//    client.join("hk________".to_owned());

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.unwrap();
}
