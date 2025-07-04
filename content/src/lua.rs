use mlua::{Lua, /*Result*/};
use tokio::task;

pub async fn run_simple(lua_code: &str) -> std::io::Result<String> {
    let code = lua_code.to_string();

    // Spawn a blocking task to run Lua code synchronously without blocking async runtime
    let result = task::spawn_blocking(move || {
        let lua = Lua::new();

        // Run the Lua code and expect a String return value
        match lua.load(&code).eval::<String>() {
            Ok(e)=>Ok(e),
            Err(e)=>Err(std::io::Error::new(std::io::ErrorKind::Other,e.to_string())),
        }
    }).await;

    match result{
        Ok(Ok(e))=>Ok(e),
        Ok(Err(e))=>Err(e),
        Err(_)=>Err(std::io::Error::new(std::io::ErrorKind::Other,"could create task")),
    }

    // result
}
