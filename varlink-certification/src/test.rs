use assert_cmd::cargo::{main_binary_path, CargoError};
use std::io;
use std::result;
use std::{thread, time};
use varlink::Connection;
use Result;

fn run_self_test(address: String) -> Result<()> {
    let client_address = address.clone();

    let child = thread::spawn(move || {
        if let Err(e) = ::run_server(&address, 4) {
            match e.kind() {
                ::varlink::ErrorKind::Timeout => {}
                _ => panic!("error: {}", e),
            }
        }
    });

    // give server time to start
    thread::sleep(time::Duration::from_secs(1));

    let ret = ::run_client(Connection::with_address(&client_address)?);
    if let Err(e) = ret {
        panic!("error: {:?}", e);
    }
    if let Err(e) = child.join() {
        Err(io::Error::new(io::ErrorKind::ConnectionRefused, format!("{:#?}", e)).into())
    } else {
        Ok(())
    }
}

#[test]
fn test_unix() {
    assert!(run_self_test("unix:/tmp/org.varlink.certification".into()).is_ok());
}

#[test]
#[cfg(any(target_os = "linux", target_os = "android"))]
fn test_unix_abstract() {
    assert!(run_self_test("unix:@org.varlink.certification".into()).is_ok());
}

#[test]
fn test_tcp() {
    assert!(run_self_test("tcp:0.0.0.0:23456".into()).is_ok());
}

fn get_exec() -> result::Result<String, CargoError> {
    Ok(main_binary_path()?.to_string_lossy().to_string())
}

#[test]
fn test_exec() {
    match get_exec() {
        Err(_) => {
            eprintln!("test test::test_exec ... skipping, no varlink-certification binary found");
            return;
        }
        Ok(program) => {
            assert!(
                ::run_client(
                    Connection::with_activate(&format!("{} --varlink=$VARLINK_ADDRESS", program))
                        .unwrap(),
                ).is_ok()
            );
        }
    }
}

#[test]
fn test_wrong_address_1() {
    assert!(::run_server("tcpd:0.0.0.0:12345".into(), 1).is_err());
}
