use log::{debug, info};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    let filepath = "./data/FSG.txt";
    let input = std::fs::read_to_string(filepath).expect("Failed to load {filepath}");
    let phase1 = remove_comment_lines(&input);
    let phase2 = fsg_resolve_naive(&phase1);
    let fsg_doc = ParsedFsg::from(&phase2).expect("failed to preprocess FSG.txt");
    for word in fsg_doc.words {
        // eprintln!("{word}");
    }

    let p1 = fsg_doc.document.paragraphs.iter().next().unwrap();
    eprintln!("{p1}");
}

/// FSG.txt is a combination of several different transcriptions,
/// so some pages have either-or expressions like "(O|A)HCCG".
/// For arbitrarily choose the 'left' option of any binary choice.
///
/// This is implemented with a simple state-machine parser,
/// with a loop like so:
///
/// Read -> ChooseLeftHandSide -> DiscardRightHandSide -> Read -> ...
///
/// Transitions are:
/// * an open paren '('
/// * a vertical bar '|'
/// * a close paren ')'
///
/// Any other transitions are invalid and result in a panic
///
#[derive(Debug)]
enum RsgResolveNaiveState {
    Read,
    ChooseLeftHandSide,
    DiscardRightHandSide,
}

fn remove_comment_lines(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    for line in input.lines() {
        let first_char = line.chars().next();
        if first_char.is_none() {
            debug!("preprocess: SKIP empty line {}", line);
        } else {
            let first_char = first_char.unwrap();
            if first_char == '#' {
                debug!("preprocess: SKIP comment line {}", line);
            } else {
                debug!("preprocess: KEEP line {}", line);
                output.push_str(line.trim());
                output.push('\n');
            }
        }
    }
    output
}

fn fsg_resolve_naive(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut state = RsgResolveNaiveState::Read;

    for next_char in input.chars() {
        match state {
            RsgResolveNaiveState::Read => match next_char {
                '|' | ')' => fsg_resolve_panic(&next_char, &state, &output),
                '(' => state = RsgResolveNaiveState::ChooseLeftHandSide,
                _ => output.push(next_char),
            },
            RsgResolveNaiveState::ChooseLeftHandSide => match next_char {
                '(' | ')' => fsg_resolve_panic(&next_char, &state, &output),
                '|' => state = RsgResolveNaiveState::DiscardRightHandSide,
                _ => output.push(next_char),
            },
            RsgResolveNaiveState::DiscardRightHandSide => {
                match next_char {
                    '(' | '|' => fsg_resolve_panic(&next_char, &state, &output),
                    ')' => state = RsgResolveNaiveState::Read,
                    _ => {} // discard, so NOOP
                }
            }
        }
    }
    output
}

fn fsg_resolve_panic(unexpected_char: &char, during_state: &RsgResolveNaiveState, buffer: &str) {
    panic!("Unexpected character {unexpected_char} during state {during_state:?}. Dumping buffer: {buffer}");
}

struct Document {
    paragraphs: Vec<Paragraph>,
}

impl Document {
    fn new() -> Self {
        Document { paragraphs: vec![] }
    }

    fn push_paragraph(&mut self, p: Paragraph) {
        self.paragraphs.push(p);
    }
}

struct Paragraph {
    lines: Vec<Vec<String>>,
}

impl Paragraph {
    fn new() -> Self {
        Paragraph { lines: vec![] }
    }

    fn push_line(&mut self, l: Vec<String>) {
        self.lines.push(l);
    }
}

impl Display for Paragraph {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for line in &self.lines {
            write!(f, "{line:?}");
        }
        Ok(())
    }
}

struct ParsedFsg {
    words: HashSet<String>,
    document: Document,
}

impl ParsedFsg {
    fn from(input: &str) -> Result<Self, String> {
        let mut document = Document::new();
        let mut unique_words = HashSet::new();
        let mut current_paragraph = Paragraph::new();
        for line in input.lines() {
            if line.trim().is_empty() {
                debug!("ParsedFsg: skipping empty line {line}")
            } else {
            let words = Self::split_words(&line);
            let before_count = unique_words.len();
            for word in &words {
                if !unique_words.contains(word) {
                    unique_words.insert(word.to_string());
                }
            }
            let after_count = unique_words.len();
            let diff = after_count - before_count;
            debug!("this line contributed {diff} new words (new total {after_count})");
            debug!("preprocess: pushing this line into the current paragraph");
            current_paragraph.push_line(words);

            let last_char = line.chars().next_back();
            if last_char == Some('=') {
                debug!("End of paragraph; starting a new one");
                document.push_paragraph(current_paragraph);
                current_paragraph = Paragraph::new();
            }
            }
        }
        let n_total_paragraphs: usize = document.paragraphs.len();
        let n_total_lines: usize = document.paragraphs.iter().map(|it| it.lines.len()).sum();
        let n_total_words: usize = document
            .paragraphs
            .iter()
            .map(|it| it.lines.iter().map(|it| it.len()).sum::<usize>())
            .sum();
        let n_unique_words: usize = unique_words.len();
        info!(
            "Done preprocessing; total paragraphs: {n_total_paragraphs}, total lines: {n_total_lines}, total words: {n_total_words}, unique words: {n_unique_words}"
        );
        Ok(Self {
            words: unique_words,
            document: document,
        })
    }

    fn split_words(line: &str) -> Vec<String> {
        line.split(',')
            .map(|it| {
                let mut word = it.trim().to_string();
                // remove end-of-line/end-of-paragraph characters
                let last_char = word.chars().next_back();
                if last_char == Some('-') || last_char == Some('=') {
                    word.remove(word.len() - 1);
                }
                word
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn split_words_removes_trailing_hyphens_equals() {
        let line = String::from("some,words-,with=,characters,to,strip=");
        let words = ParsedFsg::split_words(&line);
        assert_eq!(
            words,
            vec!["some", "words", "with", "characters", "to", "strip"]
        );
    }

    #[test]
    fn split_words_removes_extra_whitespace() {
        let line = String::from(" some, extra, whitespace , here, and , there");
        let words = ParsedFsg::split_words(&line);
        assert_eq!(
            words,
            vec!["some", "extra", "whitespace", "here", "and", "there"]
        );
    }
}
