use crate::{ SearchDirection, highlighting, HighlightingOptions };
use std::cmp;
use crossterm::style::{ SetForegroundColor, Color };
use unicode_segmentation::UnicodeSegmentation; // comment
// Test
#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
    highlighting: Vec<highlighting::Type>,
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        Self {
            string: String::from(value),
            len: value.graphemes(true).count(),
            highlighting: Vec::new(),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut result = String::new();
        let mut current_highlighting = &highlighting::Type::None;
        result.push_str(
            format!("{}", SetForegroundColor(current_highlighting.to_color())).as_str()
        );
        for (index, grapheme) in self.string[..]
            .graphemes(true)
            .enumerate()
            .skip(start)
            .take(end - start) {
            if let Some(c) = grapheme.chars().next() {
                let highlighting_type = self.highlighting
                    .get(index)
                    .unwrap_or(&highlighting::Type::None);
                if highlighting_type != current_highlighting {
                    current_highlighting = highlighting_type;
                    result.push_str(
                        format!("{}", SetForegroundColor(highlighting_type.to_color())).as_str()
                    );
                }
                if c == '\t' {
                    result.push_str(" ");
                } else {
                    result.push(c);
                }
            }
        }
        result.push_str(format!("{}", SetForegroundColor(Color::Reset)).as_str());
        result
    }
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
            self.len += 1;
            return;
        }
        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            length += 1;
            if index == at {
                length += 1;
                result.push(c);
            }
            result.push_str(grapheme);
        }
        self.len = length;
        self.string = result;
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        }
        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index != at {
                length += 1;
                result.push_str(grapheme);
            }
        }
        self.len = length;
        self.string = result;
    }

    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.len += new.len();
    }

    pub fn split(&mut self, at: usize) -> Self {
        let mut row: String = String::new();
        let mut length = 0;
        let mut splitted_row: String = String::new();
        let mut splitted_length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index < at {
                length += 1;
                row.push_str(grapheme);
            } else {
                splitted_length += 1;
                splitted_row.push_str(grapheme);
            }
        }
        self.string = row;
        self.len = length;
        Self { string: splitted_row, len: splitted_length, highlighting: Vec::new() }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    pub fn find(&self, query: &str, at: usize, direction: SearchDirection) -> Option<usize> {
        if at > self.len || query.is_empty() {
            return None;
        }
        let start = if direction == SearchDirection::Forward { at } else { 0 };
        let end = if direction == SearchDirection::Forward { self.len } else { at };

        let substring: String = self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect();
        let matching_byte_index = if direction == SearchDirection::Forward {
            substring.find(query)
        } else {
            substring.rfind(query)
        };
        if let Some(matching_byte_index) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in substring[..]
                .grapheme_indices(true)
                .enumerate() {
                if matching_byte_index == byte_index {
                    return Some(start + grapheme_index);
                }
            }
        }
        None
    }

    pub fn highlight(&mut self, opts: HighlightingOptions, word: Option<&str>) {
        self.highlighting = Vec::new();
        let chars: Vec<char> = self.string.chars().collect();
        let mut index = 0;
        while let Some(c) = chars.get(index) {
            if
                self.highlight_char(&mut index, opts, *c, &chars) ||
                self.highlight_comment(&mut index, opts, *c, &chars) ||
                self.highlight_string(&mut index, opts, *c, &chars) ||
                self.highlight_number(&mut index, opts, *c, &chars)
            {
                continue;
            }
            self.highlighting.push(highlighting::Type::None);
            index += 1;
        }
        self.highlight_match(word);
    }

    fn highlight_match(&mut self, word: Option<&str>) {
        if let Some(word) = word {
            if word.is_empty() {
                return;
            }
            let mut index = 0;
            while let Some(search_match) = self.find(word, index, SearchDirection::Forward) {
                if
                    let Some(next_index) = search_match.checked_add(
                        word[..].graphemes(true).count()
                    )
                {
                    for i in search_match..next_index {
                        self.highlighting[i] = highlighting::Type::Match;
                    }
                    index = next_index;
                } else {
                    break;
                }
            }
        }
    }

    fn highlight_char(
        &mut self,
        index: &mut usize,
        opts: HighlightingOptions,
        c: char,
        chars: &[char]
    ) -> bool {
        if !opts.characters() || c != '\'' {
            return false;
        }
        if let Some(next_char) = chars.get(index.saturating_add(1)) {
            let closing_index = if *next_char == '\\' {
                index.saturating_add(3)
            } else {
                index.saturating_add(2)
            };
            if let Some(closing_char) = chars.get(closing_index) {
                if *closing_char == '\'' {
                    for _ in 0..=closing_index.saturating_sub(*index) {
                        self.highlighting.push(highlighting::Type::Character);
                        *index += 1;
                    }
                    return true;
                }
            }
        }
        false
    }

    fn highlight_comment(
        &mut self,
        index: &mut usize,
        opts: HighlightingOptions,
        c: char,
        chars: &[char]
    ) -> bool {
        if !opts.comments() || c != '/' || *index >= chars.len() {
            return false;
        }

        if let Some(next_char) = chars.get(index.saturating_add(1)) {
            if *next_char == '/' {
                for _ in *index..chars.len() {
                    self.highlighting.push(highlighting::Type::Comment);
                    *index += 1;
                }
                return true;
            }
        }
        false
    }

    fn highlight_number(
        &mut self,
        index: &mut usize,
        opts: HighlightingOptions,
        c: char,
        chars: &[char]
    ) -> bool {
        if !(opts.numbers() && c.is_ascii_digit()) {
            return false;
        }
        if *index > 0 {
            let prev_char = chars[*index - 1];
            if !prev_char.is_ascii_punctuation() && !prev_char.is_ascii_whitespace() {
                return false;
            }
        }
        loop {
            self.highlighting.push(highlighting::Type::Number);
            *index += 1;
            if let Some(next_char) = chars.get(*index) {
                if *next_char != '.' && !next_char.is_ascii_digit() {
                    break;
                }
            } else {
                break;
            }
        }
        true
    }

    fn highlight_string(
        &mut self,
        index: &mut usize,
        opts: HighlightingOptions,
        c: char,
        chars: &[char]
    ) -> bool {
        if !opts.strings() || c != '"' {
            return false;
        }
        loop {
            self.highlighting.push(highlighting::Type::String);
            *index += 1;
            if let Some(next_char) = chars.get(*index) {
                if *next_char == '"' {
                    break;
                }
            } else {
                break;
            }
        }
        self.highlighting.push(highlighting::Type::String);
        *index += 1;

        true
    }
}
