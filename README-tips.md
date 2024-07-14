# private key to solana cli json file
echo '<VOTRE_CLÉ_PRIVÉE>' | solana-keygen recover -o ~/.config/solana/id.json --force
echo '[1,2,3,...]' | solana-keygen recover -o ~/.config/solana/id.json --force

# public key from private key with solana cli format 
solana-keygen pubkey ~/.config/solana/id.json

# balance 
solana balance
solana balance --url https://api.devnet.solana.com

# laucnh local blockchain
solana-test-validator

# check programm id 
solana-keygen pubkey target/deploy/mpl_core_anchor_wrapper-keypair.json
then compare with the lib.rs 's one

anchor keys sync =>  entre le build et le deploy, ça permet d'harmoniser le programId entre l'idl.json et le fichier rust dans program/src/et le fichier anchor.toml 

https://solscan.io/account/BGDWR7vHYwqw43d43XjepHPYBfF2LVNqgLT6DFcYfkM9?cluster=devnet

# anchor command

anchor build
anchor deploy
anchor test

