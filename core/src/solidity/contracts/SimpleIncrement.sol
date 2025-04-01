// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SimpleIncrement {
    uint256 private _value;

    event ValueIncremented(address indexed by, uint256 newValue);

    constructor() {
        _value = 0;
    }

    function increment() public returns (uint256) {
        _value += 1;
        emit ValueIncremented(msg.sender, _value);
        return _value;
    }

    function getValue() public view returns (uint256) {
        return _value;
    }
}
