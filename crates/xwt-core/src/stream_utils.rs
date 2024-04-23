use crate::stream;

pub type ReadErrorFor<T> = <T as stream::Read>::Error;

pub type WriteErrorFor<T> = <T as stream::Write>::Error;

pub type ReadChunkErrorFor<T, Data> = <T as stream::ReadChunk<Data>>::Error;

pub type WriteChunkErrorFor<T, Data> = <T as stream::WriteChunk<Data>>::Error;
