#!/usr/bin/env node
import * as cdk from "aws-cdk-lib";
import { VoicevoxProxyStack } from "../lib/voicevox-proxy-stack";

const app = new cdk.App();

new VoicevoxProxyStack(app, "VoicevoxProxyStack", {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: process.env.CDK_DEFAULT_REGION,
  },
});
