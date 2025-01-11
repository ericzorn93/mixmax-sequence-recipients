extern crate dotenv;

use dotenv::dotenv;
use std::{env, error::Error};

use mixmax_csv_uploader::{
    http::{self, Empty, SendRequest},
    parse_people, Recipient, RecipientVariables, RequestBody,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let mut reader = csv::Reader::from_path("data.csv")?;
    let people = parse_people(&mut reader)?;
    println!("Length of people is {}", people.len());

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

    // Environment variables
    let api_key = env::var("MIXMAX_API_KEY").expect("MIXMAX API Key expected to be present");
    let sequence_id = env::var("MIXMAX_SEQUENCE_ID").expect("MIXMAX Sequence ID is expected");

    // Construct URL for Mix Max Sequence receipients
    let full_url =
        mixmax_csv_uploader::MIX_MAX_SEQUENCE_RECIPIENTS_URL.replace(":id", &sequence_id);
    println!("Full URL: {full_url}");

    // Send request via HTTP to MixMax
    let sender = http::Requester::<Empty>::new(api_key, full_url);
    let res_body = sender.send_request(body).await;
    println!("{res_body:?}");

    Ok(())
}
