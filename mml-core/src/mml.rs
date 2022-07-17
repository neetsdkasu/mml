// mml-core
// author: Leonardone @ NEETSDKASU

use crate::tone_control;

#[derive(Debug)]
pub enum MMLError {
    InvalidTempo,
}

pub(crate) fn parse(src: &str) -> Result<Vec<i8>, MMLError> {
    let mut mml = Mml::new(src);
    mml.parse_tempo()?;

    todo!("TODO")
}

#[derive(Debug)]
struct Mml<'a> {
    src: std::str::Chars<'a>,
    cur: Option<char>,
    next_block_id: i32,
    tempo: i32,
    resolution: i32,
    octave: i32,
    duration: i32,
}

impl<'a> Mml<'a> {
    fn new(src: &'a str) -> Self {
        let mut chars = src.chars();
        let cur = chars.next();
        Self {
            src: chars,
            cur,
            next_block_id: 0,
            tempo: 120,
            resolution: 64,
            octave: tone_control::C4 as i32,
            duration: 16,
        }
    }

    fn has_char(&self) -> bool {
        self.cur.is_some()
    }

    fn get_char(&self) -> Option<char> {
        self.cur
    }

    fn next_char(&mut self) -> bool {
        let cur = self.src.next();
        self.cur = cur;
        self.has_char()
    }

    fn skip_whitespaces(&mut self) {
        while let Some(ch) = self.get_char() {
            if ch.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }

    fn parse_number(&mut self) -> i32 {
        let mut t: i32 = 0;
        while let Some(ch) = self.get_char() {
            if ch.is_ascii_digit() {
                let d = ch.to_digit(10).unwrap() as i32;
                t = t * 10 + d;
                if t > 1000 {
                    // 元のソースコードがこうなっているが
                    // 何故こう処理してるのかわからんし
                    // nextCharしてないのも謎
                    // つまり最後の文字は消費しない形になっている
                    return 0x10000;
                }
                self.next_char();
            } else {
                break;
            }
        }
        t
    }

    fn parse_tempo(&mut self) -> Result<(), MMLError> {
        self.skip_whitespaces();
        let ch = match self.get_char() {
            Some(ch) => ch,
            None => return Ok(()),
        };
        if ch != 'T' || ch != 't' {
            return Ok(());
        }
        self.next_char();
        let t = self.parse_number();
        if t < 20 || 508 < t {
            Err(MMLError::InvalidTempo)
        } else {
            self.tempo = t;
            Ok(())
        }
    }
}
