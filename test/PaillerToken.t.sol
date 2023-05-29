// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "lib/forge-std/src/Test.sol";
import "src/PaillerToken.sol";

contract ERC20Test is Test {
    PaillerToken public paillerToken;

    address payable public owner;
    address payable alice = payable(makeAddr("alice"));
    address payable bob = payable(makeAddr("bob"));

    function setUp() public {
        paillerToken = new PaillerToken(8);
        owner = paillerToken.owner();
        deal(alice, 100 ether);
        deal(bob, 100 ether);
    }

    function testMintTokens() public {
        uint256 amount = 10 ether;

        vm.startPrank(alice);

        (bool sent, ) = address(paillerToken).call{value: amount}("");

        assertEq(paillerToken.balance(alice), amount);
    }
}
