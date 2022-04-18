#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;

use lambda::error::HandlerError;

use std::error::Error;
use std::collections::HashMap;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

async fn wrapper(event: CustomEvent, cx: lambda::Context) -> Result<CustomOutput, HandlerError> {
    let res = my_handler(event, cx).await;

    if let Err(e) = &res {
        error!("got error from handler: {:?}", &e);
    }
    res
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Warn).unwrap();

    lambda::run(lambda::Handler(wrapper)).await?;

    Ok(())
}

async fn my_handler(e: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    if e.first_name == "" {
        error!("Empty first name in request {}", c.aws_request_id);
        return Err(c.new_error("Empty first name"));
    }

    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);

    Ok(CustomOutput {
        message: format!("Hello, {}!", e.first_name),
    })
}