use std::{error::Error, fmt::Debug, fmt::Display,
    // io
};

/// Basic, ubiquitous Result type for micmute and its MiMuError.
pub type MiMuResult<T> = Result<T, MiMuError<MiMuErrorKind>>;

/// Different error types that may be wrapped by a MiMuError.
pub enum MiMuWrappedError<Kind> {
    MiMu(Box<MiMuError<Kind>>),
    
    // io::Error currently serves as a sort of experimental test-dummy
    // in order to aid in developing the polymorphic aspects of everything
    // MiMuError related.
    // 
    // Io(io::Error)
}

impl<Kind: 'static> MiMuWrappedError<Kind> {
    fn unwrap_as_display(&self) -> &dyn Display {
        match self {
            MiMuWrappedError::MiMu(e) => e,
            // MiMuWrappedError::Io(e) => e,
        }
    }

    fn unwrap_as_debug(&self) -> &dyn Debug {
        match self {
            MiMuWrappedError::MiMu(e) => e,
            // MiMuWrappedError::Io(e) => e,
        }
    }

    fn unwrap_as_static_error(&self) -> &(dyn Error + 'static) {
        match self {
            MiMuWrappedError::MiMu(e) => e,
            // MiMuWrappedError::Io(e) => e,
            
        }
    }
}

impl<Kind: 'static> Display for MiMuWrappedError<Kind> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.unwrap_as_display())
    }
}

impl<Kind: 'static> Debug for MiMuWrappedError<Kind> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.unwrap_as_debug())
    }
}

impl<Kind: 'static> Error for MiMuWrappedError<Kind> {

    /// Calls .source on the error we're wrapping and returns the result.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.unwrap_as_static_error().source()
    }
}

pub enum MiMuErrorKind {
    Misc
}

/// Principal Error for anything micmute related.
pub struct MiMuError<Kind> {
    kind: Kind,
    msg: String,
    wrapped: Option<MiMuWrappedError<Kind>>
}

impl<Kind: 'static> Display for MiMuError<Kind> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.msg)
    }
}

impl<Kind: 'static> Debug for MiMuError<Kind> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:?}", self.msg, match &self.wrapped {
            Some(e) => format!("[WRAPPED ERROR]: {}", e),
            None => String::new(), // Empty string.
        })
    }
}

impl<Kind: 'static> MiMuError<Kind> {

    /// Create new MiMuError with a message.
    /// 
    /// This will set the wrapped error field to None.
    /// For wrapping errors, use ::wrap.
    pub fn new(kind: Kind, msg: &str) -> Self {
        Self {
            kind: kind,
            msg: msg.into(),
            wrapped: None
        }
    }

    /// Like ::new, but for wrapping third party errors.
    /// 
    /// This can only wrap errors for which MiMuWrappedError has a variant.
    pub fn wrap(kind: Kind, msg: &str, wrapped: MiMuWrappedError<Kind>)
    -> Self {
        Self {
            kind: kind,
            msg: msg.into(),
            wrapped: Some(wrapped)
        }
    }
}

impl<Kind: 'static> Error for MiMuError<Kind> {

    /// If we're wrapping an error, calls .source on it and returns
    /// the result.
    /// 
    /// Returns None if we're not wrapping an error (or if the wrapped)
    /// error's .source method returns None).
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.wrapped {
            Some(e) => e.source(),
            None => None,
        }
    }
}