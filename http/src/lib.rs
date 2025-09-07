#[allow(warnings)]
mod bindings;
use anyhow::{bail, Context};
use bindings::Guest;
use serde::Deserialize;
use wstd::{http::{Client, Request}, io, runtime::block_on};


struct Component;

impl Guest for Component {
    fn sample_request() -> Result<String, String> {
        let result = block_on(async {
            get_httpbin().await
        });
        match result {
            Ok(url) => Ok(url),
            Err(e) => {
                let msg = format!("Error occurred: {}", e);
                Err(msg)
            }
        }
    }
}

#[derive(Deserialize)]
struct HttpBinResponse {
    url: String,
}


async fn get_httpbin() -> anyhow::Result<String> {
    let url = "https://httpbin.org/get";

    let request = Request::get(url).body(io::empty())?;
    let mut response = Client::new().send(request).await?;

    if response.status() != 200 {
        bail!("Unexpected status code: {}", response.status());
    }

    let body = response.body_mut().bytes().await?;

    let result: HttpBinResponse = serde_json::from_slice(&body).with_context(|| {
        let preview = String::from_utf8_lossy(&body);
        format!("Failed to parse JSON. Body: {preview}")
    })?;
    
    Ok(result.url)
}


bindings::export!(Component with_types_in bindings);
