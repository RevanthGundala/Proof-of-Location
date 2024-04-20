source .env

export BONSAI_API_KEY=$BONSAI_API_KEY
export BONSAI_API_URL=$BONSAI_API_URL
export ETH_WALLET_PRIVATE_KEY=$ETH_WALLET_PRIVATE_KEY
export SEPOLIA_ETHERSCAN_API_KEY=$SEPOLIA_ETHERSCAN_API_KEY
export ARBITRUM_ETHERSCAN_API_KEY=$ARBITRUM_ETHERSCAN_API_KEY

# Deploy + Verify to Sepolia
# forge script --rpc-url $SEPOLIA_RPC_URL --broadcast --etherscan-api-key $SEPOLIA_ETHERSCAN_API_KEY --verify script/Deploy.s.sol

# Deploy + Verify + Verifyto Arbitrum Sepolia
# echo "Deploying to Arbitrum Sepolia"
# forge script --rpc-url $ARBITRUM_RPC_URL --broadcast --etherscan-api-key $ARBITRUM_ETHERSCAN_API_KEY --verify script/Deploy.s.sol

# Deploy + Verify to Morph
# echo "Deploying to Morph"
# forge script --rpc-url $MORPH_RPC_URL --broadcast script/Deploy.s.sol

# Deploy + Verify to OP Avail Sepolia
#echo "Deploying to OP Avail Sepolia"
#forge script --rpc-url $AVAIL_RPC_URL --broadcast script/Deploy.s.sol
#forge verify-contract --verifier blockscout --verifier-url https://op-avail-sepolia-explorer.alt.technology//api --chain-id 20240219 0xf54c37b9f7fBd9180c6Dc79992839fd2B030b5B1 contracts/Verifier.sol:Verifier



# Sepolia
cargo run -- --chain-id 11155111 \
--eth-wallet-private-key $ETH_WALLET_PRIVATE_KEY \
--rpc-url $SEPOLIA_RPC_URL \
--contract 0xA21ad661f0511504BD7B3Aa71656383611D56ec0 

# Interact with Morph
# cargo run -- --chain-id 2710 \
# --eth-wallet-private-key $ETH_WALLET_PRIVATE_KEY \
# --rpc-url $MORPH_RPC_URL \
# --contract 0xA21ad661f0511504BD7B3Aa71656383611D56ec0 