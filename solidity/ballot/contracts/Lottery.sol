// SPDX-License-Identifier: MIT
pragma solidity >=0.6.12 <0.9.0;

contract Lottery {
  address public manager;
  address payable[] public players;

  constructor() {
    manager = msg.sender;
  }

  function enter() public payable {
    require(msg.value > .01 ether);

    players.push();
    players[players.length - 1] = payable(msg.sender);
  }

  function random() private view returns (uint) {
    return uint(keccak256(abi.encodePacked(block.prevrandao, block.timestamp, players)));
  }

  function pickWinner () public restricted {
    uint index = random() % players.length;
    players[index].transfer(address(this).balance);

    delete players;
  }

  modifier restricted() {
    require(msg.sender == manager);
    _;
  }

  function getPlayers () public view restricted returns (address payable[] memory) {
    return players;
  }
}
