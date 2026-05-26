use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
pub use reqwless::request::Method;
use reqwless::headers::ContentType;
use reqwless::request::RequestBuilder;
use reqwless::client::HttpClient;

#[cfg(feature = "json")]
use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "https")]
use reqwless::client::{TlsConfig, TlsVerify};

use embedded_io_async::Read;

use crate::net::tcp::MreTcpStack;
use crate::net::dns::MreDnsStack;

pub async fn fetch(
    method: Method,
    url: &str,
    headers: &[(&str, &str)],
    body: Option<&[u8]>,
    content_type: Option<ContentType>,
) -> Result<Vec<u8>, String> {
    #[cfg(feature = "https")]
    {
        if !crate::stack::is_on_custom_stack() {
            return Err("FATAL: HTTPS request must be executed inside mre_callback! (custom stack)".into());
        }
        
        if crate::stack::custom_stack_size() < 32 * 1024 {
            return Err("FATAL: Custom stack is too small for HTTPS. Minimum 32KB required.".into());
        }
    }
    
    let tcp_stack = MreTcpStack;
    let dns_stack = MreDnsStack;
    
    let mut rx_buf = alloc::vec![0u8; 4*1024];

    #[cfg(feature = "https")]
    let mut tls_read_buf = alloc::vec![0u8; 8*2048]; 
    #[cfg(feature = "https")]
    let mut tls_write_buf = alloc::vec![0u8; 8*2048];

    #[cfg(feature = "https")]
    let tls_config = {
        let seed = (crate::time::instant::Instant::now().ticks as u64) | ((crate::time::datetime::utc_timestamp().unwrap_or(0) as u64) << 32);

        TlsConfig::new(
            seed,
            &mut tls_read_buf,
            &mut tls_write_buf,
            TlsVerify::None,
        )
    };

    #[cfg(feature = "https")]
    let mut client = HttpClient::new_with_tls(&tcp_stack, &dns_stack, tls_config);

    #[cfg(not(feature = "https"))]
    let mut client = HttpClient::new(&tcp_stack, &dns_stack);

    let mut all_headers = alloc::vec![
        ("User-Agent", "MreEngine/3.0"),
        ("Connection", "close"),
    ];

    all_headers.extend_from_slice(headers);

    let connection_future = Box::pin(client.request(method, url));

    let req = connection_future
        .await
        .map_err(|e| format!("Connect Err: {:?}", e))?
        .headers(&all_headers);

    let mut req_body_storage = None;
    let mut req_no_body_storage = None;

    let send_future = if let Some(b) = body {
        let mut r = req.body(b);
        if let Some(ct) = content_type {
            r = r.content_type(ct);
        }

        let stored_req = req_body_storage.insert(r);
        stored_req.send(&mut rx_buf).await
    } else {
        let stored_req = req_no_body_storage.insert(req);
        stored_req.send(&mut rx_buf).await
    };

    let res = send_future.map_err(|e| format!("Send Err: {:?}", e))?;

    if !res.status.is_successful() {
        return Err(format!("Server returned HTTP {}", res.status.0));
    }

    let mut body_bytes = Vec::new();
    let mut chunk = alloc::vec![0u8; 1024];
    let mut reader = res.body().reader();

    loop {
        match reader.read(&mut chunk).await {
            Ok(0) => break,
            Ok(n) => body_bytes.extend_from_slice(&chunk[..n]),
            Err(_) => return Err("Body read error".into()),
        }
    }

    Ok(body_bytes)
}

#[cfg(feature = "json")]
pub async fn get_json<T: DeserializeOwned>(
    url: &str,
) -> Result<T, String> {
    let headers = [("Accept", "application/json")];
    
    let bytes = fetch(Method::GET, url, &headers, None, None).await?;
    
    serde_json::from_slice(&bytes).map_err(|e| format!("JSON Parse Err: {}", e))
}

#[cfg(feature = "json")]
pub async fn post_json<Req: Serialize, Res: DeserializeOwned>(
    url: &str,
    payload: &Req
) -> Result<Res, String> {
    let json_body = serde_json::to_string(payload).map_err(|_| "JSON Serialize Err")?;
    
    let headers = [
        ("Accept", "application/json"),
    ];
    
    let bytes = fetch(
        Method::POST, 
        url, 
        &headers, 
        Some(json_body.as_bytes()),
        Some(ContentType::ApplicationJson)
    ).await?;
    
    serde_json::from_slice(&bytes).map_err(|e| format!("JSON Parse Err: {}", e))
}