// use std::sync::Arc;

// use deno_core::{
//     op, Extension, JsRuntime, OpState, RuntimeOptions, Snapshot, SnapshotCreator, anyhow::Error,
// };
// use serde_json::json;
use rust_http::http1::handler::Http1Socket;
use crate::structs::SharedData;
// use rust_http::client::HttpClient;


pub fn _run(_conn: &mut Http1Socket, _shared: &SharedData)->Result<serde_json::Value,std::io::Error>{
    Err(std::io::Error::new(std::io::ErrorKind::Unsupported,"run hasnt been written yet"))
}
pub fn create_snapshot()->Vec<u8>{
    Vec::new()
}