use std::collections::VecDeque;

pub(crate) struct RecursiveCharacterTextSplitter {
    chunk_size: usize,
    chunk_overlap: usize,
    separators: Vec<String>,
    keep_separator: bool,
}

impl RecursiveCharacterTextSplitter {
    pub fn new(
        chunk_size: usize,
        chunk_overlap: usize,
        separators: Option<Vec<String>>,
        keep_separator: bool,
    ) -> Self {
        RecursiveCharacterTextSplitter {
            chunk_size,
            chunk_overlap,
            separators: separators.unwrap_or_else(|| {
                [&"\n\n", &"\n", &" ", &""]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            }),
            keep_separator,
        }
    }

    fn split_text_with_separator(&self, text: &str, separator: &str) -> Vec<String> {
        let mut results = Vec::new();
        let mut last = 0;

        for (start, part) in text.match_indices(separator) {
            if self.keep_separator {
                results.push(text[last..start + part.len()].to_string());
            } else {
                results.push(text[last..start].to_string());
            }
            last = start + part.len();
        }

        if last < text.len() {
            results.push(text[last..].to_string());
        }

        results
    }

    fn recursive_split(&self, text: &str, separator_index: usize) -> Vec<String> {
        if separator_index >= self.separators.len() {
            return vec![text.to_string()];
        }

        let separator = &self.separators[separator_index];
        let parts = if separator.is_empty() {
            text.chars().map(|c| c.to_string()).collect()
        } else {
            self.split_text_with_separator(text, separator)
        };

        let mut chunks = Vec::new();
        let mut buffer = VecDeque::new();

        for part in parts {
            if part.len() >= self.chunk_size {
                if !buffer.is_empty() {
                    chunks.push(self.merge_buffer(&mut buffer));
                }
                chunks.extend(self.recursive_split(&part, separator_index + 1));
                continue;
            }

            buffer.push_back(part);
            if buffer.iter().map(String::len).sum::<usize>() >= self.chunk_size {
                chunks.push(self.merge_buffer(&mut buffer));
            }
        }

        if !buffer.is_empty() {
            chunks.push(self.merge_buffer(&mut buffer));
        }

        chunks
    }

    fn merge_buffer(&self, buffer: &mut VecDeque<String>) -> String {
        let mut merged = String::new();
        while let Some(chunk) = buffer.pop_front() {
            merged.push_str(&chunk);
            if merged.len() >= self.chunk_size - self.chunk_overlap {
                break;
            }
        }
        merged
    }

    pub fn split_text(&self, text: &str) -> Vec<String> {
        self.recursive_split(text, 0)
    }
}

#[cfg(test)]
mod tests_text_splitter {
    use crate::{
        ingest::pipeline::recursive_character_text_splitter::RecursiveCharacterTextSplitter,
        test_data::{SUPREME_COURT_VOL_129_PARSE_RESULT, SUPREME_COURT_VOL_129_SPLIT_RESULT},
    };

    #[test]
    fn split_huge_article() {
        let process = SUPREME_COURT_VOL_129_PARSE_RESULT;
        let split = RecursiveCharacterTextSplitter::new(1024, 128, None, true);
        let splits = split.split_text(process);

        assert_eq!(splits, SUPREME_COURT_VOL_129_SPLIT_RESULT);
    }
}
