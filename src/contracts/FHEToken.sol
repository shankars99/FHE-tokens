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

    modifier onlyOwner() {
        require(
            msg.sender == owner,
            "FHEToken: only owner can call this function"
        );
        _;
    }

    event Deposit(address indexed from, uint256 amount, bytes pk);
    event Withdrawal(address indexed to, uint256 amount);
    event RecvNewTx(
        uint256 indexed id,
        address indexed from,
        address indexed to,
        bytes fhe_tx,
        bytes fhe_proof
    );
    event newUser(address indexed user);
    event ReveivedEther(address indexed from, uint256 amount);

    address payable public owner;
    uint256 public last_tx_id = 0;
    uint256 public fees = 0 ether;

    mapping(address => bool) public hasUser;

    constructor(uint8 _decimals) ERC20("FHEToken", "FHT", _decimals) {
        owner = payable(msg.sender);
        hasUser[msg.sender] = true;
    }

    function recvTx(
        address _to,
        bytes calldata _fhe_tx,
        bytes calldata _fhe_proof
    ) public payable senderIsUser receiverIsUser(_to) {
        require(
            msg.value == fees,
            "FHEToken_recvTx: amount must be equal to fees"
        );

        last_tx_id += 1;

        emit RecvNewTx(last_tx_id, msg.sender, _to, _fhe_tx, _fhe_proof);
    }

    function deposit(
        address _to,
        uint256 _amount,
        bytes calldata _pk
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

    function buy_tokens(bytes calldata _pk) external payable {
        deposit(msg.sender, msg.value, _pk);
    }

    receive() external payable {
        emit ReveivedEther(msg.sender, msg.value);
    }
}
