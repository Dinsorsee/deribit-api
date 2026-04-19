#[derive(Debug, Clone)]
pub struct DotEnvyConfig {
    pub deribit_api: DeribitApi,
}

#[derive(Debug, Clone)]
pub struct DeribitApi {
    pub url: String,
}
