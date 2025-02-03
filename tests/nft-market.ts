import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftMarketplace } from "../target/types/nft_marketplace";

describe("NFT Marketplace Tests", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.NftMarketplace as Program<NftMarketplace>;

  it("Mints an NFT", async () => {
    const tx = await program.methods.mintNft("NFT Title", "NFT", "https://example.com/nft.json")
      .rpc();
    console.log("Mint NFT Transaction Signature:", tx);
  });

  it("Transfers an NFT", async () => {
    const tx = await program.methods.transferNft()
      .rpc();
    console.log("Transfer NFT Transaction Signature:", tx);
  });
});
