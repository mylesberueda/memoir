output "price_id_plus_monthly" {
  value = stripe_price.plus_monthly.id
}

output "price_id_plus_annual" {
  value = stripe_price.plus_annual.id
}

output "price_id_pro_monthly" {
  value = stripe_price.pro_monthly.id
}

output "price_id_pro_annual" {
  value = stripe_price.pro_annual.id
}

output "price_id_enterprise_monthly" {
  value = stripe_price.enterprise_monthly.id
}

output "price_id_enterprise_annual" {
  value = stripe_price.enterprise_annual.id
}

output "webhook_secret" {
  description = "Stripe webhook signing secret (empty for localhost)"
  value       = length(stripe_webhook_endpoint.api) > 0 ? stripe_webhook_endpoint.api[0].secret : ""
  sensitive   = true
}

output "portal_configuration_id" {
  value = stripe_portal_configuration.default.id
}

output "prices" {
  value = {
    plus_monthly       = stripe_price.plus_monthly.id
    plus_annual        = stripe_price.plus_annual.id
    pro_monthly        = stripe_price.pro_monthly.id
    pro_annual         = stripe_price.pro_annual.id
    enterprise_monthly = stripe_price.enterprise_monthly.id
    enterprise_annual  = stripe_price.enterprise_annual.id
  }
}
