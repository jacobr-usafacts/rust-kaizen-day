
all:
	cargo build --release
	kind create cluster --name=rust-api-cluster
	docker build -t rust-api:latest .
	kind load docker-image rust-api:latest --name=rust-api-cluster
	kubectl apply -f k8s/deployment.yaml

clean:
	kind delete cluster --name=rust-api-cluster