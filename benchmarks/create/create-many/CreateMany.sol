// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.17;

contract CreatedChild {
    uint256 internal immutable stored;

    constructor(uint256 initialValue) {
        stored = initialValue;
    }

    function value() external view returns (uint256) {
        return stored;
    }
}

contract CreateMany {
    uint256 public checksum;

    constructor() {
        checksum = 1;
    }

    function Benchmark() external {
        uint256 acc = checksum;
        for (uint256 i = 0; i < 32; i++) {
            CreatedChild child = new CreatedChild(acc + i);
            acc ^= child.value();
        }
        checksum = acc;
    }
}
