#[macro_use]
extern crate serde_derive;
extern crate log;
extern crate simple_logger;

use lambda_runtime::{service_fn, LambdaEvent, Error};
use aws_sdk_sns::{Client};
use serde_json::{Value};

use std::env;
use aws_sdk_sns::model::Topic;


#[derive(Deserialize, Clone)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

#[derive(Deserialize, Debug)]
struct PageBlocks {
    results: Vec<Block>
}

#[derive(Deserialize, Debug)]
struct Block {
    r#type: String,
    to_do: Option<Todo>
    
}

#[derive(Deserialize, Debug)]
struct Todo {
    checked: bool,
    rich_text: Vec<RichText>
}

#[derive(Deserialize, Debug)]
struct RichText {
    plain_text: String
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::init_with_level(log::Level::Warn).unwrap();

    let func = service_fn(my_handler);

    lambda_runtime::run(func).await?;

    Ok(())
}

async fn my_handler(event: LambdaEvent<Value>) -> Result<CustomOutput, Error> {
    let _context = event.into_parts();

    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();

    let owned_string: String = "Bearer ".to_owned();
    let borrowed_string: &str = &env::var("NOTION_SECRET").unwrap();

    let new_owned_string = owned_string + borrowed_string;

    headers.insert("Notion-Version", "2022-02-22".parse().unwrap());
    headers.insert("Authorization", new_owned_string.parse().unwrap());

    let resp: PageBlocks = client.get("https://api.notion.com/v1/blocks/f612825f-64bf-4a46-97c8-48010c2da73f/children")
        .headers(headers).send()
        .await?
        .json()
        .await?;

        let mut todos: Vec<String> = Vec::new();

        for i in 0..resp.results.len() {
            let block = &resp.results[i];

            if block.r#type == "to_do" && !block.to_do.as_ref().unwrap().checked {
                todos.push(block.to_do.as_ref().unwrap().rich_text[0].plain_text.clone());
            }
        }

    let aws_config = aws_config::from_env().region("us-east-1").load().await;
    let aws_client = Client::new(&aws_config);

    let rsp = aws_client
        .publish()
        .topic_arn("arn:aws:sns:us-east-1:573834862121:Reminders")
        .message("hello sns!")
        .send()
        .await?;

    Ok(CustomOutput {
        message: format!("{:#?}", todos),
    })
}