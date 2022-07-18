// mml-core
// author: Leonardone @ NEETSDKASU

use crate::tone_control;
use java_data_io::JavaDataOutput;
use std::io;
use MMLError::*;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub character: Option<char>,
    pub col: usize,
    pub row: usize,
}

#[derive(Debug)]
pub enum MMLError {
    EmptySequence(Position),
    InvalidBlock(Position),
    InvalidBlockEnd(Position),
    InvalidBlockId(Position),
    InvalidChangeOctave(Position),
    InvalidCharacter(Position),
    InvalidDecreaseOctave(Position),
    InvalidDefaultDurationValue(Position),
    InvalidDuration(Position),
    InvalidDurationEnd(Position),
    InvalidIncreaseOctave(Position),
    InvalidLength(Position),
    InvalidNote(Position),
    InvalidOctaveValue(Position),
    InvalidResolution(Position),
    InvalidTempo(Position),
    IoError(io::Error),
    UnexpectedRemains(Position),
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
        return mml.error(EmptySequence);
    }

    mml.validate_remains()?;

    Ok(buf)
}

#[derive(Debug)]
struct Mml<'a> {
    src: std::str::Chars<'a>,
    cur: Position,

    next_block_id: i32,
    tempo: i32,
    resolution: i32,

    // current octave (note C value)
    octave: i32,

    // default duration
    duration: i32,
}

impl<'a> Mml<'a> {
    fn new(src: &'a str) -> Self {
        let mut chars = src.chars();
        let cur = chars.next();
        Self {
            src: chars,
            cur: Position {
                character: cur,
                col: 1,
                row: 1,
            },
            next_block_id: 0,
            tempo: 120,
            resolution: 64,
            octave: tone_control::C4.into(),
            duration: 16,
        }
    }

    fn error<T>(&self, f: fn(p: Position) -> MMLError) -> Result<T> {
        Err(f(self.cur))
    }

    fn validate_remains(&mut self) -> Result<()> {
        self.skip_whitespaces();
        if self.has_char() {
            self.error(UnexpectedRemains)
        } else {
            Ok(())
        }
    }

    fn has_char(&self) -> bool {
        self.cur.character.is_some()
    }

    fn get_char(&self) -> Option<char> {
        self.cur.character
    }

    fn next_char(&mut self) -> Option<char> {
        let character = self.src.next();
        self.cur.col += 1;
        // if matches!(character, Some('\n')) { // マッチかイフレットか
        if let Some('\n') = character {
            self.cur.col = 1;
            self.cur.row += 1;
        }
        self.cur.character = character;
        character
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
            self.error(InvalidTempo)
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
            self.error(InvalidResolution)
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
            return self.error(InvalidBlockId);
        }

        let id = self.parse_number();
        if id != self.next_block_id || 127 < id {
            return self.error(InvalidBlockId);
        }

        dst.write_byte(tone_control::BLOCK_START.into())?;
        dst.write_byte(id)?;

        self.set_default();

        let event: i32 = self.parse_sequence(dst)?;
        if event == 0 {
            return self.error(InvalidBlock);
        }

        self.skip_whitespaces();

