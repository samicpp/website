use crate::{ 
    structs::SharedData,
};
use rust_http::{
    http1::handler::Http1Socket,
    // traits::HttpSocket,
};

use std::io;

use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_core::v8;

pub fn _run(_conn: &mut Http1Socket, _shared: &SharedData)->Result<serde_json::Value,std::io::Error>{
    Err(std::io::Error::new(std::io::ErrorKind::Unsupported,"run hasnt been written yet"))
}

pub fn _create_snapshot()->Vec<u8>{
    Vec::new()
}

pub async fn run_simple(name: &str, code: &str) -> io::Result<serde_json::Value>{
    let mut runtime = JsRuntime::new(RuntimeOptions::default());
    let res = runtime.execute_script(name.to_owned(), code.to_owned());
    match res {
        Ok(global) => {
            let scope = &mut runtime.handle_scope();
            let local = v8::Local::new(scope, global);
            // Deserialize a `v8` object into a Rust type using `serde_v8`,
            // in this case deserialize to a JSON `Value`.
            let deserialized_value =
            deno_core::serde_v8::from_v8::<serde_json::Value>(scope, local);

            match deserialized_value {
                Ok(value) => Ok(value),
                Err(err) => Err(io::Error::new(io::ErrorKind::Other,format!("Cannot deserialize value: {err:?}"))),
            }
        }
        Err(err) => Err(io::Error::new(io::ErrorKind::Other,format!("Evaling error: {err:?}"))),
    }
}

