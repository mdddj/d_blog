import React, { createRef, useState } from 'react';
import useAxios from 'axios-hooks';
import { Loading } from '@/components/loading';
import { Dialog, DialogActions, DialogBody, DialogCloseBtn, DialogTitle } from '@/components/dialog';
import { closeDialogById, defaultToastMsg, showDialogById } from '@/utils/core';
import { ApiResponse } from '@/models/response';
import { Permission } from '@/pages/Permission/model';
import toast from 'react-hot-toast';
import RoleTableList from './list';

export default function Page() {


  const [{ data: permissionResult }] = useAxios<ApiResponse<Permission[]>>({ url: '/api/permission' });
  const [{ loading: addLoading }, doPostAdd] = useAxios<ApiResponse<number>>({
    url: '/api/permission_role', method: 'POST',
  }, { manual: true });//添加权限
  const inputRef = createRef<HTMLInputElement>();
  const [selectPermission, setSelectPermission] = useState<Permission[]>([]);

  let pList: Permission[] = permissionResult?.data ?? [];


  //显示添加角色弹窗
  const showAddDialog = () => {
    showDialogById('add-role-dialog');
  };


  const clickPermissionItem = (item: Permission) => {
    let find: Permission | undefined = selectPermission.find(value => value.id == item.id);
    if (find) {
      setSelectPermission(
        [...selectPermission.filter(value => value !== find)],
      );
    } else {
      setSelectPermission([...selectPermission, item]);
    }
  };

  const fetchAddRole = async () => {
    let name = inputRef.current && inputRef.current.value;
    if (name) {
      let result = await toast.promise(doPostAdd({
        data: {
          name,
          permission_ids: [
            ...selectPermission.map(value => value.id),
          ],
        },
      }), defaultToastMsg());
      if (result.data.code === 0) {
        closeDialogById('add-role-dialog');
      }
    }
  };


  return (
    <div>
      <button onClick={showAddDialog} className={'btn btn-primary'}>添加角色</button>

      <RoleTableList />




      <Dialog id={'add-role-dialog'}>
        <DialogBody>
          <DialogTitle title={'添加角色'} />
          <input ref={inputRef} className={'input input-bordered'} placeholder={'角色名称'} />
          <div>选择权限</div>
          <div className={'flex flex-wrap gap-2'}>
            {
              pList.map(value => <div onClick={() => clickPermissionItem(value)}
                                      className={`badge ${selectPermission.find(value1 => value1.id == value.id) ? 'badge-neutral' : 'badge-outline'}`}
                                      key={value.id}>
                {value.name} - {value.permission_url}
              </div>)
            }
          </div>
          <DialogActions>
            <button disabled={addLoading} onClick={fetchAddRole} className={'btn'} type={'submit'}>
              {
                addLoading && <span className="loading loading-spinner"></span>
              }
              添加角色
            </button>
          </DialogActions>
        </DialogBody>
        <DialogCloseBtn />
      </Dialog>
    </div>
  );
}
