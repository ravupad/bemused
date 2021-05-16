deploy: setup stop update-web update-server start clean

setup:
	sudo apt-get install nginx
	sudo mv nginx.conf /etc/nginx/sites-available/assemblyheaven
	sudo rm /etc/nginx/sites-enabled/assemblyheaven
	sudo ln -s /etc/nginx/sites-available/assemblyheaven /etc/nginx/sites-enabled/assemblyheaven
	sudo mv systemd.service /lib/systemd/system/bemused.service
	sudo systemctl daemon-reload
	sudo mkdir -p /srv/bemused/deployment

stop:
	sudo systemctl stop bemused.service
	sudo systemctl stop nginx.service

update-web:
	sudo rm -rf /srv/bemused/deployment/dist
	sudo mv web-dist /srv/bemused/deployment/dist

update-server:
	sudo rm -rf /srv/bemused/deployment/rustserver
	sudo mv bemused-server /srv/bemused/deployment/rustserver
	sudo mv server.toml /srv/bemused/deployment/Config.toml

start:
	sudo systemctl start bemused.service
	sudo systemctl start nginx.service

clean:
	rm -rf ../temp
	rm -rf ../package.tar

.PHONY: setup stop update-web update-server start clean