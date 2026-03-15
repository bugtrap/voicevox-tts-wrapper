# Code Summary: voicevox-proxy

## Generated Files

### Rust Lambda (`voicevox-proxy/`)
| File | Purpose |
|---|---|
| `Cargo.toml` | プロジェクト定義・依存関係 |
| `voice-mapping.json` | model+voice → speaker_id マッピング |
| `src/main.rs` | Lambda ハンドラー、ルーティング、初期化 |
| `src/models.rs` | SpeechRequest, ModelsResponse, ErrorResponse DTOs |
| `src/error.rs` | AppError enum, OpenAI 互換エラーレスポンス変換 |
| `src/voice_mapping.rs` | voice-mapping.json ローダー、model+voice 解決 |
| `src/voicevox_client.rs` | VOICEVOX REST API HTTP クライアント |
| `src/speech_service.rs` | 音声合成バリデーション・オーケストレーション |
| `src/models_service.rs` | モデル一覧生成 |

### CDK Infrastructure (`cdk/`)
| File | Purpose |
|---|---|
| `package.json` | CDK プロジェクト定義 |
| `tsconfig.json` | TypeScript 設定 |
| `cdk.json` | CDK アプリ設定 |
| `bin/cdk.ts` | CDK エントリーポイント |
| `lib/voicevox-proxy-stack.ts` | Lambda + REST API + API Key + Usage Plan |
