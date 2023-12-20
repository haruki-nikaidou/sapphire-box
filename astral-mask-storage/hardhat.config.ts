import dotenv from "dotenv";
import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

// Save Private key and API key in .env file
const { PRIVATE_KEY, API_URL } = dotenv.config().parsed!;

const config: HardhatUserConfig = {
  solidity: "0.8.6",
  defaultNetwork: "sepolia",
  networks: {
    hardhat: {},
    sepolia: {
      url: API_URL,
      accounts: [`0x${PRIVATE_KEY}`]
    }
  }
};

export default config;
