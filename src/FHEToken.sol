// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "lib/solmate/src/tokens/ERC20.sol";

contract FHEToken is ERC20 {
    modifier receiverIsUser(address _receiver) {
        require(hasUser[_receiver] == true, "FHEToken: receiver is not a user");
        _;
    }

    modifier senderIsUser() {
        require(hasUser[msg.sender] == true, "FHEToken: sender is not a user");
        _;
    }

    address payable public owner;

    event Deposit(address indexed from, uint256 amount, string pk);
    event Withdrawal(address indexed to, uint256 amount);
    event RecvNewTx(
        uint8 indexed id,
        address indexed from,
        address indexed to,
        string fhe_tx,
        string proof
    );
    event newUser(address indexed user);

    struct Tx {
        uint8 id;
        address from;
        address to;
        bytes fhe_tx;
        string proof;
    }
    struct Confirmedblocks {
        Tx[] txs;
    }

    uint8 public last_tx_id = 0;
    uint256 public fees = 0 ether;

    Tx[] public mempool;
    mapping(uint256 => Confirmedblocks) confirmedblocks;
    mapping(address => bool) public hasUser;

    constructor(uint8 _decimals) ERC20("FHEToken", "FHT", _decimals) {
        owner = payable(msg.sender);
    }

    function recvTx(
        address _to,
        bytes calldata _fhe_tx,
        string calldata _proof
    ) public payable senderIsUser receiverIsUser(_to) {
        require(msg.value == fees, "FHEToken: amount must be equal to fees");

        last_tx_id += 1;

        Tx memory recv_tx = Tx({
            id: last_tx_id,
            from: msg.sender,
            to: _to,
            fhe_tx: _fhe_tx,
            proof: _proof
        });

        mempool.push(recv_tx);

        emit RecvNewTx(last_tx_id, msg.sender, _to, string(_fhe_tx), _proof);
    }

    function deposit(
        address _to,
        uint256 _amount,
        string calldata _pk
    ) internal {
        _mint(_to, _amount);

        if (hasUser[_to] == false) {
            hasUser[_to] = true;
            emit newUser(_to);
        }

        emit Deposit(_to, _amount, _pk);
    }

    function withdrawal(uint256 _amount) public senderIsUser {
        _burn(msg.sender, _amount);

        payable(msg.sender).transfer(_amount);

        emit Withdrawal(msg.sender, _amount);
    }

    function balance(address _account) public view returns (uint256) {
        return balanceOf[_account];
    }

    event ReveivedEther(address indexed from, uint256 amount);

    function buy_tokens(bytes calldata _pk) external payable {
        deposit(msg.sender, msg.value, string(_pk));
    }

    function string_to_bytes(
        string calldata _str
    ) public pure returns (bytes memory) {
        return (abi.encode(_str));
    }

    receive() external payable {
        emit ReveivedEther(msg.sender, msg.value);
    }
}
