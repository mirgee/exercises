import React, { useEffect, useState } from "react";
import { Button, Form, Input, Label, Message } from "semantic-ui-react";
import Campaign from '../ethereum/campaign';
import web3 from "../ethereum/web3";
import { useRouter } from "next/router";

export default (props) => {
  const { address } = props;

  const router = useRouter();
  const [value, setValue] = useState(0);
  const [errorMessage, setErrorMessage] = useState('');
  const [loading, setLoading] = useState(false);

  const onSubmit = async (event) => {
    event.preventDefault();

    const campaign = Campaign(address);

    setLoading(true);
    setErrorMessage('');

    try {
      const accounts = await web3.eth.getAccounts();
      await campaign.methods.contribute().send({
        from: accounts[0],
        value: web3.utils.toWei(value, 'ether')
      });

      router.replace(`/campaigns/${address}`);
    } catch (err) {
      console.error(err);
      setErrorMessage(err.message);
    } finally {
      setLoading(false);
    }
  }

  return (
    <Form onSubmit={onSubmit} error={!!errorMessage}>
      <Form.Field>
        <label>Amount to Contribute</label>
        <Input label="ether" labelPosition="right" value={value}
          onChange={event => { setValue(event.target.value) }}
        />
      </Form.Field>
      <Message error header="Oops!" content={errorMessage} /> 
      <Button primary loading={loading}>Contribute!</Button>
    </Form>
  )
}
