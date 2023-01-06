.PHONY: all, linux, windows

all: linux, windows

linux:
	cargo zigbuild --target x86_64-unknown-linux-gnu --release
	strip target/x86_64-unknown-linux-gnu/release/dailyworker

windows:
	cargo build --target x86_64-pc-windows-gnu --release
	strip target/x86_64-pc-windows-gnu/release/dailyworker.exe