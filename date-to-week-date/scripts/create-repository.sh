#!/bin/bash -ex

# <https://cloud.google.com/artifact-registry/docs/repositories/create-repos#gcloud>

location="${1}"
repository="${2}"

if [ -z "${location}" ] || [ -z "${repository}" ]
then
  exit 1
fi

"$(dirname "${0}")/enable-artifactregistry-api.sh"

gcloud artifacts repositories create "${repository}" \
    --repository-format=docker \
    --location="${location}"
