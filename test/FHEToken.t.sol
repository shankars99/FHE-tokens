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
        address to = bob;

        // REPLACE WITH REAL FHE TX AND PROOF
        bytes memory fhe_tx = "fhe_tx";
        bytes memory proof = "proof";

        vm.startPrank(alice);
        (bool sent, ) = address(fheToken).call{value: 0}(
            abi.encodeWithSignature(
                "recvTx(address,bytes,bytes)",
                to,
                fhe_tx,
                proof
            )
        );

        (
            uint8 _id,
            address _from,
            address _to,
            bytes memory _fhe_tx,
            bytes memory _proof
        ) = fheToken.mempool(0);

        assertEq(sent, true);
        assertEq(_id, 1);
        assertEq(_from, alice);
        assertEq(_to, to);
        assertEq(_fhe_tx, fhe_tx);
        assertEq(_proof, proof);
        vm.stopPrank();
    }
}
