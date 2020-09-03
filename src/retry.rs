use std::cmp::min;

use rand::Rng;

use crate::Action;
use crate::{Error, ErrorCode};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RetryCategory {
    Retriable,
    Unretriable,
    Depends,
}

impl RetryCategory {
    pub fn calc(err: &Error) -> RetryCategory {
        match err.code {
            ErrorCode::ClientUnknown => RetryCategory::Unretriable,
            ErrorCode::CouldntResolveHost => RetryCategory::Retriable,
            ErrorCode::CouldntConnect => RetryCategory::Retriable,
            ErrorCode::OperationTimeout => RetryCategory::Depends,
            ErrorCode::SslHandshakeFail => RetryCategory::Unretriable,
            ErrorCode::WriteRequestFail => RetryCategory::Depends,
            ErrorCode::CorruptedResponse => RetryCategory::Depends,
            ErrorCode::NoAvailableConnection => RetryCategory::Retriable,
            ErrorCode::OTSUnknown => RetryCategory::Depends,
            ErrorCode::OTSOutOfColumnCountLimit => RetryCategory::Unretriable,
            ErrorCode::OTSObjectNotExist => RetryCategory::Unretriable,
            ErrorCode::OTSServerBusy => RetryCategory::Depends,
            ErrorCode::OTSCapacityUnitExhausted => RetryCategory::Retriable,
            ErrorCode::OTSTooFrequentReservedThroughputAdjustment => RetryCategory::Retriable,
            ErrorCode::OTSInternalServerError => RetryCategory::Depends,
            ErrorCode::OTSQuotaExhausted => RetryCategory::Depends,
            ErrorCode::OTSRequestBodyTooLarge => RetryCategory::Unretriable,
            ErrorCode::OTSTimeout => RetryCategory::Depends,
            ErrorCode::OTSObjectAlreadyExist => RetryCategory::Unretriable,
            ErrorCode::OTSTableNotReady => RetryCategory::Retriable,
            ErrorCode::OTSConditionCheckFail => RetryCategory::Unretriable,
            ErrorCode::OTSOutOfRowSizeLimit => RetryCategory::Unretriable,
            ErrorCode::OTSInvalidPK => RetryCategory::Unretriable,
            ErrorCode::OTSMethodNotAllowed => RetryCategory::Unretriable,
            ErrorCode::OTSAuthFailed => RetryCategory::Unretriable,
            ErrorCode::OTSServerUnavailable => RetryCategory::Depends,
            ErrorCode::OTSParameterInvalid => RetryCategory::Unretriable,
            ErrorCode::OTSRowOperationConflict => RetryCategory::Unretriable,
            ErrorCode::OTSPartitionUnavailable => RetryCategory::Retriable,
            ErrorCode::OTSMissingHeader => RetryCategory::Unretriable,
        }
    }

    pub fn determine_with_action(&self, act: Action) -> bool {
        match self {
            RetryCategory::Retriable => true,
            RetryCategory::Unretriable => false,
            RetryCategory::Depends => {
                match act {
                    Action::ListTable => true,
                    _ => false,
                }
            }
        }
    }
}

pub trait RetryStrategy {
    fn clone(&self) -> Box<dyn RetryStrategy + Send + Sync>;
    fn retries(&self) -> usize;
    fn next_pause(&mut self, act: Action, err: &Error) -> Option<std::time::Duration>;
}

impl Clone for Box<dyn RetryStrategy + Send + Sync> {
    fn clone(&self) -> Self {
        self.as_ref().clone()
    }
}


#[derive(Debug)]
pub struct DeadlineRetryStrategy {
    timeout: std::time::Duration, // not reset while cloning
    deadline: std::time::Instant, // reset while cloning
    retries: usize, // reset while cloning
    pause_base: std::time::Duration, // reset while cloning
}

const MAX_PAUSE: std::time::Duration = std::time::Duration::from_secs(10);

impl DeadlineRetryStrategy {
    pub fn new(timeout: std::time::Duration) -> DeadlineRetryStrategy {
        DeadlineRetryStrategy{
            timeout: timeout.clone(),
            deadline: std::time::Instant::now() + timeout,
            retries: 0,
            pause_base: std::time::Duration::from_millis(1),
        }
    }
}

impl RetryStrategy for DeadlineRetryStrategy {
    fn clone(&self) -> Box<dyn RetryStrategy + Send + Sync> {
        let timeout = self.timeout.clone();
        Box::new(DeadlineRetryStrategy{
            timeout: timeout.clone(),
            deadline: std::time::Instant::now() + timeout,
            retries: 0,
            pause_base: std::time::Duration::from_millis(1),
        })
    }

    fn retries(&self) -> usize {
        self.retries
    }

    fn next_pause(&mut self, act: Action, err: &Error) -> Option<std::time::Duration> {
        let should_retry = RetryCategory::calc(err).determine_with_action(act);
        if !should_retry {
            return None;
        }
        let now = std::time::Instant::now();
        if now > self.deadline {
            return None;
        }
        self.pause_base = min(self.pause_base * 2, MAX_PAUSE);
        let mut rng = rand::thread_rng();
        let pause = self.pause_base.as_micros() as u64;
        let half_pause = pause / 2;
        let next_pause = rng.gen_range(half_pause, pause);
        Some(std::time::Duration::from_micros(next_pause))
    }
}
