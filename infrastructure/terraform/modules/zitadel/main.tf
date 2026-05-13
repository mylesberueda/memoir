# Zitadel Module - Authentication Configuration
#
# This module configures:
# - Instance-level settings (features, security)
# - Project for organizing applications
# - Machine users for each service
# - Service account keys for JWT authentication
# - API applications for OAuth capabilities
# - SMTP configuration for email verification

terraform {
  required_providers {
    zitadel = {
      source  = "zitadel/zitadel"
      version = "~> 2.7.0"
    }
    http = {
      source  = "hashicorp/http"
      version = "~> 3.4"
    }
  }
}

locals {
  one_year = "8760h"
}

# ============================================================================
# Organization Data Source
# ============================================================================

data "zitadel_orgs" "default" {
  name        = var.zitadel_org_name
  name_method = "TEXT_QUERY_METHOD_EQUALS"
}

data "zitadel_zitadel" "default" {}

data "http" "zitadel_instance" {
  url    = "http://${var.zitadel_domain}:${var.zitadel_port}/zitadel.instance.v2.InstanceService/GetInstance"
  method = "POST"

  request_headers = {
    Authorization = "Bearer ${data.zitadel_zitadel.default.token}"
    Content-Type  = "application/json"
  }

  request_body = "{}"
}

locals {
  org_id      = tolist(data.zitadel_orgs.default.ids)[0]
  instance_id = jsondecode(data.http.zitadel_instance.response_body).instance.id
}

# ============================================================================
# Instance-Level Settings
# ============================================================================

resource "zitadel_default_security_settings" "main" {
  enable_impersonation = true
}

# ============================================================================
# Project
# ============================================================================

resource "zitadel_project" "main" {
  org_id                 = local.org_id
  name                   = var.project_name
  project_role_assertion = true
  project_role_check     = true
  has_project_check      = true
}

# ============================================================================
# API Service (Rust)
# ============================================================================

resource "zitadel_machine_user" "api" {
  org_id            = local.org_id
  user_name         = "api-user"
  name              = "API Service"
  description       = "Machine user for api service - service-to-service authentication"
  access_token_type = "ACCESS_TOKEN_TYPE_JWT"
}

resource "zitadel_machine_key" "api" {
  org_id          = local.org_id
  user_id         = zitadel_machine_user.api.id
  key_type        = "KEY_TYPE_JSON"
  expiration_date = timeadd(timestamp(), local.one_year)

  lifecycle {
    ignore_changes = [expiration_date]
  }
}

resource "zitadel_application_api" "api" {
  org_id     = local.org_id
  project_id = zitadel_project.main.id
  name       = "api"
}

# ============================================================================
# Agent API Service (Python)
# ============================================================================

resource "zitadel_machine_user" "agent_api" {
  org_id            = local.org_id
  user_name         = "agent-api-user"
  name              = "Agent API Service"
  description       = "Machine user for agent-api service - service-to-service authentication"
  access_token_type = "ACCESS_TOKEN_TYPE_JWT"
}

resource "zitadel_machine_key" "agent_api" {
  org_id          = local.org_id
  user_id         = zitadel_machine_user.agent_api.id
  key_type        = "KEY_TYPE_JSON"
  expiration_date = timeadd(timestamp(), local.one_year)

  lifecycle {
    ignore_changes = [expiration_date]
  }
}

resource "zitadel_application_api" "agent_api" {
  org_id     = local.org_id
  project_id = zitadel_project.main.id
  name       = "agent-api"
}

# ============================================================================
# API Service - gRPC Microservice (Rust)
# ============================================================================

resource "zitadel_machine_user" "api_service" {
  org_id            = local.org_id
  user_name         = "api-service-user"
  name              = "API Service"
  description       = "Machine user for api-service gRPC microservice - service-to-service auth"
  access_token_type = "ACCESS_TOKEN_TYPE_JWT"
}

