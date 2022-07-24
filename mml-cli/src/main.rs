// mml-cli
// author: Leonardone @ NEETSDKASU

fn main() -> Result<(), ()> {
    match parse_args() {
        Err(None) => show_usage(),
        Err(Some(msg)) => {
            eprintln!("{}", msg);
            eprintln!();
            show_usage();
            return Err(());
        }
        Ok(Command::ListInst) => list_inst(),
        Ok(Command::MmlToSmf(args)) => {
            if let Err(msg) = mml2smf(args) {
                eprintln!("{}", msg);
                return Err(());
            }
        }
    }
    Ok(())
}

fn show_usage() {
    println!(
        r#"{pkg_name} v{version}
{description}

USAGE:
    {bin_name} mml2smf --input <mml-file> [OPTIONS]
    {bin_name} list-instruments
    {bin_name} [-h | --help]

OPTIONS:
    --output <output-file>
    --instrument <instrument-number>
"#,
        pkg_name = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION"),
        description = env!("CARGO_PKG_DESCRIPTION"),
        bin_name = env!("CARGO_BIN_NAME")
    );
}

enum Command {
    ListInst,
    MmlToSmf(MmlToSmfArgs),
}

fn parse_args() -> Result<Command, Option<String>> {
    let mut iter = std::env::args().skip(1);
    let command = match iter.next() {
        Some(command) => command,
        None => return Err(None),
    };
    match command.as_str() {
        "-h" | "--help" => Err(None),
        "mml2smf" => match MmlToSmfArgs::parse(&mut iter) {
            Ok(args) => Ok(Command::MmlToSmf(args)),
            Err(msg) => Err(Some(msg)),
        },
        "list-instruments" => {
            if iter.next().is_none() {
                Ok(Command::ListInst)
            } else {
                Err(Some(
                    "list-instrumentsコマンドでオプション引数は指定できません".into(),
                ))
            }
        }
        unknown => Err(Some(format!("不明のコマンド: {}", unknown))),
    }
}

fn list_inst() {
    for cat in mml_core::INSTRUMENT_CATEGORIES.iter() {
        println!("{}", cat.name_ja());
        for inst in cat.instruments().iter() {
            println!("   {:3} - {}", *inst as i32, inst.name_ja());
        }
    }
}

struct MmlToSmfArgs {
    input_file: String,
    output_file: Option<String>,
    instrument: mml_core::Instrument,
}

impl MmlToSmfArgs {
    fn parse<T>(iter: &mut T) -> Result<Self, String>
    where
        T: Iterator,
        T::Item: AsRef<str>,
    {
        let mut input_file: Option<T::Item> = None;
        let mut output_file: Option<T::Item> = None;
        let mut instrument: Option<T::Item> = None;
        while let Some(arg) = iter.next() {
            match arg.as_ref() {
                "--input" => match iter.next() {
                    None => return Err("<mml-file>が指定されてまいません".into()),
                    item => input_file = item,
                },
                "--output" => match iter.next() {
                    None => return Err("<output-file>が指定されてまいません".into()),
                    item => output_file = item,
                },
                "--instrument" => match iter.next() {
                    None => return Err("<instrument-number>が指定されてまいません".into()),
                    item => instrument = item,
                },
                unknown => return Err(format!("不明のオプション: {}", unknown)),
            }
        }
        let input_file = match input_file {
            Some(file) => file.as_ref().to_owned(),
            None => return Err("--input <mml-file>が指定されていません".into()),
        };
        let output_file = output_file.map(|s| s.as_ref().to_owned());
        let instrument = match instrument {
            None => mml_core::INSTRUMENTS[0],
            Some(num_str) => {
                let num_str = num_str.as_ref();
                let num = match num_str.parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        return Err(format!("<instrument-number>の指定が不正です: {}", num_str))
                    }
                };
                if (1..=mml_core::INSTRUMENTS.len()).contains(&num) {
                    mml_core::INSTRUMENTS[num - 1]
                } else {
                    return Err(format!("<instrument-number>の指定が不正です: {}", num_str));
                }
            }
        };
        Ok(MmlToSmfArgs {
            input_file,
            output_file,
            instrument,
        })
    }
}

fn mml2smf(
    MmlToSmfArgs {
        input_file,
        output_file,
        instrument,
    }: MmlToSmfArgs,
) -> Result<(), String> {
    let input_file = std::path::Path::new(&input_file);
    if !input_file.is_file() {
        return Err(format!("{}が見つかりません", input_file.display()));
    }
    let output_file = match output_file {
        Some(file) => file,
        None => format!("{}.mid", input_file.file_name().unwrap().to_string_lossy()),
    };
    let output_file = std::path::Path::new(&output_file);
    eprintln!("入力: {}", input_file.display());
    eprintln!("出力: {}", output_file.display());
    eprintln!("楽器: {} - {}", instrument as i32, instrument.name_ja());
    eprintln!();
    eprintln!("処理を開始します");
    let src = match std::fs::read_to_string(input_file) {
        Ok(src) => src,
        Err(error) => {
            return Err(format!(
                "{}を読み込めませんでした: {:?}",
                input_file.display(),
                error
            ))
        }
    };
    let dst = match mml_core::convert(&src, instrument) {
        Ok(dst) => dst,
        Err(mml_core::MMLError::IoError(error)) => return Err(format!("{:?}", error)),
        Err(error) => return Err(format!("MMLエラー: {:?}", error)),
    };
    if let Err(error) = std::fs::write(output_file, dst) {
        return Err(format!("{:?}", error));
    }
    eprintln!("MMLからSMFファイルへの変換に成功しました");
    Ok(())
}
