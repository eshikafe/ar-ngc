use pfcp;
use std::net::{IpAddr, Ipv4Addr};

#[tokio::main]
async fn main() {
    let n4_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    pfcp::server(n4_addr).await;
}
