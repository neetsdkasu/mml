// mml-core
// author: Leonardone @ NEETSDKASU

use crate::tone_control;
use java_data_io::JavaDataOutput;
use std::io;

#[derive(Debug)]
pub enum MMLError {
    EmptySequence,
    InvalidBlock,
    InvalidBlockEnd,
    InvalidBlockId,
    InvalidCharacter,
    InvalidResolution,
    InvalidTempo,
    IoError(io::Error),
    UnexpectedRemains,
}

pub(crate) type Result<T> = std::result::Result<T, MMLError>;

pub(crate) fn parse(src: &str) -> Result<Vec<u8>> {
    let mut mml = Mml::new(src);
    mml.parse_tempo()?;
    mml.parse_resolution()?;
    let mut buf = Vec::<u8>::new();
    let mut dst = JavaDataOutput::new(&mut buf);
    dst.write_byte(tone_control::VERSION.into())?;
    dst.write_byte(1)?;
    dst.write_byte(tone_control::TEMPO.into())?;
    dst.write_byte(mml.tempo >> 2)?;
    dst.write_byte(tone_control::RESOLUTION.into())?;
    dst.write_byte(mml.resolution)?;
    while mml.parse_block(&mut dst)? {}
    mml.set_default();
    let event: i32 = mml.parse_sequence(&mut dst)?;
    if event == 0 {
        return Err(MMLError::EmptySequence);
    }
    mml.validate_remains()?;
    Ok(buf)
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
            octave: tone_control::C4.into(),
            duration: 16,
        }
    }

    fn validate_remains(&mut self) -> Result<()> {
        self.skip_whitespaces();
        if self.has_char() {
            Err(MMLError::UnexpectedRemains)
        } else {
            Ok(())
        }
    }

    fn has_char(&self) -> bool {
        self.cur.is_some()
    }

    fn get_char(&self) -> Option<char> {
        self.cur
    }

    fn next_char(&mut self) -> Option<char> {
        let cur = self.src.next();
        self.cur = cur;
        cur
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
        let mut number: i32 = 0;
        while let Some(ch) = self.get_char() {
            if ch.is_ascii_digit() {
                let d = ch.to_digit(10).unwrap() as i32;
                number = number * 10 + d;
                if number > 1000 {
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
        number
    }

    fn parse_tempo(&mut self) -> Result<()> {
        self.skip_whitespaces();

        if self
            .get_char()
            .filter(|ch| *ch == 'T' || *ch == 't')
            .is_none()
        {
            return Ok(());
        }
        self.next_char();

        let tempo: i32 = self.parse_number();

        // if matches!(tempo, 20..=508) { // レンジとマッチどっちがいいのか？
        if (20..=508).contains(&tempo) {
            self.tempo = tempo;
            Ok(())
        } else {
            Err(MMLError::InvalidTempo)
        }
    }

    fn parse_resolution(&mut self) -> Result<()> {
        self.skip_whitespaces();

        if self.get_char().filter(|ch| *ch == '%').is_none() {
            return Ok(());
        }
        self.next_char();

        let resolution: i32 = self.parse_number();

        // if matches!(resolution, 1..=127) { // レンジとマッチどっちがいいのか？
        if (1..=127).contains(&resolution) {
            self.resolution = resolution;
            Ok(())
        } else {
            Err(MMLError::InvalidResolution)
        }
    }

    fn set_default(&mut self) {
        self.octave = tone_control::C4.into();
        self.duration = (self.resolution >> 2).max(1);
    }

    fn parse_block<W: io::Write>(&mut self, dst: &mut JavaDataOutput<W>) -> Result<bool> {
        self.skip_whitespaces();

        if self.get_char().filter(|ch| *ch == '{').is_none() {
            return Ok(false);
        }

        if self.next_char().filter(char::is_ascii_digit).is_none() {
            return Err(MMLError::InvalidBlockId);
        }

        let id = self.parse_number();
        if id != self.next_block_id || 127 < id {
            return Err(MMLError::InvalidBlockId);
        }

        dst.write_byte(tone_control::BLOCK_START.into())?;
        dst.write_byte(id)?;

        self.set_default();

        let event: i32 = self.parse_sequence(dst)?;
        if event == 0 {
            return Err(MMLError::InvalidBlock);
        }

        self.skip_whitespaces();

        if self.get_char().filter(|ch| *ch == '}').is_none() {
            return Err(MMLError::InvalidBlockEnd);
        }

        self.next_char();

        dst.write_byte(tone_control::BLOCK_END.into())?;
        dst.write_byte(id)?;

        self.next_block_id += 1;

        Ok(true)
    }

    fn parse_sequence<W: io::Write>(&mut self, dst: &mut JavaDataOutput<W>) -> Result<i32> {
        let mut event: i32 = 0;
        self.skip_whitespaces();
        while self.has_char() {
            if self.parse_change_octave()? {
                self.skip_whitespaces();
                continue;
            } else if self.parse_change_duration()? {
                self.skip_whitespaces();
                continue;
            } else if self.parse_note(dst)? {
                event += 2;
            } else if self.parse_rest(dst)? {
                event += 2;
            } else if self.parse_note_value(dst)? {
                event += 2;
            } else if self.parse_play_block(dst)? {
            } else if self.parse_repeat(dst)? {
            } else if self.parse_volume(dst)? {
            } else if self
                .get_char()
                .filter(|ch| *ch == ']' || *ch == '}')
                .is_some()
            {
                break;
            } else {
                return Err(MMLError::InvalidCharacter);
            }
            self.skip_whitespaces();
            event |= 1;
        }
        Ok(event)
    }

    fn parse_change_octave(&mut self) -> Result<bool> {
        todo!()
    }

    fn parse_change_duration(&mut self) -> Result<bool> {
        todo!()
    }

    fn parse_note<W: io::Write>(&mut self, dst: &mut JavaDataOutput<W>) -> Result<bool> {
        todo!()
    }

    fn parse_rest<W: io::Write>(&mut self, dst: &mut JavaDataOutput<W>) -> Result<bool> {
        todo!()
    }

    fn parse_note_value<W: io::Write>(&mut self, dst: &mut JavaDataOutput<W>) -> Result<bool> {
        todo!()
    }

    fn parse_play_block<W: io::Write>(&mut self, dst: &mut JavaDataOutput<W>) -> Result<bool> {
        todo!()
    }

    fn parse_repeat<W: io::Write>(&mut self, dst: &mut JavaDataOutput<W>) -> Result<bool> {
        todo!()
    }

    fn parse_volume<W: io::Write>(&mut self, dst: &mut JavaDataOutput<W>) -> Result<bool> {
        todo!()
    }
}

impl From<java_data_io::Error> for MMLError {
    fn from(error: java_data_io::Error) -> Self {
        use java_data_io::Error::*;
        match error {
            IoError(io_error) => MMLError::IoError(io_error),
            UtfDataFormatError => unreachable!("UtfDataFormatError"),
        }
    }
}
