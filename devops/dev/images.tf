resource "docker_image" "matchmaker_server" {
  name = "matchmaker_server"

  build {
    context    = "../.."
    dockerfile = "devops/units/matchmaker_server.dockerfile"
    label      = { project : "traffic-game" }
  }


  triggers = {
    time = timestamp()
    #version_sha1 = filesha1("../../version.txt")
  }
}

resource "docker_image" "web_server_dev" {
  name = "web_server_dev"

  build {
    context    = "../.."
    dockerfile = "devops/units/web_server_dev.dockerfile"
    label      = { project : "traffic-game" }
  }

  triggers = {
    time = timestamp()
    #version_sha1 = filesha1("../../version.txt")
  }
}
