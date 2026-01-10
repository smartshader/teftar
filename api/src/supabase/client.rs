use reqwest::Client;
use std::env;

#[derive(Clone)]
pub struct SupabaseClient {
    client: Client,
    pub(crate) url: String,
    anon_key: String,
}

impl SupabaseClient {
    pub fn new() -> Result<Self, String> {
        let url =
            env::var("SUPABASE_URL").map_err(|_| "Missing SUPABASE_URL environment variable")?;
        let anon_key = env::var("SUPABASE_ANON_KEY")
            .map_err(|_| "Missing SUPABASE_ANON_KEY environment variable")?;

        Ok(Self {
            client: Client::new(),
            url,
            anon_key,
        })
    }

    pub fn auth_url(&self, path: &str) -> String {
        format!("{}/auth/v1{}", self.url, path)
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn anon_key(&self) -> &str {
        &self.anon_key
    }
}
