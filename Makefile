http-server:
	RUST_ENV=local APP_NAME=http-server RUST_BACKTRACE=full cargo run --bin http-server

grpc:
	RUST_ENV=local APP_NAME=grpc RUST_BACKTRACE=full cargo run --bin grpc

