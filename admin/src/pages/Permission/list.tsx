import React from 'react';
import useAxios from 'axios-hooks';
import { ApiResponse } from '@/models/response';
import { Permission } from '@/pages/Permission/model';


const PermissionListTable: React.FC = () => {

  const [{data,loading}] = useAxios<ApiResponse<Permission[]>>({url: "/api/permission", method: 'GET'});

  return <table className={'table table-striped'}>
    <thead>

    <tr>
      <th>Id</th>
      <th>名称</th>
      <th>URL</th>
      <th>类型</th>
      <th>方法</th>
      <th>分组</th>
      <th>备注</th>
      <th>操作</th>
    </tr>
    </thead>
    <tbody>
    {
      data?.data?.map((item: Permission) => {
        return <tr key={item.id}>
          <th>{item.id}</th>
          <th>{item.name}</th>
          <th>{item.permission_url}</th>
          <th>{item.type}</th>
          <th>{item.method}</th>
          <th>{item.group}</th>
          <th>{item.description}</th>
          <th>
            <div className={'inline-flex gap-2'}>
              <button className={'btn btn-error btn-sm'}>删除</button>
            </div>
          </th>
        </tr>
      })
    }
    </tbody>
  </table>
}
export default PermissionListTable;