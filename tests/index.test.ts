import * as borsh from "borsh";
import { expect, test } from "bun:test";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { COUNTER_SIZE, schema } from "./types";

let adminAccount = Keypair.generate();
let dataAccount = Keypair.generate();

const PROGRAM_ID = new PublicKey("EhQBB8NYwDt6eXH1o5prdmVDbPZ3T1yBUbsJ57jBzPu4");
const connection = new Connection("http://127.0.0.1:8899");

test("Account is initialed", async () => {
    const txn = await connection.requestAirdrop(adminAccount.publicKey, 1 * LAMPORTS_PER_SOL);
    await connection.confirmTransaction(txn);

    await connection.getAccountInfo(adminAccount.publicKey);

    const lamports = await connection.getMinimumBalanceForRentExemption(COUNTER_SIZE);

    const ix = SystemProgram.createAccount({
        fromPubkey: adminAccount.publicKey,
        lamports,
        space: COUNTER_SIZE,
        programId: PROGRAM_ID,
        newAccountPubkey: dataAccount.publicKey
    })
    const createAccountTxn = new Transaction();
    createAccountTxn.add(ix);
    const signature = await connection.sendTransaction(createAccountTxn, [adminAccount, dataAccount]);

    await connection.confirmTransaction(signature);
    console.log(dataAccount.publicKey.toBase58());

    const dataAccoutInfo = await connection.getAccountInfo(dataAccount.publicKey);
    const counter = borsh.deserialize(schema, dataAccoutInfo?.data); // { count: 10 }
    console.log(counter.count);
    expect(counter.count).toBe(0);
});

test("make sure incress works", async () => {
    // Write the code to intract with the contract to call incress or decress instruction

    ////
    const dataAccoutInfo = await connection.getAccountInfo(dataAccount.publicKey);
    const counter = borsh.deserialize(schema, dataAccoutInfo?.data); // { count: 10 }
    console.log(counter.count);
    expect(counter.count).toBe(10);
})