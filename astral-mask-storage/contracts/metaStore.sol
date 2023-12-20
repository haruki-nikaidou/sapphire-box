// SPDX-License-Identifier: MIT
pragma solidity ^0.8.6;

contract MetaStorage {
    string private namePrefix;
    string private description;
    string private image;
    mapping(uint256 => string) private sheetNames;
    address private keyValueStore;

    address private owner;

    constructor() {
        owner = msg.sender;
    }

    modifier onlyOwner() {
        require(msg.sender == owner, "Not owner");
        _;
    }

    modifier onlyKvStore() {
        require(msg.sender == keyValueStore, "only KeyValueStore Conrtact Can Call this Function");
        _;
    }

    function setNamePrefix(string calldata _namePrefix) external onlyOwner {
        namePrefix = _namePrefix;
    }

    function setDescription(string calldata _description) external onlyOwner {
        description = _description;
    }

    function setKvStoreAddress(address _kvAddress) external onlyOwner {
        keyValueStore = _kvAddress;
    }

    function setImage(string calldata _image) external onlyOwner {
        image = _image;
    }

    function getTokenMeta(uint256 tokenId) external view returns (string memory) {
        string memory tokenName = string(abi.encodePacked(namePrefix, " #", uintToString(tokenId)));
        string memory json = string(abi.encodePacked(
            '{"name":"', tokenName, 
            '", "image":"', image,
            '", "description":"', description,
            '", "attributes": { "trait_type": "Sheet Name", "value": "', sheetNames[tokenId], '"}}'
        ));
        return json;
    }

    function addMeta(uint256 tokenId, string calldata _sheetName) external onlyKvStore {
        sheetNames[tokenId] = _sheetName;
    }

    function uintToString(uint _i) internal pure returns (string memory _uintAsString) {
        if (_i == 0) {
            return "0";
        }
        uint j = _i;
        uint len;
        while (j != 0) {
            len++;
            j /= 10;
        }
        bytes memory bstr = new bytes(len);
        uint k = len;
        while (_i != 0) {
            k = k-1;
            uint8 temp = (48 + uint8(_i - _i / 10 * 10));
            bytes1 b1 = bytes1(temp);
            bstr[k] = b1;
            _i /= 10;
        }
        return string(bstr);
    }
}
