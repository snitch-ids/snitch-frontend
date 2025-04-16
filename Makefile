run_dev:
	echo "go to http://localhost:8080 (not IP 127.0.0.1) as cookies will not work"
	SNITCH_BACKEND_URL=http://localhost:8081 trunk serve

docker_build:
	trunk clean
	SNITCH_BACKEND_URL=https://api.snitch.cool trunk build --release
	docker build --build-arg TARGETPLATFORM=linux/x86_64 -t emrius11/snitch-frontend:main --no-cache .

docker_push:
	docker push emrius11/snitch-frontend:main
