# ビルド手順

## 前提条件

| 項目 | 要件 |
|---|---|
| Rust | stable (1.75+) |
| cargo-lambda | `cargo install cargo-lambda` |
| Node.js | 22.x |
| AWS CDK CLI | `npm install -g aws-cdk` |
| AWS CLI | v2 (デプロイ・API Key 取得用) |
| AWS 認証情報 | `~/.aws/credentials` または環境変数 |

## 環境変数

| 変数 | 用途 | 設定タイミング |
|---|---|---|
| `VOICEVOX_API_URL` | VOICEVOX REST API ベース URL | CDK context で指定 |
| `CDK_DEFAULT_ACCOUNT` | AWS アカウント ID | デプロイ時 |
| `CDK_DEFAULT_REGION` | AWS リージョン | デプロイ時 |

---

## 1. Rust Lambda ビルド

```bash
# プロジェクトルートから実行
cd voicevox-proxy

# 依存関係の確認
cargo check

# ARM64 Lambda 向けリリースビルド
cargo lambda build --release --arm64
```

### ビルド成功の確認
- `voicevox-proxy/target/lambda/voicevox-proxy/bootstrap` が生成されること
- コンパイルエラー・警告がないこと

---

## 2. CDK ビルド

```bash
cd cdk

# 依存関係インストール
npm install

# TypeScript → JavaScript ビルド (esbuild)
npm run build
```

### ビルド成功の確認
- `cdk/dist/cdk.js` が生成されること

---

## 3. CDK デプロイ

```bash
cd cdk

# VOICEVOX API URL を指定してデプロイ
npx cdk deploy --context voicevoxApiUrl=http://<VOICEVOX_HOST>:50021

# 初回の場合は bootstrap が必要
# npx cdk bootstrap
```

### デプロイ成功の確認
- CloudFormation スタック `VoicevoxProxyStack` が CREATE_COMPLETE
- Outputs に `ApiUrl` と `ApiKeyId` が表示される

---

## 4. API Key 取得

```bash
# デプロイ出力の ApiKeyId を使用
aws apigateway get-api-key --api-key <ApiKeyId> --include-value --query 'value' --output text
```

---

## トラブルシューティング

### cargo lambda が見つからない
```bash
cargo install cargo-lambda
```

### CDK bootstrap エラー
```bash
npx cdk bootstrap aws://<ACCOUNT_ID>/<REGION>
```

### Lambda コードパスエラー
`cargo lambda build` 実行後に `target/lambda/voicevox-proxy/bootstrap` が存在することを確認。CDK スタックは `../voicevox-proxy/target/lambda/voicevox-proxy/` を参照。
