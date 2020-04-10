use mpd_client::{Client, Command, Frame, Subsystem};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use tokio::net::TcpStream;
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let connection = TcpStream::connect("localhost:6600").await?;

    // The client is used to issue commands, and state_changes is an async stream of state change
    // notifications
    let (client, mut state_changes) = Client::connect(connection).await?;

    // // Shorthand for the above behavior
    // let (client, mut state_changes) = Client::connect_to("localhost:6600").await?;

    // // Shorthand for connecting to unix socket
    // let (client, mut state_changes) = Client::connect_unix("/run/user/1000/mpd").await?;

    // Get the song playing right as we connect
    let initial = client.command(Command::new("currentsong")).await?;
    print_current_song(initial);

    // Wait for state change notifications being emitted by MPD
    while let Some(subsys) = state_changes.next().await {
        let subsys = subsys?;

        if subsys == Subsystem::Player {
            let current = client.command(Command::new("currentsong")).await?;
            print_current_song(current);
        }
    }

    Ok(())
}

fn print_current_song(response: Frame) {
    if response.is_empty() {
        println!("(none)");
    } else {
        println!(
            "\"{}\" by \"{}\"",
            response.find("Title").unwrap_or("(no title"),
            response.find("Artist").unwrap_or("(no artist)")
        );
    }
}
