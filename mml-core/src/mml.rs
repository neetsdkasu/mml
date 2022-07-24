// mml-core
// author: Leonardone @ NEETSDKASU

use crate::tone_control;
use java_data_io::JavaDataOutput;
use std::io;
use MMLError::*;

#[derive(Debug, Clone)]
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
    InvalidNoteValue(Position),
    InvalidNoteValueEnd(Position),
    InvalidNoteValueStart(Position),
    InvalidOctaveValue(Position),
    InvalidPlayBlockId(Position),
    InvalidRepeat(Position),
    InvalidRepeatEnd(Position),
    InvalidRepeatNumber(Position),
    InvalidResolution(Position),
    InvalidTempo(Position),
    InvalidVolume(Position),
    IoError(io::Error),
    UnexpectedRemains(Position),
}

type Result<T> = std::result::Result<T, MMLError>;

// MMLで記述されたコマンドをトーンシーケンスイベント列に変換する
pub(crate) fn parse(src: &str) -> Result<Vec<u8>> {
    let mut mml = Mml::new(src);

    mml.parse_tempo()?;

    mml.parse_resolution()?;

    let mut buf: Vec<u8> = Vec::new();
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
        Err(f(self.cur.clone()))
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
        if matches!(character, Some('\n')) {
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

    // 数字があれば数値として読み込む。
    // 呼び出し時に数字が無い場合は0を返す。
    // 数字以外の文字があったらそこで読み込みをやめる。
    // 数値が1000を超える場合は0x10000を返す。(何故･･･)。
    // 数字があることが前提なら呼び出し側が事前にチェックする必要がある。(頭文字が数字かを確認すればOK)。
    fn parse_number(&mut self) -> i32 {
        let mut number: i32 = 0;
        while let Some(ch) = self.get_char().filter(char::is_ascii_digit) {
            let d: i32 = ch.to_digit(10).unwrap() as i32;
            number = number * 10 + d;
            if number > 1000 {
                // 元のソースコードがこうなっているが
                // 何故こう処理してるのかわからんし
                // nextCharしてないのも謎
                // つまり最後の文字は消費しない形になっている
                return 0x10000;
            }
            self.next_char();
        }
        number
    }

    fn parse_tempo(&mut self) -> Result<()> {
        self.skip_whitespaces();

        if !matches!(self.get_char(), Some('T' | 't')) {
            return Ok(());
        }
        self.next_char();

        let tempo: i32 = self.parse_number();

        if (20..=508).contains(&tempo) {
            self.tempo = tempo;
            Ok(())
        } else {
            self.error(InvalidTempo)
        }
    }

    fn parse_resolution(&mut self) -> Result<()> {
        self.skip_whitespaces();

        if !matches!(self.get_char(), Some('%')) {
            return Ok(());
        }

        self.next_char();

        let resolution: i32 = self.parse_number();

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

        if !matches!(self.get_char(), Some('{')) {
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

        if !matches!(self.get_char(), Some('}')) {
            return self.error(InvalidBlockEnd);
        }

        self.next_char();

        dst.write_byte(tone_control::BLOCK_END.into())?;
        dst.write_byte(id)?;

        self.next_block_id += 1;

        Ok(true)
    }

    // シーケンスコマンドをトーンシーケンスイベントに変換する
    fn parse_sequence<W: io::Write>(&mut self, dst: &mut JavaDataOutput<W>) -> Result<i32> {
        self.skip_whitespaces();

        // 最下位ビット: トーンシーケンスイベントの有無
        // それ以外のビット: 音出し・無音のトーンシーケンスイベントの個数(イベントが1個だけであるときに呼び出し側が特別な処理をする場合がある)
        let mut event: i32 = 0;

        while self.has_char() {
            // clippyさん･･･何故わかってくれぬ･･･
            #[allow(clippy::if_same_then_else)]
            if self.parse_change_octave()? {
                self.skip_whitespaces();
                // これは内部データの更新コマンドだから次のコマンドへ
                continue;
            } else if self.parse_change_duration()? {
                self.skip_whitespaces();
                // これは内部データの更新コマンドだから次のコマンドへ
                continue;
            } else if self.parse_note(dst)? {
                // 音出しコマンド (CDEFGABで指定)
                event += 2;
            } else if self.parse_rest(dst)? {
                // 無音コマンド
                event += 2;
            } else if self.parse_note_value(dst)? {
                // 音出しコマンド (ノート番号で指定)
                event += 2;
            } else if self.parse_play_block(dst)? {
                // 指定IDのブロックの再生コマンド
            } else if self.parse_repeat(dst)? {
                // リピート記述の読み込み
            } else if self.parse_volume(dst)? {
                // ボリューム変更コマンド
            } else if matches!(self.get_char(), Some(']' | '}')) {
                // ブロック/リピートの終了
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

        if (0..=127).contains(&self.octave) {
            Ok(true)
        } else {
            self.error(InvalidOctaveValue)
        }
    }

    fn parse_change_duration(&mut self) -> Result<bool> {
        if !matches!(self.get_char(), Some('L' | 'l')) {
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

        if !(0..=127).contains(&note) {
            return self.error(InvalidNote);
        }

        let dur: i32 = self.parse_duration()?;

        dst.write_byte(note)?;
        dst.write_byte(dur)?;

        Ok(true)
    }

    fn parse_rest<W: io::Write>(&mut self, dst: &mut JavaDataOutput<W>) -> Result<bool> {
        if !matches!(self.get_char(), Some('R' | 'r')) {
            return Ok(false);
        }

        self.next_char();

        let dur: i32 = self.parse_duration()?;

        dst.write_byte(tone_control::SILENCE.into())?;
        dst.write_byte(dur)?;

        Ok(true)
    }

    fn parse_note_value<W: io::Write>(&mut self, dst: &mut JavaDataOutput<W>) -> Result<bool> {
        if !matches!(self.get_char(), Some('N' | 'n')) {
            return Ok(false);
        }

        if self.next_char().filter(|ch| *ch == '(').is_none() {
            return self.error(InvalidNoteValueStart);
        }

        if self.next_char().filter(char::is_ascii_digit).is_none() {
            return self.error(InvalidNoteValue);
        }

        let note: i32 = self.parse_number();

        if !(0..=127).contains(&note) {
            return self.error(InvalidNoteValue);
        }

        if !matches!(self.get_char(), Some(')')) {
            return self.error(InvalidNoteValueEnd);
        }

        self.next_char();

        // NOTE-VALUEコマンドはデフォルトの音長がOKなのか…？

        let dur: i32 = self.parse_duration()?;

        dst.write_byte(note)?;
        dst.write_byte(dur)?;

        Ok(true)
    }

    fn parse_play_block<W: io::Write>(&mut self, dst: &mut JavaDataOutput<W>) -> Result<bool> {
        if !matches!(self.get_char(), Some('$')) {
            return Ok(false);
        }

        if self.next_char().filter(char::is_ascii_digit).is_none() {
            return self.error(InvalidPlayBlockId);
        }

        let id: i32 = self.parse_number();

        if !(0..self.next_block_id).contains(&id) {
            return self.error(InvalidPlayBlockId);
        }

        dst.write_byte(tone_control::PLAY_BLOCK.into())?;
        dst.write_byte(id)?;

        Ok(true)
    }

    fn parse_repeat<W: io::Write>(&mut self, dst: &mut JavaDataOutput<W>) -> Result<bool> {
        if !matches!(self.get_char(), Some('[')) {
            return Ok(false);
        }

        if self.next_char().filter(char::is_ascii_digit).is_none() {
            return self.error(InvalidRepeat);
        }

        let multiplier: i32 = self.parse_number();

        if !(2..=127).contains(&multiplier) {
            return self.error(InvalidRepeatNumber);
        }

        let mut buf: Vec<u8> = Vec::new();
        let mut tmp = JavaDataOutput::new(&mut buf);

        let event: i32 = self.parse_sequence(&mut tmp)?;

        if event == 0 || buf.is_empty() {
            return self.error(InvalidRepeat);
        }

        self.skip_whitespaces();

        if !matches!(self.get_char(), Some(']')) {
            return self.error(InvalidRepeatEnd);
        }

        self.next_char();

        if event == 3 && buf.len() == 2 {
            dst.write_byte(tone_control::REPEAT.into())?;
            dst.write_byte(multiplier)?;
            dst.write(&buf)?;
        } else {
            for _ in 0..multiplier {
                dst.write(&buf)?;
            }
        }

        Ok(true)
    }

    fn parse_volume<W: io::Write>(&mut self, dst: &mut JavaDataOutput<W>) -> Result<bool> {
        if !matches!(self.get_char(), Some('V' | 'v')) {
            return Ok(false);
        }

        if self.next_char().filter(char::is_ascii_digit).is_none() {
            return self.error(InvalidVolume);
        }

        let vol: i32 = self.parse_number();

        if !(0..=100).contains(&vol) {
            return self.error(InvalidVolume);
        }

        dst.write_byte(tone_control::SET_VOLUME.into())?;
        dst.write_byte(vol)?;

        Ok(true)
    }

    // 音長の記述があれば読み込む。
    // 音長の記述が無ければデフォルトの音長を返す。
    // 音長の記述の書式が不正ならばエラーを返す。
    //　音長の記述を必須にしたい場合は呼び出し側で確認する必要がある。(音長の書式の頭文字のみ確認すればOK)。
    fn parse_duration(&mut self) -> Result<i32> {
        match self.get_char() {
            Some(ch) if ch.is_ascii_digit() => {}
            Some('(') => {
                if self.next_char().filter(char::is_ascii_digit).is_none() {
                    return self.error(InvalidDuration);
                }
                let dur: i32 = self.parse_number();
                if !(0..=127).contains(&dur) {
                    return self.error(InvalidDuration);
                }
                if matches!(self.get_char(), Some(')')) {
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

impl From<io::Error> for MMLError {
    fn from(error: io::Error) -> Self {
        MMLError::IoError(error)
    }
}

impl std::fmt::Display for MMLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for MMLError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MMLError::IoError(error) => Some(error),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SUMM_SUMM_SUMM: &str = r#"
        {0 O5 L4 D C > B R }
        {1 O4 L8 A B < C > A G4 R4 }
        {2 $0 $1 }
        $2 [2 O4 L8 B < C D > B A B < C > A ] $2
    "#;

    const KUCKUCK_KUCKUCK_RUFTS_AUS_DEM_WALD: &str = r#"
        T150%96{0GFGF2.}O5[2C>AR]$0GGAB-2GAAB-<[3C2>A]$0
    "#;

    const MORGEN_KOMMT_DER_WEIHNACHTSMANN: &str = r#"
        T104
        {0
            L8
            FF<CC
            DDC4
            >B-B-AA
            G4FR
        }
        $0
        [2
            L8
            <CC>B-B-
            AAG4
        ]
        $0    
    "#;

    #[test]
    fn it_works() {
        let res1 = parse(SUMM_SUMM_SUMM);

        assert!(res1.is_ok(), "{:?}", res1);

        let res2 = parse(KUCKUCK_KUCKUCK_RUFTS_AUS_DEM_WALD);

        assert!(res2.is_ok(), "{:?}", res2);

        let res3 = parse(MORGEN_KOMMT_DER_WEIHNACHTSMANN);

        assert!(res3.is_ok(), "{:?}", res3);
    }
}
