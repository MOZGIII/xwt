//! Operations on the WebTransport streams.

use core::future::Future;
use core::num::NonZeroUsize;

use crate::utils::{maybe, Error};

/// The error code that can be passed when aborting the send or receive side
/// of the stream.
///
/// Some WebTransport implementations allow not specifying the error codes
/// explicitly - this is equivalent to passing the error code `0`.
pub type ErrorCode = u32;

/// Returns the error code for the stream operation error when the error
/// originates from a stream abort.
/// If an error
///
/// Useful to read the error from when the stream send side or receive side is
/// aborted.
///
/// If no error code was passed, this call returns `0`.
pub trait ErrorAsErrorCode: maybe::Send {
    /// Returns the stream error code.
    ///
    /// If error is an internal implementation error as opposed to
    /// a protocol error - returns `None`.
    fn as_error_code(&self) -> Option<ErrorCode>;
}

/// Read the data from the stream.
pub trait Read: maybe::Send {
    /// An error that can occur while reading the stream.
    type Error: Error + ErrorAsErrorCode + maybe::Send + maybe::Sync + 'static;

    /// Read the data from the stream into a given buffer and return the amount
    /// of bytes filled in the buffer or `None` if the stream is closed and does
    /// not have any pending unread data.
    ///
    /// A would-be zero-length read should be reported as a error
    /// with a zero error code.
    /// There is a handy way to check that condition:
    ///
    /// ```ignore
    /// let result = stream.read(buf).await;
    /// let data = match result {
    ///     Ok(read) => buf[..read.get()],
    ///     Err(err) if err.is_closed() => {
    ///         // Stream ended.
    ///         return;
    ///     }
    ///     Err(err) => {
    ///         panic!("something went wrong");
    ///     }
    /// };
    /// ```
    fn read(
        &mut self,
        buf: &mut [u8],
    ) -> impl Future<Output = Result<NonZeroUsize, Self::Error>> + maybe::Send;
}

/// Write the data to a stream.
pub trait Write: maybe::Send {
    /// An error that can occur while writing to the stream.
    type Error: Error + ErrorAsErrorCode + maybe::Send + maybe::Sync + 'static;

    /// Write the data from the given buffer into the stream, returning
    /// the amount of of bytes that were successfully written into the stream.
    /// If the returned amount is smaller than the size of the data that was
    /// being written, the user should try writing the remaining data again.
    fn write(
        &mut self,
        buf: &[u8],
    ) -> impl Future<Output = Result<NonZeroUsize, Self::Error>> + maybe::Send;
}

/// Abort the read stream.
///
/// Sends a signal to the peer that the read side of the stream has been
/// aborted.
/// Discards the receive buffer; the peer is typically expected to abort
/// the corresponding send side in response.
///
/// An unsigned 8-bit error code can be supplied as a part of the signal
/// to the peer; if omitted, the error code is presumed to be `0`.
pub trait ReadAbort: maybe::Send {
    /// An error that can occur while stopping the stream.
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    /// Abort the stream.
    fn abort(
        self,
        error_code: ErrorCode,
    ) -> impl Future<Output = Result<(), Self::Error>> + maybe::Send;
}

/// Abort the write stream.
///
/// Sends a signal to the peer that the write side of the stream has been
/// aborted.
/// Discards the send buffer; if possible, no currently outstanding data
/// is transmitted or retransmitted.
///
/// An unsigned 8-bit error code can be supplied as a part of the signal to
/// the peer; if omitted, the error code is presumed to be `0`.
pub trait WriteAbort: maybe::Send {
    /// An error that can occur while stopping the stream.
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    /// Abort the stream.
    fn abort(
        self,
        error_code: ErrorCode,
    ) -> impl Future<Output = Result<(), Self::Error>> + maybe::Send;
}

/// Wait for the write stream to abort.
///
/// This can happen when the "read" part aborts the stream.
pub trait WriteAborted: maybe::Send {
    /// An error that can occur while waiting for a stream to be aborted.
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    /// Wait for a stream's corresponding read side to abort.
    fn aborted(self) -> impl Future<Output = Result<ErrorCode, Self::Error>> + maybe::Send;
}

/// Wait for the read stream to abort.
///
/// This can happen when the "write" part aborts the stream.
pub trait ReadAborted: maybe::Send {
    /// An error that can occur while waiting for a stream to be aborted.
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    /// Wait for a stream's corresponding write side to abort.
    fn aborted(self) -> impl Future<Output = Result<ErrorCode, Self::Error>> + maybe::Send;
}

/// Finish the write stream.
///
/// Call when all data has been submitted and no further data will be written.
pub trait Finish: maybe::Send {
    /// An error that can occur while finishing the stream.
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    /// Finish the stream.
    fn finish(self) -> impl Future<Output = Result<(), Self::Error>> + maybe::Send;
}

/// Wait for the read stream to finish.
///
/// This can happen when the "write" part finishes the stream.
pub trait Finished: maybe::Send {
    /// An error that can occur while waiting for a stream to be aborted.
    type Error: Error + ErrorAsErrorCode + maybe::Send + maybe::Sync + 'static;

    /// Wait for a stream to finish.
    fn finished(self) -> impl Future<Output = Result<(), Self::Error>> + maybe::Send;
}

/// An chunk of data with an explicit offset in the stream.
#[derive(Debug)]
pub struct Chunk<Data> {
    /// The offset of the data in the stream.
    pub offset: u64,
    /// The data.
    pub data: Data,
}

/// Read a [`Chunk`] from a stream.
pub trait ReadChunk<ChunkType: ?Sized + ReadableChunk>: maybe::Send {
    /// An error that can occur while reading a chunk from the stream.
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    /// Read the data with the offset information from a stream.
    fn read_chunk(
        &mut self,
        max_length: usize,
        ordered: bool,
    ) -> impl Future<
        Output = Result<Option<Chunk<<ChunkType as ReadableChunk>::Data<'_>>>, Self::Error>,
    > + maybe::Send;
}

/// Write a [`Chunk`] to a stream.
pub trait WriteChunk<ChunkType: ?Sized + WriteableChunk>: maybe::Send {
    /// An error that can occur while writing the chunk to a stream.
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    /// Write the data with the offset information from a stream.
    fn write_chunk<'a>(
        &'a mut self,
        buf: <ChunkType as WriteableChunk>::Data<'a>,
    ) -> impl Future<Output = Result<(), Self::Error>> + 'a + maybe::Send;
}

/// Something that specifies the data type for a [`Chunk`] that can
/// be read from a stream.
pub trait ReadableChunk: maybe::Send {
    /// The type that will hold the read data.
    type Data<'a>: AsRef<[u8]>;
}

/// Something that specifies the data type for a [`Chunk`] that can
/// be written to a stream.
pub trait WriteableChunk: maybe::Send {
    /// The type that will hold the data to write.
    type Data<'a>: From<&'a [u8]>;
}

#[cfg(feature = "alloc")]
pub mod chunk {
    //! Provided [`ReadableChunk`] and [`WriteableChunk`] implementations.

    use super::*;

    /// A chunk type that represents operations that carry the data as [`u8`]
    /// [`slice`]s or [`alloc::vec::Vec`]s
    #[derive(Debug)]
    pub struct U8;

    impl WriteableChunk for U8 {
        type Data<'b> = &'b [u8];
    }

    impl ReadableChunk for U8 {
        type Data<'b> = alloc::vec::Vec<u8>;
    }
}
