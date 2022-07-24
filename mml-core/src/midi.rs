// mml-core
// author: Leonardone @ NEETSDKASU

use crate::tone_control;
use java_data_io::JavaDataOutput;
use std::io;

// トーンシーケンスイベント列をMIDIフォーマットのバイト列に変換する。
// トーンシーケンスイベント列に不正は無いことを前提としている。
// トーンシーケンスイベント列はMML-on-OAPのMMLから変換されていることを前提としている。
pub(crate) fn translate(tseq: &[u8], inst: i8) -> Result<Vec<u8>, io::Error> {
    use java_data_io::Error::*;
    match do_translate(tseq, inst) {
        Ok(res) => Ok(res),
        Err(IoError(error)) => Err(error),
        Err(UtfDataFormatError) => unreachable!("UtfDataFormatError"),
    }
}

trait GetInt {
    fn get_i8(&self, pos: usize) -> Option<i8>;
    fn get_i32(&self, pos: usize) -> Option<i32>;
    fn get_usize(&self, pos: usize) -> Option<usize>;
}

impl GetInt for &[u8] {
    fn get_i8(&self, pos: usize) -> Option<i8> {
        self.get(pos).map(|v| *v as i8)
    }
    fn get_i32(&self, pos: usize) -> Option<i32> {
        self.get(pos).map(|v| *v as i32)
    }
    fn get_usize(&self, pos: usize) -> Option<usize> {
        self.get(pos).map(|v| *v as usize)
    }
}

