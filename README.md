# Kana Type

A terminal-based Romaji to Kana converter built with Rust. Convert Roman characters (romaji) to Japanese Hiragana and Katakana in real-time.

<img height="400" alt="image" src="https://github.com/user-attachments/assets/8c7a1c15-e389-4bce-97f8-400560ba674c" />


## Features

- Real-time conversion from Romaji to Hiragana/Katakana
- Toggle between Hiragana and Katakana modes
- Copy converted text to clipboard
- Clean and intuitive terminal user interface

## Installation

1. Make sure you have Rust and Cargo installed
2. Clone this repository
3. Build and run the project:

```bash
git clone https://github.com/yourusername/to-kana.git
cd to-kana
cargo run
```

## Usage

### Basic Controls

- Type romaji (roman characters) to see the immediate conversion
- Use arrow keys to navigate the input text
- Press `Tab` to toggle between Hiragana and Katakana modes
- Press `Delete` to clear the input
- Press `Ctrl+C` to copy the converted text
- Press `Esc` or `Ctrl+Q` to quit

### Input Examples

Here are some example conversions:

Hiragana mode:
- `konnichiwa` → こんにちわ
- `ohayou` → おはよう
- `sayounara` → さようなら

Katakana mode:
- `paatii` → パーティー
- `konpyuutaa` → コンピューター
- `sutoresu` → ストレス

### Input Rules

- Double vowels are converted to long vowel marks in katakana (e.g., `aa` → アー)
- Double consonants create small つ/ツ (e.g., `kitte` → きって)
- Use 'n' for ん/ン (e.g., `shinbun` → しんぶん)
- Special combinations like 'kya', 'shu', 'ryo' are supported
