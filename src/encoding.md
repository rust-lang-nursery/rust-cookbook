# エンコード

| Recipe | Crates | Categories |
|--------|--------|------------|
| [文字列をパーセントエンコードする][ex-percent-encode] | [![url-badge]][url] | [![cat-encoding-badge]][cat-encoding] |
| [文字列をapplication/x-www-form-urlencodedとしてエンコードする][ex-urlencoded] | [![url-badge]][url] | [![cat-encoding-badge]][cat-encoding] |
| [hexのエンコード、デコード][ex-hex-encode-decode] | [![data-encoding-badge]][data-encoding] | [![cat-encoding-badge]][cat-encoding] |
| [base64のエンコード、デコード][ex-base64] | [![base64-badge]][base64] | [![cat-encoding-badge]][cat-encoding] |
| [CSVレコードを読み込む][ex-csv-read] | [![csv-badge]][csv] | [![cat-encoding-badge]][cat-encoding] |
| [違う区切りでCSVレコードを読み込む][ex-csv-delimiter] | [![csv-badge]][csv] | [![cat-encoding-badge]][cat-encoding] |
| [述語にマッチするCSVレコードをフィルタリング][ex-csv-filter] | [![csv-badge]][csv] | [![cat-encoding-badge]][cat-encoding] |
| [Serdeで無効なCSVデータをハンドル][ex-invalid-csv] | [![csv-badge]][csv] [![serde-badge]][serde] | [![cat-encoding-badge]][cat-encoding] |
| [レコードをCSVにシリアライズする][ex-serialize-csv] | [![csv-badge]][csv] | [![cat-encoding-badge]][cat-encoding] |
| [Serdeを使ってレコードをCSVにシリアライズする][ex-csv-serde] | [![csv-badge]][csv] [![serde-badge]][serde] | [![cat-encoding-badge]][cat-encoding] |
| [CSVファイルのカラムを一つ変換する][ex-csv-transform-column] | [![csv-badge]][csv] [![serde-badge]][serde] | [![cat-encoding-badge]][cat-encoding] |
| [構造化されていないJSONのシリアライズ、デシリアライズ][ex-json-value] | [![serde-json-badge]][serde-json] | [![cat-encoding-badge]][cat-encoding] |
| [TOML設定ファイルのデシリアライズ][ex-toml-config] | [![toml-badge]][toml] | [![cat-encoding-badge]][cat-encoding] |
| [リトルエンディアンのバイト順で整数を読み書きする][ex-byteorder-le] | [![byteorder-badge]][byteorder] | [![cat-encoding-badge]][cat-encoding] |

[ex-percent-encode]: encoding/strings.html#percent-encode-a-string
[ex-urlencoded]: encoding/strings.html#encode-a-string-as-applicationx-www-form-urlencoded
[ex-hex-encode-decode]: encoding/strings.html#encode-and-decode-hex
[ex-base64]: encoding/strings.html#encode-and-decode-base64
[ex-csv-read]: encoding/csv.html#read-csv-records
[ex-csv-delimiter]: encoding/csv.html#read-csv-records-with-different-delimiter
[ex-csv-filter]: encoding/csv.html#filter-csv-records-matching-a-predicate
[ex-invalid-csv]: encoding/csv.html#handle-invalid-csv-data-with-serde
[ex-serialize-csv]: encoding/csv.html#serialize-records-to-csv
[ex-csv-serde]: encoding/csv.html#serialize-records-to-csv-using-serde
[ex-csv-transform-column]: encoding/csv.html#transform-csv-column
[ex-json-value]: encoding/complex.html#serialize-and-deserialize-unstructured-json
[ex-toml-config]: encoding/complex.html#deserialize-a-toml-configuration-file
[ex-byteorder-le]: encoding/complex.html#read-and-write-integers-in-little-endian-byte-order


{{#include links.md}}
