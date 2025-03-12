pub mod http_utils {
    use reqwest::{Client, Error as reqwestError, RequestBuilder, Response, Url};
    use tokio::{spawn, task::JoinHandle};
    use crate::data::data_structs::{Device, Host};


    async fn make_query(url: String, headers: Vec<(String, String)>, params: Vec<(String, String)>) -> Result<Response, reqwestError> {
        let url_with_params: Url = Url::parse_with_params(&url, params)
            .unwrap_or_else(|_| Url::parse(&url).unwrap());

        let query: RequestBuilder = Client::new()
            .get(url_with_params);

        headers
            .iter()
            .fold(query, |query, (name, value)| query.header(name, value))
            .send()
            .await
    }

    async fn get_devices_from_response(response: Response) -> Result<Vec<Device>, String> {
        response
            .json::<Vec<Device>>()
            .await
            .map_err(|err| err.to_string())
    }

    pub async fn get_devices(host: Host) -> Result<Vec<Device>, String> {
        const DEVICE_ENDPOINT: &str = "/api/v2/meters/";
        // let url = host.get_host_url().to_string() + DEVICE_ENDPOINT;
        let result_future: JoinHandle<Result<Response, reqwestError>> = spawn(
            make_query(
                host.get_host_url() + DEVICE_ENDPOINT,
                host.get_headers(),
                vec![]
            )
        );

        let result_join: Result<Result<Response, reqwestError>, _> = result_future.await;
        match result_join {
            Ok(result) => {
                match result {
                    Ok(response) => get_devices_from_response(response).await,
                    Err(err) => Err(err.to_string()),
                }
            },
            Err(err) => Err(err.to_string())
        }
    }

}
