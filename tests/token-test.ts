import * as anchor from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import { SystemProgram, SYSVAR_INSTRUCTIONS_PUBKEY } from "@solana/web3.js";
import { BN } from "bn.js";
import { program } from "./config";
import { METADATA_PROGRAM_ID, nftMint } from "./constants";
import { findMetadataAddress } from "./pda";

import { payer } from "./config";

describe("token-test", () => {
	// it("Creates the Mint Account!", async () => {
	// 	const args = {
	// 		name: "Disappointed Fan",
	// 		uri: "https://raw.githubusercontent.com/Calyptus-Learn/workshops/main/solana/meme-token/metadata.json",
	// 		symbol: "DSF",
	// 		supply: new BN(1_000_000),
	// 		decimals: 9,
	// 	};
	// 	const mint = anchor.web3.Keypair.generate();
	// 	const txHash = await program.methods
	// 		.create(args)
	// 		.accounts({
	// 			payer: anchor.AnchorProvider.env().wallet.publicKey,
	// 			mint: mint.publicKey,
	// 			metadataAccount: findMetadataAddress(mint.publicKey),
	// 			metadataProgram: METADATA_PROGRAM_ID,
	// 			tokenProgram: TOKEN_PROGRAM_ID,
	// 			systemProgram: SystemProgram.programId,
	// 			sysvarInstruction: SYSVAR_INSTRUCTIONS_PUBKEY,
	// 		})
	// 		.signers([mint])
	// 		.rpc({ skipPreflight: true });
	// 	console.log(
	// 		`create tx Hash: https://explorer.solana.com/tx/${txHash}?cluster=devnet`
	// 	);
	// });

	// it("Mints Tokens!", async () => {
	// 	const args = {
	// 		amount: new BN(1), // mint once DSF token
	// 	};
	// 	const recipientTokenAccount = getAssociatedTokenAddressSync(
	// 		nftMint,
	// 		payer.publicKey
	// 	);
	// 	const txHash = await program.methods
	// 		.mint(args)
	// 		.accounts({
	// 			payer: anchor.AnchorProvider.env().wallet.publicKey,
	// 			mint: nftMint,
	// 			recipientTokenAccount,
	// 			tokenProgram: TOKEN_PROGRAM_ID,
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.rpc({ skipPreflight: true });
	// 	console.log(
	// 		`mint tx Hash: https://explorer.solana.com/tx/${txHash}?cluster=devnet`
	// 	);
	// });

	// it("Transfers Tokens!", async () => {
	// 	const args = {
	// 		amount: new BN(1000),
	// 	};
	// 	const payerTokenAccount = getAssociatedTokenAddressSync(
	// 		nftMint,
	// 		payer.publicKey
	// 	);
	// 	const recipientTokenAccount = getAssociatedTokenAddressSync(
	// 		nftMint,
	// 		payer.publicKey
	// 	);
	// 	const txHash = await program.methods
	// 		.transfer(args)
	// 		.accounts({
	// 			payer: anchor.AnchorProvider.env().wallet.publicKey,
	// 			mint: nftMint,
	// 			payerTokenAccount,
	// 			recipientTokenAccount,
	// 			tokenProgram: TOKEN_PROGRAM_ID,
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.rpc({ skipPreflight: true });
	// 	console.log(
	// 		`transfer tokens tx Hash: https://explorer.solana.com/tx/${txHash}?cluster=devnet`
	// 	);
	// });
	// it("Revokes Mint Authority!", async () => {
	// 	const txHash = await program.methods
	// 		.revokeMintAuth()
	// 		.accounts({
	// 			payer: anchor.AnchorProvider.env().wallet.publicKey,
	// 			mint: nftMint,
	// 			tokenProgram: TOKEN_PROGRAM_ID,
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.rpc({ skipPreflight: true });
	// 	console.log(
	// 		`revoke mint authority tx Hash: https://explorer.solana.com/tx/${txHash}?cluster=devnet`
	// 	);
	// });
});
