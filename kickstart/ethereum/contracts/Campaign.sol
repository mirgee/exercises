// SPDX-License-Identifier: MIT
pragma solidity >=0.6.12 <0.9.0;

contract CampaignFactory {
  address[] public deployedCampaigns;

  function createCampaign(uint minimum) public {
    address newCampaign = address(new Campaign(minimum, msg.sender));
    deployedCampaigns.push(newCampaign);
  }

  function getDeployedCampaigns() public view returns (address[] memory) {
    return deployedCampaigns;
  }
}

contract Campaign {
  struct Request {
    string description;
    uint value;
    address payable recipient;
    bool complete;
    uint approvalCount;
  }

  address public manager;
  uint public minimumContribution;
  uint private approversCount;
  mapping(address => bool) public approvers;
  mapping(uint => mapping(address => bool)) approvals;
  Request[] public requests;

  modifier restricted() {
    require(msg.sender == manager);
    _;
  }

  constructor(uint minimum, address creator) {
    manager = creator;
    minimumContribution = minimum;
  }

  function contribute() public payable {
    require(msg.value > minimumContribution);
    approvers[msg.sender] = true;
    approversCount++;
  }

  function createRequest(string memory description, uint value, address payable recipient) public restricted {
    Request memory newRequest = Request({
      description: description,
      value: value,
      recipient: recipient,
      complete: false,
      approvalCount: 0
    });

    requests.push(newRequest);
  }

  function approveRequest(uint index) public {
    Request storage request = requests[index];

    require(approvers[msg.sender]);
    require(!approvals[index][msg.sender]);
    approvals[index][msg.sender] = true;
    request.approvalCount++;
  }

  function finalizeRequest(uint index) public restricted {
    Request memory request = requests[index];
    require(!request.complete);
    require(request.approvalCount > (approversCount / 2));
    request.complete = true;
    request.recipient.transfer(request.value);
  }

  function getSummary() public view returns (
    uint, uint, uint, uint, address
  ) {
    return (
      minimumContribution,
      address(this).balance,
      requests.length,
      approversCount,
      manager
    );
  }
  
  function getRequestsCount() public view returns (uint) {
    return requests.length;
  }
}
