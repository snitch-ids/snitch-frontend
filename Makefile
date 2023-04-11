run_dev:
	echo "go to http://localhost:8080 (not IP 127.0.0.1) as cookies will not work"
	SNITCH_BACKEND_URL=http://localhost:8081 trunk serve

docker_build:
	docker build --build-arg TARGETPLATFORM=linux/arm64 -t emrius11/snitch-frontend:main .

docker_push:
	docker push emrius11/snitch-frontend:main
