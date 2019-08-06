# 開発ツール

{{#include development_tools/debugging.md}}

## バージョン管理

| Recipe | Crates | Categories |
|--------|--------|------------|
| [パースとバージョン文字列のインクリメント][ex-semver-increment] | [![semver-badge]][semver] | [![cat-config-badge]][cat-config] |
| [複雑なバージョン文字列のパース][ex-semver-complex] | [![semver-badge]][semver] | [![cat-config-badge]][cat-config] |
| [プレリリースバージョンかチェックする][ex-semver-prerelease] | [![semver-badge]][semver] | [![cat-config-badge]][cat-config] |
| [与えられた範囲で最新バージョンを検索する][ex-semver-latest] | [![semver-badge]][semver] | [![cat-config-badge]][cat-config] |
| [外部コマンドのバージョン互換性を確認する][ex-semver-command] | [![semver-badge]][semver] | [![cat-text-processing-badge]][cat-text-processing] [![cat-os-badge]][cat-os]

## ビルド時

| Recipe | Crates | Categories |
|--------|--------|------------|
| [バンドルされたCライブラリをコンパイルし静的にリンクする][ex-cc-static-bundled] | [![cc-badge]][cc] | [![cat-development-tools-badge]][cat-development-tools] |
| [バンドルされたC++ライブラリをコンパイルし静的にリンクする][ex-cc-static-bundled-cpp] | [![cc-badge]][cc] | [![cat-development-tools-badge]][cat-development-tools] |
| [カスタム定義が設定されている時、Cライブラリをコンパイルする][ex-cc-custom-defines] | [![cc-badge]][cc] | [![cat-development-tools-badge]][cat-development-tools] |

[ex-semver-increment]: development_tools/versioning.html#parse-and-increment-a-version-string
[ex-semver-complex]: development_tools/versioning.html#parse-a-complex-version-string
[ex-semver-prerelease]: development_tools/versioning.html#check-if-given-version-is-pre-release
[ex-semver-latest]: development_tools/versioning.html#find-the-latest-version-satisfying-given-range
[ex-semver-command]: development_tools/versioning.html#check-external-command-version-for-compatibility

[ex-cc-static-bundled]: development_tools/build_tools.html#compile-and-link-statically-to-a-bundled-c-library
[ex-cc-static-bundled-cpp]: development_tools/build_tools.html#compile-and-link-statically-to-a-bundled-c-library-1
[ex-cc-custom-defines]: development_tools/build_tools.html#compile-a-c-library-while-setting-custom-defines

{{#include links.md}}
