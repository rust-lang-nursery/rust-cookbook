## コマンドラインの引数をパースする

[![clap-badge]][clap] [![cat-command-line-badge]][cat-command-line]


このアプリケーションでは、clapのビルダースタイルを使用してコマンドラインインターフェイスの構造を紹介します。[documentation]にはアプリケーションをインスタンス化する方法が二つ書かれています。

ビルダースタイルでは、`with_name`は`value_of`が渡された値を取得するために使用する一意の識別子です。短いオプションと長いオプションは、ユーザーが入力すると予想されるフラグを制御します。短いフラグは-f、長いフラグは--fileのようなものです。

```rust
extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("My Test Program")
        .version("0.1.0")
        .author("Hackerman Jones <hckrmnjones@hack.gov>")
        .about("Teaches argument parsing")
        .arg(Arg::with_name("file")
                 .short("f")
                 .long("file")
                 .takes_value(true)
                 .help("A cool file"))
        .arg(Arg::with_name("num")
                 .short("n")
                 .long("number")
                 .takes_value(true)
                 .help("Five less than your favorite number"))
        .get_matches();

    let myfile = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", myfile);

    let num_str = matches.value_of("num");
    match num_str {
        None => println!("No idea what your favorite number is."),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("Your favorite number must be {}.", n + 5),
                Err(_) => println!("That's not a number! {}", s),
            }
        }
    }
}
```

Usageは`clap`により生成されます。サンプルアプリのusageはこちらです。

```
My Test Program 0.1.0
Hackerman Jones <hckrmnjones@hack.gov>
Teaches argument parsing

USAGE:
    testing [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>     A cool file
    -n, --number <num>    Five less than your favorite number
```

次のコマンドでアプリをテストすることができます。

```
$ cargo run -- -f myfile.txt -n 251
```

出力は:

```
The file passed is: myfile.txt
Your favorite number must be 256.
```

[documentation]: https://docs.rs/clap/
