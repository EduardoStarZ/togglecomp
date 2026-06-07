LOCAL_DIR = ~/.local/bin

build:
	cargo build --release
	cp target/release/togglecomp $(LOCAL_DIR)
