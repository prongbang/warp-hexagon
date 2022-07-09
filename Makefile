## Example by
## - https://levelup.gitconnected.com/building-an-api-using-warp-and-tokio-26a52173860a
## - https://blog.logrocket.com/create-an-async-crud-web-service-in-rust-with-warp/
## Source:
## - https://github.com/andrewleverette/rust_warp_api
## - https://github.com/zupzup/warp-postgres-example

run_postgres:
	# database: postgres
	# user: postgres
	# pass: no password
	docker run -p 5432:5432 -d postgres:9.6.12

run:
	cargo run