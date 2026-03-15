# Code Generation Plan: voicevox-proxy

## Unit Context
- **Project Type**: Greenfield single unit
- **Language**: Rust (Lambda) + TypeScript (CDK)
- **Code Location**: Workspace root

## Project Structure
```
voicevox-proxy/           # Rust Lambda project
├── Cargo.toml
├── src/
│   ├── main.rs           # Lambda handler + routing
│   ├── models.rs         # Request/Response DTOs
│   ├── error.rs          # AppError + OpenAI error response
│   ├── speech_service.rs # Speech synthesis orchestration
│   ├── models_service.rs # Models list service
│   ├── voicevox_client.rs# VOICEVOX REST API client
│   └── voice_mapping.rs  # voice-mapping.json loader + resolver
├── voice-mapping.json    # model+voice → speaker_id mapping
cdk/                      # CDK infrastructure
├── bin/
│   └── cdk.ts
├── lib/
│   └── voicevox-proxy-stack.ts
├── cdk.json
├── package.json
└── tsconfig.json
```

## Generation Steps

### Step 1: Rust Project Setup
- [x] Create `voicevox-proxy/Cargo.toml` with dependencies (lambda_http, lambda_runtime, reqwest, serde, serde_json, tokio, tracing, tracing-subscriber)
- [x] Create `voice-mapping.json` with sample mapping

### Step 2: Models & Error Types
- [x] Create `voicevox-proxy/src/models.rs` (SpeechRequest, ModelsResponse, ModelObject, ErrorResponse, ErrorDetail)
- [x] Create `voicevox-proxy/src/error.rs` (AppError enum, to_response, error_type, status_code)

### Step 3: Voice Mapping
- [x] Create `voicevox-proxy/src/voice_mapping.rs` (load, resolve, models, voices)

### Step 4: VOICEVOX Client
- [x] Create `voicevox-proxy/src/voicevox_client.rs` (new, audio_query, synthesis)

### Step 5: Services
- [x] Create `voicevox-proxy/src/speech_service.rs` (synthesize, validate_request)
- [x] Create `voicevox-proxy/src/models_service.rs` (list_models)

### Step 6: Lambda Handler
- [x] Create `voicevox-proxy/src/main.rs` (handler, routing, initialization)

### Step 7: CDK Infrastructure
- [x] Create `cdk/package.json`
- [x] Create `cdk/tsconfig.json`
- [x] Create `cdk/cdk.json`
- [x] Create `cdk/bin/cdk.ts`
- [x] Create `cdk/lib/voicevox-proxy-stack.ts` (Lambda + REST API + API Key + Usage Plan)

### Step 8: Documentation
- [x] Create `aidlc-docs/construction/voicevox-proxy/code/code-summary.md`
