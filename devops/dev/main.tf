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
    external = "${var.backend_port}"
  }
}

resource "docker_container" "web_server" {
  name  = "web_server"
  image = docker_image.web_server.image_id

  ports {
    internal = 80
    external = "${var.frontend_port}"
  }

}
