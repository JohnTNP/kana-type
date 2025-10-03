use to_kana::to_kana;

use crate::kana_type::KanaType;

pub struct RomajiConverter;

impl RomajiConverter {
    pub fn new() -> Self {
        Self
    }

    pub fn convert(&self, input: &str, kana_type: KanaType) -> String {
        let input_lower = input.to_lowercase();
        let mut result = String::new();
        let chars: Vec<char> = input_lower.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let mut matched = false;

            // Try 4-char sequences (for combinations like "kya")
            if i + 3 < chars.len() {
                let four = &chars[i..i + 4];
                if let Some(kana) = self.romaji_to_kana(four, kana_type) {
                    result.push_str(&kana);
                    i += 4;
                    matched = true;
                }
            }

            // Try 3-char sequences
            if !matched && i + 2 < chars.len() {
                let three = &chars[i..i + 3];

                // Handle double consonants (small tsu)
                if chars[i] == chars[i + 1]
                    && chars[i] != 'n'
                    && chars[i].is_ascii_alphabetic()
                {
                    result.push(if kana_type == KanaType::Hiragana {
                        'っ'
                    } else {
                        'ッ'
                    });
                    i += 1;
                    matched = true;
                } else if let Some(kana) = self.romaji_to_kana(three, kana_type) {
                    result.push_str(&kana);
                    i += 3;
                    matched = true;
                }
            }

            // Try 2-char sequences
            if !matched && i + 1 < chars.len() {
                let two = &chars[i..i + 2];
                if let Some(kana) = self.romaji_to_kana(two, kana_type) {
                    result.push_str(&kana);
                    i += 2;
                    matched = true;
                }
            }

            // Try single char
            if !matched {
                let one = &chars[i..i + 1];
                if let Some(kana) = self.romaji_to_kana(one, kana_type) {
                    result.push_str(&kana);
                    i += 1;
                    matched = true;
                }
            }

            // If no match, keep original character
            if !matched {
                result.push(chars[i]);
                i += 1;
            }
        }

        result
    }

    fn romaji_to_kana(&self, chars: &[char], kana_type: KanaType) -> Option<String> {
        let s: String = chars.iter().collect();
        let romaji = s.as_str();

        let result = match kana_type {
            KanaType::Hiragana => to_kana!("{:H}", romaji),
            KanaType::Katakana => to_kana!("{:K}", romaji),
        };

        if result.is_ok() {
            Some(result.unwrap())
        } else {
            None
        }
    }
}