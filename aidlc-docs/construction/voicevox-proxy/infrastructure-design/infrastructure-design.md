# Infrastructure Design: voicevox-proxy

## Infrastructure Overview

| Component | AWS Service | Configuration |
|---|---|---|
| API | API Gateway REST API (v1) | REST エンドポイント + API Key |
| Compute | Lambda (provided.al2023) | Rust カスタムランタイム |
| Authentication | API Gateway API Key + Usage Plan | x-api-key ヘッダー |
| Logging | CloudWatch Logs | Lambda 自動出力 |

## Service Mapping

### API Gateway REST API (v1)
- **Type**: REST API (v1) - ネイティブ API Key + Usage Plan サポート
- **Resources / Methods**:
  - `POST /v1/audio/speech` → Lambda (API Key Required)
  - `GET /v1/models` → Lambda (API Key Required)
- **API Key**: Usage Plan に紐づけた API Key で認証
  - クライアントは `x-api-key` ヘッダーで API Key を送信
  - OpenAI 互換のため `Authorization: Bearer <key>` も Lambda 内でサポート可能
- **Stage**: `prod`
- **Binary Media Types**: `audio/wav` (WAV バイナリレスポンス対応)

### Lambda Function
- **Runtime**: `provided.al2023` (Rust カスタムランタイム)
- **Architecture**: `arm64` (Graviton, コスト効率)
- **Memory**: 256 MB
- **Timeout**: 30 秒（VOICEVOX API 応答待ち考慮）
- **Environment Variables**:
  - `VOICEVOX_API_URL`: VOICEVOX REST API ベース URL
- **Bundling**: `cargo lambda build --release --arm64` でビルドした bootstrap バイナリ

### CloudWatch Logs
- **Log Group**: `/aws/lambda/{function-name}`
- **Retention**: 14 日間
- **Format**: `tracing` クレートによる構造化 JSON ログ

## CDK Stack Structure

```
VoicevoxProxyStack
├── Lambda Function (Rust, arm64, provided.al2023)
│   └── Environment: VOICEVOX_API_URL
├── REST API (API Gateway v1)
│   ├── Resource: /v1/audio/speech
│   │   └── POST → Lambda (apiKeyRequired: true)
│   ├── Resource: /v1/models
│   │   └── GET → Lambda (apiKeyRequired: true)
│   ├── Stage: prod
│   └── Binary Media Types: audio/wav
├── Usage Plan
│   └── API Key
└── CloudWatch Log Group
    └── Retention: 14 days
```

## Network Architecture

```
Client
  |
  | HTTPS (x-api-key header)
  v
API Gateway REST API (API Key validation)
  |
  | Lambda Invoke (sync)
  v
Lambda Function
  |
  | HTTP (reqwest)
  v
VOICEVOX API (external endpoint)
```

- API Key 認証は API Gateway レベルで処理（Lambda に到達する前に検証）
- Lambda → VOICEVOX API: Lambda のデフォルトインターネットアクセス（VPC 外）
- VPC 不要: VOICEVOX API は外部エンドポイント、データストアなし
