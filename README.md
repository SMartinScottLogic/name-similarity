# name-similarity
Generate output of similarly named files (based on cosine similarity)

# OpenTelemetry
## Running Locally
Launch Jaeger in docker:
* docker run -d -e COLLECTOR_OTLP_ENABLED=true -p 14269:14269 -p 16686:16686 -p 4317:4317 -p 4318:4318 jaegertracing/all-in-one:latest

Set the below environment variables:
* OTEL_SERVICE_NAME: Desired *service* name to identify the application in Jaeger (e.g. *nom*)
* OTEL_EXPORTER_OTLP_ENDPOINT: HTTP endpoint to use to communicate with the collector (e.g. *http://localhost:4318/v1/traces*)