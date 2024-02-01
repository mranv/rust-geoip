use ipinfo::{BatchReqOpts, IpInfo, IpInfoConfig};
use std::env;

#[tokio::main]
async fn main() {
    let token = env::args().nth(1);

    let config = IpInfoConfig {
        token,
        ..Default::default()
    };

    let mut ipinfo = IpInfo::new(config).expect("should construct");

    let res2 = ipinfo
        .lookup_batch(&["8.8.8.8", "4.2.2.4"], BatchReqOpts::default())
        .await;

    match res2 {
        Ok(r) => {
            println!("8.8.8.8: {:?}", r["8.8.8.8"]);
            println!("4.2.2.4: {:?}", r["4.2.2.4"]);
        }
        Err(e) => println!("error occurred: {}", &e.to_string()),
    }
}
