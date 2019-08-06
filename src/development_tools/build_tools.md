# ビルド時のツール

This section covers "build-time" tooling, or code that is run prior to compiling a crate's source code.
Conventionally, build-time code lives in a **build.rs** file and is commonly referred to as a "build script".
Common use cases include rust code generation and compilation of bundled C/C++/asm code.
See crates.io's [documentation on the matter][build-script-docs] for more information.

このセクションではビルド時のツール、またはクレートをコンパイルする前に実行するコードについて説明しています。従来、ビルド時のコードは**build.rs**にあり、一般に”ビルドスクリプト”と呼ばれています。一般的な使用例にはrustコードの生成やバンドルされたC/C++/アセンブリコードのコンパイルが含まれます。詳細についてはcrates.ioの[documentation on the matter][build-script-docs]を見てください。
{{#include build_tools/cc-bundled-static.md}}

{{#include build_tools/cc-bundled-cpp.md}}

{{#include build_tools/cc-defines.md}}

{{#include ../links.md}}

[build-script-docs]: http://doc.crates.io/build-script.html
