// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.17;

interface IDeepCallNode {
    function step(uint256 value) external returns (uint256);
}

contract DeepCallLeaf is IDeepCallNode {
    function step(uint256 value) external pure returns (uint256) {
        uint256 acc = value;
        for (uint256 i = 0; i < 64; i++) {
            acc = uint256(keccak256(abi.encodePacked(acc, i)));
        }
        return acc;
    }
}

contract DeepCallNode is IDeepCallNode {
    IDeepCallNode internal immutable next;

    constructor(IDeepCallNode nextNode) {
        next = nextNode;
    }

    function step(uint256 value) external returns (uint256) {
        return next.step(value + 1) ^ value;
    }
}

contract DeepCallChain {
    IDeepCallNode internal immutable head;
    uint256 public checksum;

    constructor() {
        IDeepCallNode node = new DeepCallLeaf();
        for (uint256 i = 0; i < 6; i++) {
            node = new DeepCallNode(node);
        }
        head = node;
        checksum = 1;
    }

    function Benchmark() external {
        checksum = checksum ^ head.step(checksum);
    }
}
