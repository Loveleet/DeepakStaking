
process.env.ANCHOR_PROVIDER_URL = 'https://api.devnet.solana.com';
process.env.ANCHOR_WALLET = '/Users/lk/Downloads/deepakClient/test-new-loveleet.json';


const anchor = require("@project-serum/anchor");
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const args = require('minimist')(process.argv.slice(2));

async function main() {
  const idl = JSON.parse(require("fs").readFileSync("./idl.json", "utf8"));

  const programId = new anchor.web3.PublicKey("kScf9gaYZfjVStDAL6V6tfhLWR29UKi2mK7aojJA8Xp");
  const owner = new anchor.web3.PublicKey("6JxLdTweYt6cb6UeyCiqkPCVTo4RBxswfoQwHWe5aHzY");

  const program = new anchor.Program(idl, programId);

  const stakingAccount = anchor.web3.Keypair.generate();

  console.log("Staking Account PublicKey:", stakingAccount.publicKey.toString());

  try {
    let tx = await program.rpc.initialize(owner, {
      accounts: {
        stakingAccount: stakingAccount.publicKey,
        tokenMint: new anchor.web3.PublicKey("Azia2MRh34sWejk7ZpPxvXS2tQdcoq2xZja6RN3Q26o3"),
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: new anchor.web3.PublicKey("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb")
      },
      signers: [stakingAccount],
      options: { commitment: "confirmed" },
    });

    console.log("Fetching transaction logs...");
    let t = await provider.connection.getConfirmedTransaction(tx, "confirmed");
    console.log(t.meta.logMessages);
  } catch (error) {
    console.error("Transaction failed:", error);
  }
}

console.log("Running client...");
main().then(() => console.log("Success")).catch(err => console.error("Error:", err));
