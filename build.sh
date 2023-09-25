aws ecr get-login-password --region eu-north-1 | podman login --username AWS --password-stdin 861821644425.dkr.ecr.eu-north-1.amazonaws.com
podman build . -t 861821644425.dkr.ecr.eu-north-1.amazonaws.com/eliittilukio-frontend
podman push 861821644425.dkr.ecr.eu-north-1.amazonaws.com/eliittilukio-frontend

# on server, docker compose pull