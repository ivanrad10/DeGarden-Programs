import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DeGarden } from "../target/types/de_garden";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import {
  getAccount,
  getAssociatedTokenAddress,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";

describe("de-garden", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.deGarden as Program<DeGarden>;

  it("Is initialized!", async () => {
    const [sensorHostStatePDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("SENSOR_HOST"), program.provider.publicKey.toBuffer()],
      program.programId
    );

    const fetched = await program.account.sensorHost.fetch(sensorHostStatePDA);

    console.log(fetched);

    const sensors = await program.account.sensor.all();

    console.log(sensors[0]);
    console.log(sensors[1]);

    // console.log(
    //   "Moisture counter: " + fetched.moistureSensorCounter.toString()
    // );

    // const counter = BigInt(fetched.moistureSensorCounter.toString());
    // const counterBuff = Buffer.alloc(8);
    // counterBuff.writeBigInt64LE(counter);

    // const [sensorPDA] = PublicKey.findProgramAddressSync(
    //   [
    //     Buffer.from("MOISTURE_SENSOR"),
    //     sensorHostStatePDA.toBuffer(),
    //     counterBuff,
    //   ],
    //   program.programId
    // );

    // await program.methods
    //   .registerMoistureSensor(new BN(33), new BN(45))
    //   .accountsStrict({
    //     host: pk,
    //     sensorHostState: sensorHostStatePDA,
    //     moistureSensor: sensorPDA,
    //     systemProgram: SYSTEM_PROGRAM_ID,
    //   })
    //   .signers([myKeypair])
    //   .rpc();

    const [tokenMintPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("TOKEN_MINT")],
      program.programId
    );

    const [vaultPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("VAULT")],
      program.programId
    );

    const hostTokenAta = await getAssociatedTokenAddress(
      tokenMintPDA,
      program.provider.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID
    );

    const vaultTokenAta = await getAssociatedTokenAddress(
      tokenMintPDA,
      vaultPDA,
      true,
      TOKEN_2022_PROGRAM_ID
    );

    const vaultTokenBalance = await getAccount(
      program.provider.connection,
      vaultTokenAta,
      "processed",
      TOKEN_2022_PROGRAM_ID
    );

    console.log("VAULT TOKEN BALANACE: " + vaultTokenBalance.amount.toString());

    const counterBuff = Buffer.alloc(8);
    counterBuff.writeBigInt64LE(BigInt(1));

    const [sensorPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("MOISTURE_SENSOR"),
        sensorHostStatePDA.toBuffer(),
        counterBuff,
      ],
      program.programId
    );

    console.log(program.provider.publicKey);
    console.log(sensorHostStatePDA);
    console.log(sensorPDA);
    console.log(tokenMintPDA);
    console.log(hostTokenAta);
    console.log(vaultPDA);
    console.log(vaultTokenAta);
    console.log(TOKEN_2022_PROGRAM_ID);

    // const tx = await program.methods
    //   .depositCollateral("MOISTURE_SENSOR", new BN(1))
    //   .accountsStrict({
    //     host: program.provider.publicKey,
    //     sensorHostState: sensorHostStatePDA,
    //     sensor: sensorPDA,
    //     tokenMint: tokenMintPDA,
    //     hostTokenAta: hostTokenAta,
    //     vault: vaultPDA,
    //     vaultTokenAta: vaultTokenAta,
    //     tokenProgram: TOKEN_2022_PROGRAM_ID,
    //   })
    //   .signers([])
    //   .rpc();
  });
});
