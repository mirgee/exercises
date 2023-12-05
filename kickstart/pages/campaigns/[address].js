import { useRouter } from 'next/router';
import Layout from '../../components/Layout';
import Campaign from '../../ethereum/campaign';
import { Button, Card, Grid } from 'semantic-ui-react';
import ContributeForm from '../../components/ContributeForm';
import Link from 'next/link';

export async function getServerSideProps(context) {
  const { address } = context.params;
  const campaign = Campaign(address);
  const summary = await campaign.methods.getSummary().call();
  return {
    props: {
      minimumContribution: summary[0].toString(),
      balance: summary[1].toString(),
      requestsCount: summary[2].toString(),
      approversCount: summary[3].toString(),
      manager: summary[4]
    }
  }
}

const CampaignDetails = (props) => {
  const router = useRouter();
  const { address } = router.query;

  const renderCards = () => {
    const {
      minimumContribution,
      balance,
      requestsCount,
      approversCount,
      manager
    } = props;

    const items = [
      {
        header: manager,
        meta: 'Address of Manager',
        description: 'The manager created this campaign and can create requests to withdraw money',
        style: { overflowWrap: 'break-word' }
      },
      {
        header: minimumContribution,
        meta: 'Minimum Contribution (ether)',
        description: 'You must contribute at least this much wei to become an approver'
      },
      {
        header: requestsCount,
        meta: 'Number of Requests',
        description: 'A request tries to withdraw money from the contract. Requests must be approved by approvers'
      },
      {
        header: approversCount,
        meta: 'Number of Approvers',
        description: 'Number of people who have already donated to this campaign'
      },
      {
        header: balance,
        meta: 'Campaign Balance (wei)',
        description: 'The balance is how much money this campaign has left to spend'
      }
    ];

    return <Card.Group items={items} />;
  }

  return (
    <Layout>
      <h1>Campaign Details</h1>
      <p>Campaign Address: {address}</p>
      <Grid>
        <Grid.Row>
          <Grid.Column width={10}>
            {renderCards()}
          </Grid.Column>
          <Grid.Column width={6}>
            <ContributeForm address={address}/>
          </Grid.Column>
        </Grid.Row>
        <Grid.Row>
          <Grid.Column>
            <Link href={`/campaigns/${address}/requests`}>
              <Button primary>
                View Requests
              </Button>
            </Link>
          </Grid.Column>
        </Grid.Row>
      </Grid>
    </Layout>
  );
};

export default CampaignDetails;
