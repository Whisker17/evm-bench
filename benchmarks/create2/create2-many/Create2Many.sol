// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.17;

contract Created2Child {
    uint256 internal immutable stored;

    constructor(uint256 initialValue) {
        stored = initialValue;
    }

    function value() external view returns (uint256) {
        return stored;
    }
}

contract Create2Many {
    uint256 public checksum;
    uint256 internal saltNonce;

    constructor() {
        checksum = 1;
        saltNonce = 1;
    }

    function Benchmark() external {
        uint256 acc = checksum;
        uint256 baseSalt = saltNonce * 1000;
        for (uint256 i = 0; i < 24; i++) {
            Created2Child child = new Created2Child{salt: bytes32(baseSalt + i)}(acc + i);
            acc ^= child.value();
        }
        saltNonce++;
        checksum = acc;
    }
}
