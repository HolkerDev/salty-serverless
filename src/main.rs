use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest as AwsRequest, ApiGatewayProxyResponse as AwsResponse};
use http::HeaderMap;
use lambda_runtime::{Context, Error, handler_fn};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
    lambda_runtime::run(handler_fn(search_eat_place_by_name)).await?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

impl ErrorResponse {
    pub fn from(error_message: &str) -> ErrorResponse {
        ErrorResponse {
            error: String::from(error_message)
        }
    }
}

pub(crate) async fn search_eat_place_by_name(
    event: AwsRequest,
    _ctx: Context,
) -> Result<AwsResponse, Error> {
    let place_name = event.query_string_parameters.get("name");
    if place_name.is_none() {
        return Ok(respond(400, &ErrorResponse::from("The name of the place was not provided")));
    }

    Ok(respond_str(200, format!("Found place with the name: {}", place_name.unwrap())))
}

pub fn respond<T>(status: i64, value: &T) -> AwsResponse
    where
        T: ?Sized + Serialize,
{
    AwsResponse {
        status_code: status,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(serde_json::to_string(value).unwrap())),
        is_base64_encoded: Some(false),
    }
}

pub fn respond_str(status: i64, string: String) -> AwsResponse {
    AwsResponse {
        status_code: status,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(string)),
        is_base64_encoded: Some(false),
    }
}