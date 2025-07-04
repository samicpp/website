// use std::sync::Arc;

use deno_error;
use deno_ops::op2;
use deno_core::{
    snapshot::{ CreateSnapshotOptions, self, },
    *,
};
use rust_http::{http1::handler::Http1Socket, /*traits::HttpSocket*/};
use crate::structs::SharedData;

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

pub fn _run(_conn: &'static mut Http1Socket, shared: &'static SharedData, _code: &str)->Result<serde_json::Value,std::io::Error>{
    let snapshot = &shared.deno_snapshot;
    
    const DECL: OpDecl = op_sum();
    let ext0 = Extension {
        name: "my_ext",
        ops: std::borrow::Cow::Borrowed(&[DECL]),
        ..Default::default()
    };

    // conn.close(b"bytes");

    let mut _runtime = JsRuntime::new(RuntimeOptions { 
        startup_snapshot: Some(snapshot),
        extensions: vec![ext0],
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

    // println!("len {}",snapshot.len());

    Err(std::io::Error::new(std::io::ErrorKind::Unsupported,"run hasnt been written yet"))
}

