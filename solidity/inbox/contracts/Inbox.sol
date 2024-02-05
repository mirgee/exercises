// SPDX-License-Identifier: MIT
pragma solidity >=0.6.12 <0.9.0;

contract Inbox {
  string public message;

  constructor(string memory initialMessage) {
    message = initialMessage;
  }

  function setMessage (string memory newMessage) public {
    message = newMessage;
  }
}
