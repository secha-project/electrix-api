use reqwest::{Client, Error as reqwestError, RequestBuilder, Response, Url};
use serde::de::DeserializeOwned;
use tokio::{spawn, task::JoinHandle};


async fn make_query(
    url: String,
    headers: Vec<(String, String)>,
    params: Vec<(String, String)>,
    allow_invalid_certs: bool,
) -> Result<Response, reqwestError> {
    let url_with_params: Url = Url::parse_with_params(&url, params)
        .unwrap_or_else(|_| Url::parse(&url).unwrap());

    let query: RequestBuilder = Client::builder()
        .danger_accept_invalid_certs(allow_invalid_certs)
        .timeout(std::time::Duration::from_secs(20))
        .build()?
        .get(url_with_params);

    headers
        .iter()
        .fold(query, |query, (name, value)| query.header(name, value))
        .send()
        .await
}

async fn handle_response<T: DeserializeOwned>(response: Response) -> Result<T, String> {
    let status_code: u16 = response.status().as_u16();
    let url: String = response.url().to_string();
    let error_info: String = format!(" ({status_code}) ({url})");

    let response_body: String = response
        .text()
        .await
        .map_err(|err| err.to_string() + &error_info)?;

    if status_code != 200 {
        return Err(response_body + &error_info);
    }

    match serde_json::from_str(&response_body) {
        Ok(event) => Ok(event),
        Err(err) => Err(err.to_string() + &error_info),
    }
}

pub async fn get_data<T: DeserializeOwned>(
    url: String,
    headers: Vec<(String, String)>,
    params: Vec<(String, String)>,
    allow_invalid_certs: bool
) -> Result<T, String> {
    let query_future: JoinHandle<Result<Response, reqwestError>> = spawn(
        make_query(url, headers, params, allow_invalid_certs),
    );

    let query_response: Result<Response, reqwestError> = query_future
        .await
        .map_err(|err| err.to_string())?;

    match query_response {
        Ok(response) => handle_response(response).await,
        Err(err) => Err(err.to_string()),
    }
}
