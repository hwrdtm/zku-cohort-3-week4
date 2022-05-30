import Greeter from "artifacts/contracts/Greeters.sol/Greeters.json";
import detectEthereumProvider from "@metamask/detect-provider";
import { Strategy, ZkIdentity } from "@zk-kit/identity";
import { generateMerkleProof, Semaphore } from "@zk-kit/protocols";
import { providers, Contract, utils } from "ethers";
import Head from "next/head";
import React, { useEffect } from "react";
import { TextField } from "@mui/material";
import styles from "../styles/Home.module.css";

function HomeInner({ onChainGreeting = "" }) {
  const [logs, setLogs] = React.useState("Connect your wallet and greet!");

  async function greet() {
    setLogs("Creating your Semaphore identity...");

    const provider = (await detectEthereumProvider()) as any;

    await provider.request({ method: "eth_requestAccounts" });

    const ethersProvider = new providers.Web3Provider(provider);
    const signer = ethersProvider.getSigner();
    const message = await signer.signMessage(
      "Sign this message to create your identity!"
    );

    const identity = new ZkIdentity(Strategy.MESSAGE, message);
    const identityCommitment = identity.genIdentityCommitment();
    const identityCommitments = await (
      await fetch("./identityCommitments.json")
    ).json();

    const merkleProof = generateMerkleProof(
      20,
      BigInt(0),
      identityCommitments,
      identityCommitment
    );

    setLogs("Creating your Semaphore proof...");

    const greeting = `${new Date().toISOString()}`;

    const witness = Semaphore.genWitness(
      identity.getTrapdoor(),
      identity.getNullifier(),
      merkleProof,
      merkleProof.root,
      greeting
    );

    const { proof, publicSignals } = await Semaphore.genProof(
      witness,
      "./semaphore.wasm",
      "./semaphore_final.zkey"
    );
    const solidityProof = Semaphore.packToSolidityProof(proof);

    const response = await fetch("/api/greet", {
      method: "POST",
      body: JSON.stringify({
        greeting,
        nullifierHash: publicSignals.nullifierHash,
        solidityProof: solidityProof,
      }),
    });

    if (response.status === 500) {
      const errorMessage = await response.text();

      setLogs(errorMessage);
    } else {
      setLogs("Your anonymous greeting is onchain :)");
    }
  }

  return (
    <div className={styles.container}>
      <Head>
        <title>Greetings</title>
        <meta
          name="description"
          content="A simple Next.js/Hardhat privacy application with Semaphore."
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <h1 className={styles.title}>Greetings</h1>

        <p className={styles.description}>
          A simple Next.js/Hardhat privacy application with Semaphore.
        </p>

        <div className={styles.logs}>{logs}</div>

        <div onClick={() => greet()} className={styles.button}>
          Greet
        </div>

        <div style={{ padding: "50px" }}>
          <div>On-Chain Greeting</div>
          <TextField
            id="outlined-read-only-greeting"
            value={onChainGreeting}
            InputProps={{
              readOnly: true,
            }}
          />
        </div>
      </main>
    </div>
  );
}

export default function Home() {
  const [onChainGreeting, setOnChainGreeting] = React.useState("");

  useEffect(() => {
    const provider = new providers.JsonRpcProvider("http://localhost:8545");
    const greetersContract = new Contract(
      "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512",
      Greeter.abi,
      provider
    );

    greetersContract.on("NewGreeting", (greeting) => {
      setOnChainGreeting(utils.parseBytes32String(greeting));
    });
  });

  return <HomeInner onChainGreeting={onChainGreeting} />;
}
