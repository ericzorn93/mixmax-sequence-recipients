pub mod cli;
pub mod errors;
pub mod http;

use std::{error::Error, fs::File};

use serde::{Deserialize, Serialize};

pub const MIX_MAX_SEQUENCE_RECIPIENTS_URL: &str =
    "https://api.mixmax.com/v1/sequences/:id/recipients";

#[derive(Clone, Deserialize, Debug)]
pub struct Person {
    #[serde(rename = "First Name")]
    pub first_name: String,

    #[serde(rename = "Last Name")]
    pub last_name: String,

    #[serde(rename = "Company Name")]
    pub company_name: String,

    #[serde(rename = "Job Title")]
    pub job_title: String,

    #[serde(rename = "Email")]
    pub email_address: String,

    #[serde(rename = "LinkedIn URL")]
    pub linkedin_url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RequestBody {
    pub recipients: Vec<Recipient>,
    #[serde(rename = "scheduledAt")]
    pub scheduled_at: bool,
}

#[derive(Debug, Deserialize)]
pub struct RecipientResponse {
    pub email: String,
    pub status: String,
    pub errors: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseBody {
    pub recipients: Vec<RecipientResponse>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Recipient {
    #[serde(rename = "email")]
    pub email_address: String,
    #[serde(rename = "variables")]
    pub variables: RecipientVariables,
}

#[derive(Debug, Clone, Serialize)]
pub struct RecipientVariables {
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

pub fn parse_people(mut csv_reader: csv::Reader<File>) -> Result<Vec<Person>, Box<dyn Error>> {
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