fn do_translate(tseq: &[u8], inst: i8) -> Result<Vec<u8>, java_data_io::Error> {
    let mut tempo: i32 = 120;
    let mut resolution: i32 = 64;

    // まぁ固定長配列でもいいかもだが･･･
    let mut block_pos: Vec<usize> = vec![0; 128];

    let mut buf: Vec<u8> = Vec::new();
    let mut dst = JavaDataOutput::new(&mut buf);

    let mut pos: usize = 2;

    if matches!(tseq.get_i8(pos), Some(tone_control::TEMPO)) {
        tempo = (tseq.get_i32(pos + 1).unwrap() & 0x7F) << 2;
        pos += 2;
    }

    if matches!(tseq.get_i8(pos), Some(tone_control::RESOLUTION)) {
        resolution = tseq.get_i32(pos + 1).unwrap() & 0x7F;
        pos += 2;
    }

    while matches!(tseq.get_i8(pos), Some(tone_control::BLOCK_START)) {
        let block_id: usize = tseq[pos + 1].into();
        block_pos[block_id] = pos + 2;
        pos += 2;
        while let Some(cmd) = tseq.get_i8(pos) {
            if cmd == tone_control::BLOCK_END {
                pos += 2;
                break;
            }
            if cmd == tone_control::REPEAT {
                pos += 4;
            } else {
                pos += 2;
            }
        }
    }

    // header signature MThd
    dst.write(b"MThd")?; // total 4 bytes

    // header length 32bit BE
    dst.write_int(6)?; // total 8 bytes

    // header midi format (= 0)  16bit BE
    dst.write_short(0)?; // total 10 bytes

    // header n-tracks (= 1) 16bit BE
    dst.write_short(1)?; // total 12 bytes

    // header division (= resolution / 4) 1bit(= 0) + 15bit BE
    dst.write_short((resolution >> 2).max(1))?; // total 14 bytes

    // track header MTrk
    dst.write(b"MTrk")?; // total 18 bytes

    // track length (= dummy value) 32bit BE
    const TRACK_LENGTH_POS: usize = 18;
    dst.write_int(0)?; // total 22 bytes

    const TRACK_START_POS: usize = 22;

    // set tempo (FF 51 03 tttttt)
    dst.write_byte(0)?; // delta time (= 0)
    dst.write(&[0xFF, 0x51, 0x03])?;
    // (120 bpm = 500,000 usec/beat) (bpm = beats/minute)
    let usec_tempo: i32 = 60_000_000 / tempo;
    dst.write(&usec_tempo.to_be_bytes()[1..])?; // tttttt

    // program change (Cn xx) (n = channel, xx = inst id)
    dst.write_byte(0)?; // delta time (= 0)
    dst.write_byte(0xC0)?; // Cn
    dst.write_byte(inst.into())?; // xx

    let mut last_note_on = false;
    let mut delta_time: i32 = 0;
    let mut volume: i32 = 127;

    let mut pos_stack: Vec<usize> = Vec::new();

    while let Some(cmd) = tseq.get_i8(pos) {
        match cmd {
            tone_control::PLAY_BLOCK => {
                pos_stack.push(pos + 2);
                let block_id: usize = tseq.get_usize(pos + 1).unwrap() & 0x7F;
                pos = block_pos[block_id];
            }
            tone_control::BLOCK_END => {
                pos = pos_stack.pop().unwrap();
            }
            tone_control::SET_VOLUME => {
                let vol: i32 = tseq.get_i32(pos + 1).unwrap() & 0x7F;
                volume = (127 * vol / 100) & 0x7F;
                pos += 2;
            }
            tone_control::SILENCE => {
                delta_time += tseq.get_i32(pos + 1).unwrap() & 0x7F;
                pos += 2;
            }
            tone_control::REPEAT => {
                let multiplier: i32 = tseq.get_i32(pos + 1).unwrap() & 0xFF;
                let cmd: i8 = tseq.get_i8(pos + 2).unwrap();
                let len: i32 = tseq.get_i32(pos + 3).unwrap() & 0x7F;
                if cmd == tone_control::SILENCE {
                    delta_time += multiplier * len;
                } else {
                    let note: i32 = (cmd & 0x7F).into();
                    // first note on
                    if delta_time <= 127 {
                        dst.write_byte(delta_time)?;
                    } else {
                        write_delta_time(&mut dst, delta_time)?;
                    }
                    if !last_note_on {
                        // note on status (9n kk vv)
                        dst.write_byte(0x90)?; // 9n
                    }
                    dst.write_byte(note)?; // kk
                    dst.write_byte(volume)?; // vv
                                             // note off (not on status (vel=0)) (running status kk 00)
                    dst.write_byte(len)?; // delta_time (len <= 127)
                    dst.write_byte(note)?; // kk
                    dst.write_byte(0)?; // 00
                                        // repeat
                    for _ in 1..multiplier {
                        // note on (running status kk vv)
                        dst.write_byte(0)?; // delta time
                        dst.write_byte(note)?; // kk
                        dst.write_byte(volume)?; // vv
                                                 // note off (running status kk 00)
                        dst.write_byte(len)?; // delta time (len <= 127)
                        dst.write_byte(note)?; // kk
                        dst.write_byte(0)?; // 00
                    }
                    last_note_on = true;
                    delta_time = 0;
                }
                pos += 4;
            }
            _ => {
                let note: i32 = (cmd & 0x7F).into();
                let len: i32 = tseq.get_i32(pos + 1).unwrap() & 0x7F;
                // note on
                if delta_time <= 127 {
                    dst.write_byte(delta_time)?;
                } else {
                    write_delta_time(&mut dst, delta_time)?;
                }
                if !last_note_on {
                    // note on status (9n kk vv)
                    dst.write_byte(0x90)?; // 9n
                }
                dst.write_byte(note)?; // kk
                dst.write_byte(volume)?; // vv
                                         // note off (note on status (running status kk 00))
                dst.write_byte(len)?; // delta_time (len <= 127)
                dst.write_byte(note)?; // kk
                dst.write_byte(0)?; // 00
                last_note_on = true;
                delta_time = 0;
                pos += 2;
            }
        }
    }

    // end of track (FF 2F 00)
    dst.write_byte(0)?; // delta time
    dst.write(&[0xFF, 0x2F, 0x00])?;

    let track_size: u32 = (buf.len() - TRACK_START_POS) as u32;

    buf[TRACK_LENGTH_POS..TRACK_LENGTH_POS + 4].copy_from_slice(&track_size.to_be_bytes());

    Ok(buf)
}

fn write_delta_time<W: io::Write>(
    dst: &mut JavaDataOutput<W>,
    mut delta_time: i32,
) -> Result<(), java_data_io::Error> {
    let mut temp: i32 = delta_time & 0x7F;
    loop {
        delta_time >>= 7;
        if delta_time <= 0 {
            break;
        }
        temp = (temp << 8) | 0x80 | (delta_time & 0x7F);
    }
    loop {
        dst.write_byte(temp)?;
        if (temp & 0x80) == 0 {
            break;
        }
        temp >>= 8;
    }
    Ok(())
}
