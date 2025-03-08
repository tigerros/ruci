use serde::Deserialize;
use shakmaty::uci::UciMove;

pub fn serialize<S>(uci_move: &UciMove, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&uci_move.to_string())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<UciMove, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer)?
        .parse()
        .map_err(|e| D::Error::custom(format!("uci move parse error: {e}")))
}

pub mod option {
    use serde::de::Error;
    use serde::Deserialize;
    use shakmaty::uci::UciMove;
    use std::str::FromStr;

    // CLIPPY: Serde requires it
    #[allow(clippy::ref_option)]
    pub fn serialize<S>(uci_move: &Option<UciMove>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match uci_move {
            Some(uci_move) => serializer.serialize_some(&uci_move.to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<UciMove>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match Option::<String>::deserialize(deserializer)? {
            Some(uci_move) => Ok(Some(
                UciMove::from_str(&uci_move).map_err(D::Error::custom)?,
            )),
            None => Ok(None),
        }
    }
}
