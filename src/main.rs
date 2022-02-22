use clap::Parser;
use log::error;
use paho_mqtt as mqtt;
use std::process;

// Search for a pattern in file and display the lines that contain it.
#[derive(Parser)]
struct Args {
    #[clap(default_value = "tcp://localhost:1883")]
    broker: String,
}

fn main() {
    // Initialize the logger from the environment
    env_logger::init();

    let args = Args::parse();
    // Create a client & define connect options
    let cli = mqtt::Client::new(args.broker).unwrap_or_else(|e| {
        error!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    // Connect and wait for it to complete or fail
    if let Err(e) = cli.connect(None) {
        error!("Unable to connect: {:?}", e);
        process::exit(1);
    }

    if !cli.is_connected() {
        error!("Not currently connected to an MQTT broker");
        process::exit(1);
    }
    println!("pong");

    // Disconnect from the broker
    cli.disconnect(None).unwrap();
}
