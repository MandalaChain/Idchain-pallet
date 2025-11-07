import { ContractFactory, Signer, Wallet, providers } from "ethers";
import * as ethers from "ethers";

const deployContract = async (
  signer: Signer,
  contractJSON: any,
  args: Array<any>
) => {
  const factory = new ContractFactory(
    contractJSON.abi,
    contractJSON.bytecode,
    signer
  );
  return factory.deploy(...args);
};

async function exec() {
  const provider = new providers.WebSocketProvider("http://127.0.0.1:9944");

  const pk = {
    moonbeam:
      "0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133",
    idchain: "87dd27c0b70d114c04d9fdeeab3e21723294bb37127e23c35edb2da223f16fb4",
    ganache:
      "0xe0016f39c73ca5bda8b5cfe10a0a04be5bdd1b3ecd325d9d910aa8158ddcfe8c",
  };

  const wallet = new Wallet(pk.idchain, provider);
  const abi = require("./Dummy");

  const contract = new ethers.Contract(
    "0x00000000000000000000000000000000000007d0",
    abi,
    wallet
  );

  const a = await contract.nonces(wallet.address);
  console.log(a);
  
}

exec();
