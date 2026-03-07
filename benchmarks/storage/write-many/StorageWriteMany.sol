// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.17;

contract StorageWriteMany {
    mapping(uint256 => uint256) internal slots;

    constructor() {
        for (uint256 i = 0; i < 512; i++) {
            slots[i] = i + 1;
        }
    }

    function Benchmark() external {
        unchecked {
            for (uint256 i = 0; i < 512; i++) {
                slots[i] = slots[i] + i + 1;
            }
        }
    }
}
