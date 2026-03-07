// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.17;

contract MemoryExpandCopy {
    function Benchmark() external pure returns (bytes32 acc) {
        bytes memory chunk = new bytes(256);
        for (uint256 i = 0; i < chunk.length; i++) {
            chunk[i] = bytes1(uint8(i));
        }

        bytes memory buf = abi.encodePacked(chunk, chunk, chunk, chunk);
        for (uint256 i = 0; i < 24; i++) {
            buf = abi.encodePacked(buf, chunk, bytes32(i), acc);
            acc = keccak256(buf);
        }
    }
}
