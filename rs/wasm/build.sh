#!/usr/bin/env bash

which wasm-pack 2>/dev/null || { echo "Please install wasm-pack with 'cargo install wasm-pack'" && exit 1; }

if [[ "$1" == "--release" ]]
then
	RUSTFLAGS="-Cpanic=abort -Ccodegen-units=1 -Cembed-bitcode=yes -Clto=fat -Cstrip=symbols -Copt-level=z" wasm-pack build --target web --no-typescript --no-pack --release
else
	wasm-pack build --target web --no-typescript --no-pack --debug
fi

which wasm-opt >/dev/null 2>&1 && wasm-opt -Oz ./pkg/glicol_wasm_bg.wasm -o ./pkg/glicol_wasm_bg.wasm

cp -f ./pkg/glicol_wasm{.js,_bg.wasm} ../../js/src/
cp -f ./pkg/glicol_wasm{.js,_bg.wasm} ../../js/npm/
