use std::borrow::Cow;

#[derive(Clone)]
pub struct AppEnvVars {
    pub host_address: Cow<'static, str>,
    pub host_origin: Cow<'static, str>,
    pub fe_dev_origin: Cow<'static, str>,
}

impl AppEnvVars {
    pub fn init() -> anyhow::Result<Self> {
        dotenv::dotenv()?;

        Ok(Self {
            host_address: Cow::from(
                dotenv::var("HOST_ADDRESS").unwrap_or("127.0.0.1:3000".to_string()),
            ),
            host_origin: Cow::from(
                dotenv::var("HOST_ORIGIN").unwrap_or("http://127.0.0.1:3000".to_string()),
            ),
            fe_dev_origin: Cow::from(
                dotenv::var("FE_DEV_ORIGIN").unwrap_or("http://127.0.0.1:5173".to_string()),
            ),
        })
    }
}
