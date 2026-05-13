# Stripe Module - Billing Configuration
#
# This module configures:
# - Products for each pricing tier
# - Monthly and annual recurring prices
# - Webhook endpoint for subscription events
# - Customer portal configuration

terraform {
  required_providers {
    stripe = {
      source  = "lukasaron/stripe"
      version = "~> 3.4"
    }
  }
}

locals {
  annual_discount_multiplier = 0.8
}

# ============================================================================
# Products
# ============================================================================

resource "stripe_product" "plus" {
  product_id  = var.product_slug_plus
  name        = "Plus Plan"
  description = "5 Agents, 1,000 messages/month, Email support, Advanced analytics, Custom tools"
  active      = true
}

resource "stripe_product" "pro" {
  product_id  = var.product_slug_pro
  name        = "Pro Plan"
  description = "Unlimited Agents, 10,000 messages/month, Priority support, Advanced analytics, Custom tools, API access"
  active      = true
}

resource "stripe_product" "enterprise" {
  product_id  = var.product_slug_enterprise
  name        = "Enterprise Plan"
  description = "Everything in Pro, Unlimited messages, Dedicated support, Custom integrations, SLA guarantee, SSO/SAML"
  active      = true
}

# ============================================================================
# Prices - Plus Tier
# ============================================================================

resource "stripe_price" "plus_monthly" {
  product        = stripe_product.plus.id
  lookup_key     = "${var.product_slug_plus}_monthly"
  currency       = "usd"
  unit_amount    = var.stripe_price_plus_cents
  billing_scheme = "per_unit"

  recurring {
    interval       = "month"
    interval_count = 1
  }
}

resource "stripe_price" "plus_annual" {
  product        = stripe_product.plus.id
  lookup_key     = "${var.product_slug_plus}_annual"
  currency       = "usd"
  unit_amount    = floor(var.stripe_price_plus_cents * 12 * local.annual_discount_multiplier)
  billing_scheme = "per_unit"

  recurring {
    interval       = "year"
    interval_count = 1
  }
}

# ============================================================================
# Prices - Pro Tier
# ============================================================================

resource "stripe_price" "pro_monthly" {
  product        = stripe_product.pro.id
  lookup_key     = "${var.product_slug_pro}_monthly"
  currency       = "usd"
  unit_amount    = var.stripe_price_pro_cents
  billing_scheme = "per_unit"

  recurring {
    interval       = "month"
    interval_count = 1
  }
}

resource "stripe_price" "pro_annual" {
  product        = stripe_product.pro.id
  lookup_key     = "${var.product_slug_pro}_annual"
  currency       = "usd"
  unit_amount    = floor(var.stripe_price_pro_cents * 12 * local.annual_discount_multiplier)
  billing_scheme = "per_unit"

  recurring {
    interval       = "year"
    interval_count = 1
  }
}

# ============================================================================
# Prices - Enterprise Tier
# ============================================================================

resource "stripe_price" "enterprise_monthly" {
  product        = stripe_product.enterprise.id
  lookup_key     = "${var.product_slug_enterprise}_monthly"
  currency       = "usd"
  unit_amount    = var.stripe_price_enterprise_cents
  billing_scheme = "per_unit"

  recurring {
    interval       = "month"
    interval_count = 1
  }
}

resource "stripe_price" "enterprise_annual" {
  product        = stripe_product.enterprise.id
  lookup_key     = "${var.product_slug_enterprise}_annual"
  currency       = "usd"
  unit_amount    = floor(var.stripe_price_enterprise_cents * 12 * local.annual_discount_multiplier)
  billing_scheme = "per_unit"

  recurring {
    interval       = "year"
    interval_count = 1
  }
}

# ============================================================================
# Webhook Endpoint
# ============================================================================
# Only created for non-localhost URLs (staging/production).
# For local development, use Stripe CLI: docker compose up stripe-cli

resource "stripe_webhook_endpoint" "api" {
  count = startswith(var.stripe_webhook_url, "http://localhost") ? 0 : 1

  url = "${var.stripe_webhook_url}/api/webhooks/stripe"

  enabled_events = [
    "checkout.session.completed",
    "customer.subscription.created",
    "customer.subscription.updated",
    "customer.subscription.deleted",
    "invoice.payment_failed",
  ]

  description = "API service webhook endpoint (${var.environment})"
}

# ============================================================================
# Customer Portal Configuration
# ============================================================================

resource "stripe_portal_configuration" "default" {
  business_profile {
    headline = "Manage your Startup AI subscription"
  }

  features {
    customer_update {
      enabled         = true
      allowed_updates = ["email", "address", "phone", "tax_id"]
    }

    invoice_history {
      enabled = true
    }

    payment_method_update {
      enabled = true
    }

    subscription_cancel {
      enabled            = true
      mode               = "at_period_end"
      proration_behavior = "none"
      cancellation_reason {
        enabled = true
        options = ["too_expensive", "missing_features", "switched_service", "unused", "other"]
      }
    }

    subscription_update {
      enabled                 = true
      default_allowed_updates = ["price", "quantity", "promotion_code"]
      proration_behavior      = "always_invoice"

      products {
        product = stripe_product.plus.id
        prices  = [stripe_price.plus_monthly.id, stripe_price.plus_annual.id]
      }

      products {
        product = stripe_product.pro.id
        prices  = [stripe_price.pro_monthly.id, stripe_price.pro_annual.id]
      }

      products {
        product = stripe_product.enterprise.id
        prices  = [stripe_price.enterprise_monthly.id, stripe_price.enterprise_annual.id]
      }
    }
  }

  default_return_url = "${var.app_login_url}/settings/billing"
}
