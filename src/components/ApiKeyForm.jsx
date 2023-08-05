import React from 'react';
import { Form, Input, Button } from 'antd';
import { invoke } from '@tauri-apps/api/tauri';

const ApiKeyForm = () => {
  const onFinish = (values) => {
    console.log('Received values:', values);
    invoke('save_todoist_api_key', { key: values.apiKey })
  };

  return (
    <Form onFinish={onFinish}>
      <Form.Item
        name="apiKey"
        rules={[
        ]}
      >
        <Input.Password placeholder="Enter Your Key" />
      </Form.Item>
      <Form.Item>
        <Button type="primary" htmlType="submit">
          Save
        </Button>
      </Form.Item>
    </Form>
  );
};

export default ApiKeyForm;
