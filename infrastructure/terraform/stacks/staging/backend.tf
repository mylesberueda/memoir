# Remote state in GCS
#
# Before running terraform init, create the bucket:
#   gsutil mb -p PROJECT_ID -l us-central1 gs://startup-ai-terraform-state
#   gsutil versioning set on gs://startup-ai-terraform-state

terraform {
  backend "gcs" {
    bucket = "startup-ai-terraform-state"
    prefix = "clusters/staging"
  }
}
