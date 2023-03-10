FUNCTION_NAME=discord_chatbot

.PHONY: watch
watch:
	cargo lambda watch

.PHONY: build
build:
	sam build

.PHONY: deploy
deploy:
	sam deploy --stack-name DiscordChatGPTBot --resolve-s3 --capabilities CAPABILITY_IAM

.PHONY: clean-deploy
clean-deploy: build deploy


.PHONY: build-discord_chatbot
build-discord_chatbot:
	cargo lambda build --release --arm64 --bin discord_chatbot

.PHONY: build-command_stream
build-command_stream:
	cargo lambda build --release --arm64 --bin command_stream

.PHONY: build-cli
build-cli:
	cargo build --release --bin cli

create-command:
	cargo run --bin cli

.PHONY: deploy-DiscordWebhookReceiverFunction
build-DiscordWebhookReceiverFunction: build-discord_chatbot
	cp ./target/lambda/discord_chatbot/bootstrap $(ARTIFACTS_DIR)

.PHONY: deploy-DircordCommandStreamFunction
build-DircordCommandStreamFunction: build-command_stream
	cp ./target/lambda/command_stream/bootstrap $(ARTIFACTS_DIR)
