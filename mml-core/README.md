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


### MMLの例

###### 例1: 曲名『 Summ, summ, summ 』
```
{0 O5 L4 D C > B R }
{1 O4 L8 A B < C > A G4 R4 }
{2 $0 $1 }
$2 [2 O4 L8 B < C D > B A B < C > A ] $2
```

###### 例2: 曲名『 Kuckuck, Kuckuck, ruft’s aus dem Wald 』
```
T150%96{0GFGF2.}O5[2C>AR]$0GGAB-2GAAB-<[3C2>A]$0
```

###### 例3: 曲名『 Morgen kommt der Weihnachtsmann 』
```
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
```


### MML構文

```
MML構文

*1テンポ *1分解能 *128ブロック 1*コード
※コード部はO4L4開始
※開始時の音量は100
※大文字小文字は区別しない

テンポ (4の倍数、20～508、省略時120)
T数字  120bpmなら T120

分解能(1～127、省略時64)
%数字   64なら %64
※分解能64なら全音符が64の長さ、四分音符が16の長さ

ブロック (番号 0～127、連番昇順、O4L4開始)
{番号 1*コード }
例  {0 CDEFGAB }

コード

音符コード (長さ 1～分解能)
 音 C D E F G A B
 シャープ C+ D+ F+ G+ A+
 (#も可) C# D# F# G# A#
 フラット D- E- G- A- B-
 休符 R
四分音符のCの音なら  C4
付点八分音符のCの音なら  C8.
分解能指定で四分音符(16)のCの音なら  C(16)
長さ省略時はデフォ長さ デフォ長さのCの音なら  C

オクターブ指定コード (数字、 -1～9)
O数字   オクターブ5なら O5
※オクターブ4のAの音が440Hz

オクターブ上げコード (1上げる)
<

オクターブ下げコード (1下げる)
>

デフォ長さ指定コード
L長さ
四分音符の長さなら  L4
付点八分音符の長さなら  L8.
分解能指定で四分音符(16)の長さなら  L(16)

ブロック再生コード
$番号   5番ブロック再生なら $5

リピート再生コード (回数 2～127)
[回数 1*コード ]
例 [3 CDEFGAB ]

音量指定コード (音量 0～100)
V音量   音量70なら V70

音値コード (音値 0～127)
  O-1のCの音値が0
  O4のCの音値が60
  O4のAの音値が69
  O9のGの音値が127
四分音符のO4のCの音なら  N(60)4
付点八分音符のO4のCの音なら  N(60)8.
分解能指定で四分音符(16)のO4のCの音なら  N(60)(16)
長さ省略時はデフォ長さ デフォ長さのO4のCの音なら  N(60)

オクターブとデフォ長さは指定コード以降に記述される一連コードに影響
例
 {0 O7L16 A B }
 O5L8 C < D [2 E O3L2 F ] $0 G
 は
 O5L8 C < D O6L8 E O3L2 F O6L8 E O3L2 F O7L16 A B O3L2 G
 に相当
 
音量は指定コード以降のすべての発音に影響
例
 {0 A V70 B }
 V90 C D [2 E V80 F ] $0 G
 は
 V90 C D E V80 F E F A V70 B G
 に相当
```