// SPDX-License-Identifier: MIT
pragma solidity ^0.8.6;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract KeyValueStore is ERC721URIStorage {
    // Each ERC721 token is a key to access the k-v sheet
    mapping(uint256 => mapping(string => string)) private keyValueStore;

    uint256 private tokenIdCounter;

    string private baseURI;

    // set `baseURI` (only owner)
    // the URI of each token is `baseURL + metaId`
    // When update `baseURI`, old tokens will not modify their URI
    function updateBaseURI(string memory newBaseURI) public onlyOwner {
        baseURI = newBaseURI;
    }

    constructor(string memory _baseTokenURI) ERC721("AstralMaskStore", "AMS") {
        setBaseURI(_baseTokenURI);
    }

    // Set MetaId when mint
    function mintToken(string memory metaId) public {
        uint256 newTokenId = tokenIdCounter;
        _safeMint(msg.sender, newTokenId);
        _setTokenURI(newTokenId, string(abi.encodePacked(baseTokenURI, metaId)));
        tokenIdCounter++;
    }

    function setKeyValue(uint256 tokenId, string memory key, string memory value) public {
        require(ownerOf(tokenId) == msg.sender, "Not the owner of the token");
        keyValueStore[tokenId][key] = value;
    }

    function getKeyValue(uint256 tokenId, string memory key) public view returns (string memory) {
        return keyValueStore[tokenId][key];
    }

    function deleteKeyValue(uint256 tokenId, string memory key) public {
        require(ownerOf(tokenId) == msg.sender, "Not the owner of the token");
        delete keyValueStore[tokenId][key];
    }
}
