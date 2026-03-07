// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.17;

contract StorageReadMany {
    mapping(uint256 => uint256) internal slots;
    uint256 public checksum;

    constructor() {
        checksum = 1;
        for (uint256 i = 0; i < 1024; i++) {
            slots[i] = i + 1;
        }
    }

    function Benchmark() external {
        uint256 acc;
        unchecked {
            for (uint256 i = 0; i < 1024; i++) {
                acc += slots[i];
            }
        }
        checksum = checksum + acc;
    }
}
