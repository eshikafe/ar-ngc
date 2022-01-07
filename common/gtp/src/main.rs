// Copyright (c) 2022 vPlane Telecom

use gtp;
use std::net::{IpAddr, Ipv4Addr};

#[tokio::main]
async fn main() {
    let s11_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    gtp::server(s11_addr).await;
}
