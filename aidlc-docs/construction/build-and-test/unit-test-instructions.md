# ユニットテスト手順

## 概要

本プロジェクトは PoC レベルのため、コード生成フェーズではユニットテストを生成していません。
以下は将来的にテストを追加する場合のガイドラインです。

---

## テスト実行

```bash
cd voicevox-proxy
cargo test
```

---

## テスト対象と推奨テストケース

### voice_mapping.rs
| テストケース | 内容 |
|---|---|
| 正常マッピング | `resolve("zundamon", "normal")` → `Ok(3)` |
| 存在しない model | `resolve("unknown", "normal")` → `Err(VoiceMapping)` |
| 存在しない voice | `resolve("zundamon", "unknown")` → `Err(VoiceMapping)` |
| models 一覧 | `models()` が全モデル名を返す |

### speech_service.rs (validate_request)
| テストケース | 内容 |
|---|---|
| 正常リクエスト | 全フィールド有効 → `Ok(())` |
| 空の model | → `Err(Validation)` |
| 空の input | → `Err(Validation)` |
| 空の voice | → `Err(Validation)` |
| 不正な response_format | `"mp3"` → `Err(Validation)` |
| speed 範囲外 (低) | `0.1` → `Err(Validation)` |
| speed 範囲外 (高) | `5.0` → `Err(Validation)` |

### voicevox_client.rs (urlencoded)
| テストケース | 内容 |
|---|---|
| ASCII テキスト | `"hello"` → `"hello"` |
| 日本語テキスト | `"こんにちは"` → 正しい UTF-8 % エンコード |
| 特殊文字 | `"a b&c"` → `"a%20b%26c"` |

### error.rs
| テストケース | 内容 |
|---|---|
| Validation エラー | status 400, type `invalid_request_error` |
| VoiceMapping エラー | status 400, code `invalid_voice` |
| VoicevoxApi エラー | status 500, type `server_error` |

### models_service.rs
| テストケース | 内容 |
|---|---|
| モデル一覧 | voice-mapping.json のモデルが全て返却される |
| レスポンス形式 | `object: "list"`, 各要素に `id`, `owned_by: "voicevox"` |

---

## テストカバレッジ

```bash
# cargo-tarpaulin でカバレッジ計測 (オプション)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```
