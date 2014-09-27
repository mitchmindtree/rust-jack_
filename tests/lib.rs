extern crate jack;
use jack::JackClient;

#[test]
fn new_client_and_close() {
    let client = JackClient::open("new_client",jack::JackNullOption);
    assert!(client.close());
}

#[test]
fn client_name_size() {
    let size = JackClient::name_size();
    assert!(size == 33);
}

#[test]
fn get_client_name() {
    let name = "check_me";
    let client = JackClient::open(name,jack::JackNullOption);
    let get_name = client.get_name();
    client.close();
    assert!(get_name.as_slice() == name);
}

#[test]
fn activate() {
    let client = JackClient::open("activate_client",jack::JackNullOption);
    assert!(client.activate());
    assert!(client.deactivate());
    assert!(client.close());
}
