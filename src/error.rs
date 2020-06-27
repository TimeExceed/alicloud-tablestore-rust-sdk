use crate::protocol;
use quick_protobuf::{MessageRead, BytesReader};
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct Error {
    pub code: ErrorCode,
    pub message: String,
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    NetworkUnknown,
    CouldntResolveHost,
    CouldntConnect,
    OperationTimeout,
    SslHandshakeFail,
    WriteRequestFail,
    CorruptedResponse,
    NoAvailableConnection,

    OTSUnknown,
    OTSOutOfColumnCountLimit,
    OTSObjectNotExist,
    OTSServerBusy,
    OTSCapacityUnitExhausted,
    OTSTooFrequentReservedThroughputAdjustment,
    OTSInternalServerError,
    OTSQuotaExhausted,
    OTSRequestBodyTooLarge,
    OTSTimeout,
    OTSObjectAlreadyExist,
    OTSTableNotReady,
    OTSConditionCheckFail,
    OTSOutOfRowSizeLimit,
    OTSInvalidPK,
    OTSMethodNotAllowed,
    OTSAuthFailed,
    OTSServerUnavailable,
    OTSParameterInvalid,
    OTSRowOperationConflict,
    OTSPartitionUnavailable,
    OTSMissingHeader,
}

impl From<hyper::Error> for Error {
    fn from(h: hyper::Error) -> Error {
        let ec = match &h {
            x if x.is_parse() => ErrorCode::CorruptedResponse,
            x if x.is_user() => ErrorCode::CorruptedResponse,
            x if x.is_canceled() => ErrorCode::WriteRequestFail,
            x if x.is_closed() => ErrorCode::WriteRequestFail,
            x if x.is_connect() => ErrorCode::CouldntConnect,
            x if x.is_incomplete_message() => ErrorCode::CorruptedResponse,
            x if x.is_body_write_aborted() => ErrorCode::WriteRequestFail,
            x if x.is_timeout() => ErrorCode::OperationTimeout,
            _ => ErrorCode::NetworkUnknown,
        };
        Error{
            code: ec,
            message: h.to_string(),
        }
    }
}

impl From<quick_protobuf::errors::Error> for Error {
    fn from(qe: quick_protobuf::errors::Error) -> Error {
        Error{
            code: ErrorCode::CorruptedResponse,
            message: qe.to_string(),
        }
    }
}

impl From<http::Error> for Error {
    fn from(he: http::Error) -> Self {
        Error{
            code: ErrorCode::WriteRequestFail,
            message: he.to_string(),
        }
    }
}

impl From<http::header::InvalidHeaderValue> for Error {
    fn from(he: http::header::InvalidHeaderValue) -> Self {
        Error{
            code: ErrorCode::WriteRequestFail,
            message: he.to_string(),
        }
    }
}

impl From<crate::protocol::Error> for Error {
    fn from(x: crate::protocol::Error) -> Error {
        let ec = match x.code.as_str() {
            "OTSOutOfColumnCountLimit" => ErrorCode::OTSOutOfColumnCountLimit,
            "OTSObjectNotExist" => ErrorCode::OTSObjectNotExist,
            "OTSServerBusy" => ErrorCode::OTSServerBusy,
            "OTSCapacityUnitExhausted" => ErrorCode::OTSCapacityUnitExhausted,
            "OTSTooFrequentReservedThroughputAdjustment"   
                => ErrorCode::OTSTooFrequentReservedThroughputAdjustment,
            "OTSInternalServerError" => ErrorCode::OTSInternalServerError,
            "OTSQuotaExhausted" => ErrorCode::OTSQuotaExhausted,
            "OTSRequestBodyTooLarge" => ErrorCode::OTSRequestBodyTooLarge,
            "OTSTimeout" => ErrorCode::OTSTimeout,
            "OTSObjectAlreadyExist" => ErrorCode::OTSObjectAlreadyExist,
            "OTSTableNotReady" => ErrorCode::OTSTableNotReady,
            "OTSConditionCheckFail" => ErrorCode::OTSConditionCheckFail,
            "OTSOutOfRowSizeLimit" => ErrorCode::OTSOutOfRowSizeLimit,
            "OTSInvalidPK" => ErrorCode::OTSInvalidPK,
            "OTSRequestTimeout" => ErrorCode::OTSTimeout,
            "OTSMethodNotAllowed" => ErrorCode::OTSMethodNotAllowed,
            "OTSAuthFailed" => ErrorCode::OTSAuthFailed,
            "OTSServerUnavailable" => ErrorCode::OTSServerUnavailable,
            "OTSParameterInvalid" => ErrorCode::OTSParameterInvalid,
            "OTSRowOperationConflict" => ErrorCode::OTSRowOperationConflict,
            "OTSPartitionUnavailable" => ErrorCode::OTSPartitionUnavailable,
            "OTSMissingHeader" => ErrorCode::OTSMissingHeader,
            _ => ErrorCode::OTSUnknown,
        };
        let mut err = Error{
            code: ec,
            message: String::new(),
        };
        if let Some(msg) = x.message {
            err.message = msg;
        }
        err
    }
}

impl TryFrom<&[u8]> for Error {
    type Error = Error;

    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BytesReader::from_bytes(v);
        let error = protocol::Error::from_reader(&mut reader, v)?;
        Err(error.into())
    }
}
