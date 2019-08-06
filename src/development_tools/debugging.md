## デバッグ

| Recipe | Crates | Categories |
|--------|--------|------------|
| [コンソールにデバッグログメッセージを表示する][ex-log-debug] | [![log-badge]][log] [![env_logger-badge]][env_logger] | [![cat-debugging-badge]][cat-debugging] |
| [コンソールにエラーログメッセージを表示する][ex-log-error] | [![log-badge]][log] [![env_logger-badge]][env_logger] | [![cat-debugging-badge]][cat-debugging] |
| [stderrを使ったログの表示][ex-log-stdout] | [![log-badge]][log] [![env_logger-badge]][env_logger] | [![cat-debugging-badge]][cat-debugging] |
| [カスタムロガーを使ってメッセージを記録する][ex-log-custom-logger] | [![log-badge]][log] | [![cat-debugging-badge]][cat-debugging] |
| [Unix syslogに記録する][ex-log-syslog] | [![log-badge]][log] [![syslog-badge]][syslog] | [![cat-debugging-badge]][cat-debugging] |
| [モジュールごとのログレベルを許可する][ex-log-mod] | [![log-badge]][log] [![env_logger-badge]][env_logger] | [![cat-debugging-badge]][cat-debugging] |
| [ロギングのセットアップにカスタム環境戦数を使う][ex-log-env-variable] | [![log-badge]][log] [![env_logger-badge]][env_logger] | [![cat-debugging-badge]][cat-debugging] |
| [タイムスタンを含めたログメッセージ][ex-log-timestamp] | [![log-badge]][log] [![env_logger-badge]][env_logger] [![chrono-badge]][chrono] | [![cat-debugging-badge]][cat-debugging] |
| [カスタムした場所にログメッセージを記録する。][ex-log-custom] | [![log-badge]][log] [![log4rs-badge]][log4rs] | [![cat-debugging-badge]][cat-debugging] |

[ex-log-debug]: development_tools/debugging/log.html#log-a-debug-message-to-the-console
[ex-log-error]: development_tools/debugging/log.html#log-an-error-message-to-the-console
[ex-log-stdout]: development_tools/debugging/log.html#log-to-stdout-instead-of-stderr
[ex-log-custom-logger]:  development_tools/debugging/log.html#log-messages-with-a-custom-logger
[ex-log-syslog]: development_tools/debugging/log.html#log-to-the-unix-syslog
[ex-log-mod]: development_tools/debugging/config_log.html#enable-log-levels-per-module
[ex-log-env-variable]: development_tools/debugging/config_log.html#use-a-custom-environment-variable-to-set-up-logging
[ex-log-timestamp]: development_tools/debugging/config_log.html#include-timestamp-in-log-messages
[ex-log-custom]: development_tools/debugging/config_log.html#log-messages-to-a-custom-location

{{#include ../links.md}}
