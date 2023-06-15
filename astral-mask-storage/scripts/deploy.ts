import { ethers } from "hardhat";

async function deployStorage() {
  const Storage = await ethers.getContractFactory("KeyValueStore");
  return await Storage.deploy();
}

async function deployShow() {
  const Show = await ethers.getContractFactory("Show");
  return await Show.deploy();
}

async function main() {
  const show = await deployShow();
  console.log("Contract Deployed to Address:", await show.getAddress());
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
