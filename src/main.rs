#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;

use lambda_runtime::{service_fn, LambdaEvent, Error};

use serde_json::{Value};

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

// async fn wrapper(event: CustomEvent, cx: lambda::Context) -> Result<CustomOutput, Error> {
//     let res = my_handler(event, cx).await;

//     if let Err(e) = &res {
//         error!("got error from handler: {:?}", &e);
//     }
//     res
// }


#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::init_with_level(log::Level::Warn).unwrap();

    let func = service_fn(my_handler);

    lambda_runtime::run(func).await?;

    Ok(())
}

async fn my_handler(event: LambdaEvent<Value>) -> Result<CustomOutput, Error> {
    let (event, _context) = event.into_parts();
    let first_name = event["firstName"].as_str().unwrap_or("world");

    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);

    Ok(CustomOutput {
        message: format!("Hello, {}!", first_name),
    })
}