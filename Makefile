FUNCTION_NAME=discord_chatbot

.PHONY: watch
watch:
	cargo lambda watch

.PHONY: build
build:
	cargo lambda build --release --arm64

.PHONY: deploy
deploy:
	cargo lambda deploy --enable-function-url --env-vars DISCORD_BOT_PUBLIC_KEY=${DISCORD_BOT_PUBLIC_KEY} $(FUNCTION_NAME)

create-command:
	cargo run --bin cli
