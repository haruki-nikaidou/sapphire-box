// SPDX-License-Identifier: MIT
pragma solidity ^0.8.6;

contract KeyValueStore {
    // storage the mapping of the key-value
    mapping(string => string) public data;

    // storage the array of the keys
    string[] public keys;

    // the owner of the contract
    address public owner;

    // Set the `owner` to the sender
    constructor() {
        owner = msg.sender;
    }

    // authorizes the caller
    modifier onlyOwner {
        require(msg.sender == owner, "Only owner can call this function.");
        _;
    }

    // set the key-value pair
    function set(string memory key, string memory value) public onlyOwner {
        data[key] = value;
        keys.push(key);
    }

    // change the owner of the contract
    function transferOwnership(address newOwner) public onlyOwner {
        require(newOwner != address(0), "New owner cannot be the zero address.");
        owner = newOwner;
    }

    // get all key-value pairs
    function getAll() public view returns (string[] memory, string[] memory) {
        string[] memory values = new string[](keys.length);

        for (uint i = 0; i < keys.length; i++) {
            values[i] = data[keys[i]];
        }

        return (keys, values);
    }
}