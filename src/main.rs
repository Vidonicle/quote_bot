use std::env;

fn main() {
    // Load ,env, dev side
    dotenvy::dotenv().ok();

    // Read token from enviromnent
    let token: String = env::var("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN not set");

    // Print
    println!("DISCORD_TOKEN = {token}")
}
