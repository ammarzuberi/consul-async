extern crate consul;
use consul::kv::KVPair;
use consul::{Client, Config};

#[test]
fn kv_test() {
    use tokio::runtime::Runtime;
    let mut rt = Runtime::new().unwrap();

    use consul::kv::KV;
    let config = Config::new().unwrap();
    let client = Client::new(config);
    let r = rt.block_on(client.list("", None)).unwrap();
    assert!(r.0.is_empty());

    let pair = KVPair {
        Key: String::from("testkey"),
        Value: Some(String::from("testvalue")),
        ..Default::default()
    };

    assert!(rt.block_on(client.put(&pair, None)).unwrap().0);

    let r = rt.block_on(client.list("t", None)).unwrap();
    assert!(!r.0.is_empty());

    rt.block_on(client.delete("testkey", None)).unwrap();

    let r = rt.block_on(client.list("", None)).unwrap();
    assert!(r.0.is_empty());
}
