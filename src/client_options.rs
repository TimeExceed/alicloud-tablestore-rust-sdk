use crate::{RetryStrategy, DeadlineRetryStrategy};

#[derive(Clone)]
pub struct ClientOptions {
    pub concurrency: i64,
    pub retry_strategy: Box<dyn RetryStrategy + Send + Sync>,
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self{
            concurrency: 1000,
            retry_strategy: Box::new(DeadlineRetryStrategy::new(std::time::Duration::from_secs(300))),
        }
    }
}

impl std::fmt::Debug for ClientOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientOptions")
            .field("concurrency", &self.concurrency)
            .finish()
    }
}
