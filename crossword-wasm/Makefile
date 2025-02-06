test:
	cargo test
run:
	cat data/some_words.txt | cargo run
wasm:
	wasm-pack build --target web
server:
	echo http://localhost:8000
	python3 -m http.server
copy_to_app:
	rm -rf ../crossword-app/src/crossword_wasm 
	cp -r pkg/ ../crossword-app/src/crossword_wasm 
all: test run wasm copy_to_app
