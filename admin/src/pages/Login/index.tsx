import { PageContainer } from '@ant-design/pro-components';
import { request } from '@umijs/max';
import { Button, Form, Input } from 'antd';


let testUser = {
  username: "admin",
  password: "123456"
}

export default function Page() {
  async function onFinish(values: any) {
    let response = await request('/api/login', {
      data: values,
      method: 'POST',
    });
    console.log(response);
  }

  return (
    <PageContainer title={'登录'}>
      <Form onFinish={onFinish} initialValues={testUser}>
        <Form.Item name={'username'} label={'用户名'}>
          <Input />
        </Form.Item>
        <Form.Item name={'password'} label={'密码'}>
          <Input />
        </Form.Item>
        <Form.Item>
          <Button htmlType="submit" type={'primary'}>
            登录
          </Button>
        </Form.Item>
      </Form>
    </PageContainer>
  );
}
