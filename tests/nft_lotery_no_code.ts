import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DataState, MPL_CORE_PROGRAM_ID } from "@metaplex-foundation/mpl-core";
import { NftLoteryNoCode } from "../target/types/nft_lotery_no_code";


describe("nft_lotery_no_code", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NftLoteryNoCode as Program<NftLoteryNoCode>;

  it("Jacpot Vault Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initJackpotVault().rpc();
    console.log("Your transaction signature", tx);
  });


  it("Can create an Asset", async () => {
    const asset = anchor.web3.Keypair.generate();
    // Add your test here.
    const tx = await program.methods.createNftV1({
      name: "Hello Anchor!",
      uri: "www.example.com",
      plugins: null,
    })
      .accounts({
        asset: asset.publicKey,
        collection: null,
        signer: anchor.getProvider().publicKey,
        owner: null,
        updateAuthority: null,
        logWrapper: null,
      })
      .signers([asset])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Can create a Collection", async () => {
    const collection = anchor.web3.Keypair.generate();
    // Add your test here.
    const tx = await program.methods.createCollectionV1({
      name: "Hello Anchor!",
      uri: "www.example.com",
      plugins: []
    })
      .accounts({
        collection: collection.publicKey,
        payer: anchor.getProvider().publicKey,
        updateAuthority: null,
      })
      .signers([collection])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Can transfer an Asset", async () => {
    const asset = anchor.web3.Keypair.generate();
    // Add your test here.
    await program.methods.createNftV1({
      name: "Hello Anchor!",
      uri: "www.example.com",
      plugins: null,
    })
      .accounts({
        asset: asset.publicKey,
        collection: null,
        signer: anchor.getProvider().publicKey,
        owner: null,
        updateAuthority: null,
        logWrapper: null,
      })
      .signers([asset])
      .rpc();

    const tx = await program.methods.transferV1({})
      .accounts({
        asset: asset.publicKey,
        collection: null,
        payer: anchor.getProvider().publicKey,
        newOwner: anchor.web3.Keypair.generate().publicKey,
        systemProgram: null,
        logWrapper: null,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });

});
