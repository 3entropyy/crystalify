use futures_util::StreamExt;
use librespot::core::authentication::Credentials;
use librespot::core::config::SessionConfig;
use librespot::core::session::Session;
use librespot::discovery::DeviceType;
use std::env;
use std::io::{self, Write};

pub async fn authenticate() -> Result<Session, Box<dyn std::error::Error>> {
    let username = env::var("SPOTIFY_USERNAME").unwrap_or_default();
    let password = env::var("SPOTIFY_PASSWORD").unwrap_or_default();

    let credentials = if !username.is_empty() && !password.is_empty() {
        Credentials::with_password(username, password)
    } else {
        print!("Enter Spotify Username (or press Enter for Spotify Connect discovery): ");
        io::stdout().flush()?;
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;
        let user_input = user_input.trim().to_string();

        if user_input.is_empty() {
            println!("Launching Spotify Connect discovery as 'crystalify'...");
            println!("Open Spotify on your phone/PC and select 'crystalify' in available devices.");
            let mut discovery = librespot::discovery::Discovery::builder(
                SessionConfig::default().device_id,
                "crystalify".to_string(),
            )
            .name("crystalify")
            .device_type(DeviceType::Computer)
            .launch()?;

            match discovery.next().await {
                Some(creds) => creds,
                None => return Err("Discovery closed without credentials".into()),
            }
        } else {
            print!("Enter Spotify Password: ");
            io::stdout().flush()?;
            let mut pass_input = String::new();
            io::stdin().read_line(&mut pass_input)?;
            let pass_input = pass_input.trim().to_string();
            Credentials::with_password(user_input, pass_input)
        }
    };

    println!("Connecting to Spotify...");
    let session_config = SessionConfig::default();
    let session = Session::new(session_config, None);
    session.connect(credentials, true).await?;
    println!("Logged in as: {}", session.username());

    Ok(session)
}
