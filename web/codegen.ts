import type { CodegenConfig } from "@graphql-codegen/cli";

const config: CodegenConfig = {
  schema: "http://localhost:8083/graphql",
  documents: ["src/**/*.tsx", "src/**/*.ts", "src/graphql/**/*.graphql"],
  generates: {
    "./src/generated/": {
      preset: "client",
      config: {
        scalars: {
          UUID: "string",
          DateTime: "string",
          JSON: "Record<string, unknown>",
        },
      },
    },
  },
  ignoreNoDocuments: true,
};

export default config;
