use crate::{engine, gui, ReadError};
use core::str::FromStr;
use std::io;
use std::io::{stdin, stdout, BufRead, StdinLock, StdoutLock, Write};

/// Communicate with a chess GUI.
#[derive(Debug)]
pub struct Gui<E, G> {
    /// The output of the engine.
    pub engine: E,
    /// The output of the GUI.
    pub gui: G,
}

impl Gui<StdoutLock<'_>, StdinLock<'_>> {
    /// Constructs a new [`Gui`] from the [`stdout`] and [`stdin`] locks of the current process.
    ///
    /// Make sure you don't hold a lock to [`stdin`] already.
    ///
    /// # Note on async
    /// Doesn't have a corresponding async version due to tokio's recommendation to not use
    /// `tokio::io::stdin` for interactive uses (such as as this).
    /// Instead, one should use the blocking [`stdin`] on a separate thread.
    ///
    /// This complexity can't be covered by a simple constructor, so it is left to the user.
    pub fn from_stdio() -> Self {
        Self {
            engine: stdout().lock(),
            gui: stdin().lock(),
        }
    }
}

impl<E, G> Gui<E, G>
where
    E: Write,
{
    // CLIPPY: Message is implemented for borrows as well
    #[allow(clippy::needless_pass_by_value)]
    /// Sends a message.
    ///
    /// # Errors
    /// See [`Write::write_all`].
    pub fn send<M>(&mut self, message: M) -> io::Result<()>
    where
        M: engine::traits::Message,
    {
        self.engine
            .write_all((message.to_string() + "\n").as_bytes())
    }

    /// Specialized function for sending an [`Info`](crate::Info) message that's only composed
    /// of a string. More efficient than doing the equivalent with [`Self::send`].
    ///
    /// Use when you're trying to send generic information to the GUI that's not better
    /// conveyed by another message.
    ///
    /// # Errors
    /// See [`Write::write_all`].
    pub fn send_string(&mut self, info: &str) -> io::Result<()> {
        let mut s = String::with_capacity(info.len().saturating_add("info string \n".len()));
        s.push_str("info string ");
        s.push_str(info);
        s.push('\n');
        self.engine.write_all(s.as_bytes())
    }
}

