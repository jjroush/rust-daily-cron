compile:
	cargo build --release --target x86_64-unknown-linux-musl

run:
	cd ./target/x86_64-unknown-linux-musl/release && \
	pwd && ls && \
	docker run -e NOTION_SECRET=$${NOTION_SECRET} --rm -v "$$PWD":/var/task:ro,delegated lambci/lambda:provided target/x86_64-unknown-linux-musl/release/bootstrap '{"firstName": "Jacob"}'

compile-and-run: compile run