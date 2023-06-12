# Setup
Install Terraform [here](https://developer.hashicorp.com/terraform/downloads?ajs_aid=f5451dc5-9b26-4866-a59a-67701043f523&product_intent=terraform)

Ensure you have a docker daemon running.

Enter the "dev" environment folder and run `terraform init`

# Configuration
Checkout the `dev.tfvars` for parameters to change

# Run
Inside the dev environment, run `terraform apply -var-file dev.tfvars`. Rerun the command to recreate the environment.

# Stop
Inside the dev environment, run `terraform destroy -var-file dev.tfvars`
