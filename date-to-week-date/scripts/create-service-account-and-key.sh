#!/bin/bash -ex

# <https://cloud.google.com/iam/docs/creating-managing-service-accounts>

project_id=
service_account_name=
description=
display_name="${service_account_name}"
key_file="${service_account_name}.json"

gcloud iam service-accounts create "${service_account_name}" \
    --description="${description}" \
    --display-name="${display_name}"

# gcloud iam service-accounts list

# TODO: set role

gcloud iam service-accounts keys create "${key_file}" \
    --iam-account="${service_account_name}@${project_id}.iam.gserviceaccount.com"

# gcloud iam service-accounts keys list \
#     --iam-account="${service_account_name}@${project_id}.iam.gserviceaccount.com"