resource "zitadel_machine_key" "api_service" {
  org_id          = local.org_id
  user_id         = zitadel_machine_user.api_service.id
  key_type        = "KEY_TYPE_JSON"
  expiration_date = timeadd(timestamp(), local.one_year)

  lifecycle {
    ignore_changes = [expiration_date]
  }
}

resource "zitadel_application_api" "api_service" {
  org_id     = local.org_id
  project_id = zitadel_project.main.id
  name       = "api-service"
}

resource "zitadel_org_member" "api_service_user_manager" {
  org_id  = local.org_id
  user_id = zitadel_machine_user.api_service.id
  roles   = ["ORG_USER_MANAGER"]
}

# ============================================================================
# Rig Service - gRPC Microservice (Rust)
# ============================================================================

resource "zitadel_machine_user" "rig_service" {
  org_id            = local.org_id
  user_name         = "rig-service-user"
  name              = "Rig Service"
  description       = "Machine user for rig-service gRPC microservice - service-to-service auth"
  access_token_type = "ACCESS_TOKEN_TYPE_JWT"
}

resource "zitadel_machine_key" "rig_service" {
  org_id          = local.org_id
  user_id         = zitadel_machine_user.rig_service.id
  key_type        = "KEY_TYPE_JSON"
  expiration_date = timeadd(timestamp(), local.one_year)

  lifecycle {
    ignore_changes = [expiration_date]
  }
}

resource "zitadel_application_api" "rig_service" {
  org_id     = local.org_id
  project_id = zitadel_project.main.id
  name       = "rig-service"
}

# ============================================================================
# Chat Service - gRPC Microservice (Rust)
# ============================================================================

resource "zitadel_machine_user" "chat_service" {
  org_id            = local.org_id
  user_name         = "chat-service-user"
  name              = "Chat Service"
  description       = "Machine user for chat-service gRPC microservice - service-to-service auth"
  access_token_type = "ACCESS_TOKEN_TYPE_JWT"
}

resource "zitadel_machine_key" "chat_service" {
  org_id          = local.org_id
  user_id         = zitadel_machine_user.chat_service.id
  key_type        = "KEY_TYPE_JSON"
  expiration_date = timeadd(timestamp(), local.one_year)

  lifecycle {
    ignore_changes = [expiration_date]
  }
}

resource "zitadel_application_api" "chat_service" {
  org_id     = local.org_id
  project_id = zitadel_project.main.id
  name       = "chat-service"
}

# ============================================================================
# Notification Service - gRPC Microservice (Rust)
# ============================================================================

resource "zitadel_machine_user" "notification_service" {
  org_id            = local.org_id
  user_name         = "notification-service-user"
  name              = "Notification Service"
  description       = "Machine user for notification-service gRPC microservice - service-to-service auth"
  access_token_type = "ACCESS_TOKEN_TYPE_JWT"
}

resource "zitadel_machine_key" "notification_service" {
  org_id          = local.org_id
  user_id         = zitadel_machine_user.notification_service.id
  key_type        = "KEY_TYPE_JSON"
  expiration_date = timeadd(timestamp(), local.one_year)

  lifecycle {
    ignore_changes = [expiration_date]
  }
}

resource "zitadel_application_api" "notification_service" {
  org_id     = local.org_id
  project_id = zitadel_project.main.id
  name       = "notification-service"
}

# ============================================================================
# Project Roles - Access Control
# ============================================================================

resource "zitadel_project_role" "user" {
  org_id       = local.org_id
  project_id   = zitadel_project.main.id
  role_key     = "user"
  display_name = "User"
  group        = "default"
}

resource "zitadel_project_role" "admin" {
  org_id       = local.org_id
  project_id   = zitadel_project.main.id
  role_key     = "admin"
  display_name = "Administrator"
  group        = "staff"
}

resource "zitadel_project_role" "developer" {
  org_id       = local.org_id
  project_id   = zitadel_project.main.id
  role_key     = "developer"
  display_name = "Developer"
  group        = "staff"
}

