import Campaign from "../ethereum/campaign";
import { Button, Table } from "semantic-ui-react";
import web3 from "../ethereum/web3";

const RequestRow = (props) => {
  const { Row, Cell } = Table;
  const { address, id } = props;
  const readyToFinalize = props.request.approvalCount > props.approversCount / 2;

  const onApprove = async () => {
    try {
      const campaign = Campaign(address);
      const accounts = await web3.eth.getAccounts();
      await campaign.methods.approveRequest(id).send({
        from: accounts[0]
      });
    } catch (err) {
      console.error(err);
    }
  };

  const onFinalize = async () => {
    try {
      const campaign = Campaign(address);
      const accounts = await web3.eth.getAccounts();
      await campaign.methods.finalizeRequest(id).send({
        from: accounts[0]
      });
    } catch (err) {
      console.error(err);
    }
  };

  console.log(props.request);

  return (
    <Row disabled={props.request.complete} positive={readyToFinalize && !props.request.complete}>
      <Cell>{props.id}</Cell>
      <Cell>{props.request.description}</Cell>
      <Cell>{props.request.value}</Cell>
      <Cell>{props.request.recipient}</Cell>
      <Cell>{props.request.approvalCount}/{props.approversCount}</Cell>
      <Cell>
        { props.request.complete ? null : (
            <Button color="green" basic onClick={onApprove}>Approve</Button>
          )
        }
      </Cell>
      <Cell>
        { props.request.complete ? null : (
            <Button color="green" basic onClick={onFinalize}>Finalize</Button>
          )
        }
      </Cell>
    </Row>
  );
};

export default RequestRow;
