push: clean build push-web push-backend

push-web: clean-web build-web
	scp -r web/dist ravi@13.71.112.248:./
	ssh ravi@13.71.112.248 "cd /srv/bemused/deployment && sudo rm -rf dist && sudo mv ~/dist ./"

push-backend: build-backend
	scp backend/target/release/rustserver ravi@13.71.112.248:./
	ssh ravi@13.71.112.248 "cd /srv/bemused/deployment && sudo rm rustserver && sudo mv ~/rustserver ./ && sudo systemctl restart bemused.service"

build: build-backend build-web

build-backend:
	cd backend && cargo build --release

build-web:
	cd web && npm run build

clean: clean-web

clean-web:
	rm -rf web/dist

.PHONY: build-backend build-web build clean-web clean push-web push-backend push
