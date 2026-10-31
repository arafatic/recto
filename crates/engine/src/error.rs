//! The errors editing and combining report.

use crate::measure;

/// Why a file can't be a source for Combine Files (or Insert / Replace pages).
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum SourceProblem {
    #[error("it is password-protected")]
    Password,
    #[error("the password is wrong")]
    WrongPassword,
    #[error("its security settings don't allow copying pages")]
    NotPermitted,
    #[error("{0}")]
    Unreadable(String),
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum EditError {
    #[error("no such document")]
    NoDocument,
    #[error("this document can't be edited: {0}")]
    ReadOnly(String),
    #[error("the document's security settings don't allow {0}; open it with the owner password to make this change")]
    NotPermitted(&'static str),
    #[error("{0}")]
    Organize(#[from] recto_organize::OrganizeError),
    #[error("{0}")]
    Bookmark(#[from] recto_organize::OutlineError),
    #[error("{0}")]
    Comment(#[from] recto_annot::AnnotError),
    #[error(transparent)]
    Measure(#[from] measure::MeasureError),
    #[error("{0}")]
    Protection(String),
    #[error("the password is incorrect")]
    WrongPassword,
    #[error("{0}")]
    Form(#[from] recto_forms::FormError),
    #[error(transparent)]
    Redact(#[from] recto_redact::RedactError),
    #[error("{0}")]
    Print(String),
    #[error("{0}")]
    Accessibility(String),
    #[error(transparent)]
    Data(#[from] recto_xfdf::DataError),
    #[error("{0}")]
    Edit(#[from] recto_edit::EditError),
    #[error("{0}")]
    Create(#[from] recto_create::CreateError),
    #[error("the edited document could not be written: {0}")]
    Write(String),
    #[error("the edited document could not be reopened: {0}")]
    Reopen(String),
    #[error("couldn't use {0}")]
    Source(String),
    #[error("nothing to undo")]
    NothingToUndo,
    #[error("nothing to redo")]
    NothingToRedo,
    #[error("{0}")]
    Sign(String),
    #[error("{0}")]
    Optimize(String),
    #[error("cancelled")]
    Cancelled,
    #[error("{0} isn't possible in a signed document: it would rewrite the file and invalidate the signatures")]
    SignedRewrite(String),
    #[error("this document is signed: rewriting it would invalidate its signatures (save it incrementally instead)")]
    Signed,
    #[error("{0}")]
    Invalid(String),
}

impl From<recto_sign::SignError> for EditError {
    fn from(e: recto_sign::SignError) -> Self {
        EditError::Sign(e.to_string())
    }
}
