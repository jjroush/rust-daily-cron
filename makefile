compile:
	cargo build --release --target x86_64-unknown-linux-musl

zip:
    zip -r9 -j bootstrap.zip ./target/x86_64-unknown-linux-musl/release/bootstrap

run:
	cd ./target/x86_64-unknown-linux-musl/release && \
	docker run -e NOTION_SECRET=$${NOTION_SECRET} -e AWS_ACCESS_KEY_ID= -e AWS_SECRET_ACCESS_KEY= --rm -v "$$PWD":/var/task:ro,delegated lambci/lambda:provided target/x86_64-unknown-linux-musl/release/bootstrap '{"firstName": "Jacob"}'

compile-and-run: compile run