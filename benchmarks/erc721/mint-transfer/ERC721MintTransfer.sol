// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.17;

contract ERC721MintTransfer {
    mapping(uint256 => address) internal ownerOf;
    mapping(address => uint256) internal balanceOf;
    uint256 public nextTokenId;

    constructor() {
        nextTokenId = 1;
    }

    function _mint(address to, uint256 tokenId) internal {
        require(ownerOf[tokenId] == address(0), "already minted");
        ownerOf[tokenId] = to;
        balanceOf[to] += 1;
    }

    function _transfer(address from, address to, uint256 tokenId) internal {
        require(ownerOf[tokenId] == from, "wrong owner");
        ownerOf[tokenId] = to;
        balanceOf[from] -= 1;
        balanceOf[to] += 1;
    }

    function Benchmark() external {
        address recipient = address(0xBEEF);
        uint256 start = nextTokenId;
        for (uint256 i = 0; i < 128; i++) {
            _mint(msg.sender, start + i);
        }
        for (uint256 i = 0; i < 128; i++) {
            _transfer(msg.sender, recipient, start + i);
        }
        nextTokenId = start + 128;
    }
}
