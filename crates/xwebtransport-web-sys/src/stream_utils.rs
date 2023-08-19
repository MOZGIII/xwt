use crate::Error;

pub fn get_reader(
    readable_stream: web_sys::ReadableStream,
) -> web_sys::ReadableStreamDefaultReader {
    let reader: wasm_bindgen::JsValue = readable_stream.get_reader().into();
    reader.into()
}

pub fn get_writer(
    writable_stream: web_sys::WritableStream,
) -> web_sys::WritableStreamDefaultWriter {
    writable_stream.get_writer().unwrap()
}

pub async fn read(reader: &web_sys::ReadableStreamDefaultReader) -> Result<Option<Vec<u8>>, Error> {
    let fut = wasm_bindgen_futures::JsFuture::from(reader.read());
    let result = fut.await.map_err(Error)?;
    let result: crate::sys::ReadableStreamDefaultReaderValue = result.into();
    let value = result.value();

    let Some(js_buf) = value else {
        if result.is_done() {
            return Ok(None);
        }
        unreachable!("no value and we are also not done, this should be impossible");
    };

    Ok(Some(js_buf.to_vec()))
}

pub async fn write<T>(writer: &web_sys::WritableStreamDefaultWriter, buf: T) -> Result<(), Error>
where
    js_sys::Uint8Array: From<T>,
{
    let chunk = js_sys::Uint8Array::from(buf);
    let fut = wasm_bindgen_futures::JsFuture::from(writer.write_with_chunk(chunk.as_ref()));
    fut.await.map_err(Error)?;
    Ok(())
}
