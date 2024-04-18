source .env

export BONSAI_API_KEY=$BONSAI_API_KEY
export BONSAI_API_URL=$BONSAI_API_URL
export ETH_WALLET_PRIVATE_KEY=$ETH_WALLET_PRIVATE_KEY
export SEPOLIA_ETHERSCAN_API_KEY=$SEPOLIA_ETHERSCAN_API_KEY

# Deploy + Verify to Sepolia
# forge script --rpc-url $SEPOLIA_RPC_URL --broadcast --etherscan-api-key $SEPOLIA_ETHERSCAN_API_KEY --verify script/Deploy.s.sol

# Deploy + Verify + Verifyto Arbitrum One
# forge script --rpc-url $ARBITRUM_RPC_URL --broadcast -etherscan-api-key <YOUR_ETHERSCAN_API_KEY> script/Deploy.s.sol

# Deploy + Verify to Morph
# forge script --rpc-url $MORPH_RPC_URL --broadcast -etherscan-api-key <YOUR_ETHERSCAN_API_KEY> script/Deploy.s.sol

# Deploy + Verify to OP Avail Sepolia
# forge script --rpc-url $AVAIL_RPC_URL --broadcast -etherscan-api-key <YOUR_ETHERSCAN_API_KEY> script/Deploy.s.sol


# Verify

# Interact with Sepolia
cargo run -- --chain-id 11155111 \
--eth-wallet-private-key $ETH_WALLET_PRIVATE_KEY \
--rpc-url $SEPOLIA_RPC_URL \
--contract 0x690b559B4E6d51a76fd252b292dF41E9A8be142E 