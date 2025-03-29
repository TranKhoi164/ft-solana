import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FtSolana } from "../target/types/ft_solana";
import * as web3 from "@solana/web3.js";
import BN from 'bn.js'
import { expect } from "chai";
import assert from 'assert'
import {
	TOKEN_PROGRAM_ID,
	createMint,
	getOrCreateAssociatedTokenAccount,
	mintTo,
	burn,
} from "@solana/spl-token";

describe("ft_solana program test", () => {
	// Configure the client to use the local cluster.
	// anchor.setProvider(anchor.AnchorProvider.env());
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);
	const program = anchor.workspace.FtSolana as anchor.Program<FtSolana>;

  const METADAT_SEED = "metadata"; 

	const admin = anchor.web3.Keypair.generate();
	const minter = anchor.web3.Keypair.generate();
	const user = anchor.web3.Keypair.generate();
	const pauser = anchor.web3.Keypair.generate();
	const maxSupply = new anchor.BN(2_000_000_000 * 10 ** 9);

	
});
