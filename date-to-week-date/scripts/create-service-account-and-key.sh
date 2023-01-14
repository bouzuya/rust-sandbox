#!/bin/bash -ex

# create a service account for deployment

# <https://cloud.google.com/iam/docs/creating-managing-service-accounts>

project_id="${1}"
service_account_name="${2}"
description=
display_name="${service_account_name}"
key_file="${service_account_name}.json"

if [ -z "${project_id}" ] || [ -z "${service_account_name}" ]
then
  exit 1
fi

"$(dirname "${0}")/enable-iam-api.sh"

gcloud iam service-accounts create "${service_account_name}" \
    --description="${description}" \
    --display-name="${display_name}"

# gcloud iam service-accounts list

gcloud projects add-iam-policy-binding "${project_id}" \
    --member="serviceAccount:${service_account_name}@${project_id}.iam.gserviceaccount.com" \
    --role="roles/artifactregistry.writer"

gcloud projects add-iam-policy-binding "${project_id}" \
    --member="serviceAccount:${service_account_name}@${project_id}.iam.gserviceaccount.com" \
    --role="roles/run.developer"

gcloud projects add-iam-policy-binding "${project_id}" \
    --member="serviceAccount:${service_account_name}@${project_id}.iam.gserviceaccount.com" \
    --role="roles/iam.serviceAccountUser"

gcloud iam service-accounts keys create "${key_file}" \
    --iam-account="${service_account_name}@${project_id}.iam.gserviceaccount.com"

# gcloud iam service-accounts keys list \
#     --iam-account="${service_account_name}@${project_id}.iam.gserviceaccount.com"
