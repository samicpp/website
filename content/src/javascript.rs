#![allow(clippy::print_stdout)]
#![allow(clippy::print_stderr)]

use crate::module_loader;

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use deno_runtime::deno_core::error::AnyError;
use deno_runtime::deno_core::op2;
use deno_runtime::deno_core::ModuleSpecifier;
use deno_runtime::deno_fs::RealFs;
use deno_runtime::deno_permissions::set_prompter;
use deno_runtime::deno_permissions::PermissionPrompter;
use deno_runtime::deno_permissions::Permissions;
use deno_runtime::deno_permissions::PermissionsContainer;
use deno_runtime::deno_permissions::PromptResponse;
use deno_runtime::permissions::RuntimePermissionDescriptorParser;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::worker::WorkerServiceOptions;

use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_core::v8;

use colored::*;

use module_loader::TypescriptModuleLoader;

#[op2]
#[string]
fn example_custom_op(#[string] text: &str) -> String {
    println!("Hello {} from an op!", text);
    text.to_string() + " from Rust!"
}

deno_runtime::deno_core::extension!(
  example_extension,
  ops = [example_custom_op],
  esm_entry_point = "ext:example_extension/bootstrap.js",
  esm = [dir "src", "bootstrap.js"]
);

struct CustomPrompter;

impl PermissionPrompter for CustomPrompter {
    fn prompt(&mut self, _message: &str, _name: &str, _api_name: Option<&str>, _is_unary: bool) -> PromptResponse {
        PromptResponse::AllowAll
    }
}


pub fn _run(_conn: &mut Http1Socket, _shared: &SharedData)->Result<serde_json::Value,std::io::Error>{
    Err(std::io::Error::new(std::io::ErrorKind::Unsupported,"run hasnt been written yet"))
}

pub fn create_snapshot()->Vec<u8>{
    Vec::new()
}

pub async fn run_simple(name: &str, code: &str) -> std::io::Result<(JsRuntime,serde_json::Value)>{
    let mut runtime = JsRuntime::new(RuntimeOptions::default());
    let res = runtime.execute_script(name, code);
    match res {
        Ok(global) => {
            let scope = &mut runtime.handle_scope();
            let local = v8::Local::new(scope, global);
            // Deserialize a `v8` object into a Rust type using `serde_v8`,
            // in this case deserialize to a JSON `Value`.
            let deserialized_value =
            serde_v8::from_v8::<serde_json::Value>(scope, local);

            match deserialized_value {
                Ok(value) => Ok((runtime,value)),
                Err(err) => Err(format!("Cannot deserialize value: {err:?}")),
            }
        }
        Err(err) => Err(format!("Evaling error: {err:?}")),
    }
}

/*pub async fn _run2(path: &str){
    let js_path = Path::new(path);
    let main_module = ModuleSpecifier::from_file_path(js_path).unwrap();

    let source_map_store = Rc::new(RefCell::new(HashMap::new()));

    let fs = Arc::new(RealFs);
    let permission_desc_parser = Arc::new(RuntimePermissionDescriptorParser::new(fs.clone()));
    let permission_container = PermissionsContainer::new(permission_desc_parser, Permissions::none_with_prompt());

    set_prompter(Box::new(CustomPrompter));

    let mut worker = MainWorker::bootstrap_from_options(
        main_module.clone(),
        WorkerServiceOptions {
            module_loader: Rc::new(TypescriptModuleLoader {
                source_maps: source_map_store,
            }),
            // File-only loader
            // module_loader: Rc::new(FsModuleLoader),
            permissions: permission_container,
            blob_store: Default::default(),
            broadcast_channel: Default::default(),
            feature_checker: Default::default(),
            node_services: Default::default(),
            npm_process_state_provider: Default::default(),
            root_cert_store_provider: Default::default(),
            shared_array_buffer_store: Default::default(),
            compiled_wasm_module_store: Default::default(),
            v8_code_cache: Default::default(),
            fs,
        },
        WorkerOptions {
            extensions: vec![example_extension::init_ops_and_esm()],
            ..Default::default()
        },
    );
    worker.execute_main_module(&main_module).await?;
    worker.run_event_loop(false).await?;

    println!("Exit code: {}", worker.exit_code());

    Ok(())
}*/

