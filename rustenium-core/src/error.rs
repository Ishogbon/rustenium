use std::fmt;

/// Error codes that can be returned by the browser automation API
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorCode {
    /// Invalid argument provided to a command
    InvalidArgument,
    /// Invalid selector used to find an element
    InvalidSelector,
    /// Invalid session ID
    InvalidSessionId,
    /// Invalid web extension
    InvalidWebExtension,
    /// Move target is out of bounds
    MoveTargetOutOfBounds,
    /// No such alert exists
    NoSuchAlert,
    /// No such element exists
    NoSuchElement,
    /// No such frame exists
    NoSuchFrame,
    /// No such handle exists
    NoSuchHandle,
    /// No such history entry exists
    NoSuchHistoryEntry,
    /// No such intercept exists
    NoSuchIntercept,
    /// No such node exists
    NoSuchNode,
    /// No such request exists
    NoSuchRequest,
    /// No such script exists
    NoSuchScript,
    /// No such storage partition exists
    NoSuchStoragePartition,
    /// No such user context exists
    NoSuchUserContext,
    /// No such web extension exists
    NoSuchWebExtension,
    /// Session could not be created
    SessionNotCreated,
    /// Unable to capture screen
    UnableToCaptureScreen,
    /// Unable to close browser
    UnableToCloseBrowser,
    /// Unable to set cookie
    UnableToSetCookie,
    /// Unable to set file input
    UnableToSetFileInput,
    /// Underspecified storage partition
    UnderspecifiedStoragePartition,
    /// Unknown command
    UnknownCommand,
    /// Unknown error occurred
    UnknownError,
    /// Unsupported operation
    UnsupportedOperation,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCode::InvalidArgument => write!(f, "invalid argument"),
            ErrorCode::InvalidSelector => write!(f, "invalid selector"),
            ErrorCode::InvalidSessionId => write!(f, "invalid session id"),
            ErrorCode::InvalidWebExtension => write!(f, "invalid web extension"),
            ErrorCode::MoveTargetOutOfBounds => write!(f, "move target out of bounds"),
            ErrorCode::NoSuchAlert => write!(f, "no such alert"),
            ErrorCode::NoSuchElement => write!(f, "no such element"),
            ErrorCode::NoSuchFrame => write!(f, "no such frame"),
            ErrorCode::NoSuchHandle => write!(f, "no such handle"),
            ErrorCode::NoSuchHistoryEntry => write!(f, "no such history entry"),
            ErrorCode::NoSuchIntercept => write!(f, "no such intercept"),
            ErrorCode::NoSuchNode => write!(f, "no such node"),
            ErrorCode::NoSuchRequest => write!(f, "no such request"),
            ErrorCode::NoSuchScript => write!(f, "no such script"),
            ErrorCode::NoSuchStoragePartition => write!(f, "no such storage partition"),
            ErrorCode::NoSuchUserContext => write!(f, "no such user context"),
            ErrorCode::NoSuchWebExtension => write!(f, "no such web extension"),
            ErrorCode::SessionNotCreated => write!(f, "session not created"),
            ErrorCode::UnableToCaptureScreen => write!(f, "unable to capture screen"),
            ErrorCode::UnableToCloseBrowser => write!(f, "unable to close browser"),
            ErrorCode::UnableToSetCookie => write!(f, "unable to set cookie"),
            ErrorCode::UnableToSetFileInput => write!(f, "unable to set file input"),
            ErrorCode::UnderspecifiedStoragePartition => write!(f, "underspecified storage partition"),
            ErrorCode::UnknownCommand => write!(f, "unknown command"),
            ErrorCode::UnknownError => write!(f, "unknown error"),
            ErrorCode::UnsupportedOperation => write!(f, "unsupported operation"),
        }
    }
}

impl std::error::Error for ErrorCode {}

/// Parse a string into an ErrorCode
pub fn parse_error_code(code: &str) -> Option<ErrorCode> {
    match code {
        "invalid argument" => Some(ErrorCode::InvalidArgument),
        "invalid selector" => Some(ErrorCode::InvalidSelector),
        "invalid session id" => Some(ErrorCode::InvalidSessionId),
        "invalid web extension" => Some(ErrorCode::InvalidWebExtension),
        "move target out of bounds" => Some(ErrorCode::MoveTargetOutOfBounds),
        "no such alert" => Some(ErrorCode::NoSuchAlert),
        "no such element" => Some(ErrorCode::NoSuchElement),
        "no such frame" => Some(ErrorCode::NoSuchFrame),
        "no such handle" => Some(ErrorCode::NoSuchHandle),
        "no such history entry" => Some(ErrorCode::NoSuchHistoryEntry),
        "no such intercept" => Some(ErrorCode::NoSuchIntercept),
        "no such node" => Some(ErrorCode::NoSuchNode),
        "no such request" => Some(ErrorCode::NoSuchRequest),
        "no such script" => Some(ErrorCode::NoSuchScript),
        "no such storage partition" => Some(ErrorCode::NoSuchStoragePartition),
        "no such user context" => Some(ErrorCode::NoSuchUserContext),
        "no such web extension" => Some(ErrorCode::NoSuchWebExtension),
        "session not created" => Some(ErrorCode::SessionNotCreated),
        "unable to capture screen" => Some(ErrorCode::UnableToCaptureScreen),
        "unable to close browser" => Some(ErrorCode::UnableToCloseBrowser),
        "unable to set cookie" => Some(ErrorCode::UnableToSetCookie),
        "unable to set file input" => Some(ErrorCode::UnableToSetFileInput),
        "underspecified storage partition" => Some(ErrorCode::UnderspecifiedStoragePartition),
        "unknown command" => Some(ErrorCode::UnknownCommand),
        "unknown error" => Some(ErrorCode::UnknownError),
        "unsupported operation" => Some(ErrorCode::UnsupportedOperation),
        _ => None,
    }
} 