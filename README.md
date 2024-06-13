# Solana Token Minter

This Solana program allows you to create, mint and transfer tokens.

## Building

Please make sure that the Solana toolchain you are using is similar to the one described in the [Anchor.toml](./Anchor.toml) config file.

```bash
solana-cli 1.18.8 (src:e2d34d37; feat:3469865029, client:SolanaLabs)
anchor-cli 0.29.0
```

## Testing

1. Update the connection string in [config.ts](./tests/config.ts) to use the public RPC API.

```ts
export const connection = new anchor.web3.Connection(clusterApiUrl("devnet"));
```

Else create a `.env` file with and add a `HELIUS_API_KEY` api key.

2. If you are creating/initializing a new token mint, you will need to update the `ADMIN_ADDRESS` with your own address in [constants.rs](./programs/token-test/src/constants.rs)

3. After initializing the token mint address, run the mint test with the amount of tokens you want to mint to the ADMIN_ADDRESS.

```ts
it("Mints Tokens!", async () => {
	const args = {
		amount: new BN(1), // mint once DSF token
	};
	const recipientTokenAccount = getAssociatedTokenAddressSync(
		nftMint,
		payer.publicKey
	);
	const txHash = await program.methods
		.mint(args)
		.accounts({
			payer: anchor.AnchorProvider.env().wallet.publicKey,
			mint: nftMint,
			recipientTokenAccount,
			tokenProgram: TOKEN_PROGRAM_ID,
			systemProgram: SystemProgram.programId,
		})
		.rpc({ skipPreflight: true });
	console.log(
		`mint tx Hash: https://explorer.solana.com/tx/${txHash}?cluster=devnet`
	);
});
```

You can use the [`revoke`](./programs/token-test/src/instructions/revoke.rs), to remove the mint authority after which not tokens can be minted again.

```ts
it("Revokes Mint Authority!", async () => {
	const txHash = await program.methods
		.revokeMintAuth()
		.accounts({
			payer: anchor.AnchorProvider.env().wallet.publicKey,
			mint: nftMint,
			tokenProgram: TOKEN_PROGRAM_ID,
			systemProgram: SystemProgram.programId,
		})
		.rpc({ skipPreflight: true });
	console.log(
		`revoke mint authority tx Hash: https://explorer.solana.com/tx/${txHash}?cluster=devnet`
	);
});
```
