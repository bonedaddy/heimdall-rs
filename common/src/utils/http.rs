use std::{
    io::Read
};

use reqwest::blocking::get;
use serde_json::Value;

// make a GET request to the target URL and return the response body as JSON
pub async fn get_json_from_url(url: String, attempts_remaining: u8) -> Option<Value> {
    for _ in 0..attempts_remaining {
        let res = match reqwest::get(url.clone()).await {
            Ok(res) => res,
            Err(_) => {
    
                // retry if we have attempts remaining
                let attempts_remaining = attempts_remaining - 1;
                if attempts_remaining == 0 {
                    return None
                }
    
                std::thread::sleep(std::time::Duration::from_millis(500));
                continue;
            }
        };
        let body_bytes = res.bytes().await.ok()?;
        let body_msg = String::from_utf8(body_bytes.to_vec()).ok()?;
        return Some(serde_json::from_str(&body_msg).ok()?);
    }
    None
}