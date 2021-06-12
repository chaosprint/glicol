cargo build --target wasm32-unknown-unknown --release
xcopy /Y .\target\wasm32-unknown-unknown\release\glicol_wasm.wasm ..\..\js\src\