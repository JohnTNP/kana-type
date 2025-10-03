#[cfg(test)]
mod tests {
    use super::super::converter::RomajiConverter;
    use super::super::kana_type::KanaType;

    #[test]
    fn test_basic_hiragana() {
        let converter = RomajiConverter::new();
        let test_cases = vec![
            ("a", "あ"),
            ("i", "い"),
            ("u", "う"),
            ("e", "え"),
            ("o", "お"),
            ("ka", "か"),
            ("ki", "き"),
            ("ku", "く"),
            ("ke", "け"),
            ("ko", "こ"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                converter.convert(input, KanaType::Hiragana),
                expected,
                "Failed to convert '{}' to Hiragana", input
            );
        }
    }

    #[test]
    fn test_basic_katakana() {
        let converter = RomajiConverter::new();
        let test_cases = vec![
            ("a", "ア"),
            ("i", "イ"),
            ("u", "ウ"),
            ("e", "エ"),
            ("o", "オ"),
            ("ka", "カ"),
            ("ki", "キ"),
            ("ku", "ク"),
            ("ke", "ケ"),
            ("ko", "コ"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                converter.convert(input, KanaType::Katakana),
                expected,
                "Failed to convert '{}' to Katakana", input
            );
        }
    }

    #[test]
    fn test_dakuten() {
        let converter = RomajiConverter::new();
        let test_cases = vec![
            ("ga", "が"),
            ("gi", "ぎ"),
            ("gu", "ぐ"),
            ("ge", "げ"),
            ("go", "ご"),
            ("za", "ざ"),
            ("ji", "じ"),
            ("zu", "ず"),
            ("ze", "ぜ"),
            ("zo", "ぞ"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                converter.convert(input, KanaType::Hiragana),
                expected,
                "Failed to convert dakuten '{}' to Hiragana", input
            );
        }
    }

    #[test]
    fn test_handakuten() {
        let converter = RomajiConverter::new();
        let test_cases = vec![
            ("pa", "ぱ"),
            ("pi", "ぴ"),
            ("pu", "ぷ"),
            ("pe", "ぺ"),
            ("po", "ぽ"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                converter.convert(input, KanaType::Hiragana),
                expected,
                "Failed to convert handakuten '{}' to Hiragana", input
            );
        }
    }

    #[test]
    fn test_small_tsu() {
        let converter = RomajiConverter::new();
        let test_cases = vec![
            ("kka", "っか"),
            ("tta", "った"),
            ("ppa", "っぱ"),
            ("kitte", "きって"),
            ("motto", "もっと"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                converter.convert(input, KanaType::Hiragana),
                expected,
                "Failed to convert small tsu in '{}'", input
            );
        }
    }

    #[test]
    fn test_youon() {
        let converter = RomajiConverter::new();
        let test_cases = vec![
            ("kya", "きゃ"),
            ("kyu", "きゅ"),
            ("kyo", "きょ"),
            ("sha", "しゃ"),
            ("shu", "しゅ"),
            ("sho", "しょ"),
            ("cha", "ちゃ"),
            ("chu", "ちゅ"),
            ("cho", "ちょ"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                converter.convert(input, KanaType::Hiragana),
                expected,
                "Failed to convert youon '{}' to Hiragana", input
            );
        }
    }

    #[test]
    fn test_long_vowels() {
        let converter = RomajiConverter::new();
        let hiragana_cases = vec![
            ("aa", "ああ"),
            ("ii", "いい"),
            ("uu", "うう"),
            ("ee", "ええ"),
            ("oo", "おお"),
        ];
        let katakana_cases = vec![
            ("aa", "アー"),
            ("ii", "イー"),
            ("uu", "ウー"),
            ("ee", "エー"),
            ("oo", "オー"),
        ];

        for (input, expected) in hiragana_cases {
            assert_eq!(
                converter.convert(input, KanaType::Hiragana),
                expected,
                "Failed to convert long vowel '{}' to Hiragana", input
            );
        }

        for (input, expected) in katakana_cases {
            assert_eq!(
                converter.convert(input, KanaType::Katakana),
                expected,
                "Failed to convert long vowel '{}' to Katakana", input
            );
        }
    }

    #[test]
    fn test_n_cases() {
        let converter = RomajiConverter::new();
        let test_cases = vec![
            ("n", "ん"),
            ("nn", "ん"),
            ("shinbun", "しんぶん"),
            ("tanba", "たんば"),
            ("konkai", "こんかい"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                converter.convert(input, KanaType::Hiragana),
                expected,
                "Failed to convert n cases '{}' to Hiragana", input
            );
        }
    }

    #[test]
    fn test_case_insensitive() {
        let converter = RomajiConverter::new();
        let test_cases = vec![
            ("KONNICHIWA", "こんにちわ"),
            ("KoNnIcHiWa", "こんにちわ"),
            ("SAYOUNARA", "さようなら"),
            ("SaYoUnArA", "さようなら"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                converter.convert(input, KanaType::Hiragana),
                expected,
                "Failed case insensitive conversion for '{}'", input
            );
        }
    }

    #[test]
    fn test_common_words() {
        let converter = RomajiConverter::new();
        let test_cases = vec![
            ("konnichiwa", "こんにちわ"),
            ("sayounara", "さようなら"),
            ("ohayou", "おはよう"),
            ("arigatou", "ありがとう"),
            ("watashi", "わたし"),
            ("gomennnasai", "ごめんなさい"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                converter.convert(input, KanaType::Hiragana),
                expected,
                "Failed to convert common word '{}' to Hiragana", input
            );
        }
    }
}