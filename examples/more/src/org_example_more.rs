//! DO NOT EDIT
//! This file is automatically generated by the varlink rust generator

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use serde_json::{self, Value};
use std::io;
use std::sync::{Arc, RwLock};
use varlink;
use varlink::CallTrait;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct State {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PingReply_ {
    pub pong: String,
}

impl varlink::VarlinkReply for PingReply_ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PingArgs_ {
    pub ping: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StopServingReply_ {}

impl varlink::VarlinkReply for StopServingReply_ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StopServingArgs_ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TestMoreReply_ {
    pub state: State,
}

impl varlink::VarlinkReply for TestMoreReply_ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TestMoreArgs_ {
    pub n: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TestMoreErrorArgs_ {
    pub reason: String,
}

pub trait _CallErr: varlink::CallTrait {
    fn reply_test_more_error(&mut self, reason: String) -> io::Result<()> {
        self.reply_struct(varlink::Reply::error(
            "org.example.more.TestMoreError".into(),
            Some(serde_json::to_value(TestMoreErrorArgs_ { reason }).unwrap()),
        ))
    }
}

impl<'a> _CallErr for varlink::Call<'a> {}

#[derive(Debug)]
pub enum Error_ {
    TestMoreError(Option<TestMoreErrorArgs_>),
    VarlinkError_(varlink::Error),
    UnknownError_(varlink::Reply),
    IOError_(io::Error),
    JSONError_(serde_json::Error),
}

impl From<varlink::Reply> for Error_ {
    fn from(e: varlink::Reply) -> Self {
        if varlink::Error::is_error(&e) {
            return Error_::VarlinkError_(e.into());
        }

        match e {
            varlink::Reply {
                error: Some(ref t), ..
            } if t == "org.example.more.TestMoreError" =>
            {
                match e {
                    varlink::Reply {
                        parameters: Some(p),
                        ..
                    } => match serde_json::from_value(p) {
                        Ok(v) => Error_::TestMoreError(v),
                        Err(_) => Error_::TestMoreError(None),
                    },
                    _ => Error_::TestMoreError(None),
                }
            }
            _ => return Error_::UnknownError_(e),
        }
    }
}

impl From<io::Error> for Error_ {
    fn from(e: io::Error) -> Self {
        Error_::IOError_(e)
    }
}

impl From<serde_json::Error> for Error_ {
    fn from(e: serde_json::Error) -> Self {
        use serde_json::error::Category;
        match e.classify() {
            Category::Io => Error_::IOError_(e.into()),
            _ => Error_::JSONError_(e),
        }
    }
}

impl From<Error_> for io::Error {
    fn from(e: Error_) -> Self {
        match e {
            Error_::TestMoreError(e) => io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "org.example.more.TestMoreError: '{}'",
                    serde_json::to_string_pretty(&e).unwrap()
                ),
            ),
            Error_::VarlinkError_(e) => e.into(),
            Error_::IOError_(e) => e,
            Error_::JSONError_(e) => e.into(),
            Error_::UnknownError_(e) => io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "unknown varlink error: {}",
                    serde_json::to_string_pretty(&e).unwrap()
                ),
            ),
        }
    }
}
pub trait _CallPing: _CallErr {
    fn reply(&mut self, pong: String) -> io::Result<()> {
        self.reply_struct(PingReply_ { pong }.into())
    }
}

impl<'a> _CallPing for varlink::Call<'a> {}

pub trait _CallStopServing: _CallErr {
    fn reply(&mut self) -> io::Result<()> {
        self.reply_struct(varlink::Reply::parameters(None))
    }
}

impl<'a> _CallStopServing for varlink::Call<'a> {}

pub trait _CallTestMore: _CallErr {
    fn reply(&mut self, state: State) -> io::Result<()> {
        self.reply_struct(TestMoreReply_ { state }.into())
    }
}

impl<'a> _CallTestMore for varlink::Call<'a> {}

pub trait VarlinkInterface {
    fn ping(&self, call: &mut _CallPing, ping: String) -> io::Result<()>;
    fn stop_serving(&self, call: &mut _CallStopServing) -> io::Result<()>;
    fn test_more(&self, call: &mut _CallTestMore, n: i64) -> io::Result<()>;
    fn call_upgraded(&self, _call: &mut varlink::Call) -> io::Result<()> {
        Ok(())
    }
}

