pub struct SharedData{
    pub mime: std::collections::HashMap<&'static str,&'static str>,
    pub serve_dir: String,
    pub deno_snapshot: Vec<u8>,
}