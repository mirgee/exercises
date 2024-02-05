import React, { useEffect, useState } from "react";
import Layout from "../../components/Layout";
import { Button, Form, Input, Label, Message } from "semantic-ui-react";
import factory from '../../ethereum/factory';
import web3 from "../../ethereum/web3";
import { useRouter } from "next/router";

export default function CampaignNew () {
  const [minimumContribution, setMinimumContribution] = useState(0);
  const [errorMessage, setErrorMessage] = useState('');
  const [loading, setLoading] = useState(false);
  const router = useRouter();

  const onSubmit = async (event) => {
    event.preventDefault();

    setLoading(true);
    
    try {
      const accounts = await web3.eth.getAccounts();
      await factory.methods.createCampaign(minimumContribution).send({
        from: accounts[0]
      })

      router.push('/');
    } catch (err) {
      console.error(err);
      setErrorMessage(err.message);
    } finally {
      setLoading(false);
    }
  }
  
  return (
    <Layout>
      <h1>New Campaign</h1>
      <Form onSubmit={onSubmit} error={!!errorMessage}>
        <Form.Field>
          <label>Minimum Contribution</label>
          <Input label="wei" labelPosition="right" value={minimumContribution}
            onChange={event => { setMinimumContribution(event.target.value) }}
          />
        </Form.Field>
        <Message error header="Oops!" content={errorMessage} /> 
        <Button primary loading={loading}>Create</Button>
      </Form>
    </Layout>
  )
}
