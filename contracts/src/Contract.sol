// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.13;

contract SimpleStorage {
    uint256 public value;

    constructor(uint256 _value) {
        value = _value;
    }

    function set(uint256 newValue) external {
        value = newValue;
    }

    function get() external view returns (uint256) {
        return value;
    }
}
