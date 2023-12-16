use std::net::IpAddr;

#[test]
fn test_main() {
    let local: IpAddr = "127.0.0.1".parse().unwrap();
    assert!(local.is_loopback());

    let global: IpAddr = IpAddr::from([0, 0, 0x1c9, 0, 0, 0xafc8, 0, 0x1]);
}
