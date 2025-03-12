pub mod data_structs {
    use serde::Deserialize;

    #[derive(Debug)]
    pub struct Host {
        host_url: String,
        token_header: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Device {
        pub id: i32,
        pub location: String,
        pub serialnumber: String,
        pub reading: i32,
        pub ik: i32,  // the current factor
        pub uk: i32,  // the voltage factor
    }


    impl Host {
        pub fn get_from_env() -> Result<Self, String> {
            const HOST_URL_STR: &str = "HOST_URL";
            const ACCESS_TOKEN_STR: &str = "ACCESS_TOKEN";
            const ERROR_MESSAGE: &str = "environment variable is not set";

            let host_url: String = std::env::var(HOST_URL_STR)
                .map_or_else(
                    |_| Err(format!("{HOST_URL_STR} {ERROR_MESSAGE}")),
                    Ok
                    // |url| Ok(url)
                )?;

            let token_header: String = std::env::var(ACCESS_TOKEN_STR)
                .map_or_else(
                    |_| Err(format!("{ACCESS_TOKEN_STR} {ERROR_MESSAGE}")),
                    |token| Ok(format!("Api-Key {token}"))
                )?;

            Ok(
                Self {
                    host_url,
                    token_header,
                }
            )
        }

        pub fn get_host_url(&self) -> String {
            self.host_url.clone()
        }

        pub fn get_headers(&self) -> Vec<(String, String)> {
            vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                ("Authorization".to_string(), self.token_header.clone()),
            ]
        }
    }


    impl Device {
        pub fn pretty_print(&self) -> String {
            let id: i32 = self.id;
            let serial_number = self.serialnumber.clone();
            let location = self.location.clone();
            let reading = self.reading;
            let current_factor = self.ik;
            let voltage_factor = self.uk;

            "Device:\n".to_string() +
            &format!("- id: {id}\n") +
            &format!("- serial number: {serial_number}\n") +
            &format!("- location: {location}\n") +
            &format!("- reading: {reading}\n") +
            &format!("- current factor: {current_factor}\n") +
            &format!("- voltage factor: {voltage_factor}\n")
        }
    }

}
