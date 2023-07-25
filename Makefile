push:
	docker push kekss1k/rest_rust

docker:
	docker build -t kekss1k/rest_rust .

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