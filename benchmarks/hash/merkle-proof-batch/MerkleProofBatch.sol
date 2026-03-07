// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.17;

contract MerkleProofBatch {
    bytes32[8] internal siblings = [
        bytes32(uint256(0x1001)),
        bytes32(uint256(0x1002)),
        bytes32(uint256(0x1003)),
        bytes32(uint256(0x1004)),
        bytes32(uint256(0x1005)),
        bytes32(uint256(0x1006)),
        bytes32(uint256(0x1007)),
        bytes32(uint256(0x1008))
    ];

    function Benchmark() external view returns (bytes32 acc) {
        for (uint256 i = 0; i < 48; i++) {
            bytes32 computed = keccak256(abi.encodePacked(i, uint256(0xabcdef)));
            for (uint256 j = 0; j < siblings.length; j++) {
                bytes32 sibling = siblings[j];
                if (computed < sibling) {
                    computed = keccak256(abi.encodePacked(computed, sibling));
                } else {
                    computed = keccak256(abi.encodePacked(sibling, computed));
                }
            }
            acc = keccak256(abi.encodePacked(acc, computed));
        }
    }
}
