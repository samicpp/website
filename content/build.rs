use rust_http::http1::handler::Http1Socket;
use rust_http::traits::HttpSocket;

use deno_core::extension;
use deno_core::op2;
use deno_core::snapshot::CreateSnapshotOptions;
use deno_core::snapshot::create_snapshot;
use deno_core::Extension;
use deno_core::OpDecl;
use deno_core::OpState;

use std::env;
use std::fs;
use std::path::PathBuf;
// use std::io;
use std::rc::Rc;
use std::cell::RefCell;

extension!(runjs_extension, ops = [op_call_rust,],);
#[op2(fast)]
fn op_call_rust(#[string] value: String) {
    println!("Received this value from JS: {value}");
}

#[op2(fast)]
fn test(#[string] s: String) {
    println!("test func called {}", s);
    // v8::Value::from(s.chars().rev().collect::<String>())
}

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
    conn.set_header(&name,&value);
    Ok(())
}

// #[op2(fast)]
// fn string_to_buff(#[string] value: String){
//     value.as_bytes()
// }

static _START_SCRIPT: &'static str=r#"
// const http=new class Http{
//   close(str){ return Deno.core.ops.http_close(str); }
//   write(str){ return Deno.core.ops.http_close(str); }
//   setHeader(name,value){ return Deno.core.ops.http_set_header(name,value); }
// }
"#;

fn main() {
    extension!(
      runjs_extension,
      // Must specify an entrypoint so that our module gets loaded while snapshotting:
      esm_entry_point = "sys:bootstrap",
      esm = [
        dir ".",
        "sys:bootstrap" = "bootstrap.js",
      ],
    );

    const DECL0:OpDecl=test();
    const DECL1:OpDecl=http_close();
    const DECL2:OpDecl=http_write();
    const DECL3:OpDecl=http_set_header();

    let options = CreateSnapshotOptions {
        cargo_manifest_dir: env!("CARGO_MANIFEST_DIR"),
        startup_snapshot: None,
        extensions: vec![
            Extension {
                name: "extensions",
                ops: std::borrow::Cow::Borrowed(&[DECL0, DECL1, DECL2, DECL3,]),
                ..Default::default()
            },
            runjs_extension::init(),
        ],
        with_runtime_cb: None,
        skip_op_registration: false,
        extension_transpiler: None,
    };
    let warmup_script = None;

    let snapshot = create_snapshot(options, warmup_script).expect("Error creating snapshot");

    // Save the snapshot for use by our source code:
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = out_dir.join("snapshot.bin");
    fs::write(file_path, snapshot.output).expect("Failed to write snapshot");

    // Let cargo know that builds depend on these files:
    for path in snapshot.files_loaded_during_snapshot {
        println!("cargo:rerun-if-changed={}", path.display());
    }
}
