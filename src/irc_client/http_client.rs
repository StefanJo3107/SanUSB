use anyhow::bail;
use esp_idf_svc::http::client::EspHttpConnection;
use embedded_svc::{
    http::{client::Client as HttpClient, Method},
    utils::io,
};
use log::info;

pub struct PayloadHttpClient {
    http_client: HttpClient<EspHttpConnection>,
}

impl PayloadHttpClient {
    pub fn new() -> anyhow::Result<PayloadHttpClient> {
        let client = HttpClient::wrap(EspHttpConnection::new(&Default::default())?);
        Ok(PayloadHttpClient {
            http_client: client
        })
    }

    pub fn get_payload(&mut self, url: &str) -> anyhow::Result<Vec<u8>> {
        let headers = [("accept", "text/plain")];

        let request = self.http_client.request(Method::Get, url, &headers)?;
        info!("-> GET {}", url);
        let mut response = request.submit()?;

        let status = response.status();
        info!("<- {}", status);
        if status != 200 {
            bail!("HTTP request return status code: {}", status);
        }

        let mut payload: Vec<u8> = vec![];
        let mut buf = [0u8; 5000];
        io::try_read_full(&mut response, &mut buf).map_err(|e| e.0)?;
        payload.append(&mut buf.to_vec());
        while response.read(&mut buf)? > 0 {
            payload.append(&mut buf.to_vec());
        }

        Ok(payload)
    }
}