// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.17;

contract LogBurst {
    event Burst(uint256 indexed index, uint256 indexed lane, bytes32 digest, bytes payload);

    function Benchmark() external {
        bytes memory payload = abi.encodePacked(
            uint256(0x1234),
            bytes32(uint256(0x5678)),
            bytes32(uint256(0x9abc))
        );

        for (uint256 i = 0; i < 192; i++) {
            emit Burst(i, i % 7, keccak256(abi.encodePacked(i, payload)), payload);
        }
    }
}
