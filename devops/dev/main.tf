terraform {
  required_providers {
    docker = {
      source  = "kreuzwerker/docker"
      version = "~> 3.0.1"
    }
  }
}

provider "docker" {}

resource "docker_container" "matchmaker_server" {
  name  = "matchmaker_server"
  image = docker_image.matchmaker_server.image_id

  ports {
    internal = 3000
    external = var.backend_port
  }
}

resource "docker_container" "web_server_dev" {
  name  = "web_server_dev"
  image = docker_image.web_server_dev.image_id

  ports {
    internal = 8080
    external = var.frontend_port
  }

  env = ["SETUID=${var.setuid}", "SETGID=${var.setgid}"]
  volumes {
    container_path = "/app"
    host_path      = "${path.cwd}/../../web-app"
  }

}