# ============================================================================
# CLI Machine User - Administrative Operations
# ============================================================================

resource "zitadel_machine_user" "cli" {
  org_id            = local.org_id
  user_name         = "cli"
  name              = "CLI"
  description       = "Machine user for CLI tool PAT authentication"
  access_token_type = "ACCESS_TOKEN_TYPE_JWT"
}

resource "zitadel_personal_access_token" "cli" {
  org_id          = local.org_id
  user_id         = zitadel_machine_user.cli.id
  expiration_date = timeadd(timestamp(), local.one_year)

  lifecycle {
    ignore_changes = [expiration_date]
  }
}

resource "zitadel_org_member" "cli_org_owner" {
  org_id  = local.org_id
  user_id = zitadel_machine_user.cli.id
  roles   = ["ORG_OWNER"]
}

resource "zitadel_instance_member" "cli_iam_owner" {
  user_id = zitadel_machine_user.cli.id
  roles   = ["IAM_OWNER"]
}

# ============================================================================
# Web Application - OIDC with Token Exchange
# ============================================================================

resource "zitadel_application_oidc" "web" {
  org_id         = local.org_id
  project_id     = zitadel_project.main.id
  name           = "web"
  redirect_uris  = [var.zitadel_redirect_uri]
  response_types = ["OIDC_RESPONSE_TYPE_CODE"]
  grant_types = [
    "OIDC_GRANT_TYPE_AUTHORIZATION_CODE",
    "OIDC_GRANT_TYPE_REFRESH_TOKEN",
    "OIDC_GRANT_TYPE_TOKEN_EXCHANGE"
  ]
  app_type                    = "OIDC_APP_TYPE_WEB"
  auth_method_type            = "OIDC_AUTH_METHOD_TYPE_PRIVATE_KEY_JWT"
  version                     = "OIDC_VERSION_1_0"
  clock_skew                  = "0s"
  dev_mode                    = true
  access_token_type           = "OIDC_TOKEN_TYPE_JWT"
  access_token_role_assertion = true
  id_token_role_assertion     = true
  id_token_userinfo_assertion = true
  additional_origins          = []

  login_version {
    login_v2 {
      base_uri = var.app_login_url
    }
  }
}

resource "zitadel_application_key" "web" {
  org_id          = local.org_id
  project_id      = zitadel_project.main.id
  app_id          = zitadel_application_oidc.web.id
  key_type        = "KEY_TYPE_JSON"
  expiration_date = timeadd(timestamp(), local.one_year)

  lifecycle {
    ignore_changes = [expiration_date]
  }
}

resource "zitadel_machine_user" "web_service" {
  org_id            = local.org_id
  user_name         = "web-service-user"
  name              = "Web Service"
  description       = "Machine user for web app backend - user management APIs"
  access_token_type = "ACCESS_TOKEN_TYPE_JWT"
}

resource "zitadel_machine_key" "web_service" {
  org_id          = local.org_id
  user_id         = zitadel_machine_user.web_service.id
  key_type        = "KEY_TYPE_JSON"
  expiration_date = timeadd(timestamp(), local.one_year)

  lifecycle {
    ignore_changes = [expiration_date]
  }
}

resource "zitadel_instance_member" "web_service_login_client" {
  user_id = zitadel_machine_user.web_service.id
  roles   = ["IAM_LOGIN_CLIENT"]
}

# ============================================================================
# Profile service (Rust microservice)
# ============================================================================

resource "zitadel_application_api" "profile_service" {
  org_id     = local.org_id
  project_id = zitadel_project.main.id
  name       = "profile-service"
}

resource "zitadel_org_member" "api_user_manager" {
  org_id  = local.org_id
  user_id = zitadel_machine_user.api.id
  roles   = ["ORG_USER_MANAGER", "ORG_END_USER_IMPERSONATOR"]
}

