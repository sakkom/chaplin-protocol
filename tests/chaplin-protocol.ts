import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ChaplinProtocol } from "../target/types/chaplin_protocol";
import { assert } from "chai";

describe("chaplin-protocol", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ChaplinProtocol as Program<ChaplinProtocol>;
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  it("Creates a film with specified actor", async () => {
    const collectionMint = anchor.web3.Keypair.generate().publicKey;
    const label = anchor.web3.Keypair.generate().publicKey;

    const actor = {
      creator: [anchor.web3.Keypair.generate().publicKey, anchor.web3.Keypair.generate().publicKey],
      coCreator: [
        anchor.web3.Keypair.generate().publicKey,
        anchor.web3.Keypair.generate().publicKey,
        anchor.web3.Keypair.generate().publicKey,
      ],
    };

    const [film] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("film"), collectionMint.toBuffer()],
      program.programId
    );

    await program.methods
      .createFilm(collectionMint, label, actor)
      .accounts({
        user: provider.wallet.publicKey,
        collectionMint: collectionMint,
        film: film,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const filmAccount = await program.account.film.fetch(film);
    console.log(filmAccount);
    // assert.equal(filmAccount.collectionMint.toString(), collectionMint.toString());
    // assert.equal(filmAccount.label.toString(), label.toString());
    // assert.deepEqual(filmAccount.actor.creatro.map((key: anchor.web3.PublicKey) => key.toString()), actor.creatro.map((key: anchor.web3.PublicKey) => key.toString()));
    // assert.deepEqual(filmAccount.actor.co_creator.map((key: anchor.web3.PublicKey) => key.toString()), actor.co_creator.map((key: anchor.web3.PublicKey) => key.toString()));
  });
});
