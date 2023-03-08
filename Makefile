.PHONY: watch
watch:
	cargo lambda watch

.PHONY: build
build:
	cargo lambda build --release --arm64

.PHONY: deploy
deploy:
	cargo lambda deploy --enable-function-url
