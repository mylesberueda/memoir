# Memorystore Outputs

output "instance_name" {
  description = "Redis instance name"
  value       = google_redis_instance.redis.name
}

output "host" {
  description = "Redis host (private IP)"
  value       = google_redis_instance.redis.host
}

output "port" {
  description = "Redis port"
  value       = google_redis_instance.redis.port
}

output "connection_string" {
  description = "Redis connection URL"
  value       = "redis://${google_redis_instance.redis.host}:${google_redis_instance.redis.port}"
}

output "current_location_id" {
  description = "Current zone where Redis is located"
  value       = google_redis_instance.redis.current_location_id
}
