use crate::{ 
    structs::SharedData,
};
use deno_core::Extension;
use deno_core::OpDecl;
use deno_core::OpState;
use rust_http::traits::HttpSocket;
use rust_http::{
    http1::handler::Http1Socket,
    // traits::HttpSocket,
};

use std::io;
use std::rc::Rc;
use std::cell::RefCell;

use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_core::v8;
use deno_core::serde_v8;
// use deno_core::serde;
use deno_core::op2;
// use serde_json::Value;

#[op2(fast)]
fn test(#[string] s: String) {
    println!("test func called {}",s);
    // v8::Value::from(s.chars().rev().collect::<String>())
}

// #[op2]
// #[string]
// fn test_string()->String{
//     "string content".to_owned()
// }

#[op2(async)]
async fn http_close(state: Rc<RefCell<OpState>>, #[string] response: String)->Result<(),deno_error::JsErrorBox>{
    let mut state=state.borrow_mut();
    let conn=state.borrow_mut::<Rc<RefCell<Http1Socket>>>();
    let mut conn=conn.borrow_mut();
    let _=conn.close(response.as_bytes()).await;
    Ok(())
}

#[op2(async)]
async fn http_write(state: Rc<RefCell<OpState>>, #[string] content: String)->Result<(),deno_error::JsErrorBox>{
    let mut state=state.borrow_mut();
    let conn=state.borrow_mut::<Rc<RefCell<Http1Socket>>>();
    let mut conn=conn.borrow_mut();
    let _=conn.write(content.as_bytes()).await;
    Ok(())
}

#[op2(fast)]
fn http_set_header(state: &mut OpState, #[string] name: String, #[string] value: String)->Result<(),deno_error::JsErrorBox>{
    // let mut state=state.borrow_mut();
    let conn=state.borrow_mut::<Rc<RefCell<Http1Socket>>>();
    let mut conn=conn.borrow_mut();
    let _=conn.set_header(&name,&value);
    Ok(())
}

pub async fn run(name: String, source: &[u8], conn: Http1Socket, shared: &SharedData)->io::Result<serde_json::Value>{
    let script=if let Ok(s)=str::from_utf8(source){s.to_owned()}else{"".to_owned()};
    let rc_conn = Rc::new(RefCell::new(conn));
    // let name=name.to_owned();
    const DECL0:OpDecl=test();
    const DECL1:OpDecl=http_close();
    const DECL2:OpDecl=http_write();
    const DECL3:OpDecl=http_set_header();
    let mut runtime = JsRuntime::new(RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        startup_snapshot: Some(shared.deno_snapshot),
        extensions: vec![
            Extension {
                name: "extensions",
                ops: std::borrow::Cow::Borrowed(&[
                    DECL0,
                    DECL1,
                    DECL2,
                    DECL3,
                ]),
                ..Default::default()
            },
        ],
        ..Default::default()
    });

    runtime.op_state().borrow_mut().put::<Rc<RefCell<Http1Socket>>>(rc_conn);

    match runtime.execute_script(name, script){
        Ok(result)=>{
            // let _=runtime.run_event_loop(deno_core::PollEventLoopOptions::default()).await;
            let scope=&mut runtime.handle_scope();
            let loc: v8::Local<'_, v8::Value>=v8::Local::new(scope, result);
            match serde_v8::from_v8(scope, loc){ Ok(s)=>Ok(s), Err(e)=>Err(std::io::Error::new(std::io::ErrorKind::Other,format!("{:?}",e))) }
        },
        Err(e)=>Err(std::io::Error::new(std::io::ErrorKind::Other,format!("{:?}",e))),
    }
    // Err(std::io::Error::new(std::io::ErrorKind::Unsupported,"run hasnt been written yet"))
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

