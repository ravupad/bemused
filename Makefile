push: package
	scp package.tar ravi@13.71.112.248:./
	rm package.tar
	ssh ravi@13.71.112.248 "tar -xf package.tar && cd temp && make deploy"

package: build
	rm -rf temp && mkdir temp
	mv web/dist temp/web-dist
	mv server/target/release/bemused-server temp/
	cp -r conf/* temp/
	tar -cf package.tar temp
	rm -rf temp

build:
	cd server && cargo build --release
	cd web && npm run build

# With phony target name is not confused with file name
# e.g. if a file with name build exists, then target with
# name build will not be associated with that file.
.PHONY: build push package
