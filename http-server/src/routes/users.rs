use std::convert::Infallible;

use http_body_util::Full;
use hyper::{body::Bytes, Request, Response};
use serde_json::json;
use serde::Deserialize;

#[derive(Deserialize)]
struct QueryParams {
    message: Option<String>,
}

pub fn index(request: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let query = request.uri().query().unwrap_or("");
    let query_params: QueryParams = serde_urlencoded::from_str(query).unwrap();
    let json_data = json!({
        "message": query_params.message.unwrap_or("".to_string())
    });

    let response = Response::builder()
        .header("Content-Type", "application/json")
        .body(Full::from(json_data.to_string().into_bytes()))
        .unwrap();

    Ok(response)
}
