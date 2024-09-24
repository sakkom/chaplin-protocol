import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ChaplinProtocol } from "../target/types/chaplin_protocol";
import { assert } from "chai";

describe("chaplin-protocol", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ChaplinProtocol as Program<ChaplinProtocol>;
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  it("Pushes history and checks for duplicates", async () => {
    const name = 'roro'

    const [userProfilePda, _] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("user-profile-2"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    // Push collectionMint to history
    await program.methods
      .createUser(name)
      .accounts({
        userProfile: userProfilePda,
        user: provider.wallet.publicKey,
        authority: provider.wallet.publicKey
      })
      .rpc();
    // Fetch the user profile account and check the history
    const userProfileAccount = await program.account.userProfile.fetch(userProfilePda);
    console.log(userProfileAccount);
  });
});