        if self.get_char().filter(|ch| *ch == '}').is_none() {
            return self.error(InvalidBlockEnd);
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
            // clippyさん･･･何故わかってくれぬ･･･
            #[allow(clippy::if_same_then_else)]
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
                return self.error(InvalidCharacter);
            }
            self.skip_whitespaces();
            event |= 1;
        }
        Ok(event)
    }

    fn parse_change_octave(&mut self) -> Result<bool> {
        match self.get_char() {
            Some('o' | 'O') => {}
            Some('>') => {
                self.octave -= 12;
                if self.octave < 0 {
                    return self.error(InvalidDecreaseOctave);
                } else {
                    self.next_char();
                    return Ok(true);
                }
            }
            Some('<') => {
                self.octave += 12;
                if self.octave > 127 {
                    return self.error(InvalidIncreaseOctave);
                } else {
                    self.next_char();
                    return Ok(true);
                }
            }
            _ => return Ok(false),
        }

        match self.next_char() {
            Some('-') => {}
            Some(ch) if ch.is_ascii_digit() => {
                let oct: i32 = self.parse_number();
                self.octave = (tone_control::C4 as i32) + (oct - 4) * 12;
                // if matches!(self.octave, 0..=127) {
                if (0..=127).contains(&self.octave) {
                    return Ok(true);
                } else {
                    return self.error(InvalidOctaveValue);
                }
            }
            _ => return self.error(InvalidChangeOctave),
        }

        if self.next_char().filter(char::is_ascii_digit).is_none() {
            return self.error(InvalidOctaveValue);
        }

        let oct: i32 = self.parse_number();
        self.octave = (tone_control::C4 as i32) - (oct + 4) * 12;
        // if matches!(self.octave, 0..=127) {
        if (0..=127).contains(&self.octave) {
            Ok(true)
        } else {
            self.error(InvalidOctaveValue)
        }
    }

    fn parse_change_duration(&mut self) -> Result<bool> {
        if self
            .get_char()
            .filter(|ch| *ch == 'L' || *ch == 'l')
            .is_none()
        {
            return Ok(false);
        }

        match self.next_char() {
            Some('(') => {}
            Some(ch) if ch.is_ascii_digit() => {}
            _ => return self.error(InvalidDefaultDurationValue),
        }

        self.duration = self.parse_duration()?;

        Ok(true)
    }

    fn parse_note<W: io::Write>(&mut self, dst: &mut JavaDataOutput<W>) -> Result<bool> {
        let mut note = self.octave;
        match self.get_char() {
            Some('C' | 'c') => {}
            Some('D' | 'd') => note += 2,
            Some('E' | 'e') => note += 4,
            Some('F' | 'f') => note += 5,
            Some('G' | 'g') => note += 7,
            Some('A' | 'a') => note += 9,
            Some('B' | 'b') => note += 11,
            _ => return Ok(false),
        }

        // if !matches(note, 0..=127) {
        if !(0..=127).contains(&note) {
            return self.error(InvalidNote);
        }

        match self.next_char() {
            Some('+' | '#') => {
                note += 1;
                self.next_char();
            }
            Some('-') => {
                note -= 1;
                self.next_char();
            }
            _ => {}
        }

        // if !matches(note, 0..=127) {
        if !(0..=127).contains(&note) {
            return self.error(InvalidNote);
        }

        let dur: i32 = self.parse_duration()?;

        dst.write_byte(note)?;
        dst.write_byte(dur)?;

        Ok(true)
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

    fn parse_duration(&mut self) -> Result<i32> {
        match self.get_char() {
            Some(ch) if ch.is_ascii_digit() => {}
            Some('(') => {
                if self.next_char().filter(char::is_ascii_digit).is_none() {
                    return self.error(InvalidDuration);
                }
                let dur: i32 = self.parse_number();
                // if !matches!(dur, 0..=127) {
                if !(0..=127).contains(&dur) {
                    return self.error(InvalidDuration);
                }
                // if matches!(self.get_char(), Some(')')) {
                if let Some(')') = self.get_char() {
                    self.next_char();
                    return Ok(dur);
                } else {
                    return self.error(InvalidDurationEnd);
                }
            }
            _ => return Ok(self.duration),
        }

        let mut num: i32 = self.parse_number();
        if !(1..=self.resolution).contains(&num) {
            return self.error(InvalidLength);
        }

        let mut dur: i32 = (self.resolution / num).max(1);

        while let Some('.') = self.get_char() {
            num <<= 1;
            if num > self.resolution {
                return self.error(InvalidLength);
            }
            // max(1)って計算、確実に1を足すって、それって正しいの？何故こうした昔の俺･･･
            dur += (self.resolution / num).max(1);
            self.next_char();
        }

        // if matches!(dur, 1..=127) {
        if (1..=127).contains(&dur) {
            Ok(dur)
        } else {
            self.error(InvalidLength)
        }
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
