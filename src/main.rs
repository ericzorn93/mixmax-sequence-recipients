extern crate dotenv;

use clap::Parser;
use dotenv::dotenv;
use std::{env, error::Error};

use mixmax_csv_uploader::{
    cli::Args,
    errors::CLIError,
    http::{self, Empty, SendRequest},
    parse_people, Recipient, RecipientVariables, RequestBody,
};

/// Will read in and parse the CSV,
/// send request to MixMax and return the
/// corresponding errors (if present)
async fn run() -> Result<(), CLIError> {
    // Parse CLI Arguments
    let args = Args::parse();
    if !args.is_valid() {
        return Err(CLIError::InvalidCLIArgs);
    }

    let reader = csv::Reader::from_path(&args.file_path).map_err(|_| CLIError::FileRead)?;
    let people = parse_people(reader).map_err(|_| CLIError::ParseCSV)?;
    println!("Length of people is {}", people.len());

    // Environment variables
    let api_key = env::var("MIXMAX_API_KEY").expect("MIXMAX API Key expected to be present");

    // Construct URL for Mix Max Sequence receipients
    let full_url =
        mixmax_csv_uploader::MIX_MAX_SEQUENCE_RECIPIENTS_URL.replace(":id", &args.sequence_id);
    println!("Full URL: {full_url}");

    // Send request via HTTP to MixMax
    let sender = http::Requester::<Empty>::new(api_key, full_url);
    let body = RequestBody {
        recipients: people
            .iter()
            .map(|p| Recipient {
                email_address: p.email_address.clone(),
                variables: RecipientVariables::from(p.clone()),
            })
            .collect::<Vec<Recipient>>(),
        scheduled_at: false,
    };
    let res_body = sender
        .send_request(body)
        .await
        .map_err(|_| CLIError::HTTP)?;
    println!("{res_body:?}");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // Run Program
    if let Err(e) = run().await {
        eprintln!("{e}");
    }

    Ok(())
}
