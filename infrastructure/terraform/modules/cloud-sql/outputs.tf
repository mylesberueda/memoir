output "instance_name" {
  description = "Cloud SQL instance name"
  value       = google_sql_database_instance.postgres.name
}

output "instance_connection_name" {
  description = "Cloud SQL connection name for Cloud SQL Proxy"
  value       = google_sql_database_instance.postgres.connection_name
}

output "private_ip" {
  description = "Private IP address of the Cloud SQL instance"
  value       = google_sql_database_instance.postgres.private_ip_address
}

output "databases" {
  description = "Map of database names"
  value       = { for db in google_sql_database.databases : db.name => db.name }
}

output "users" {
  description = "Map of database users with their credentials"
  value = {
    for name, user in google_sql_user.users : name => {
      name     = user.name
      password = random_password.db_passwords[name].result
    }
  }
  sensitive = true
}

output "connection_strings" {
  description = "PostgreSQL connection strings for each database"
  value = {
    for name in var.databases : name => "postgres://${name}:${random_password.db_passwords[name].result}@${google_sql_database_instance.postgres.private_ip_address}:5432/${name}"
  }
  sensitive = true
}
