import React from 'react';
import { useForm } from 'react-hook-form';
import toast from 'react-hot-toast';
import { defaultToastMsg } from '@/utils/core';
import axiosInstance from '@/utils/request';
import { ApiResponse } from '@/models/response';
import { PostCategory } from '@/models/post';
type FormProps = {
  name: string
}
///添加博客分类表单
const AddPostCategoryForm: React.FC<{
  onSuccess?: (newCategory: PostCategory)=>void,
  onError?: ()=>void
}> = (props) => {

  const {register,handleSubmit} = useForm<FormProps>()

  const onSubmit = async (data: FormProps) => {
    await toast.promise(postData(data), defaultToastMsg())
  }

  const postData = async (data: FormProps) => {
    const response = await axiosInstance.post<ApiResponse<PostCategory>>("/api/category",{"name": data.name})
    if(response.status === 200){
      props.onSuccess?.(response.data.data)
    }else{
      props.onError?.()
    }
  }

  return <form className={'flex flex-col gap-5'} onSubmit={handleSubmit(onSubmit)}>
    <input {...register("name")} className={'input input-bordered'} placeholder={'输入分类名称'} />
    <button type={'submit'} className={'btn'}>提交</button>
  </form>
}
export default AddPostCategoryForm