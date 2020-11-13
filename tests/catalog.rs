extern crate consul;
use consul::{Client, Config};

#[test]
fn ds_test() {
    use tokio::runtime::Runtime;
    let mut rt = Runtime::new().unwrap();

    use consul::catalog::Catalog;
    let config = Config::new_from_env().unwrap();
    let client = Client::new(config);
    let r = rt.block_on(client.datacenters()).unwrap();
    assert_eq!(r.0, ["dc1"]);
}

#[test]
fn ds_services_test() {
    use tokio::runtime::Runtime;
    let mut rt = Runtime::new().unwrap();

    use consul::catalog::Catalog;
    let config = Config::new().unwrap();
    let client = Client::new(config);
    let r = rt.block_on(client.services(Option::None)).unwrap();
    assert_ne!(r.0.len(), 0);
    match r.0.get("consul") {
        None => panic!("Should have a Consul service"),
        Some(val) => assert_eq!(val.len(), 0), // consul has no tags
    }
}
