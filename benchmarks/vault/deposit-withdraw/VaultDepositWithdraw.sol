// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.17;

contract VaultDepositWithdraw {
    mapping(address => uint256) internal balances;
    uint256 public totalAssets;
    uint256 public totalShares;
    uint256 internal round;

    constructor() {
        round = 1;
    }

    function _deposit(address user, uint256 amount) internal {
        balances[user] += amount;
        totalAssets += amount;
        totalShares += amount;
    }

    function _withdraw(address user, uint256 amount) internal {
        balances[user] -= amount;
        totalAssets -= amount;
        totalShares -= amount;
    }

    function Benchmark() external {
        uint256 base = round * 100;
        for (uint256 i = 0; i < 64; i++) {
            address user = address(uint160(i + 1));
            uint256 amount = base + i + 1;
            _deposit(user, amount);
            _withdraw(user, amount / 2);
        }
        round += 1;
    }
}
