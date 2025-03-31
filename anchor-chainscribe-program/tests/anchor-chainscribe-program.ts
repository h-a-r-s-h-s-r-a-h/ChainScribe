import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorChainscribeProgram } from "../target/types/anchor_chainscribe_program";
import { expect } from "chai";

describe("anchor-chainscribe-program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .anchorChainscribeProgram as Program<AnchorChainscribeProgram>;

  const topic = {
    topic_id: "1000",
    topic_generator_name: "Harsh",
    topic_title: "BlockChain",
    topic_description: "About Blockchain",
  };

  const [topicPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("topic"),
      Buffer.from(topic.topic_id),
      provider.wallet.publicKey.toBuffer(),
    ],
    program.programId
  );

  it("Add a topic!", async () => {
    try {
      const beforeTimestamp = Math.floor(Date.now() / 1000);
      await program.methods
        .createTopic(
          topic.topic_id,
          topic.topic_generator_name,
          topic.topic_title,
          topic.topic_description
        )
        .accounts({})
        .rpc();

      const account = await program.account.topicAccountState.fetch(topicPda);
      expect(account.topicGeneratorId.toString()).to.equal(
        provider.wallet.publicKey.toString()
      );
      expect(account.topicId).to.equal(topic.topic_id);
      expect(account.topicGeneratorName).to.equal(topic.topic_generator_name);
      expect(account.topicTitle).to.equal(topic.topic_title);
      expect(account.topicDescription).to.equal(topic.topic_description);
      expect(account.noOfBlog).to.equal(0);
      expect(account.likes).to.equal(0);
      expect(account.comments).to.equal(0);
      expect(account.isActive).to.equal(true);
      expect(account.isPublic).to.equal(true);

      const afterTimestamp = Math.floor(Date.now() / 1000);
      expect(account.lastUpdatedAt.toNumber()).to.be.within(
        beforeTimestamp-1,
        afterTimestamp
      );
    } catch (error) {
      console.error("Error adding election:", error);
      throw error;
    }
  });
});
