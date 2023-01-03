#!/bin/bash -ex

# <https://cloud.google.com/endpoints/docs/openapi/enable-api#gcloud>

# gcloud projects list
# gcloud config set project "${project_id}"

gcloud services enable iam.googleapis.com

# gcloud services list
