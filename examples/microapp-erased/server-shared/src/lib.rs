//! Portable server code that can work with any `xwt` backend.

/// An example server that showcases the use of [`xwt_erased`] to abstract
/// around the WebTransport driver and execution environment.
#[derive(Debug)]
pub struct ExampleServer {
    /// Client identity.
    pub client_id_counter: core::sync::atomic::AtomicU64,
    /// The chat broadcast sender.
    pub chat_broadcast_tx: tokio::sync::broadcast::Sender<String>,
}

impl Default for ExampleServer {
    fn default() -> Self {
        let (chat_broadcast_tx, _) = tokio::sync::broadcast::channel(8);
        Self {
            client_id_counter: Default::default(),
            chat_broadcast_tx,
        }
    }
}

impl ExampleServer {
    /// Handle the inbound connection from a client.
    pub async fn handle(&self, session: xwt_erased::Session) {
        let client_id = self
            .client_id_counter
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        // Upon connecting, the client will open a chat stream.
        let (mut chat_send, chat_read) = session.accept_bi().await.unwrap();

        // Greet the client.
        let greeting_message = format!("server: welcome! your id is {client_id}\n");
        chat_send.write(greeting_message.as_bytes()).await.unwrap();

        // Subscribe to the chat broadcast.
        // Do this before starting the inbound loop to ensure there are
        // broadcast receivers and we don't loose messages.
        let chat_broadcast_rx = self.chat_broadcast_tx.subscribe();

        // Set up the loops.
        let chat_inbound_loop = self.chat_inbound_loop(chat_read);
        let chat_outbound_loop = self.chat_outbound_loop(chat_send, chat_broadcast_rx);
        let movement_loop = self.movement_loop(client_id, &session);

        // Run the loops.
        futures_util::future::join3(chat_inbound_loop, chat_outbound_loop, movement_loop).await;
    }

    /// Handle the inbound chat messages by sending them to the chat broadcast.
    async fn chat_inbound_loop(&self, mut chat_stream: xwt_erased::stream::RecvStream) {
        let mut read_buf = [0u8; 1024];
        loop {
            // Read the message from the chat stream.
            let result = chat_stream.read(&mut read_buf).await;
            // Stop if the stream is closed, and panic on any other error.
            let len = match result {
                Ok(len) => len,
                Err(err) if err.is_error_code(0) => break, // client has closed the chat stream
                Err(err) => panic!("unexpected error: {err:?}"),
            };
            let chat_message_bytes = &read_buf[..len.get()];

            // Parse the message bytes into a string.
            let chat_message = core::str::from_utf8(chat_message_bytes).unwrap().to_owned();

            // Broadcast the chat message to everyone.
            self.chat_broadcast_tx.send(chat_message).unwrap();
        }
    }

    /// Handle the broadcasted chat messages by sending them to the chat stream.
    async fn chat_outbound_loop(
        &self,
        mut chat_stream: xwt_erased::stream::SendStream,
        mut rx: tokio::sync::broadcast::Receiver<String>,
    ) {
        loop {
            // Receive a chat broadcast.
            let msg = rx.recv().await.unwrap();

            // Represent the message as raw bytes.
            let msg = msg.as_bytes();

            // Send the broadcasted message to the client.
            let written = chat_stream.write(msg).await.unwrap();
            debug_assert_eq!(written.get(), msg.len()); // simplification for the example, do the right thing instead
        }
    }

    /// Announce the client movements to the chat.
    /// Normally you'd update the world state and replicate it, but this
    /// is a simple example :P.
    pub async fn movement_loop(&self, client_id: u64, session: &xwt_erased::Session) {
        loop {
            // Receive the movement datagram from the client.
            // This can fail if the connection closes.
            let movement_datagram = session.receive_datagram().await.unwrap();

            // Parse the datagram according to the application-level protocol,
            // in our case we turn it into a string.
            let movement = std::str::from_utf8(movement_datagram.as_ref()).unwrap();

            // Prepare the announcement message.
            let announcement_message = format!("server: client {client_id} moved {movement}\n");

            // Broadcast the movement announcement to everyone.
            self.chat_broadcast_tx.send(announcement_message).unwrap();
        }
    }
}
