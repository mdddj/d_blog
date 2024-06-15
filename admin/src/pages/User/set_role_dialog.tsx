import React, { PropsWithChildren } from 'react';
import {
  Controller, useForm,
} from 'react-hook-form';
import { Dialog, DialogActions, DialogBody, DialogCloseBtn, DialogTitle } from '@/components/dialog';
import { get_select_class, InputWrapper } from '@/components/form';
import useAxios from 'axios-hooks';
import { Role } from '@/models/role';
import toast from 'react-hot-toast';
import { closeDialogById, defaultToastMsg } from '@/utils/core';

interface UserRoleAddRequest {
  role_id: string,
}

type Prop = {
  userId: string,
  roles: Role[]
}


const MyUserSerRoleDialog: React.FC<PropsWithChildren<Prop>> = (props) => {

  const [{},execPostAdd] = useAxios({url: "/api/user_role",method: 'post'},{manual: true})

  const { register, handleSubmit, reset, control } = useForm<UserRoleAddRequest>({
    defaultValues: {
      role_id: `${props.roles[0].id}`
    },
  });
  const onFinish = async (values: UserRoleAddRequest) => {
    await toast.promise(execPostAdd({
      data: {
        user_id: props.userId,
        role_id: parseInt(values.role_id)
      },
    }),defaultToastMsg(()=>{
      closeDialogById('my-set-role-dialog');
    }));
  };
  return <>
    {props.children}
    <Dialog id={'my-set-role-dialog'} onClose={() => reset()}>
      <DialogBody>
        <DialogTitle title={'表单'} />
        <form onSubmit={handleSubmit(onFinish)}>
          <Controller render={function({ field, fieldState: { error } }) {
            return <InputWrapper label={'选择角色'} bottomLeftLabel={error?.message}>
              {
               <select {...field} {...register("role_id")}  className={`select select-bordered ${get_select_class(error?.message)}`}>
                  {
                    props.roles.map(value => {
                      return <option key={value.id} value={value.id}>{value.name}</option>
                    })
                  }
                </select>
              }
            </InputWrapper>;
          }} name={'role_id'} control={control} rules={{ required: '请选择角色' }} />
          <DialogActions>
            <button type={'submit'} className={'btn btn-primary'}>添加角色</button>
          </DialogActions>
        </form>
      </DialogBody>
      <DialogCloseBtn />
    </Dialog>
  </>
};
export { MyUserSerRoleDialog };