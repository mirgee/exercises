import { Button, Form, Input, Message } from "semantic-ui-react";
import Layout from "../../../../components/Layout";
import { useState } from "react";
import Campaign from "../../../../ethereum/campaign";
import web3 from "../../../../ethereum/web3";
import { useRouter } from "next/router";
import Link from "next/link";

export async function getServerSideProps(context) {
  const { address } = context.params;
  return {
    props: {
      address
    }
  }
}

const CreateRequest = (props) => {
  const { address } = props

  const router = useRouter();
  const [value, setValue] = useState(0);
  const [description, setDescription] = useState('');
  const [recipient, setRecipient] = useState('');
  const [errorMessage, setErrorMessage] = useState('');
  const [loading, setLoading] = useState(false);

  const onSubmit = async (event) => {
    event.preventDefault();

    setLoading(true);
    setErrorMessage('');
    
    try {
      const accounts = await web3.eth.getAccounts();
      const campaign = Campaign(address);
      await campaign.methods.createRequest(description, value, recipient).send({
        from: accounts[0]
      })

      router.push(`/campaigns/${address}/requests`);
    } catch (err) {
      console.error(err);
      setErrorMessage(err.message);
    } finally {
      setLoading(false);
    }
  }

  return (
    <Layout>
      <Link href={`/campaigns/${address}/requests`}>
        Back
      </Link>
      <h1>New Request</h1>
      <Form onSubmit={onSubmit} error={!!errorMessage}>
        <Form.Field>
          <label>Description</label>
          <Input value={description} onChange={event => { setDescription(event.target.value) }} />
        </Form.Field>
        <Form.Field>
          <label>Value</label>
          <Input label="wei" labelPosition="right" value={value} onChange={event => { setValue(event.target.value) }} />
        </Form.Field>
        <Form.Field>
          <label>Recipient</label>
          <Input value={recipient} onChange={event => { setRecipient(event.target.value) }} />
        </Form.Field>
        <Message error header="Oops!" content={errorMessage} /> 
        <Button primary loading={loading}>Create</Button>
      </Form>
    </Layout>
  );
};

export default CreateRequest;
