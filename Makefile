DEV_CONTRACT=l.herewallet.testnet

build:
	cd contract && cargo build --target wasm32-unknown-unknown --release && cd ../ && \
	cp contract/target/wasm32-unknown-unknown/release/here_linkdrop.wasm ./out/main.wasm

test:
	cd contract && RUST_BACKTRACE=1 cargo test && cd ..


deploy-dev:
	make build && \
	near deploy l.herewallet.testnet

# near delete l.herewallet.testnet herewallet.testnet

deploy-prod:
	make build && \
	NEAR_ENV=mainnet near deploy l.herewallet.near


dev-init:
	near call $(DEV_CONTRACT) new '' --accountId herewallet.testnet  --gas 242794783120800    


init:
	NEAR_ENV=mainnet near call l.herewallet.near new '' --accountId herewallet.near  --gas 242794783120800    
