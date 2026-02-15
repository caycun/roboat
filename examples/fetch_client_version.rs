use clap::Parser;
use roboat::ClientBuilder;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short, default_value = "WindowsPlayer")]
    binary_type: String,
    #[arg(long, short)]
    channel: Option<String>,
    #[arg(long, short)]
    roblosecurity: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let binary_type = args.binary_type;
    let client = ClientBuilder::new()
        .roblosecurity(args.roblosecurity)
        .build();

    let version_response = if let Some(channel) = args.channel {
        client
            .client_version_for_channel(binary_type.clone(), channel)
            .await?
    } else {
        client.client_version(binary_type.clone()).await?
    };

    println!("Version: {}", version_response.version);
    println!("Version Upload: {}", version_response.upload);

    let user_channel = client.user_channel(Some(binary_type)).await?;
    println!("User Channel: {}", user_channel.name);

    Ok(())
}
