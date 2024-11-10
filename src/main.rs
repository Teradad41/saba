#![no_std]
#![cfg_attr(target_os = "linux", no_main)]
#![no_main]

extern crate alloc;

use alloc::string::ToString;
use net_wasabi::http::HttpClient;
use noli::prelude::*;

fn main() -> u64 {
    let client = HttpClient::new();
    match client.get("host.test".to_string(), 8080, "/test.html".to_string()) {
        Ok(res) => {
            println!("{:?}", res);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
    0
}

entry_point!(main);
