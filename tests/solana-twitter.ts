import * as anchor from "@project-serum/anchor";
import {Program} from "@project-serum/anchor";
import {SolanaTwitter} from "../target/types/solana_twitter";
import * as console from "console";
import * as assert from "assert";

describe("solana-twitter", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.SolanaTwitter as Program<SolanaTwitter>;

    it("Can send a new Tweet!", async () => {
        // Add your test here.
        const tweet = anchor.web3.Keypair.generate();
        const tx = await program.methods.sendTweet("veganism", "Humans, am I right?")
            .accounts({
                tweet: tweet.publicKey,
                author: program.provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            }).signers([tweet])
            .rpc();

        const tweetAccount = await program.account.tweet.fetch(tweet.publicKey);
        console.log("Tweet Account: ", tweetAccount);
        assert.equal(tweetAccount.author.toBase58(), program.provider.wallet.publicKey.toBase58());
        assert.equal(tweetAccount.topic, 'veganism');
        assert.equal(tweetAccount.content, "Humans, am I right?");
        assert.ok(tweetAccount.timestamp);
        console.log("Your transaction signature", tx);
    });
});
