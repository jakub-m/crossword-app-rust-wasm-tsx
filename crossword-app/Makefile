serve:
	npm start
build:
	rm -rf dist
	npm run webpack-build
	find dist/
serve-build: build
	(cd dist; python3 -m http.server)
