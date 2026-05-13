output "vpc_id" {
  description = "VPC network ID"
  value       = google_compute_network.vpc.id
}

output "vpc_name" {
  description = "VPC network name"
  value       = google_compute_network.vpc.name
}

output "vpc_self_link" {
  description = "VPC network self link"
  value       = google_compute_network.vpc.self_link
}

output "subnet_id" {
  description = "Subnet ID"
  value       = google_compute_subnetwork.subnet.id
}

output "subnet_name" {
  description = "Subnet name"
  value       = google_compute_subnetwork.subnet.name
}

output "subnet_self_link" {
  description = "Subnet self link"
  value       = google_compute_subnetwork.subnet.self_link
}

output "pods_cidr_name" {
  description = "Name of the secondary range for pods"
  value       = "${var.environment}-pods"
}

output "services_cidr_name" {
  description = "Name of the secondary range for services"
  value       = "${var.environment}-services"
}

output "router_name" {
  description = "Cloud Router name"
  value       = google_compute_router.router.name
}
