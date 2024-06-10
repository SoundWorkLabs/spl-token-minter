// ! JIMII'S CONFIG'S
import * as anchor from "@coral-xyz/anchor";
import dotenv from "dotenv";
import { readFileSync } from "fs";

import { SplTokenMinter } from "../target/types/spl_token_minter";
import { PROGRAM_ID } from "./constants";

dotenv.config();

export const payer = anchor.AnchorProvider.env().wallet as anchor.Wallet;

export const connection = new anchor.web3.Connection(
	`https://devnet.helius-rpc.com/?api-key=${process.env.HELIUS_API_KEY}`
);

// ! your error might come from not changing this path
const idl = JSON.parse(
	// readFileSync(
	// 	"/home/jimii/Documents/web3/solana/spl-token-minter/target/idl/spl_token_minter.json",
	// 	"utf8"
	// )
	readFileSync("./target/idl/spl_token_minter.json", "utf8")
);

export const provider = new anchor.AnchorProvider(
	connection,
	payer,
	anchor.AnchorProvider.defaultOptions()
);

export const program = new anchor.Program(
	idl,
	PROGRAM_ID,
	provider
) as anchor.Program<SplTokenMinter>;
