pub mod http_utils {
    use reqwest::{Client, Error as reqwestError, RequestBuilder, Response, Url};
    use tokio::{spawn, task::JoinHandle};
    use crate::data::data_structs::{Device, DeviceData, DeviceEvent, DeviceEventDetails, Host, DATA_POINTS};
    use serde_json;


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

    pub async fn get_devices(host: &Host) -> Result<Vec<Device>, String> {
        const DEVICE_ENDPOINT: &str = "/api/v2/meters/";
        let query_future: JoinHandle<Result<Response, reqwestError>> = spawn(
            make_query(
                host.get_host_url() + DEVICE_ENDPOINT,
                host.get_headers(),
                vec![]
            )
        );

        let query_response: Result<Response, reqwestError> = query_future
            .await
            .map_err(|err| err.to_string())?;

        match query_response {
            Ok(response) => get_devices_from_response(response).await,
            Err(err) => Err(err.to_string()),
        }
    }

    async fn get_device_data_from_response(response: Response) -> Result<Vec<DeviceData>, String> {
        let status_code: u16 = response.status().as_u16();
        response
            .json::<Vec<DeviceData>>()
            .await
            .map_err(|err| err.to_string() + &format!(" ({status_code})"))
    }

    // assumes date is given in format YYYY-MM-DD
    pub async fn get_device_data(host: &Host, device: &Device, date: &str) -> Result<Vec<DeviceData>, String> {
        const DATA_ENDPOINT: &str = "/api/v2/measurements/";

        let query_future: JoinHandle<Result<Response, reqwestError>> = spawn(
            make_query(
                host.get_host_url() + DATA_ENDPOINT,
                host.get_headers(),
                vec![
                    ("meter".to_string(), device.id.to_string()),
                    ("start".to_string(), date.to_owned() + " 00:00:00"),
                    ("end".to_string(), date.to_owned() + " 23:59:59"),
                    ("fields".to_string(), DATA_POINTS.to_string()),
                ]
            )
        );

        let query_response: Result<Response, reqwestError> = query_future
            .await
            .map_err(|err| err.to_string())?;

        match query_response {
            Ok(response) => get_device_data_from_response(response).await,
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn get_device_events(host: &Host, device: &Device, date: &str) -> Result<Vec<DeviceEvent>, String> {
        const EVENT_ENDPOINT: &str = "/api/v2/events/";
        let query_future: JoinHandle<Result<Response, reqwestError>> = spawn(
            make_query(
                host.get_host_url() + EVENT_ENDPOINT,
                host.get_headers(),
                vec![
                    ("meter".to_string(), device.id.to_string()),
                    ("start".to_string(), date.to_owned() + " 00:00:00.000000"),
                    ("end".to_string(), date.to_owned() + " 23:59:59.999999"),
                ]
            )
        );

        let query_response: Result<Response, reqwestError> = query_future
            .await
            .map_err(|err| err.to_string())?;

        match query_response {
            Ok(response) => {
                let status_code: u16 = response.status().as_u16();
                let response_body = response.text().await.map_err(|err| err.to_string() + &format!(" ({status_code})"))?;
                match serde_json::from_str(&response_body) {
                    Ok(events) => Ok(events),
                    Err(err) => {
                        println!("Failed to decode JSON: {err} (status: {status_code})");
                        println!("Response body: {response_body}");
                        Err(err.to_string() + &format!(" ({status_code})"))
                    }
                }
            },
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn get_event_data(host: &Host, event_id: u32) -> Result<DeviceEventDetails, String> {
        let event_details_endpoint: String = format!("/api/v2/events/{event_id}/");
        let query_future: JoinHandle<Result<Response, reqwestError>> = spawn(
            make_query(
                host.get_host_url() + event_details_endpoint.as_str(),
                host.get_headers(),
                vec![]
            )
        );

        let query_response: Result<Response, reqwestError> = query_future
            .await
            .map_err(|err| err.to_string())?;

        match query_response {
            Ok(response) => {
                let status_code: u16 = response.status().as_u16();
                let response_body = response.text().await.map_err(|err| err.to_string() + &format!(" ({status_code})"))?;
                match serde_json::from_str(&response_body) {
                    Ok(event) => Ok(event),
                    Err(err) => {
                        println!("Failed to decode JSON: {err} (status: {status_code})");
                        println!("Response body: {response_body}");
                        Err(err.to_string() + &format!(" ({status_code})"))
                    }
                }
            },
            Err(err) => Err(err.to_string()),
        }
    }

}