pub trait VarlinkClientInterface {
    fn ping(&mut self, ping: String) -> varlink::MethodCall<PingArgs_, PingReply_, Error_>;
    fn stop_serving(&mut self) -> varlink::MethodCall<StopServingArgs_, StopServingReply_, Error_>;
    fn test_more(&mut self, n: i64) -> varlink::MethodCall<TestMoreArgs_, TestMoreReply_, Error_>;
}

pub struct VarlinkClient {
    connection: Arc<RwLock<varlink::Connection>>,
    more: bool,
    oneway: bool,
}

impl VarlinkClient {
    pub fn new(connection: Arc<RwLock<varlink::Connection>>) -> Self {
        VarlinkClient {
            connection,
            more: false,
            oneway: false,
        }
    }
    pub fn more(&self) -> Self {
        VarlinkClient {
            connection: self.connection.clone(),
            more: true,
            oneway: false,
        }
    }
    pub fn oneway(&self) -> Self {
        VarlinkClient {
            connection: self.connection.clone(),
            more: false,
            oneway: true,
        }
    }
}

impl VarlinkClientInterface for VarlinkClient {
    fn ping(&mut self, ping: String) -> varlink::MethodCall<PingArgs_, PingReply_, Error_> {
        varlink::MethodCall::<PingArgs_, PingReply_, Error_>::new(
            self.connection.clone(),
            "org.example.more.Ping".into(),
            PingArgs_ { ping },
        )
    }
    fn stop_serving(&mut self) -> varlink::MethodCall<StopServingArgs_, StopServingReply_, Error_> {
        varlink::MethodCall::<StopServingArgs_, StopServingReply_, Error_>::new(
            self.connection.clone(),
            "org.example.more.StopServing".into(),
            StopServingArgs_ {},
        )
    }
    fn test_more(&mut self, n: i64) -> varlink::MethodCall<TestMoreArgs_, TestMoreReply_, Error_> {
        varlink::MethodCall::<TestMoreArgs_, TestMoreReply_, Error_>::new(
            self.connection.clone(),
            "org.example.more.TestMore".into(),
            TestMoreArgs_ { n },
        )
    }
}

pub struct _InterfaceProxy {
    inner: Box<VarlinkInterface + Send + Sync>,
}

pub fn new(inner: Box<VarlinkInterface + Send + Sync>) -> _InterfaceProxy {
    _InterfaceProxy { inner }
}

impl varlink::Interface for _InterfaceProxy {
    fn get_description(&self) -> &'static str {
        r#"# Example Varlink service
interface org.example.more

# Enum, returning either start, progress or end
# progress: [0-100]
type State (
  start: ?bool,
  progress: ?int,
  end: ?bool
)

# Returns the same string
method Ping(ping: string) -> (pong: string)

# Dummy progress method
# n: number of progress steps
method TestMore(n: int) -> (state: State)

# Stop serving
method StopServing() -> ()

# Something failed in TestMore
error TestMoreError (reason: string)
"#
    }

    fn get_name(&self) -> &'static str {
        "org.example.more"
    }

    fn call_upgraded(&self, call: &mut varlink::Call) -> io::Result<()> {
        self.inner.call_upgraded(call)
    }

    fn call(&self, call: &mut varlink::Call) -> io::Result<()> {
        let req = call.request.unwrap();
        match req.method.as_ref() {
            "org.example.more.Ping" => {
                if let Some(args) = req.parameters.clone() {
                    let args: PingArgs_ = serde_json::from_value(args)?;
                    return self.inner.ping(call as &mut _CallPing, args.ping);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "org.example.more.StopServing" => {
                return self.inner.stop_serving(call as &mut _CallStopServing);
            }
            "org.example.more.TestMore" => {
                if let Some(args) = req.parameters.clone() {
                    let args: TestMoreArgs_ = serde_json::from_value(args)?;
                    return self.inner.test_more(call as &mut _CallTestMore, args.n);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }

            m => {
                return call.reply_method_not_found(String::from(m));
            }
        }
    }
}
