import { defineBackend } from "@aws-amplify/backend";
import { auth } from "./auth/resource";
import { data } from "./data/resource";
import { postUpdateStature } from "./functions/post-update-stature/recource";
import * as apigw from "aws-cdk-lib/aws-apigateway";
/**
 * @see https://docs.amplify.aws/react/build-a-backend/ to add storage, functions, and more
 */
const backend = defineBackend({
  auth,
  data,
  postUpdateStature,
});

// Create the REST API
const api = new apigw.RestApi(
  backend.postUpdateStature.resources.lambda.stack,
  "StatureApi",
  {
    restApiName: "StatureApi",
    defaultCorsPreflightOptions: {
      allowOrigins: apigw.Cors.ALL_ORIGINS, // For production, replace with ['http://localhost:3000', 'yourdomain.com']
      allowMethods: apigw.Cors.ALL_METHODS,
      allowHeaders: [
        "Content-Type",
        "X-Amz-Date",
        "Authorization",
        "X-Api-Key",
        "x-stature-secret",
      ],
    },
  }
);

const updateStature = api.root.addResource("post-update-stature");
updateStature.addMethod(
  "POST",
  new apigw.LambdaIntegration(backend.postUpdateStature.resources.lambda)
);

backend.addOutput({
  custom: {
    API: {
      [api.restApiName]: {
        endpoint: api.url,
        region: api.stack.region,
        apiName: api.restApiName,
      },
    },
  },
});
