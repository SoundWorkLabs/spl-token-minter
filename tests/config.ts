// ! JIMII'S CONFIG'S
import * as anchor from "@coral-xyz/anchor";
import dotenv from "dotenv";
import { readFileSync } from "fs";

import { TokenTest } from "../target/types/token_test";
import { PROGRAM_ID } from "./constants";

dotenv.config();

export const payer = anchor.AnchorProvider.env().wallet as anchor.Wallet;

export const connection = new anchor.web3.Connection(
	`https://devnet.helius-rpc.com/?api-key=${process.env.HELIUS_API_KEY}`
);

const idl = JSON.parse(
	readFileSync(
		"/home/jimii/Documents/web3/solana/token-test/target/idl/token_test.json",
		"utf8"
	)
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
) as anchor.Program<TokenTest>;
