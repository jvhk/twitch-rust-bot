use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::TwitchIRCClient;
use twitch_irc::SecureTCPTransport;
//use std::env;
use dotenv;

//pub struct CredentialsPair{//hk -> oauth token(OAuth access token, without leading oauth: prefix.)

/* nao sei se vou usar ainda

pub struct ClientConfig<L:LoginCredentials>{
    pub login_credentials: L,
}
*/

//dotenv::from_filename("enviroment.env").ok();

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
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            println!("Received message: {:?}", message);
        }
    });

    // join a channel
    client.join("hk________".to_owned());

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.unwrap();
}
