extern crate dotenv;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{env, error::Error, fs::File};

const MIX_MAX_SEQUENCE_RECIPIENTS_URL: &str = "https://api.mixmax.com/v1/sequences/:id/recipients";

#[derive(Clone, Deserialize, Debug)]
struct Person {
    #[serde(rename = "First Name")]
    first_name: String,

    #[serde(rename = "Last Name")]
    last_name: String,

    #[serde(rename = "Company Name")]
    company_name: String,

    #[serde(rename = "Job Title")]
    job_title: String,

    #[serde(rename = "Email")]
    email_address: String,

    #[serde(rename = "LinkedIn URL")]
    linkedin_url: String,
}

#[derive(Debug, Clone, Serialize)]
struct RequestBody {
    recipients: Vec<Recipient>,
    #[serde(rename = "scheduledAt")]
    scheduled_at: bool,
}

#[derive(Debug, Clone, Serialize)]
struct Recipient {
    #[serde(rename = "email")]
    email_address: String,
    #[serde(rename = "variables")]
    variables: RecipientVariables,
}

#[derive(Debug, Clone, Serialize)]
struct RecipientVariables {
    #[serde(rename = "First Name")]
    first_name: String,

    #[serde(rename = "Last Name")]
    last_name: String,

    #[serde(rename = "Company Name")]
    company_name: String,

    #[serde(rename = "Job Title")]
    job_title: String,

    #[serde(rename = "Email")]
    email_address: String,

    #[serde(rename = "LinkedIn URL")]
    linkedin_url: String,
}

impl From<Person> for RecipientVariables {
    fn from(person: Person) -> Self {
        RecipientVariables {
            first_name: person.first_name,
            last_name: person.last_name,
            company_name: person.company_name,
            job_title: person.job_title,
            email_address: person.email_address,
            linkedin_url: person.linkedin_url,
        }
    }
}

fn parse_people(csv_reader: &mut csv::Reader<File>) -> Result<Vec<Person>, Box<dyn Error>> {
    let people: Vec<Person> = csv_reader.deserialize().collect::<Result<_, _>>()?;
    let people = people
        .into_iter()
        .filter(|p| {
            let cleaned_email = p
                .email_address
                .chars()
                .filter(|&c| c != '\u{2060}')
                .collect::<String>();

            !cleaned_email.is_empty()
        })
        .collect::<Vec<Person>>();

    Ok(people)
}

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
    let full_url = MIX_MAX_SEQUENCE_RECIPIENTS_URL.replace(":id", &sequence_id);
    println!("Full URL: {full_url}");

    // Send Request
    let client = reqwest::Client::new();
    let res = client
        .post(full_url)
        .header("X-API-TOKEN", api_key)
        .json(&body)
        .send()
        .await?;

    println!("The status is {}", res.status());

    let res_body: Value = res.json().await?;
    println!("{res_body:?}");

    Ok(())
}
