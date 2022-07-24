# mml-core

[MML-on-OAP](https://github.com/neetsdkasu/MML-on-OAP)のMMLからMIDIファイル生成部分を移植＆ライブラリ化したもの



#### Cargo.toml
```toml
[dependencies]
mml-core = { package = "mml-core", git = "https://github.com/neetsdkasu/mml.git" }
```

#### Example
```rust
fn main() {
    let inst = mml_core::Instrument::AcousticGrandPiano;
    let mml_src: &str = "T150%96{0GFGF2.}O5[2C>AR]$0GGAB-2GAAB-<[3C2>A]$0";
    match mml_core::convert(mml_src, inst) {
        Err(error) => eprintln!("{:?}", error),
        Ok(smf_data) => {
            std::fs::write("music.mid", smf_data).unwrap();
        }
    }
}
```