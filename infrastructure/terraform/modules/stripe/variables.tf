variable "environment" {
  description = "Environment name (local, staging, production)"
  type        = string
}

variable "product_slug_plus" {
  description = "Stable identifier for the Plus product. Used as Stripe product_id and as the prefix for monthly/annual lookup_key. Survives terraform state loss."
  type        = string
  default     = "startup_ai_plus"
}

variable "product_slug_pro" {
  description = "Stable identifier for the Pro product. Used as Stripe product_id and as the prefix for monthly/annual lookup_key. Survives terraform state loss."
  type        = string
  default     = "startup_ai_pro"
}

variable "product_slug_enterprise" {
  description = "Stable identifier for the Enterprise product. Used as Stripe product_id and as the prefix for monthly/annual lookup_key. Survives terraform state loss."
  type        = string
  default     = "startup_ai_enterprise"
}

variable "stripe_price_plus_cents" {
  description = "Plus tier monthly price in cents"
  type        = number
  default     = 2000
}

variable "stripe_price_pro_cents" {
  description = "Pro tier monthly price in cents"
  type        = number
  default     = 10000
}

variable "stripe_price_enterprise_cents" {
  description = "Enterprise tier monthly price in cents"
  type        = number
  default     = 50000
}

variable "stripe_webhook_url" {
  description = "Base URL for Stripe webhooks"
  type        = string
  default     = "http://localhost:5154"
}

variable "app_login_url" {
  description = "Base URL for app (used in portal return URL)"
  type        = string
  default     = "http://localhost:3000"
}
