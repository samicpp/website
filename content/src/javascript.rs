// use std::sync::Arc;

use deno_error;
use deno_ops::op2;
use deno_core::{
    snapshot::{ CreateSnapshotOptions, self, },
    *,
};
use rust_http::{http1::handler::Http1Socket, traits::HttpSocket};
use crate::structs::SharedData;
use std::rc::Rc;
use std::cell::RefCell;

// obtained from https://github.com/denoland/deno_core/blob/HEAD/core/examples/hello_world.rs
/// An op for summing an array of numbers. The op-layer automatically
/// deserializes inputs and serializes the returned Result & value.
#[op2]
fn op_sum(#[serde] nums: Vec<f64>) -> Result<f64, deno_error::JsErrorBox> {
  // Sum inputs
  let sum = nums.iter().fold(0.0, |a, v| a + v);
  // return as a Result<f64, OpError>
  Ok(sum)
}

pub async fn run_simple(code: &str) -> std::io::Result<String> {
    // Build a deno_core::Extension providing custom ops
    const DECL: OpDecl = op_sum();
    let ext = Extension {
        name: "my_ext",
        ops: std::borrow::Cow::Borrowed(&[DECL]),
        ..Default::default()
    };

    // Initialize a runtime instance
    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![ext],
        ..Default::default()
    });

    let output = eval(&mut runtime, code);

    match output{
        Ok(res)=>{
            let s=if let Some(v)=res.as_str(){v}else{""};
            let s=s.to_owned();

            std::result::Result::Ok(s)
        },
        Err(e)=>Err(std::io::Error::new(std::io::ErrorKind::Other,e)),
    }
}

fn eval(context: &mut JsRuntime, code: &str) -> Result<serde_json::Value, String> {
    let res = context.execute_script("<anon>", code.to_owned());
    match res {
        Ok(global) => {
            let scope = &mut context.handle_scope();
            let local = v8::Local::new(scope, global);
            // Deserialize a `v8` object into a Rust type using `serde_v8`,
            // in this case deserialize to a JSON `Value`.
            let deserialized_value =
            serde_v8::from_v8::<serde_json::Value>(scope, local);

            match deserialized_value {
                Ok(value) => Ok(value),
                Err(err) => Err(format!("Cannot deserialize value: {err:?}")),
            }
        }
        Err(err) => Err(format!("Evaling error: {err:?}")),
    }
}

pub fn create_snapshot()->std::io::Result<Vec<u8>>{
    let options = CreateSnapshotOptions {
        cargo_manifest_dir: env!("CARGO_MANIFEST_DIR"),
        startup_snapshot: None,
        extensions: vec![],//vec![runjs_extension::init()],
        with_runtime_cb: None,
        skip_op_registration: false,
        extension_transpiler: None,
    };
    let warmup_script = None;

    let snapshot = snapshot::create_snapshot(options, warmup_script);

    match snapshot {
        Ok(v)=>Ok(v.output.to_vec()),
        Err(e)=>Err(std::io::Error::new(std::io::ErrorKind::Other,e.to_string())),
    }
}

#[derive(Clone)]
struct AllowAllTimers;
impl deno_web::TimersPermission for AllowAllTimers {
    fn allow_hrtime(&mut self) -> bool {
        true
    }
}

#[op2(async)]
async fn http_close(state: Rc<RefCell<OpState>>, #[serde] data: Vec<u8>) -> Result<(), deno_error::JsErrorBox> {
    let mut state=state.borrow_mut();
    let rc_conn = state.borrow_mut::<std::rc::Rc<std::cell::RefCell<Http1Socket>>>().clone();
    let mut conn = rc_conn.borrow_mut();
    let _ = conn.close(&data).await;
    Ok(())
}

pub async fn run(conn: Http1Socket, shared: &SharedData, code: &str)->Result<serde_json::Value,std::io::Error>{
    let _snapshot = std::sync::Arc::clone(&shared.deno_snapshot);
    let rc_conn=std::rc::Rc::new(std::cell::RefCell::new(conn));
    let rc_conn_clone = rc_conn.clone();


    const DECL: OpDecl = http_close();
    let ext0 = Extension {
        name: "my_ext",
        ops: std::borrow::Cow::Borrowed(&[DECL]),
        op_state_fn: Some(Box::new(move |state| {
            state.put(rc_conn_clone.clone());
        })),

        ..Default::default()
    };
    // let native_ext=vec![
    //     deno_web::deno_web::lazy_init(),
    // ];

    // let mut exts=vec![ext0];
    
    // conn.close(b"bytes");

    let mut runtime = JsRuntime::new(RuntimeOptions { 
        // startup_snapshot: Some(snapshot),
        extensions: vec![
            ext0,
            deno_webidl::deno_webidl::lazy_init(),
            deno_console::deno_console::lazy_init(),
            deno_url::deno_url::lazy_init(),
            deno_web::deno_web::lazy_init::<AllowAllTimers>(),
        ],
        ..Default::default()
        // module_loader: (), 
        // extension_code_cache: (), 
        // extension_transpiler: (), 
        // op_metrics_factory_fn: (), 
        // extensions: (), 
        // startup_snapshot: (), 
        // skip_op_registration: (), 
        // create_params: (), 
        // v8_platform: (), 
        // shared_array_buffer_store: (), 
        // compiled_wasm_module_store: (), 
        // inspector: (), 
        // is_main: (), 
        // validate_import_attributes_cb: (), 
        // wait_for_inspector_disconnect_callback: (), 
        // custom_module_evaluation_cb: (), 
        // eval_context_code_cache_cbs: (), 
        // import_assertions_support: (), 
        // maybe_op_stack_trace_callback: () 
    });

    let _ = runtime.execute_script("<bootstrap>", r#"
import "ext:deno_web/08_text_encoding.js";
import "ext:deno_webidl/00_webidl.js";
import "ext:deno_web/01_dom_exception.js";
import "ext:deno_web/02_event.js";
import "ext:deno_web/03_abort_signal.js";
import "ext:deno_web/04_global_interfaces.js";
import "ext:deno_web/05_base64.js";
import "ext:deno_web/06_streams.js";
import "ext:deno_web/09_file.js";
import "ext:deno_web/10_filereader.js";
import "ext:deno_web/13_message_port.js";
import "ext:deno_web/14_compression.js";
import "ext:deno_web/15_performance.js";
import "ext:deno_web/16_image_data.js";
import "ext:deno_web/00_infra.js";
import "ext:deno_web/02_structured_clone.js";
import "ext:deno_web/01_mimesniff.js";
import "ext:deno_url/00_url.js";
import "ext:deno_url/01_urlpattern.js";
import "ext:deno_console/01_console.js";
import "ext:deno_web/02_timers.js";
import "ext:deno_web/12_location.js";
"#);
    let res: Result<v8::Global<v8::Value>, error::CoreError> = runtime.execute_script("<anon>", code.to_owned());

    match res{
        Ok(global)=>{
            let scope=&mut runtime.handle_scope();
            let local=v8::Local::new(scope, global);
            let res=serde_v8::from_v8::<serde_json::Value>(scope, local);
            
            match res{
                Ok(t)=>Ok(t),
                Err(_err)=>Err(std::io::Error::new(std::io::ErrorKind::Other,"could not deserialize code result")),
            }
        },
        Err(err)=>Err(std::io::Error::new(std::io::ErrorKind::Other,err.to_string())),
    }

    // println!("len {}",snapshot.len());

    // Err(std::io::Error::new(std::io::ErrorKind::Unsupported,"run hasnt been written yet"))
}

