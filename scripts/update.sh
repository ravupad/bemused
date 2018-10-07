#!/bin/sh
set -e
git fetch
git reset --hard origin/master

echo 'Source code update complete'

cd web
npm i
npm run build
cd ..

echo 'Webpack build complete'

cd backend
/home/ravi/.cargo/bin/cargo build --release
cd ..

echo 'Cargo release build complete'

rm -r deployment/dist
mv web/dist deployment/
mv backend/target/release/rustserver deployment/rustserver

echo 'Replaced all deployment files'

