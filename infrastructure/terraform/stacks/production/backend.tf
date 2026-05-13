# Remote state in GCS
#
# Before running terraform init, create the bucket:
#   gsutil mb -p PROJECT_ID -l us-central1 gs://memoir-terraform-state
#   gsutil versioning set on gs://memoir-terraform-state

terraform {
  backend "gcs" {
    bucket = "memoir-terraform-state"
    prefix = "clusters/production"
  }
}
