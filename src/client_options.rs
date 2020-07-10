#[derive(Debug, Clone)]
pub struct ClientOptions {
    pub concurrency: i64,
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self{
            concurrency: 1000,
        }
    }
}
