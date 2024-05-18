import React from 'react';
import { ApiResponse } from '@/models/response';
import { AddOrUpdateForm } from '@/pages/User/add';
import useAxios from 'axios-hooks';

interface PropValue {
  id: string,
  username: string,
}

const UserListTable: React.FC = () => {

  const [{data,loading}] = useAxios<ApiResponse<PropValue[]>>({url: '/api/users',method: 'GET'});
  const [{},execDelete] = useAxios({method: 'DELETE'},{manual: true});
  return <>
    <div></div>
  </>;
};

export default UserListTable;