// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "lib/solmate/src/tokens/ERC20.sol";

contract FHEToken is ERC20 {
    address payable public owner;

    event Deposit(address indexed from, uint256 amount);
    event Withdrawal(address indexed to, uint256 amount);
    event RecvNewTx(Tx new_tx);

    struct Tx {
        uint8 id;
        address from;
        bytes32 to;
        bytes32 sharedKey;
        bytes32 amount;
    }
    struct Confirmedblocks {
        Tx[] txs;
    }

    uint8 public last_tx_id = 0;
    uint256 public fees = 0 ether;

    Tx[] public mempool;
    mapping(uint256 => Confirmedblocks) confirmedblocks;

    constructor(uint8 _decimals) ERC20("FHEToken", "FHT", _decimals) {
        owner = payable(msg.sender);
    }

    function recvTx(
        bytes32 _to,
        bytes32 _sharedKey,
        bytes32 _amount
    ) public payable {
        require(
            msg.value == fees,
            "FHEToken: amount must be equal to fees"
        );

        last_tx_id += 1;

        Tx memory recv_tx = Tx({
            id: last_tx_id,
            from: msg.sender,
            to: _to,
            sharedKey: _sharedKey,
            amount: _amount
        });

        mempool.push(recv_tx);

        emit RecvNewTx(recv_tx);
    }

    function deposit(address _to, uint256 _amount) internal {
        _mint(_to, _amount);

        emit Deposit(_to, _amount);
    }

    function withdrawal(uint256 _amount) public {
        _burn(msg.sender, _amount);

        payable(msg.sender).transfer(_amount);

        emit Withdrawal(msg.sender, _amount);
    }

    function balance(address _account) public view returns (uint256) {
        return balanceOf[_account];
    }

    event ReveivedEther(address indexed from, uint256 amount);

    receive() external payable {
        emit ReveivedEther(msg.sender, msg.value);

        deposit(msg.sender, msg.value);
    }
}
