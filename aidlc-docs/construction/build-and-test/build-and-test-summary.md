# ビルド・テスト サマリ

## ビルド状況

| コンポーネント | ビルドコマンド | 成果物 |
|---|---|---|
| Rust Lambda | `cargo lambda build --release --arm64` | `target/lambda/voicevox-proxy/bootstrap` |
| CDK | `npm run build` | `dist/cdk.js` |

## テスト実行サマリ

### ユニットテスト
- **状態**: テストコード未生成 (PoC レベル)
- **推奨テストケース**: 文書化済み (`unit-test-instructions.md`)

### 結合テスト
- **ローカルテスト**: `cargo lambda watch` + VOICEVOX ENGINE
- **デプロイ後テスト**: API Gateway + API Key 認証
- **テストシナリオ**: 音声合成、モデル一覧、エラーケース、認証

### パフォーマンステスト
- **状態**: N/A (PoC レベル、NFR スキップ)

### その他テスト
- **セキュリティテスト**: N/A (セキュリティ拡張スキップ)
- **E2E テスト**: N/A
- **コントラクトテスト**: N/A

## 全体ステータス

| 項目 | 状態 |
|---|---|
| ビルド手順 | ✅ 文書化完了 |
| ユニットテスト手順 | ✅ 文書化完了 |
| 結合テスト手順 | ✅ 文書化完了 |
| Operations 準備 | ✅ デプロイ手順はビルド手順に含む |

## 生成ファイル一覧

| ファイル | 内容 |
|---|---|
| `build-instructions.md` | ビルド・デプロイ手順 |
| `unit-test-instructions.md` | ユニットテスト推奨ケース |
| `integration-test-instructions.md` | ローカル・デプロイ後の結合テスト |
| `build-and-test-summary.md` | 本ファイル |
