import { ethers } from 'ethers';
import abi from './abi';

const contractAddress = "0x4E29896289A672E3356548260a736165376a38dA";

const provider = new ethers.JsonRpcProvider("https://polygon-rpc.com")

const contract = new ethers.Contract(contractAddress, abi, provider);

export default async function callPolygon(params: string) {
    return await contract.getTokenMeta(params);
}