use super::*;

/// IGC file record type representing a single line
#[derive(Debug)]
pub enum Record {
    /// FR manufacturer and FR serial no.
    A,

    /// Fix
    B(BRecord),

    /// Task/declaration
    C,

    /// Differential GPS
    D,

    /// Event
    E,

    /// Satellite constellation
    F,

    /// Security
    G,

    /// File header
    H,

    /// List of additional data included at end of each B-record
    I(IRecord),

    /// List of additional data included at end of each K-record
    J(JRecord),

    /// Frequent data, additional to the B-record
    K,

    /// Logbook/comments
    L,

    /// Empty line
    Empty,
}
