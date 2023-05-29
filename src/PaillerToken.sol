// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "lib/solmate/src/tokens/ERC20.sol";

contract PaillerToken is ERC20 {
    address payable public owner;

    constructor(uint8 _decimals) ERC20("PaillerToken", "Pai", _decimals) {
        owner = payable(msg.sender);
    }

    function mint(address _to, uint256 _amount) internal {
        _mint(_to, _amount);
    }

    function balance(address _account) public view returns (uint256) {
        return balanceOf[_account];
    }

    receive() external payable {
        mint(msg.sender, msg.value);
    }
}
