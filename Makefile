release:
	cargo build --release

docker:
	docker build -t rest_rust:tes .

compose:
	docker compose -p rest up -d 

podman:
	podman build -t rest_rust .

clean:
	@echo "Cleaning up..."
	rm ./target/*

run:
	docker run -d -p 3000:3000 --name rest rest_rust:tes 

rm:
	docker rm --force rest 