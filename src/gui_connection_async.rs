// TODO: tests
// TODO: change docs of async functions to just link to the original function
use crate::{engine, gui, Gui, ReadError};
use core::str::FromStr;
use std::io;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt};

impl<E, G> Gui<E, G>
where
    E: AsyncWrite + Unpin,
    G: AsyncBufRead + Unpin,
{
    // CLIPPY: Message is implemented for borrows as well
    #[allow(clippy::needless_pass_by_value)]
    /// Sends a message.
    ///
    /// # Errors
    /// See [`Write::write_all`].
    pub async fn send_async<M>(&mut self, message: M) -> io::Result<()>
    where
        M: engine::traits::Message,
    {
        self.engine
            .write_all((message.to_string() + "\n").as_bytes())
            .await
    }

    #[allow(clippy::missing_errors_doc)]
    /// Reads a line and attempts to parse it into a given engine message.
    ///
    /// Skips lines which are only composed of whitespace.
    pub async fn read_async<'m>(&mut self) -> Result<gui::Message<'m>, ReadError> {
        let mut line = String::new();

        loop {
            self.gui.read_line(&mut line).await.map_err(ReadError::Io)?;

            if line.is_empty() || line.chars().all(char::is_whitespace) {
                line.clear();
                continue;
            }

            break;
        }

        gui::Message::from_str(&line).map_err(ReadError::Parse)
    }

    /// Specialized function for sending an [`Info`](crate::Info) message that's only composed
    /// of a string. More efficient than doing the equivalent with [`Self::send`].
    ///
    /// Use when you're trying to send generic information to the GUI that's not better
    /// conveyed by another message.
    ///
    /// # Errors
    /// See [`Write::write_all`].
    pub async fn send_string_async(&mut self, info: &str) -> io::Result<()> {
        let mut s = String::with_capacity(info.len().saturating_add("info string \n".len()));
        s.push_str("info string ");
        s.push_str(info);
        s.push('\n');
        self.engine.write_all(s.as_bytes()).await
    }
}
