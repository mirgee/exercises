import { Button, Card, Table } from "semantic-ui-react";
import Layout from "../../../components/Layout";
import Campaign from "../../../ethereum/campaign";
import Link from "next/link";
import RequestRow from "../../../components/RequestRow";

export async function getServerSideProps(context) {
  const { address } = context.params;
  const campaign = Campaign(address);
  let requests = await campaign.methods.getRequests().call()
  const approversCount = await campaign.methods.approversCount().call();
  requests = requests.map(request => {;
    const { description, value, recipient, complete, approvalCount } = request;
    return {
      description: description.toString(),
      value: value.toString(),
      recipient: recipient.toString(),
      complete: complete,
      approvalCount: approvalCount.toString(),
    };
  });
  return {
    props: {
      address,
      requests,
      approversCount: approversCount.toString()
    }
  }
}

function renderRequestRows (props) {
  const { requests, approversCount, address } = props;

  return requests.map((request, index) => {
    return <RequestRow key={index} id={index} request={request} approversCount={approversCount} address={address}/>;
  });
}

function renderRequests (props) {
  const { Header, Row, HeaderCell, Body } = Table;
  const items = props.requests.map(request => {
    const { description, value, recipient, complete, approvalCount } = request;
    return {
      header: recipient,
      description,
      fluid: true
    };
  });
  return (
    <Table>
      <Header>
        <Row>
          <HeaderCell>ID</HeaderCell>
          <HeaderCell>Description</HeaderCell>
          <HeaderCell>Amount</HeaderCell>
          <HeaderCell>Recipient</HeaderCell>
          <HeaderCell>Approval Count</HeaderCell>
          <HeaderCell>Approve</HeaderCell>
          <HeaderCell>Finalize</HeaderCell>
        </Row>
      </Header>
      <Body>
        {renderRequestRows(props)}
      </Body>
    </Table>
  );
}

const CampaignRequests = (props) => {
  const { address } = props
  return (
    <Layout>
      <h3>Requests</h3>
      <Link href={`/campaigns/${address}/requests/new`}>
        <Button primary floated="right" style={{ marginBottom: 10 }}>
          Create Request
        </Button>
      </Link>
      {renderRequests(props)}
    </Layout>
  );
};

export default CampaignRequests;
