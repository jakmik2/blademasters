#!/usr/bin/env bash

PROJECT_NAME="combative-survivors"

# Build
RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --target wasm32-unknown-unknown --release --no-default-features

# Generate bindgen outputs
mkdir -p out
mkdir -p out/assets

cp -r assets/* out/assets

wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
   --out-name "$PROJECT_NAME" \
   ./target/wasm32-unknown-unknown/release/$PROJECT_NAME.wasm

cat > ./out/index.html <<- EOM
<!doctype html>
<html lang="en">
    <body style="margin: 0px">
        <script type="module">
            import init from "./combative-survivors.js";

            init().catch((error) => {
                if (
                    !error.message.startsWith(
                        "Using exceptions for control flow, don't mind me. This isn't actually an error!",
                    )
                ) {
                    throw error;
                }
            });
        </script>
    </body>
</html>
EOM
