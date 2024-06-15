import { Permission } from './model';
import useAxios from 'axios-hooks';
import React, { useEffect } from 'react';
import { Dialog, DialogActions, DialogBody, DialogCloseBtn, DialogTitle } from '@/components/dialog';
import {
  Controller, useForm,
} from 'react-hook-form';
import { get_input_class, get_textarea_class, InputWrapper } from '@/components/form';
import { closeDialogById, defaultToastMsg, showDialogById } from '@/utils/core';
import { ButtonLoading } from '@/components/button_loading';
import toast from 'react-hot-toast';


type Prop = {
  initValues?: Permission | undefined;
  onCancel?: () => void;
};


const AddOrUpdateFormByPermission: React.FC<Prop> = (props) => {
  const { register, handleSubmit, reset, control, watch, formState: { isLoading } } = useForm<Permission>({
    defaultValues: {
      type: 'URL',
      ...props.initValues,
    },
  });

  let isUpdate = props.initValues !== undefined;
  const [_, execPost] = useAxios({ url: '/api/permission', method: 'POST' }, { manual: true });
  const [__, execPut] = useAxios({ method: 'PUT' }, { manual: true });

  //添加
  async function apiPermissionAdd(data: Permission): Promise<any> {
    return execPost({ data });
  }

  //修改
  async function apiPermissionUpdate(id: number, data: Permission): Promise<any> {
    return execPut({ url: `/api/permission/${id}`, data });
  }


  //提交数据
  const onFinish = async (values: Permission) => {
    let model = props.initValues;
    if (isUpdate && model && model.id) {
      await toast.promise(apiPermissionUpdate(model.id, values), defaultToastMsg(closeDialog))
    } else {
      await toast.promise(apiPermissionAdd(values),defaultToastMsg(closeDialog));
    }
  };

  const type = watch('type');


  const closeDialog = () => {
    reset()
    closeDialogById('permission-add-dialog');
  }

  let getUrlLabel = (): string => {
    switch (type) {
      case 'URL' : {
        return 'URL';
      }
      case 'PAGE': {
        return '输入页面URL';
      }
      case 'REGEX' : {
        return '输入正则表达式';
      }
    }
  };

  useEffect(() => {
    if (props.initValues) {
      showDialogById('permission-add-dialog');
    }
  }, [props.initValues]);

  return (
    <Dialog id={'permission-add-dialog'} onClose={() => {
      reset();
      props.onCancel?.();
    }}>
      <DialogBody>
        <DialogTitle title={'添加权限'} />
        <form className={'flex flex-col gap-4'} onSubmit={handleSubmit(onFinish)}>
          <Controller render={function({ field, fieldState: { error } }) {
            return <InputWrapper label={'权限名称'} bottomLeftLabel={error?.message}>
              <input  {...field} {...register('name')} className={get_input_class(error?.message)}
                      placeholder={'权限名称'} />
            </InputWrapper>;
          }} name={'name'} control={control} rules={{ required: '请输入内容' }} />
          <Controller render={function({ field, fieldState: { error } }) {
            return <InputWrapper label={getUrlLabel()} bottomLeftLabel={error?.message}>
              <input {...field} {...register('permission_url')} className={get_input_class(error?.message)}
                     placeholder={getUrlLabel()} />
            </InputWrapper>;
          }} name={'permission_url'} control={control} rules={{ required: '请输入内容' }} />
          <Controller defaultValue={'URL'} render={function({ field }) {
            return <InputWrapper label={'类型'}>
              <div className={'flex gap-4 justify-between'}>
                <div className={'inline'}>
                  <input type="radio" {...field} className="radio mr-2" value={'URL'} placeholder={'URL'}
                         checked={field.value == 'URL'} />
                  <span>URL</span>
                </div>
                <div className={'inline'}>
                  <input type="radio" {...field} className="radio mr-2" value={'REGEX'} placeholder={'URL'}
                         checked={field.value == 'REGEX'} />
                  <span>正则</span>
                </div>
                <div className={'inline'}>
                  <input type="radio" {...field} className="radio mr-2" value={'PAGE'} placeholder={'PAGE'}
                         checked={field.value == 'PAGE'} />
                  <span>PAGE</span>
                </div>
              </div>
            </InputWrapper>;
          }} name={'type'} control={control} />

          <Controller disabled={type === 'PAGE'} render={function({ field, fieldState: { error } }) {
            return <InputWrapper label={'方法'} bottomLeftLabel={error?.message}>
              <select  {...field} {...register('method')} className="select select-bordered w-full">
                <option value={''} disabled defaultChecked={true}>请选择Method</option>
                <option value={'GET'}>GET</option>
                <option value={'POST'}>POST</option>
                <option value={'PUT'}>PUT</option>
                <option value={'DELETE'}>DELETE</option>
                <option value={'OPTION'}>OPTION</option>
              </select>
            </InputWrapper>;
          }} name={'method'} control={control} />

          <Controller render={function({ field, fieldState: { error } }) {
            return <InputWrapper label={'分组标识'} bottomLeftLabel={error?.message}>
              <input type={'text'} {...field} {...register('group')} className={get_input_class(error?.message)}
                     placeholder={'分组标识'} />
            </InputWrapper>;
          }} name={'group'} control={control} />
          <Controller render={function({ field, fieldState: { error } }) {
            return <InputWrapper label={'备注'} bottomLeftLabel={error?.message}>
              <textarea {...field} {...register('description')} className={get_textarea_class(error?.message)}
                        placeholder={'备注'} />
            </InputWrapper>;
          }} name={'description'} control={control} rules={{ required: '请输入内容' }} />
          <DialogActions>
            <button type={'submit'} className={'btn btn-primary'}>
              {isLoading && <ButtonLoading />}
              {props.initValues ? '修改' : '新增'}</button>
          </DialogActions>
        </form>

      </DialogBody>
      <DialogCloseBtn />
    </Dialog>
  );
};
export { AddOrUpdateFormByPermission };
