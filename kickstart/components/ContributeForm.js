import React, { useEffect, useState } from "react";
import Layout from "../../components/Layout";
import { Button, Form, Input, Label, Message } from "semantic-ui-react";
import factory from '../../ethereum/factory';
import web3 from "../../ethereum/web3";
import { useRouter } from "next/router";

export default () => {
  return (
    <Layout>
      <h1>New Campaign</h1>
      <Form onSubmit={onSubmit} error={errorMessage}>
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
