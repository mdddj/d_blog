import React from 'react';
import { ApiResponse } from '@/models/response';
import useAxios from 'axios-hooks';
import { User } from '@/models/user';
import { Loading } from '@/components/loading';
import { MyUserSerRoleDialog } from '@/pages/User/set_role_dialog';
import { MyDialog, showDialogById } from '@/utils/core';
import { Role } from '@/models/role';
import axiosInstance from '@/utils/request';
import { AddOrUpdateForm } from '@/pages/User/add';
import { SimpleDialog } from '@/components/dialog';


const UserListTable: React.FC = () => {
  const [{ data: roles, loading: rolesLoading }] = useAxios<ApiResponse<Role[]>>({ url: '/api/permission_role' });
  const [{ data, loading }] = useAxios<ApiResponse<User[]>>({ url: '/api/users', method: 'GET' });
  const myDialog = new MyDialog('add-user-form')
  const [{}, execDelete] = useAxios({ method: 'DELETE' }, { manual: true });


  //查询用户角色
  const selectRole = async  (id: string) => {
    const r =  await axiosInstance.get(`/api/user_role/${id}`)
    console.log(r)
  }

  if(loading){
    return <Loading />
  }
  return <>
    <div>
      <button className={'btn btn-primary'} onClick={myDialog.show}>添加用户</button>
    </div>
    <table className={'table table-striped'}>
      <thead>
      <tr>
        <th>Id</th>
        <th>Username</th>
        <th>头像</th>
        <th>昵称</th>
        <th>操作</th>
      </tr>
      </thead>
      <tbody>
      {
        data?.data?.map((item: User) => {
          return <tr key={item.id}>
            <td>{item.id}</td>
            <td>{item.username}</td>
            <td>{item.avatar !== '' && <img alt={item.avatar} src={item.avatar} className={'avatar rounded-full size-8'} />}</td>
            <td>{item.nick_name ?? '-'}</td>
            <td className={'flex flex-row gap-2'}>
              {
                !rolesLoading && roles?.data && roles.data.length !== 0 && <MyUserSerRoleDialog userId={item.id} roles={roles?.data??[]}>
                  <button className={'btn btn-sm btn-primary'} onClick={()=>showDialogById('my-set-role-dialog')}>角色</button>
                </MyUserSerRoleDialog>
              }
              <button className={'btn btn-sm'} onClick={()=>selectRole(item.id)}>查询</button>
              <button className={'btn btn-sm btn-warning'}>删除</button>
            </td>
          </tr>;
        })
      }
      </tbody>
    </table>
    <SimpleDialog id={'add-user-form'} title={'添加用户'}>
      <AddOrUpdateForm />
    </SimpleDialog>

  </>;
};

export default UserListTable;