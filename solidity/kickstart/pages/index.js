import React, { useEffect } from "react";
import { Card, Button } from 'semantic-ui-react';
import factory from "../ethereum/factory";
import Layout from '../components/Layout';
import Link from "next/link";

export async function getServerSideProps() {
  const campaigns = await factory.methods.getDeployedCampaigns().call();
  console.log(campaigns);

  return {
    props: {
      campaigns
    },
  };
}

function renderCampaigns (props) {
  const items = props.campaigns.map(address => {
    return {
      header: address,
      description: <Link href={`/campaigns/${address}`}>View Campaign</Link>,
      fluid: true
    };
  });

  return <Card.Group items={items}></Card.Group>;
}

export default function CampaignIndex (props) {
  return (
    <Layout>
      <div>
        <h3>Open Campaigns</h3>
        <Link href="/campaigns/new" className="item">
          <Button
            content="Create Campaign"
            icon="add circle"
            primary
            floated="right"
          />
        </Link>
        {renderCampaigns(props)}
      </div>
    </Layout>
  );
}
