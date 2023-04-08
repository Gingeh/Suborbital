default: run-web

build-web:
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --no-typescript --out-name bevy_app --out-dir web/build --target web target/wasm32-unknown-unknown/release/suborbital.wasm
    cp -r assets web/
    cd web && zip ../web.zip -r .

run-web: build-web
    sfz -r ./web -p 5000
