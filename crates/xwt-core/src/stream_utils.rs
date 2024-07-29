//! Utilitites for the WebTransport streams.

use crate::stream;

/// A shortcut for the error type for a given [`stream::Read`] type.
pub type ReadErrorFor<T> = <T as stream::Read>::Error;

/// A shortcut for the error type for a given [`stream::Write`] type.
pub type WriteErrorFor<T> = <T as stream::Write>::Error;

/// A shortcut for the error type for a given [`stream::ReadChunk] type.
pub type ReadChunkErrorFor<T, Data> = <T as stream::ReadChunk<Data>>::Error;

/// A shortcut for the error type for a given [`stream::WriteChunk] type.
pub type WriteChunkErrorFor<T, Data> = <T as stream::WriteChunk<Data>>::Error;
