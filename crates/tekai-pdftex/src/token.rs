#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CatCode {
    Escape,
    BeginGroup,
    EndGroup,
    MathShift,
    AlignmentTab,
    EndOfLine,
    Parameter,
    Superscript,
    Subscript,
    Ignored,
    Space,
    Letter,
    Other,
    Active,
    Comment,
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    ControlSequence(String),
    ControlSymbol(char),
    Character { ch: char, catcode: CatCode },
}

#[derive(Debug, Clone)]
pub struct CatCodeTable {
    ascii: [CatCode; 128],
}

impl CatCodeTable {
    pub fn plain_tex() -> Self {
        let mut ascii = [CatCode::Other; 128];
        for ch in b'a'..=b'z' {
            ascii[ch as usize] = CatCode::Letter;
        }
        for ch in b'A'..=b'Z' {
            ascii[ch as usize] = CatCode::Letter;
        }
        ascii[b'\\' as usize] = CatCode::Escape;
        ascii[b'{' as usize] = CatCode::BeginGroup;
        ascii[b'}' as usize] = CatCode::EndGroup;
        ascii[b'$' as usize] = CatCode::MathShift;
        ascii[b'&' as usize] = CatCode::AlignmentTab;
        ascii[b'\n' as usize] = CatCode::EndOfLine;
        ascii[b'\r' as usize] = CatCode::EndOfLine;
        ascii[b'#' as usize] = CatCode::Parameter;
        ascii[b'^' as usize] = CatCode::Superscript;
        ascii[b'_' as usize] = CatCode::Subscript;
        ascii[0] = CatCode::Ignored;
        ascii[b' ' as usize] = CatCode::Space;
        ascii[b'\t' as usize] = CatCode::Space;
        ascii[b'%' as usize] = CatCode::Comment;
        ascii[b'~' as usize] = CatCode::Active;
        Self { ascii }
    }

    pub fn get(&self, ch: char) -> CatCode {
        if ch.is_ascii() {
            self.ascii[ch as usize]
        } else if ch.is_alphabetic() {
            CatCode::Letter
        } else {
            CatCode::Other
        }
    }

    pub fn set_ascii(&mut self, ch: u8, catcode: CatCode) {
        self.ascii[ch as usize] = catcode;
    }
}

impl Default for CatCodeTable {
    fn default() -> Self {
        Self::plain_tex()
    }
}

pub fn tokenize(source: &str, catcodes: &CatCodeTable) -> Vec<Token> {
    Tokenizer::new(source, catcodes).collect()
}

pub struct Tokenizer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    catcodes: &'a CatCodeTable,
    skipping_comment: bool,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str, catcodes: &'a CatCodeTable) -> Self {
        Self {
            chars: source.chars().peekable(),
            catcodes,
            skipping_comment: false,
        }
    }

    fn read_control(&mut self) -> Token {
        let Some(next) = self.chars.next() else {
            return Token::ControlSymbol(' ');
        };
        if self.catcodes.get(next) != CatCode::Letter {
            return Token::ControlSymbol(next);
        }

        let mut name = String::from(next);
        while let Some(&ch) = self.chars.peek() {
            if self.catcodes.get(ch) != CatCode::Letter {
                break;
            }
            name.push(ch);
            self.chars.next();
        }

        while let Some(&ch) = self.chars.peek() {
            if self.catcodes.get(ch) != CatCode::Space {
                break;
            }
            self.chars.next();
        }

        Token::ControlSequence(name)
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ch = self.chars.next()?;
            let catcode = self.catcodes.get(ch);

            if self.skipping_comment {
                if catcode == CatCode::EndOfLine {
                    self.skipping_comment = false;
                    return Some(Token::Character {
                        ch: ' ',
                        catcode: CatCode::Space,
                    });
                }
                continue;
            }

            match catcode {
                CatCode::Escape => return Some(self.read_control()),
                CatCode::Comment => {
                    self.skipping_comment = true;
                    continue;
                }
                CatCode::Ignored => continue,
                CatCode::EndOfLine => {
                    return Some(Token::Character {
                        ch: ' ',
                        catcode: CatCode::Space,
                    });
                }
                _ => return Some(Token::Character { ch, catcode }),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer_reads_control_words_and_symbols() {
        let tokens = tokenize(r"\alpha+\% text", &CatCodeTable::default());

        assert_eq!(
            tokens,
            vec![
                Token::ControlSequence("alpha".to_string()),
                Token::Character {
                    ch: '+',
                    catcode: CatCode::Other,
                },
                Token::ControlSymbol('%'),
                Token::Character {
                    ch: ' ',
                    catcode: CatCode::Space,
                },
                Token::Character {
                    ch: 't',
                    catcode: CatCode::Letter,
                },
                Token::Character {
                    ch: 'e',
                    catcode: CatCode::Letter,
                },
                Token::Character {
                    ch: 'x',
                    catcode: CatCode::Letter,
                },
                Token::Character {
                    ch: 't',
                    catcode: CatCode::Letter,
                },
            ]
        );
    }

    #[test]
    fn tokenizer_uses_mutable_catcodes() {
        let mut catcodes = CatCodeTable::default();
        catcodes.set_ascii(b'@', CatCode::Letter);

        let tokens = tokenize(r"\make@letter", &catcodes);

        assert_eq!(
            tokens,
            vec![Token::ControlSequence("make@letter".to_string())]
        );
    }

    #[test]
    fn comments_are_removed_until_end_of_line() {
        let tokens = tokenize("a% hidden\nb", &CatCodeTable::default());

        assert_eq!(
            tokens,
            vec![
                Token::Character {
                    ch: 'a',
                    catcode: CatCode::Letter,
                },
                Token::Character {
                    ch: ' ',
                    catcode: CatCode::Space,
                },
                Token::Character {
                    ch: 'b',
                    catcode: CatCode::Letter,
                },
            ]
        );
    }
}
