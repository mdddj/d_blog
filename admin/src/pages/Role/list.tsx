import useAxios from 'axios-hooks';
import { Loading } from '@/components/loading';
import React from 'react';
import { Role } from '@/models/role';
import { ApiResponse } from '@/models/response';


const RoleTableList : React.FC = () => {
  const [{ data, loading }] = useAxios<ApiResponse<Role[]>>({ url: '/api/permission_role' });

  if(loading){
    return <Loading />
  }

  return <>
    <table className={'table table-striped'}>
      <thead>
      <tr>
        <th>Id</th>
        <th>名称</th>
      </tr>
      </thead>
      <tbody>
      {
        data?.data?.map((item: Role) => {
          return <tr key={item.id}>
            <td>{item.id}</td>
            <td>{item.name}</td>
          </tr>
        })
      }
      </tbody>
    </table>
  </>
}

export default RoleTableList