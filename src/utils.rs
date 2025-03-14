pub mod http_utils {
    use reqwest::{Client, Error as reqwestError, RequestBuilder, Response, Url};
    use tokio::{spawn, task::JoinHandle};
    use crate::data::data_structs::{Device, DeviceData, Host};


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
        let included_data_points: String = vec![
            "id", "meter", "timestamp", "fhz", "pw", "qfryzevar", "q1var",
            "pl1w", "pl2w", "pl3w", "p1l1", "p1l2", "p1l3",
            "qfryzel1var", "qfryzel2var", "qfryzel3var", "q1l1", "q1l2", "q1l3",
            "sva", "sl1va", "sl2va", "sl3va", "s1l1", "s1l2", "s1l3",
            "ul1v", "ul2v", "ul3v", "ul12v", "ul23v", "ul31v",
            "il1a", "il2a", "il3a", "i_n",
        ].join(",");

        let query_future: JoinHandle<Result<Response, reqwestError>> = spawn(
            make_query(
                host.get_host_url() + DATA_ENDPOINT,
                host.get_headers(),
                vec![
                    ("meter".to_string(), device.id.to_string()),
                    ("start".to_string(), date.to_owned() + " 00:00:00"),
                    ("end".to_string(), date.to_owned() + " 23:59:59"),
                    ("fields".to_string(), included_data_points),
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

}
