use std::{error::Error, fmt::Debug, fmt::Display,
    // io
};

#[derive(Debug)]
pub enum MiMuErrorKind {
    Misc(&'static str)
}

impl From<&MiMuErrorKind> for String {
    fn from(kind: &MiMuErrorKind) -> Self {
        let msg = match kind {
            MiMuErrorKind::Misc(msg) => (*msg).into(),
        };
        msg
    }
}

impl Display for MiMuErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg: String = self.into();
        write!(f, "{}", msg)
    }
}

/// Basic, ubiquitous Result type for micmute and its MiMuError.
pub type MiMuResult<T> = Result<T, MiMuError>;

/// Different error types that may be wrapped by a MiMuError.
pub enum MiMuWrappedError {
    MiMu(Box<MiMuError>),
    
    // io::Error currently serves as a sort of experimental test-dummy
    // in order to aid in developing the polymorphic aspects of everything
    // MiMuError related.
    // 
    // Io(io::Error)
}

impl MiMuWrappedError {
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

impl Display for MiMuWrappedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.unwrap_as_display())
    }
}

impl Debug for MiMuWrappedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.unwrap_as_debug())
    }
}

impl Error for MiMuWrappedError {

    /// Calls .source on the error we're wrapping and returns the result.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.unwrap_as_static_error().source()
    }
}

/// Principal Error for anything micmute related.
pub struct MiMuError {
    kind: MiMuErrorKind,
    wrapped: Option<MiMuWrappedError>
}

impl Display for MiMuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

impl Debug for MiMuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:?}", self.kind, match &self.wrapped {
            Some(e) => format!("[WRAPPED ERROR]: {}", e),
            None => String::new(), // Empty string.
        })
    }
}

impl MiMuError {

    /// Create new MiMuError with a message.
    /// 
    /// This will set the wrapped error field to None.
    /// For wrapping errors, use ::wrap.
    pub fn new(kind: MiMuErrorKind) -> Self {
        Self {
            kind: kind,
            wrapped: None
        }
    }

    /// Like ::new, but for wrapping third party errors.
    /// 
    /// This can only wrap errors for which MiMuWrappedError has a variant.
    pub fn wrap(kind: MiMuErrorKind, wrapped: MiMuWrappedError) -> Self {
        Self {
            kind: kind,
            wrapped: Some(wrapped)
        }
    }
}

impl Error for MiMuError {

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