impl<E, G> Gui<E, G>
where
    G: BufRead,
{
    #[allow(clippy::missing_errors_doc)]
    /// Reads a line and attempts to parse it into a given engine message.
    ///
    /// Skips lines which are only composed of whitespace.
    pub fn read<'m>(&mut self) -> Result<gui::Message<'m>, ReadError> {
        let mut line = String::new();

        loop {
            self.gui.read_line(&mut line).map_err(ReadError::Io)?;

            if line.is_empty() || line.chars().all(char::is_whitespace) {
                line.clear();
                continue;
            }

            break;
        }

        gui::Message::from_str(&line).map_err(|error| ReadError::Parse { error, got: line })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::{
        Depth, Go, Info, IsReady, MessageParseError, MessageParseErrorKind, Score, ScoreBound,
        ScoreWithBound, UciOk,
    };
    use pretty_assertions::{assert_eq, assert_matches};
    use shakmaty::uci::UciMove;
    use shakmaty::{Role, Square};
    use std::io::{empty, BufReader};

    #[test]
    fn send() {
        let mut gui = Gui {
            engine: Vec::<u8>::new(),
            gui: empty(),
        };

        gui.send(UciOk).unwrap();
        assert_eq!(gui.engine, b"uciok\n");

        gui.send(UciOk).unwrap();
        assert_eq!(gui.engine, b"uciok\nuciok\n");

        let info = Info {
            depth: Some(Depth {
                depth: 12,
                seldepth: Some(5),
            }),
            time: Some(7),
            nodes: Some(usize::MAX),
            pv: (&[
                UciMove::Normal {
                    from: Square::A1,
                    to: Square::A2,
                    promotion: None,
                },
                UciMove::Normal {
                    from: Square::E6,
                    to: Square::E8,
                    promotion: Some(Role::Queen),
                },
            ])
                .into(),
            multi_pv: Some(0),
            score: Some(ScoreWithBound {
                kind: Score::MateIn(5),
                bound: Some(ScoreBound::LowerBound),
            }),
            curr_move: Some(UciMove::Normal {
                from: Square::A1,
                to: Square::G6,
                promotion: None,
            }),
            curr_move_number: Some(2),
            hash_full: None,
            nps: None,
            tb_hits: None,
            sb_hits: None,
            cpu_load: None,
            string: None,
            refutation: None,
            curr_line: None,
        };

        gui.engine.clear();
        gui.send(&info).unwrap();
        assert_eq!(
            String::from_utf8_lossy(&gui.engine),
            info.to_string() + "\n"
        );
    }

    #[test]
    fn send_string() {
        let mut gui = Gui {
            engine: Vec::<u8>::new(),
            gui: empty(),
        };

        let s = "นดินฮั่นเสื่อมโทร መካ የአሞራᛖ ᚩᚾ ᚦᚫᛗ ⠑⠁⠝ ⠞⠕ ⠎⠁⠹   ∮ E⋅da = Q,  n → ∞, ∑ f(i) = ∏ g(i), ∀x∈ℝ: ⌈x⌉ = −⌊−x⌋, α ∧ ¬β = ¬(¬α ∨ β),

        ℕ ⊆ ℕ₀ ⊂ ℤ ⊂ ℚ ⊂ ℝ ⊂ ℂ, ⊥ < a ≠ b ≡ c ≤ d ≪ ⊤ ⇒ (A ⇔ B),
        
        2H₂ + O₂ ⇌ 2H₂O, R = 4.7 kΩ, ⌀ 200 mm";
        gui.send_string(s).unwrap();

        assert_eq!(
            String::from_utf8_lossy(&gui.engine),
            Info {
                string: Some(s.into()),
                ..Default::default()
            }
            .to_string()
                + "\n"
        );
    }

    #[cfg(feature = "engine-sync")]
    #[test]
    fn read() {
        use crate::{Engine, Go};

        let mut engine: Engine<&[u8], _> = Engine {
            engine: [].as_slice(),
            gui: Vec::new(),
            strict: true,
        };

        let go_send = gui::Message::from(Go {
            search_moves: (&[UciMove::Put {
                to: Square::C7,
                role: Role::Pawn,
            }])
                .into(),
            mate: Some(196),
            infinite: true,
            ..Default::default()
        });

        engine.gui.extend_from_slice(b"  \n\t\n  ");
        engine.send(&go_send).unwrap();
        engine.gui.extend_from_slice(b"\n\t\n  ");

        let mut gui = Gui {
            engine: Vec::<u8>::new(),
            gui: [].as_slice(),
        };

        gui.gui = engine.gui.as_slice();

        let go_read = gui.read().unwrap();

        assert_eq!(go_send, go_read);
    }

    #[test]
    fn line_ending_handling() {
        // Test different line ending combinations
        let endings = ["\n", "\r\n", "\r"];
        for ending in endings {
            let command = format!("isready{ending}");
            let mut test_gui = Gui {
                engine: Vec::<u8>::new(),
                gui: command.as_bytes(),
            };

            assert_eq!(test_gui.read().unwrap(), IsReady.into());
        }
    }

    #[test]
    fn empty_input_handling() {
        let mut gui = Gui {
            engine: Vec::<u8>::new(),
            gui: b"\n\n    \n\t\ngo\n".as_slice(),
        };

        let message = gui.read().unwrap();
        assert_matches!(message, gui::Message::Go(_));
    }

    #[test]
    fn io_error_handling() {
        struct MockWriter {
            should_fail: bool,
        }

        impl Write for MockWriter {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                if self.should_fail {
                    Err(io::Error::other("Mock error"))
                } else {
                    Ok(buf.len())
                }
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let mut gui = Gui {
            engine: MockWriter { should_fail: true },
            gui: empty(),
        };

        let result = gui.send(UciOk);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::Other);
    }

    #[test]
    fn multiple_commands() {
        let mut gui = Gui {
            engine: Vec::<u8>::new(),
            gui: "ucinewgame\nposition startpos\ngo depth 10\n".as_bytes(),
        };

        // Read all three commands
        for _ in 0..3 {
            let result = gui.read();
            assert!(result.is_ok());
        }

        let mut line = String::new();
        assert_eq!(gui.gui.read_line(&mut line).unwrap(), 0);
    }

    #[test]
    fn large_message_handling() {
        let mut gui = Gui {
            engine: Vec::new(),
            gui: empty(),
        };

        // Create a large info message
        let large_string = "x".repeat(10000);
        let result = gui.send_string(&large_string);
        assert!(result.is_ok());

        // Verify the output contains the full message
        let output = String::from_utf8_lossy(&gui.engine);
        assert!(output.contains(&large_string));
        assert!(output.starts_with("info string "));
        assert!(output.ends_with('\n'));
    }

    #[test]
    #[allow(clippy::panic)]
    fn error() {
        let mut gui = Gui {
            engine: Vec::<u8>::new(),
            gui: b"invalid_command\n".as_slice(),
        };

        let ReadError::Parse { error, got } = gui.read().unwrap_err() else {
            panic!("expected Parse ReadError");
        };
        let target_got = "invalid_command\n".to_string();

        assert_eq!(
            error,
            MessageParseError {
                expected: "GUI UCI message",
                kind: MessageParseErrorKind::NoMessage,
            },
        );

        assert_eq!(got, target_got);
    }

    #[test]
    fn buffered_reading() {
        let input = b"go\nstop\nucinewgame\n".as_slice();
        let reader = BufReader::new(input);

        let mut gui = Gui {
            engine: Vec::<u8>::new(),
            gui: reader,
        };

        for _ in 0..3 {
            assert!(gui.read().is_ok());
        }
    }

    #[test]
    fn partial_message_handling() {
        let mut gui = Gui {
            engine: Vec::<u8>::new(),
            gui: b"go depth".as_slice(),
        };

        assert_eq!(gui.read().unwrap(), Go::default().into());
    }
}
