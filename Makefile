default: build

.PHONY: ebpf
ebpf:
	cargo xtask build-ebpf

.PHONY: signals
signals:
	cargo build $(FLAGS)

.PHONY: ebpf-release
ebpf-release:
	cargo xtask build-ebpf --release

.PHONY: signals-release
signals-release:
	cargo build $(FLAGS) --release

.PHONY: clean
clean:
	rm -rf target

.PHONY: build
build: ebpf signals

.PHONY: release
release: ebpf-release signals-release

.PHONY: image
image:
	podman build -t signals -f kube/Containerfile .
