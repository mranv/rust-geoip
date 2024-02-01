use ipinfo::IpInfo;

#[tokio::main]
async fn main() {
    let ipinfo = IpInfo::new(Default::default()).expect("should construct");

    let res = ipinfo.get_map(&["8.8.8.8", "4.2.2.4"]).await;
    match res {
        Ok(r) => {
            println!("Map URL: {}", r);
        }
        Err(e) => println!("error occurred: {}", &e.to_string()),
    }
}
