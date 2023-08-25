import React from 'react';
import { Form, Input, Button } from 'antd';

const FormField = (props) => {

  return (
    <Form onFinish={props.onFinish}>
      <Form.Item
        name={props.formName}
        rules={[
        ]}
      >
        <Input.Password placeholder={props.placeholder} />
      </Form.Item>
      <Form.Item>
        <Button className="h-8" type="default" htmlType="submit">
          Save
        </Button>
      </Form.Item>
    </Form>
  );
};

export default FormField;
