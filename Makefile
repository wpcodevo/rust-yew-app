install:
	cargo add yew --features csr
	cargo add serde --features derive
	cargo add uuid --features "serde v4"
	cargo add gloo
	cargo add yewdux
	cargo add wasm-bindgen
	cargo add web-sys --features "HtmlInputElement Window"

start-app:
	trunk serve --port 3000