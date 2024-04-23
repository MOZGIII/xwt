//! Operations that have the individual streams as the subject.

use core::future::Future;

use crate::utils::{maybe, Error};

pub trait Read: maybe::Send {
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn read(
        &mut self,
        buf: &mut [u8],
    ) -> impl Future<Output = Result<Option<usize>, Self::Error>> + maybe::Send;
}

pub trait Write: maybe::Send {
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn write(
        &mut self,
        buf: &[u8],
    ) -> impl Future<Output = Result<usize, Self::Error>> + maybe::Send;
}

#[derive(Debug)]
pub struct Chunk<Data> {
    pub offset: u64,
    pub data: Data,
}

pub trait ReadChunk<ChunkType: ?Sized + ReadableChunk>: maybe::Send {
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn read_chunk(
        &mut self,
        max_length: usize,
        ordered: bool,
    ) -> impl Future<
        Output = Result<Option<Chunk<<ChunkType as ReadableChunk>::Data<'_>>>, Self::Error>,
    > + maybe::Send;
}

pub trait WriteChunk<ChunkType: ?Sized + WriteableChunk>: maybe::Send {
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn write_chunk<'a>(
        &'a mut self,
        buf: <ChunkType as WriteableChunk>::Data<'a>,
    ) -> impl Future<Output = Result<(), Self::Error>> + 'a + maybe::Send;
}

pub trait ReadableChunk: maybe::Send {
    type Data<'a>: AsRef<[u8]>;
}

pub trait WriteableChunk: maybe::Send {
    type Data<'a>: From<&'a [u8]>;
}

#[cfg(feature = "alloc")]
pub mod chunk {
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
