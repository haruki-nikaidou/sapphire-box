// SPDX-License-Identifier: MIT
pragma solidity ^0.8.6;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract Show is ERC721 {
    using Counters for Counters.Counter;

    Counters.Counter private _tokenIds;

    // Map the number of tokens to the `Astral Mask Storage` contract
    mapping(uint256 => address) private _tokenURIs;

    // The collection of all `Astral Mask Storage` contract addresses
    constructor() ERC721("Astral Mask Encrypt","ASME") {}

    // To mint a new token, 
    function mint(address storeContractAddress) public {
        _tokenIds.increment();

        uint256 newItemId = _tokenIds.current();
        _mint(msg.sender, newItemId);
        _setTokenURI(newItemId, storeContractAddress);
    }

    // Set the `Astral Mask Storage` contract address for a given `tokenId`.
    function _setTokenURI(uint256 tokenId, address storeContractAddress) internal virtual {
        require(_exists(tokenId), "ERC721URIStorage: URI set of nonexistent token");
        _tokenURIs[tokenId] = storeContractAddress;
    }

    // util function to convert address to string
    function tokenURI(uint256 tokenId) public view virtual override returns (string memory) {
        require(_exists(tokenId), "ERC721URIStorage: URI query for nonexistent token");

        string memory _tokenURI = addressToString(_tokenURIs[tokenId]);
        return _tokenURI;
    }

    function addressToString(address _address) public pure returns (string memory) {
        bytes32 value = bytes32(uint256(uint160(_address)));
        bytes memory alphabet = "0123456789abcdef";

        bytes memory str = new bytes(42);
        str[0] = "0";
        str[1] = "x";

        for (uint256 i = 0; i < 20; i++) {
            str[2 + i * 2] = alphabet[uint256(uint8(value[i + 12] >> 4))];
            str[3 + i * 2] = alphabet[uint256(uint8(value[i + 12] & 0x0f))];
        }

        return string(str);
    }
}