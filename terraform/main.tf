terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.16"
    }
  }
}

provider "aws" {
  region  = var.aws_region
  profile = "default"
}

data "aws_iam_user" "current" {
  user_name = "rust-api-user"
}

resource "aws_ecr_repository" "app_ecr_repo" {
  name = "rust-api-repo"
}

resource "aws_ecs_cluster" "app_ecs_cluster" {
  name = "rust-api-cluster"
}

resource "aws_ecs_task_definition" "app_task_def" {
  family                   = "rust-api-task"
  container_definitions    = <<DEFINITION
  [
    {
      "name": "rust-api-task",
      "image": "${aws_ecr_repository.app_ecr_repo.repository_url}",
      "essential": true,
      "portMappings": [
        {
          "containerPort": 8080,
          "hostPort": 8080
        }
      ],
      "memory": 512,
      "cpu": 256
    }
  ]
  DEFINITION
  requires_compatibilities = ["FARGATE"]
  network_mode             = "awsvpc"
  memory                   = 512
  cpu                      = 256
  execution_role_arn       = data.aws_iam_user.current.arn
}
