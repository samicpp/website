pub struct SharedData{
    pub mime: std::collections::HashMap<&'static str,&'static str>,
    pub serve_dir: String,
}