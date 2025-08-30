//! Portable client code that can work with any `xwt` backend.

use rand::{seq::SliceRandom, thread_rng, Rng};

/// An example client that showcases the use of [`xwt_erased`] to abstract
/// around the WebTransport driver and execution environment.
pub struct ExampleClient {
    /// The WebTransport session to use.
    pub session: xwt_erased::Session,
    /// The name of this user.
    pub nickname: String,
    /// Function for writing the chat text to the display.
    pub chat_write: fn(&str),
}

impl ExampleClient {
    /// Run the client.
    pub async fn run(&mut self) {
        // Inform user that we are connected.
        (self.chat_write)(&format!("connected as {}\n", self.nickname));

        // Open a chat stream.
        let opening = self.session.open_bi().await.unwrap();
        let (chat_send, chat_read) = opening.wait_bi().await.unwrap();

        // Set up the loops.
        let movement_loop = self.movement_loop();
        let chat_outbound_loop = self.chat_outbound_loop(chat_send);
        let chat_inbound_loop = self.chat_inbound_loop(chat_read);

        // Run the loops.
        futures_util::future::join3(movement_loop, chat_outbound_loop, chat_inbound_loop).await;
    }

    /// Create random movements and send them as datagrams.
    pub async fn movement_loop(&self) {
        let moves = ["up", "down", "left", "right"];

        loop {
            // Select random move as input.
            let movement_input = moves.choose(&mut thread_rng()).unwrap();

            // Send it to the connection as a datagram.
            self.session
                .send_datagram(movement_input.as_ref())
                .await
                .unwrap();

            // Wait for 1 second before sending the next move.
            async_timer::new_timer(std::time::Duration::from_secs(1)).await;
        }
    }

    /// Create random chat messages and sent them via to the chat stream.
    pub async fn chat_outbound_loop(&self, mut chat_stream: xwt_erased::stream::SendStream) {
        let chat_messages = ["ggwp", "nice work team", "plz help", "go!"];

        loop {
            // Select random chat message as input.
            let chat_input = chat_messages.choose(&mut thread_rng()).unwrap();

            // Prepare a chat message.
            let chat_message = format!("{}: {}\n", self.nickname, chat_input);

            // Send it to the stream.
            chat_stream.write(chat_message.as_bytes()).await.unwrap();

            // Wait for 3 to 5 second before sending the next chat message.
            let seconds = thread_rng().gen_range(3..=5);
            async_timer::new_timer(std::time::Duration::from_secs(seconds)).await;
        }
    }

    /// Print the chat messages we get from the server on screen.
    pub async fn chat_inbound_loop(&self, mut chat_stream: xwt_erased::stream::RecvStream) {
        let mut read_buf = [0u8; 1024];
        loop {
            // Read the message from the chat stream.
            let result = chat_stream.read(&mut read_buf).await;
            // Stop if the stream is closed, and panic on any other error.
            let len = match result {
                Ok(len) => len,
                Err(err) if err.is_error_code(0) => break, // server has closed the chat stream
                Err(err) => panic!("unexpected error: {err:?}"),
            };
            let chat_message_bytes = &read_buf[..len.get()];

            // Parse the message bytes into a string ref.
            let chat_message = core::str::from_utf8(chat_message_bytes).unwrap();

            // Write the message to the chat.
            (self.chat_write)(chat_message)
        }
    }
}
