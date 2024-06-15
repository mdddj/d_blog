import React from 'react';
import useAxios from 'axios-hooks';
import { Controller, useForm } from 'react-hook-form';
import toast from 'react-hot-toast';
import { defaultToastMsg } from '@/utils/core';

type Prop = {
  initValues?: UserAddRequest | undefined
}

///请求参数
interface UserAddRequest {
  username: string,
  password: string,
  avatar: string | undefined,
  nick_name: string | undefined,
}


const AddOrUpdateForm: React.FC<Prop> = (props) => {


  const {register,handleSubmit,control} =  useForm<UserAddRequest>({
    defaultValues: {
      avatar: "https://t.alcy.cc/tx/"
    }
  })
  const [_,execPost] = useAxios({url:'/api/users/'})

  //提交数据
  const submit = async (values: UserAddRequest) => {
    await toast.promise(execPost({ data: values,method: 'post' }), defaultToastMsg())
  }

  return (
    <form className={'my-form'} onSubmit={handleSubmit(submit)}>
      <input className={'my-input'} {...register("username")} required={true} placeholder={"用户名"} />
      <input className={'my-input'} {...register("password")} required={true} placeholder={'登录密码'} />
      <Controller name={'avatar'} control={control} render={function() {
        return <input className={'my-input'} {...register('avatar')} required={true} placeholder={'头像'} />;
      }} />
      <input className={'my-input'} {...register('nick_name')} required={true} placeholder={"昵称"} />
      <button type={'submit'} className={'submit-btn'}>提交</button>
    </form>
  );
};
export {
  AddOrUpdateForm,
};
