// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "lib/forge-std/src/Test.sol";
import "src/FHEToken.sol";

contract ERC20Test is Test {
    FHEToken public fheToken;

    address public owner;
    address alice = makeAddr("alice");
    address bob = makeAddr("bob");

    function setUp() public {
        fheToken = new FHEToken(8);
        owner = fheToken.owner();
        deal(alice, 100 ether);
        deal(bob, 100 ether);
    }

    function testDeposit() public {
        vm.startPrank(alice);
        uint256 amount = 10 ether;

        (bool sent, ) = address(fheToken).call{value: amount}("");

        assertEq(fheToken.balance(alice), amount);
        vm.stopPrank();
    }

    function testRecvTx() public {
        bytes32 to = keccak256(abi.encodePacked(bob));
        bytes32 sharedKey = keccak256(abi.encodePacked(alice, bob));
        bytes32 amount = bytes32(uint256(0.1 ether));

        vm.startPrank(alice);
        (bool sent, ) = address(fheToken).call{value: 0}(
            abi.encodeWithSignature(
                "recvTx(bytes32,bytes32,bytes32)",
                to,
                sharedKey,
                amount
            )
        );

        (
            uint8 _id,
            address _from,
            bytes32 _to,
            bytes32 _sharedKey,
            bytes32 _amount
        ) = fheToken.mempool(0);

        assertEq(sent, true);
        assertEq(_id, 1);
        assertEq(_from, alice);
        assertEq(_to, to);
        assertEq(_sharedKey, sharedKey);
        assertEq(_amount, amount);
        vm.stopPrank();
    }
}
