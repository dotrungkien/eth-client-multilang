require('dotenv').config();
const { ethers } = require('ethers');

async function main() {
  const provider = new ethers.providers.JsonRpcProvider('http://127.0.0.1:8545');
  const signer = provider.getSigner();
  console.log(await signer.getAddress());

  const contractAddress = process.env['SIMPLE_STORAGE'];
  const abi = require('../abis/SimpleStorage.json').abi;

  const simpleStorage = new ethers.Contract(contractAddress, abi, signer);

  const valueBefore = await simpleStorage.get();
  console.log(valueBefore.toString());
  await simpleStorage.set(999999);
  const valueAfter = await simpleStorage.get();
  console.log(valueAfter.toString());
}

main();