# ============================================================================
# Identity Providers - Social Login (Optional)
# ============================================================================

resource "zitadel_org_idp_oauth" "discord" {
  count = var.discord_client_id != "" && var.discord_client_secret != "" ? 1 : 0

  org_id        = local.org_id
  name          = "Discord"
  client_id     = var.discord_client_id
  client_secret = var.discord_client_secret
  scopes        = ["identify", "email"]

  authorization_endpoint = "https://discord.com/api/oauth2/authorize"
  token_endpoint         = "https://discord.com/api/oauth2/token"
  user_endpoint          = "https://discord.com/api/users/@me"
  id_attribute           = "id"

  is_linking_allowed  = true
  is_creation_allowed = true
  is_auto_creation    = true
  is_auto_update      = true
  auto_linking        = "AUTO_LINKING_OPTION_EMAIL"
}

resource "zitadel_org_idp_github" "github" {
  count = var.github_client_id != "" && var.github_client_secret != "" ? 1 : 0

  org_id        = local.org_id
  name          = "GitHub"
  client_id     = var.github_client_id
  client_secret = var.github_client_secret
  scopes        = ["user:email"]

  is_linking_allowed  = true
  is_creation_allowed = true
  is_auto_creation    = true
  is_auto_update      = true
}

# ============================================================================
# SMTP Configuration - Email Verification
# ============================================================================

resource "zitadel_smtp_config" "default" {
  sender_address = var.zitadel_smtp_sender_address
  sender_name    = var.zitadel_smtp_sender_name
  host           = var.zitadel_smtp_host
  tls            = var.zitadel_smtp_tls
  user           = var.zitadel_smtp_user
  password       = var.zitadel_smtp_password
  set_active     = true
}

resource "zitadel_default_verify_email_message_text" "default" {
  language = "en"

  title       = "Verify Your Email"
  pre_header  = "Please verify your email address for Startup AI"
  subject     = "Verify your email for Startup AI"
  greeting    = "Hello {{.DisplayName}},"
  text        = "We received a request to verify this email address for your Startup AI account. Please click the button below to confirm your email."
  button_text = "Verify Email"
  footer_text = "If you didn't create an account with Startup AI, you can safely ignore this email."
}

resource "zitadel_login_policy" "default" {
  org_id = local.org_id

  user_login               = true
  allow_register           = true
  allow_external_idp       = true
  force_mfa                = false
  force_mfa_local_only     = false
  passwordless_type        = "PASSWORDLESS_TYPE_ALLOWED"
  hide_password_reset      = false
  ignore_unknown_usernames = true
  allow_domain_discovery   = false
  disable_login_with_email = false
  disable_login_with_phone = true

  password_check_lifetime       = "240h0m0s"
  external_login_check_lifetime = "240h0m0s"
  mfa_init_skip_lifetime        = "720h0m0s"
  second_factor_check_lifetime  = "24h0m0s"
  multi_factor_check_lifetime   = "24h0m0s"

  default_redirect_uri = var.zitadel_redirect_uri

  idps = concat(
    var.discord_client_id != "" && var.discord_client_secret != "" ? [zitadel_org_idp_oauth.discord[0].id] : [],
    var.github_client_id != "" && var.github_client_secret != "" ? [zitadel_org_idp_github.github[0].id] : []
  )
}

# ============================================================================
# Zitadel Actions - Tier Update Webhook
# ============================================================================

resource "zitadel_action_target" "tier_update_webhook" {
  name               = "tier-update-webhook"
  endpoint           = "${var.app_internal_url}/api/webhooks/zitadel/user-metadata-changed"
  target_type        = "REST_WEBHOOK"
  timeout            = "10s"
  interrupt_on_error = false
}

resource "zitadel_action_execution_event" "tier_update" {
  event      = "user.metadata.set"
  target_ids = [zitadel_action_target.tier_update_webhook.id]
}
