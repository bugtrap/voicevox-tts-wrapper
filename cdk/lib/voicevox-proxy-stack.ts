import * as cdk from "aws-cdk-lib";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as apigateway from "aws-cdk-lib/aws-apigateway";
import * as logs from "aws-cdk-lib/aws-logs";
import { Construct } from "constructs";

export class VoicevoxProxyStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const voicevoxApiUrl = this.node.tryGetContext("voicevoxApiUrl")
      ?? "http://localhost:50021";

    // Lambda Function
    const fn = new lambda.Function(this, "VoicevoxProxyFunction", {
      runtime: lambda.Runtime.PROVIDED_AL2023,
      architecture: lambda.Architecture.ARM_64,
      handler: "bootstrap",
      code: lambda.Code.fromAsset(
        "../voicevox-proxy/target/lambda/voicevox-proxy/"
      ),
      memorySize: 256,
      timeout: cdk.Duration.seconds(30),
      environment: {
        VOICEVOX_API_URL: voicevoxApiUrl,
      },
      logRetention: logs.RetentionDays.TWO_WEEKS,
      tracing: lambda.Tracing.ACTIVE,
    });

    // Access Log Group
    const accessLogGroup = new logs.LogGroup(this, "ApiAccessLogGroup", {
      retention: logs.RetentionDays.TWO_WEEKS,
    });

    // REST API
    const api = new apigateway.RestApi(this, "VoicevoxProxyApi", {
      restApiName: "VoicevoxProxy",
      binaryMediaTypes: ["audio/wav"],
      deployOptions: {
        stageName: "prod",
        // Access Logging
        accessLogDestination: new apigateway.LogGroupLogDestination(accessLogGroup),
        accessLogFormat: apigateway.AccessLogFormat.custom(
          JSON.stringify({
            requestId: apigateway.AccessLogField.contextRequestId(),
            ip: apigateway.AccessLogField.contextIdentitySourceIp(),
            httpMethod: apigateway.AccessLogField.contextHttpMethod(),
            requestTime: apigateway.AccessLogField.contextRequestTime(),
            resourcePath: apigateway.AccessLogField.contextResourcePath(),
            responseLength: apigateway.AccessLogField.contextResponseLength(),
            status: apigateway.AccessLogField.contextStatus(),
          })
        ),
        // Execution Logging
        loggingLevel: apigateway.MethodLoggingLevel.INFO,
        dataTraceEnabled: true,
        // X-Ray Tracing
        tracingEnabled: true,
      },
    });

    const lambdaIntegration = new apigateway.LambdaIntegration(fn);

    // Routes
    const v1 = api.root.addResource("v1");

    const audio = v1.addResource("audio");
    const speech = audio.addResource("speech");
    speech.addMethod("POST", lambdaIntegration, {
      apiKeyRequired: true,
    });

    const models = v1.addResource("models");
    models.addMethod("GET", lambdaIntegration, {
      apiKeyRequired: true,
    });

    // Usage Plan + API Key
    const plan = api.addUsagePlan("UsagePlan", {
      name: "VoicevoxProxyUsagePlan",
      throttle: {
        rateLimit: 10,
        burstLimit: 5,
      },
    });

    const apiKey = api.addApiKey("ApiKey");
    plan.addApiKey(apiKey);
    plan.addApiStage({ stage: api.deploymentStage });

    // Outputs
    new cdk.CfnOutput(this, "ApiUrl", {
      value: api.url,
      description: "API Gateway URL",
    });

    new cdk.CfnOutput(this, "ApiKeyId", {
      value: apiKey.keyId,
      description: "API Key ID (use aws apigateway get-api-key --api-key <id> --include-value to get the value)",
    });
  }
}
