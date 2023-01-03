#!/bin/bash -ex

# <https://cloud.google.com/endpoints/docs/openapi/enable-api#gcloud>

# gcloud projects list
# gcloud config set project "${project_id}"

# gcloud services list --available | grep artifact
name=artifactregistry.googleapis.com
if ! gcloud services list --enabled | grep "${name}" > /dev/null
then
  gcloud services enable "${name}"
fi

# gcloud services list --enabled
