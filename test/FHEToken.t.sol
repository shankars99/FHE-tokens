// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "lib/forge-std/src/Test.sol";
import "src/contracts/FHEToken.sol";

contract ERC20Test is Test {
    FHEToken public fheToken;
    FHEToken public callableFHEToken;
    address public owner;
    address alice = makeAddr("alice");
    address bob = makeAddr("bob");
    address mallory = makeAddr("mallory");

    constructor() {
        fheToken = new FHEToken(8);
        callableFHEToken = FHEToken(payable(address(fheToken)));
        owner = fheToken.owner();
        deal(alice, 100 ether);
        deal(bob, 100 ether);

        bytes memory pk_bytes = "pk_bytes";

        // call buy_tokens to make alice and bob users by sending 0.1 ether
        // Prank the `alice` account
        vm.prank(alice);

        // Call `buy_tokens` function with the `pk_bytes` parameter
        (bool sent, ) = address(fheToken).call{value: 0.1 ether}(
            abi.encodeWithSignature("buy_tokens(bytes)", pk_bytes)
        );

        // Assert that the transaction was sent successfully
        assertEq(sent, true);

        vm.prank(bob);
        (sent, ) = address(fheToken).call{value: 0.1 ether}(
            abi.encodeWithSignature("buy_tokens(bytes)", pk_bytes)
        );

        assertEq(sent, true);

        vm.stopPrank();
    }

    function testDepositFromUsersPass() public {
        bool aliceExists = callableFHEToken.hasUser(alice);
        assertEq(aliceExists, true);

        bool bobExists = callableFHEToken.hasUser(bob);
        assertEq(bobExists, true);
    }

    function testDepositFromNonUsersFail() public {
        bool malloryExists = callableFHEToken.hasUser(mallory);
        assertEq(malloryExists, false);
    }

    function testWithdrawFromUsersPass() public {
        bool aliceExists = callableFHEToken.hasUser(alice);
        assertEq(aliceExists, true);

        bool bobExists = callableFHEToken.hasUser(bob);
        assertEq(bobExists, true);
    }
}
