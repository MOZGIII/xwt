#![cfg(target_family = "wasm")]

use std::rc::Rc;

use xwebtransport_core::async_trait;

mod error;

pub use error::*;

#[derive(Debug, Clone)]
pub struct Endpoint;

#[async_trait(?Send)]
impl xwebtransport_core::traits::EndpointConnect for Endpoint {
    type Error = Error;
    type Params<'params> = &'params web_sys::WebTransportOptions;
    type Connecting = xwebtransport_core::utils::dummy::Connecting<Connection>;

    async fn connect(
        &self,
        url: &str,
        params: Self::Params<'_>,
    ) -> Result<Self::Connecting, Self::Error> {
        let transport = web_sys::WebTransport::new_with_options(url, params)?;
        let _ = wasm_bindgen_futures::JsFuture::from(transport.ready()).await?;
        let connection = Connection {
            transport: Rc::new(transport),
        };
        Ok(xwebtransport_core::utils::dummy::Connecting(connection))
    }
}

#[derive(Debug)]
pub struct Connection {
    pub transport: Rc<web_sys::WebTransport>,
}

impl xwebtransport_core::traits::Streams for Connection {
    type SendStream = SendStream;
    type RecvStream = RecvStream;
}

pub struct SendStream {
    pub transport: Rc<web_sys::WebTransport>,
    pub stream: web_sys::WebTransportSendStream,
    pub writer: web_sys::WritableStreamDefaultWriter,
}

pub struct RecvStream {
    pub transport: Rc<web_sys::WebTransport>,
    pub stream: web_sys::WebTransportReceiveStream,
    pub reader: web_sys::ReadableStreamDefaultReader,
}

fn wrap_recv_stream(
    transport: &Rc<web_sys::WebTransport>,
    stream: web_sys::WebTransportReceiveStream,
) -> RecvStream {
    let reader = stream.get_reader();
    let reader: wasm_bindgen::JsValue = reader.into();
    let reader = reader.into();

    RecvStream {
        transport: Rc::clone(transport),
        stream,
        reader,
    }
}

fn wrap_send_stream(
    transport: &Rc<web_sys::WebTransport>,
    stream: web_sys::WebTransportSendStream,
) -> SendStream {
    let writer = stream.get_writer().unwrap();
    SendStream {
        transport: Rc::clone(transport),
        stream,
        writer,
    }
}

fn wrap_bi_stream(
    transport: &Rc<web_sys::WebTransport>,
    stream: web_sys::WebTransportBidirectionalStream,
) -> (SendStream, RecvStream) {
    let writeable = stream.writable();
    let readable = stream.readable();

    let send_stream = wrap_send_stream(transport, writeable);
    let recv_stream = wrap_recv_stream(transport, readable);

    (send_stream, recv_stream)
}

#[async_trait(?Send)]
impl xwebtransport_core::traits::OpenBiStream for Connection {
    type Opening = xwebtransport_core::utils::dummy::OpeningBiStream<Connection>;

    type Error = Error;

    async fn open_bi(&self) -> Result<Self::Opening, Self::Error> {
        let value =
            wasm_bindgen_futures::JsFuture::from(self.transport.create_bidirectional_stream())
                .await?;
        let value: web_sys::WebTransportBidirectionalStream = value.into();
        let value = wrap_bi_stream(&self.transport, value);
        Ok(xwebtransport_core::utils::dummy::OpeningBiStream(value))
    }
}

#[async_trait(?Send)]
impl xwebtransport_core::traits::AcceptBiStream for Connection {
    type Error = Error;

    async fn accept_bi(&self) -> Result<(Self::SendStream, Self::RecvStream), Self::Error> {
        let incoming: web_sys::ReadableStream = self.transport.incoming_bidirectional_streams();
        let reader: wasm_bindgen::JsValue = incoming.get_reader().into();
        let reader: web_sys::ReadableStreamDefaultReader = reader.into();
        let read_result = wasm_bindgen_futures::JsFuture::from(reader.read()).await?;
        let value: web_sys::WebTransportBidirectionalStream = read_result.into();
        let value = wrap_bi_stream(&self.transport, value);
        Ok(value)
    }
}

#[async_trait(?Send)]
impl xwebtransport_core::traits::OpenUniStream for Connection {
    type Opening = xwebtransport_core::utils::dummy::OpeningUniStream<Connection>;
    type Error = Error;

    async fn open_uni(&self) -> Result<Self::Opening, Self::Error> {
        let value =
            wasm_bindgen_futures::JsFuture::from(self.transport.create_unidirectional_stream())
                .await?;
        let value: web_sys::WebTransportSendStream = value.into();
        let send_stream = wrap_send_stream(&self.transport, value);
        Ok(xwebtransport_core::utils::dummy::OpeningUniStream(
            send_stream,
        ))
    }
}

#[async_trait(?Send)]
impl xwebtransport_core::traits::AcceptUniStream for Connection {
    type Error = Error;

    async fn accept_uni(&self) -> Result<Self::RecvStream, Self::Error> {
        let incoming: web_sys::ReadableStream = self.transport.incoming_unidirectional_streams();
        let reader: wasm_bindgen::JsValue = incoming.get_reader().into();
        let reader: web_sys::ReadableStreamDefaultReader = reader.into();
        let read_result = wasm_bindgen_futures::JsFuture::from(reader.read()).await?;
        let value: web_sys::WebTransportReceiveStream = read_result.into();
        let recv_stream = wrap_recv_stream(&self.transport, value);
        Ok(recv_stream)
    }
}
