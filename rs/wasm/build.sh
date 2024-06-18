#!/usr/bin/env bash

which wasm-pack 2>/dev/null || { echo "Please install wasm-pack with 'cargo install wasm-pack'" && exit 1; }

if [[ "$1" == "--release" ]]
then
	RUSTFLAGS="-Cpanic=abort -Ccodegen-units=1 -Cembed-bitcode=yes -Clto=fat -Cstrip=symbols -Copt-level=z" wasm-pack build --target web --no-typescript --release
else
	wasm-pack build --target web --no-typescript --debug
fi

cp -f ./pkg/glicol_wasm{.js,_bg.js,_bg.wasm} ../../js/src/
cp -f ./pkg/glicol_wasm{.js,_bg.js,_bg.wasm} ../../js/npm/
