extern crate consul;
use consul::session::SessionEntry;
use consul::{Client, Config};

#[test]
fn session_test() {
    use tokio::runtime::Runtime;
    let mut rt = Runtime::new().unwrap();

    use consul::session::Session;
    let config = Config::new().unwrap();
    let client = Client::new(config);
    let r = rt.block_on(client.list(None)).unwrap();
    assert!(r.0.is_empty());

    let entry = SessionEntry {
        Name: Some(String::from("test session")),
        ..Default::default()
    };

    let id = rt.block_on(client.create(&entry, None)).unwrap().0.ID.unwrap();

    rt.block_on(client.renew(&id, None)).unwrap();

    rt.block_on(client.destroy(&id, None)).unwrap();
}
