use deno_error;
use deno_ops::op2;
use deno_core::{*};
use rust_http::http1::handler::Http1Socket;
use crate::structs::SharedData;

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
    Err(e)=>Err(std::io::Error::new(std::io::ErrorKind::Other,"error occoured")),
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

pub fn _run(_conn: &mut Http1Socket, _shared: &SharedData, _code: &str)->Result<serde_json::Value,std::io::Error>{
    Err(std::io::Error::new(std::io::ErrorKind::Unsupported,"run hasnt been written yet"))
}
pub fn create_snapshot()->Vec<u8>{
    Vec::new()
}