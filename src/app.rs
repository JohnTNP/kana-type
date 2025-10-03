use copypasta::{ClipboardContext, ClipboardProvider};
use std::io;
use crate::{kana_type::KanaType, converter::RomajiConverter};

pub struct App {
    pub input: String,
    pub cursor_position: usize,
    pub converter: RomajiConverter,
    pub output_mode: KanaType,
    clipboard: ClipboardContext,
}

impl App {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            cursor_position: 0,
            converter: RomajiConverter::new(),
            output_mode: KanaType::Hiragana,
            clipboard: ClipboardContext::new().expect("Failed to initialize clipboard"),
        }
    }

    pub fn copy_to_clipboard(&mut self) -> io::Result<()> {
        let output = self.get_output();
        if !output.is_empty() {
            self.clipboard.set_contents(output).map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("Failed to copy to clipboard: {}", e))
            })
        } else {
            Ok(())
        }
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;
            self.input.remove(from_left_to_current_index);
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    pub fn toggle_mode(&mut self) {
        self.output_mode = match self.output_mode {
            KanaType::Hiragana => KanaType::Katakana,
            KanaType::Katakana => KanaType::Hiragana,
        };
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
        self.cursor_position = 0;
    }

    pub fn get_output(&self) -> String {
        self.converter.convert(&self.input, self.output_mode)
    }
}