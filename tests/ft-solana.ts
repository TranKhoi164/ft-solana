import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FtSolana } from "../target/types/ft_solana";
import {
	PublicKey,
	Keypair,
	SystemProgram,
	SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import { expect } from "chai";
import {
	TOKEN_PROGRAM_ID,
	createMint,
	getOrCreateAssociatedTokenAccount,
	mintTo,
	burn,
} from "@solana/spl-token";

describe("ft-solana", () => {
	// Configure the client to use the local cluster.
	// anchor.setProvider(anchor.AnchorProvider.env());
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);
	const program = anchor.workspace.FtSolana as Program<FtSolana>;

	const admin = anchor.web3.Keypair.generate();
	const minter = anchor.web3.Keypair.generate();
	const user = anchor.web3.Keypair.generate();
	const pauser = anchor.web3.Keypair.generate();

	let tokenDataAccount: anchor.web3.Keypair;
	let mint: PublicKey;
	let adminTokenAccount: PublicKey;
	let userTokenAccount: PublicKey;

	const maxSupply = new anchor.BN(2_000_000_000 * 10 ** 9);

	before(async () => {
		// Airdrop SOL to accounts
		await Promise.all([
			provider.connection.requestAirdrop(admin.publicKey, 10000000000),
			provider.connection.requestAirdrop(minter.publicKey, 10000000000),
			provider.connection.requestAirdrop(user.publicKey, 10000000000),
			provider.connection.requestAirdrop(pauser.publicKey, 10000000000),
		]);
		// create the mint
		mint = await createMint(
			provider.connection,
			admin,
			admin.publicKey,
			null,
			9
		);

		// create token account
		tokenDataAccount = anchor.web3.Keypair.generate();
		// derive associated token account(ata) for a given wallet and token mint.
		// The ATA is a special type of token account that is linked to a wallet and a specific SPL token, ensuring a standardized way to interact with tokens.
		// Get associated token address for user
		adminTokenAccount = (
			await getOrCreateAssociatedTokenAccount(
				provider.connection,
				admin,
				mint,
				admin.publicKey
			)
		).address;
		userTokenAccount = (
			await getOrCreateAssociatedTokenAccount(
				provider.connection,
				admin,
				mint,
				user.publicKey
			)
		).address;
	});

	it("should initialize token data", async () => {
		await program.methods
			.initialize()
			.accounts({
				tokenData: tokenDataAccount.publicKey,
				admin: admin.publicKey,
				systemProgram: SystemProgram.programId,
			})
			.signers([admin, tokenDataAccount])
			.rpc();

    const tokenData = await program.account.tokenData.fetch(
      tokenDataAccount.publicKey
    );


		expect(tokenData.totalSupply.toNumber()).to.equal(0);
		expect(tokenData.maxSupply.eq(maxSupply)).to.be.true;
		expect(tokenData.isPaused).to.be.false;
		expect(tokenData.admin.equals(admin.publicKey)).to.be.true;
	});
});
