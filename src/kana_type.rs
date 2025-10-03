#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KanaType {
    Hiragana,
    Katakana,
}

impl KanaType {
    pub fn as_str(&self) -> &'static str {
        match self {
            KanaType::Hiragana => "Hiragana",
            KanaType::Katakana => "Katakana",
        }
    }
}