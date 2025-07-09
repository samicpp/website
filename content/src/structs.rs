use std::sync::Arc;

#[derive(Clone,Debug)]
pub struct SharedData{
    pub mime: std::collections::HashMap<&'static str,&'static str>,
    pub serve_dir: String,
    pub _deno_snapshot: Arc<Vec<u8>>,
}