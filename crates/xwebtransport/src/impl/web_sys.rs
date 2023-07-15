#[derive(Debug, Clone)]
pub struct Endpoint;

#[derive(Debug)]
pub struct Error(pub wasm_bindgen::JsValue);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Debug;
        self.0.fmt(f)
    }
}

impl std::error::Error for Error {}

impl From<wasm_bindgen::JsValue> for Error {
    fn from(value: wasm_bindgen::JsValue) -> Self {
        Self(value)
    }
}

impl crate::traits::EndpointConnect for Endpoint {
    type ConnectError = Error;
    type Params<'params> = &'params web_sys::WebTransportOptions;
    type Connecting = crate::utils::dummy::Connecting<web_sys::WebTransport>;

    async fn connect(
        &self,
        url: &str,
        params: Self::Params<'_>,
    ) -> Result<Self::Connecting, Self::ConnectError> {
        let transport = web_sys::WebTransport::new_with_options(url, params)?;
        let _ = wasm_bindgen_futures::JsFuture::from(transport.ready()).await?;
        Ok(crate::utils::dummy::Connecting(transport))
    }
}

impl crate::traits::OpenBiStream for web_sys::WebTransport {
    type Opening = crate::utils::dummy::OpeningBiStream<
        web_sys::WebTransportSendStream,
        web_sys::WebTransportReceiveStream,
    >;

    type Error = Error;

    async fn open_bi(&self) -> Result<Self::Opening, Self::Error> {
        let value =
            wasm_bindgen_futures::JsFuture::from(self.create_bidirectional_stream()).await?;
        let value: web_sys::WebTransportBidirectionalStream = value.into();
        let value = (value.writable(), value.readable());
        Ok(crate::utils::dummy::OpeningBiStream(value))
    }
}

impl crate::traits::AcceptBiStream for web_sys::WebTransport {
    type SendStream = web_sys::WebTransportSendStream;
    type RecvStream = web_sys::WebTransportReceiveStream;
    type Error = Error;

    async fn accept_bi(&self) -> Result<(Self::SendStream, Self::RecvStream), Self::Error> {
        let incoming: web_sys::ReadableStream = self.incoming_bidirectional_streams();
        let reader: wasm_bindgen::JsValue = incoming.get_reader().into();
        let reader: web_sys::ReadableStreamDefaultReader = reader.into();
        let read_result = wasm_bindgen_futures::JsFuture::from(reader.read()).await?;
        let value: web_sys::WebTransportBidirectionalStream = read_result.into();
        let value = (value.writable(), value.readable());
        Ok(value)
    }
}

impl crate::traits::OpenUniStream for web_sys::WebTransport {
    type Opening = crate::utils::dummy::OpeningUniStream<web_sys::WebTransportSendStream>;
    type Error = Error;

    async fn open_uni(&self) -> Result<Self::Opening, Self::Error> {
        let value =
            wasm_bindgen_futures::JsFuture::from(self.create_unidirectional_stream()).await?;
        let value: web_sys::WebTransportSendStream = value.into();
        Ok(crate::utils::dummy::OpeningUniStream(value))
    }
}

impl crate::traits::AcceptUniStream for web_sys::WebTransport {
    type RecvStream = web_sys::WebTransportReceiveStream;

    type Error = Error;

    async fn accept_uni(&self) -> Result<Self::RecvStream, Self::Error> {
        let incoming: web_sys::ReadableStream = self.incoming_unidirectional_streams();
        let reader: wasm_bindgen::JsValue = incoming.get_reader().into();
        let reader: web_sys::ReadableStreamDefaultReader = reader.into();
        let read_result = wasm_bindgen_futures::JsFuture::from(reader.read()).await?;
        let value: web_sys::WebTransportReceiveStream = read_result.into();
        Ok(value)
    }
}
