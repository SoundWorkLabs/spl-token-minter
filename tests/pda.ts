import { PublicKey } from "@solana/web3.js";
import { METADATA_PROGRAM_ID } from "./constants";

export const findMetadataAddress = (mint: PublicKey): PublicKey => {
	return PublicKey.findProgramAddressSync(
		[
			Buffer.from("metadata", "utf8"),
			METADATA_PROGRAM_ID.toBuffer(),
			mint.toBuffer(),
		],
		METADATA_PROGRAM_ID
	)[0];
};
