// TODO: tests
use crate::{engine, gui, Gui, ReadError};
use core::str::FromStr;
use std::io;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt};

impl<E, G> Gui<E, G>
where
    E: AsyncWrite + Unpin,
    G: AsyncBufRead + Unpin,
{
    #[allow(clippy::missing_errors_doc)]
    /// See [`Self::send`].
    pub async fn send_async<M>(&mut self, message: M) -> io::Result<()>
    where
        M: engine::traits::Message,
    {
        self.engine
            .write_all((message.to_string() + "\n").as_bytes())
            .await
    }

    #[allow(clippy::missing_errors_doc)]
    /// See [`Self::read`].
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

    #[allow(clippy::missing_errors_doc)]
    /// See [`Self::send_string`].
    pub async fn send_string_async(&mut self, info: &str) -> io::Result<()> {
        let mut s = String::with_capacity(info.len().saturating_add("info string \n".len()));
        s.push_str("info string ");
        s.push_str(info);
        s.push('\n');
        self.engine.write_all(s.as_bytes()).await
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::{Depth, Go, Info, Score, ScoreBound, ScoreWithBound, UciOk};
    use pretty_assertions::assert_eq;
    use shakmaty::uci::UciMove;
    use shakmaty::{Role, Square};
    use tokio::io::empty;

    #[tokio::test]
    async fn send() {
        let mut gui = Gui {
            engine: Vec::<u8>::new(),
            gui: empty(),
        };

        gui.send_async(UciOk).await.unwrap();
        assert_eq!(gui.engine, b"uciok\n");

        gui.send_async(UciOk).await.unwrap();
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
        gui.send_async(&info).await.unwrap();
        assert_eq!(
            String::from_utf8_lossy(&gui.engine),
            info.to_string() + "\n"
        );
    }

    #[tokio::test]
    async fn send_string() {
        let mut gui = Gui {
            engine: Vec::<u8>::new(),
            gui: empty(),
        };

        let s = "นดินฮั่นเสื่อมโทร መካ የአሞራᛖ ᚩᚾ ᚦᚫᛗ ⠑⠁⠝ ⠞⠕ ⠎⠁⠹   ∮ E⋅da = Q,  n → ∞, ∑ f(i) = ∏ g(i), ∀x∈ℝ: ⌈x⌉ = −⌊−x⌋, α ∧ ¬β = ¬(¬α ∨ β),

  ℕ ⊆ ℕ₀ ⊂ ℤ ⊂ ℚ ⊂ ℝ ⊂ ℂ, ⊥ < a ≠ b ≡ c ≤ d ≪ ⊤ ⇒ (A ⇔ B),

  2H₂ + O₂ ⇌ 2H₂O, R = 4.7 kΩ, ⌀ 200 mm";
        gui.send_string_async(s).await.unwrap();

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

    #[cfg(feature = "engine-async")]
    #[tokio::test]
    async fn read() {
        use crate::Engine;

        let mut engine = Engine {
            engine: [].as_slice(),
            gui: Vec::<u8>::new(),
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

        engine.send_async(&go_send).await.unwrap();

        let mut gui = Gui {
            engine: Vec::<u8>::new(),
            gui: [].as_slice(),
        };

        gui.gui = engine.gui.as_slice();

        let go_read = gui.read_async().await.unwrap();

        assert_eq!(go_send, go_read);
    }
}
