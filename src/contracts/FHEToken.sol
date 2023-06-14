// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "lib/solmate/src/tokens/ERC20.sol";

contract FHEToken is ERC20 {
    modifier onlyUser() {
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
        bytes32 fhe_tx_hash,
        bytes fhe_tx_sender,
        bytes fhe_tx_receiver,
        bytes fhe_proof
    );
    event newUser(address indexed user);
    event ReveivedEther(address indexed from, uint256 amount);

    address payable public owner;
    uint256 public block_tx;
    uint256 public immutable FEE;
    uint256 public total_fees;
    uint256 public blocks;
    mapping(address => bool) public hasUser;

    constructor(uint8 _decimals) ERC20("FHEToken", "FHT", _decimals) {
        owner = payable(msg.sender);
        hasUser[msg.sender] = true;

        block_tx = 0;
        FEE = 0.01 ether;
        total_fees = 0;
        blocks = 0;
    }

    function recvTx(
        string calldata _to,
        bytes calldata _fhe_tx_sender,
        bytes calldata _fhe_tx_receiver,
        bytes calldata _fhe_proof
    ) external payable onlyUser {
        require(
            msg.value == FEE,
            "FHEToken_recvTx: amount must be equal to fees"
        );

        address _receiver = address(0);

        if (bytes(_to).length != 0) {
            _receiver = payable(
                address(uint160(uint256(keccak256(abi.encodePacked(_to)))))
            );
        }

        _recvTx(_receiver, _fhe_tx_sender, _fhe_tx_receiver, _fhe_proof);
        total_fees += FEE;

        block_tx += 1;
    }

    function _recvTx(
        address _to,
        bytes calldata _fhe_tx_sender,
        bytes calldata _fhe_tx_receiver,
        bytes calldata _fhe_proof
    ) internal {
        bytes32 _fhe_tx_hash = keccak256(
            abi.encodePacked(
                msg.sender,
                _to,
                _fhe_tx_sender,
                _fhe_tx_receiver,
                block.number
            )
        );

        emit RecvNewTx(
            block_tx,
            msg.sender,
            _to,
            _fhe_tx_hash,
            _fhe_tx_sender,
            _fhe_tx_receiver,
            _fhe_proof
        );
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

    function new_block() external onlyOwner {
        blocks += 1;
        block_tx = 0;
    }

    function withdrawal(uint256 _amount) public onlyUser {
        _burn(msg.sender, _amount);

        payable(msg.sender).transfer(_amount);

        emit Withdrawal(msg.sender, _amount);
    }

    function balance(address _account) public view returns (uint256) {
        return balanceOf[_account];
    }

    function buy_tokens(bytes calldata _pk) external payable {
        require(
            msg.value <= 0.1 ether,
            "FHEToken_buy_tokens (dev-ing): amount must be leq to 0.1 ether"
        );
        deposit(msg.sender, msg.value, _pk);
    }

    function setOwner(address payable _owner) external onlyOwner {
        owner = _owner;
    }

    function withdrawFees() external onlyOwner {
        payable(owner).transfer(total_fees);
        total_fees = 0;
    }

    receive() external payable {
        emit ReveivedEther(msg.sender, msg.value);
    }
}
