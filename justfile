
project := "service-uptime"

alias t := test
alias pre := test-all

# run the standard tests
test:
    clear
    cargo test

# run the standard tests + clippy and fmt
test-all:
    clear
    cargo test && cargo fmt && cargo clippy

# build the debug target
build:
    clear
    cargo build

# clean the project
clean:
    cargo clean

# build the docs
docs:
    cargo doc --no-deps --open

# cover - runs code test coverage report and writes to coverage folder
cover:
  cargo tarpaulin --out html --output-dir coverage && mv coverage/tarpaulin-report.html coverage/index.html

# start a http server in the coverage folder
serve-cover:
  cd coverage && index.html && python3 -m http.server 8080

# merge the develop branch to main
merge:
    git push && git checkout main && git pull && git merge develop && git push && git checkout develop

