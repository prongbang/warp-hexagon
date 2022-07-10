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

# Test coverage
# - https://marco-c.github.io/2020/11/24/rust-source-based-code-coverage.html
# - https://github.com/mozilla/grcov
install_cov_requirement:
	cargo install grcov
	# 1. Install the llvm-tools or llvm-tools-preview component:
	rustup component add llvm-tools-preview

test_cov:
	# 2. Ensure that the following environment variable is set up:
	export RUSTFLAGS="-Cinstrument-coverage"
	# 3. Build your code:
	cargo build
	# 4. Ensure each test runs gets its own profile information by defining the LLVM_PROFILE_FILE environment variable
	# (%p will be replaced by the process ID, and %m by the binary signature):
	make generate_coverage

generate_coverage:
	# 5. Run your tests:
	LLVM_PROFILE_FILE="warp-hexagon-%p-%m.profraw" cargo test
	# 6. Generate a coverage report from coverage artifacts
	grcov . -b ./target/debug/ -s . -t lcov --branch --ignore-not-existing -o ./coverage/lcov.info
	grcov . -b ./target/debug/ -s . -t html --branch --ignore-not-existing -o ./coverage/html