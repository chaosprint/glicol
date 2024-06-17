#!/usr/bin/env bash

which wasm-pack 2>/dev/null || { echo "Please install wasm-pack with 'cargo install wasm-pack'" && exit 1; }

if [[ "$1" == "--release" ]]
then
	wasm-pack build --target web --release
else
	wasm-pack build --target web --debug
fi

cp -f ./pkg/glicol_wasm{.js,_bg.js,_bg.wasm} ../../js/src/
cp -f ./pkg/glicol_wasm{.js,_bg.js,_bg.wasm} ../../js/npm/
