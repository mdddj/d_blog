import React, { useEffect } from 'react';
import { useForm } from 'react-hook-form';
import { usePostCategory } from '@/providers/usePostCategory';
import { defaultToastMsg, MyDialog } from '@/utils/core';
import { Dialog, DialogBody, DialogCloseBtn, DialogTitle } from '@/components/dialog';
import AddPostCategoryForm from '@/components/form/AddPostCategoryForm';
import { useShallow } from 'zustand/react/shallow';
import axiosInstance from '@/utils/request';
import toast from 'react-hot-toast';

interface PostAddRequest {
  title: string,
  content: string,
  category_id: number,
  category: string
}
type Param = Omit<PostAddRequest, 'category'>

//添加博客页面
const PostAddPage: React.FC = () => {
  let cDialog = new MyDialog('add-category-form')
  const { register,handleSubmit } = useForm<PostAddRequest>({
    defaultValues: {},
  });
  const [list, fetch, add] = usePostCategory(useShallow(state => [state.data, state.fetch, state.add]));

  useEffect(() => {
    fetch().then();
  }, [fetch]);


  const onSubmit = async (data: PostAddRequest) => {
    console.log("request data",data)
    data.category_id = parseInt(data.category)
    let newData = data as Param
    const response = await toast.promise(axiosInstance.post("/api/post", newData),defaultToastMsg())
    console.log(response)
  }



  return <div className={'flex flex-col gap-4'}>
    <h4 className={'font-bold text-2xl'}>添加博客</h4>
    <form onSubmit={handleSubmit(onSubmit)} className={'flex flex-col gap-8'}>
      <input className={'input input-bordered'} placeholder={'标题'} {...register('title')} />
      <textarea rows={5} className={'textarea textarea-bordered'}
                placeholder={'输入正文内容'} {...register('content')} />
      <button type={'button'} onClick={cDialog.show}>添加分类
      </button>
      <select multiple={false} disabled={list.length === 0}
              className={'select select-bordered'} {...register('category')}>
        {
          list.length !== 0 && list.map(value => <option value={value.id}
                                                         key={value.id}>{value.name}</option>)
        }
      </select>
      <button className={'btn btn-primary'} type={'submit'}>提交博客</button>
    </form>
    <Dialog id={'add-category-form'}>
      <DialogBody>
        <DialogTitle title={'添加分类'}></DialogTitle>
        <AddPostCategoryForm onSuccess={(n) => {
          cDialog.close()
          add(n);
        }} onError={cDialog.close} />
      </DialogBody>
      <DialogCloseBtn />
    </Dialog>
  </div>;
};

export default PostAddPage;