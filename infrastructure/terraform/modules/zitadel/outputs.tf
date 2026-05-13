output "zitadel_url" {
  description = "Zitadel instance URL"
  value       = "http://${var.zitadel_domain}:${var.zitadel_port}"
}

output "project_id" {
  description = "Zitadel project ID"
  value       = zitadel_project.main.id
}

output "org_id" {
  description = "Zitadel organization ID"
  value       = local.org_id
}

# API Service
output "api_user_id" {
  value = zitadel_machine_user.api.id
}

output "api_key_id" {
  value     = zitadel_machine_key.api.id
  sensitive = true
}

output "api_private_key_base64" {
  value     = zitadel_machine_key.api.key_details
  sensitive = true
}

output "api_client_id" {
  value     = zitadel_application_api.api.client_id
  sensitive = true
}

# Agent API Service
output "agent_api_user_id" {
  value = zitadel_machine_user.agent_api.id
}

output "agent_api_key_id" {
  value     = zitadel_machine_key.agent_api.id
  sensitive = true
}

output "agent_api_private_key_base64" {
  value     = zitadel_machine_key.agent_api.key_details
  sensitive = true
}

output "agent_api_client_id" {
  value     = zitadel_application_api.agent_api.client_id
  sensitive = true
}

# API Service (gRPC)
output "api_service_user_id" {
  value = zitadel_machine_user.api_service.id
}

output "api_service_key_id" {
  value     = zitadel_machine_key.api_service.id
  sensitive = true
}

output "api_service_private_key_base64" {
  value     = zitadel_machine_key.api_service.key_details
  sensitive = true
}

output "api_service_client_id" {
  value     = zitadel_application_api.api_service.client_id
  sensitive = true
}

# Rig Service
output "rig_service_user_id" {
  value = zitadel_machine_user.rig_service.id
}

output "rig_service_key_id" {
  value     = zitadel_machine_key.rig_service.id
  sensitive = true
}

output "rig_service_private_key_base64" {
  value     = zitadel_machine_key.rig_service.key_details
  sensitive = true
}

output "rig_service_client_id" {
  value     = zitadel_application_api.rig_service.client_id
  sensitive = true
}

# Chat Service
output "chat_service_user_id" {
  value = zitadel_machine_user.chat_service.id
}

output "chat_service_key_id" {
  value     = zitadel_machine_key.chat_service.id
  sensitive = true
}

output "chat_service_private_key_base64" {
  value     = zitadel_machine_key.chat_service.key_details
  sensitive = true
}

output "chat_service_client_id" {
  value     = zitadel_application_api.chat_service.client_id
  sensitive = true
}

# Notification Service
output "notification_service_user_id" {
  value = zitadel_machine_user.notification_service.id
}

output "notification_service_key_id" {
  value     = zitadel_machine_key.notification_service.id
  sensitive = true
}

output "notification_service_private_key_base64" {
  value     = zitadel_machine_key.notification_service.key_details
  sensitive = true
}

output "notification_service_client_id" {
  value     = zitadel_application_api.notification_service.client_id
  sensitive = true
}

# CLI
output "cli_user_id" {
  value = zitadel_machine_user.cli.id
}

output "cli_pat" {
  value     = zitadel_personal_access_token.cli.token
  sensitive = true
}

# Web Application
output "web_client_id" {
  value     = zitadel_application_oidc.web.client_id
  sensitive = true
}

output "web_key_details" {
  value     = zitadel_application_key.web.key_details
  sensitive = true
}

output "web_service_user_id" {
  value = zitadel_machine_user.web_service.id
}

output "web_service_key_id" {
  value     = zitadel_machine_key.web_service.id
  sensitive = true
}

output "web_service_key_details" {
  value     = zitadel_machine_key.web_service.key_details
  sensitive = true
}

output "zitadel_webhook_signing_key" {
  value     = zitadel_action_target.tier_update_webhook.signing_key
  sensitive = true
}

# Profile service (for audience validation)
output "profile_service_client_id" {
  value     = zitadel_application_api.profile_service.client_id
  sensitive = true
}

# Roles
output "roles" {
  value = {
    user      = "user"
    admin     = "admin"
    developer = "developer"
  }
}
