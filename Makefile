js:
	cargo run -- js ./examples/js/$(file).lag ./examples/js/$(file).js && node ./examples/js/$(file).js

vm:
	cargo run -- run ./examples/vm/$(file).lag --vm

run:
	cargo run -- run $(file)