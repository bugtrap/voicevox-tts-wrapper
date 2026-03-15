# Deployment Architecture

## CDK Project Structure

```
cdk/
├── bin/
│   └── cdk.ts                  # CDK App エントリーポイント
├── lib/
│   └── voicevox-proxy-stack.ts # メインスタック定義
├── cdk.json
├── package.json
└── tsconfig.json
```

## Stack: VoicevoxProxyStack

### Lambda Function
```typescript
const fn = new lambda.Function(this, 'VoicevoxProxyFunction', {
  runtime: lambda.Runtime.PROVIDED_AL2023,
  architecture: lambda.Architecture.ARM_64,
  handler: 'bootstrap',
  code: lambda.Code.fromAsset('../target/lambda/voicevox-proxy/'),
  memorySize: 256,
  timeout: Duration.seconds(30),
  environment: {
    VOICEVOX_API_URL: '<configurable>',
  },
});
```

### REST API + Routes + API Key
```typescript
// REST API
const api = new apigateway.RestApi(this, 'VoicevoxProxyApi', {
  binaryMediaTypes: ['audio/wav'],
});

// Lambda Integration
const lambdaIntegration = new apigateway.LambdaIntegration(fn);

// Routes
const v1 = api.root.addResource('v1');
const audioSpeech = v1.addResource('audio').addResource('speech');
audioSpeech.addMethod('POST', lambdaIntegration, { apiKeyRequired: true });

const models = v1.addResource('models');
models.addMethod('GET', lambdaIntegration, { apiKeyRequired: true });

// Usage Plan + API Key
const plan = api.addUsagePlan('UsagePlan', {
  name: 'VoicevoxProxyUsagePlan',
  throttle: { rateLimit: 10, burstLimit: 5 },
});

const apiKey = api.addApiKey('ApiKey');
plan.addApiKey(apiKey);
plan.addApiStage({ stage: api.deploymentStage });
```

### 最終構成

```
VoicevoxProxyStack
├── Lambda Function
│   ├── Runtime: provided.al2023 (arm64)
│   ├── Memory: 256 MB
│   ├── Timeout: 30s
│   └── Env: VOICEVOX_API_URL
├── REST API (v1)
│   ├── POST /v1/audio/speech (apiKeyRequired)
│   ├── GET /v1/models (apiKeyRequired)
│   ├── Binary Media Types: audio/wav
│   └── Stage: prod
├── Usage Plan
│   ├── Rate Limit: 10 req/s
│   ├── Burst Limit: 5
│   └── API Key
└── CloudWatch Log Group (14 days retention)
```

## Build & Deploy Flow

```
1. cargo lambda build --release --arm64
   → target/lambda/voicevox-proxy/bootstrap

2. cd cdk && npm run build
   → TypeScript コンパイル

3. cdk deploy
   → CloudFormation でデプロイ

4. API Key 取得
   → aws apigateway get-api-keys --include-values
```

## Configuration Parameters

| Parameter | Source | Description |
|---|---|---|
| `VOICEVOX_API_URL` | CDK context / env | VOICEVOX API ベース URL |
| `memorySize` | CDK stack | Lambda メモリ (default: 256) |
| `timeout` | CDK stack | Lambda タイムアウト (default: 30s) |
| `rateLimit` | CDK stack | API スロットリング (default: 10 req/s) |
| `burstLimit` | CDK stack | API バースト制限 (default: 5) |
