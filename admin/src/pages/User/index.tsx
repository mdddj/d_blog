import React from 'react';
import { ModalForm, PageContainer, ProFormText } from '@ant-design/pro-components';
import { Button } from 'antd';
import { request } from '@@/exports';

export default function Page() {
  return (
    <PageContainer title={'用户管理'} extra={[<AddUserForm key={'add'} />]}>

    </PageContainer>
  );
}


const AddUserForm: React.FC = () => {


  let onFinish = async (values: any) => {
    await request("/api/users",{
      data: values,
      method: "POST"
    })
  };
  return <>
    <ModalForm onFinish={onFinish} trigger={<Button size={'small'} type={'primary'}>添加用户</Button>} initialValues={{username:"username",password: "password"}}>
      <ProFormText label={'用户名'} name={'username'} />
      <ProFormText label={'密码'} name={'password'} />
    </ModalForm>
  </>
}