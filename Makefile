js:
	cargo run -- js ./examples/js/$(file).lag ./examples/js/$(file).js && node ./examples/js/$(file).js

run:
	cargo run -- run $(file)