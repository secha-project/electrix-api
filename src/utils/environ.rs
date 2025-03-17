pub fn get_env<T: std::str::FromStr>(variable_name: &str) -> Result<T, String> {
    std::env::var(variable_name)
        .map_or_else(
            |_| Err(format!("{variable_name} environment variable is not set")),
            |value| value
                .parse::<T>()
                .map_err(|_| format!("{value} is not a valid value for {variable_name}")),
        )
}

pub fn get_env_or_default<T: std::str::FromStr + Clone>(variable_name: &str, default_value: &T) -> T {
    get_env::<T>(variable_name)
        .unwrap_or_else(|_| default_value.clone())
}
