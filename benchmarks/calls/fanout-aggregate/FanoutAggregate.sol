// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.17;

interface IFanoutWorker {
    function work(uint256 seed) external returns (uint256);
}

contract FanoutWorker is IFanoutWorker {
    function work(uint256 seed) external pure returns (uint256 acc) {
        acc = seed;
        for (uint256 i = 0; i < 48; i++) {
            acc = uint256(keccak256(abi.encodePacked(acc, i, seed)));
        }
    }
}

contract FanoutAggregate {
    IFanoutWorker[8] internal workers;
    uint256 public checksum;

    constructor() {
        for (uint256 i = 0; i < workers.length; i++) {
            workers[i] = new FanoutWorker();
        }
        checksum = 1;
    }

    function Benchmark() external {
        uint256 acc = checksum;
        for (uint256 i = 0; i < workers.length; i++) {
            acc ^= workers[i].work(acc + i);
        }
        checksum = acc;
    }
}
