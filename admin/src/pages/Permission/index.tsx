import { AddOrUpdateFormByPermission } from './add';
import { Permission } from './model';
import { ApiResponse } from '@/models/response';
import useAxios from 'axios-hooks';
import { showDialogById } from '@/utils/core';
import React from 'react';
import PermissionListTable from '@/pages/Permission/list';

export default function Page() {





  return (
    <div>


      <button className={'btn btn-primary'} onClick={()=>showDialogById('permission-add-dialog')}>添加权限</button>

      <PermissionListTable />

      <AddOrUpdateFormByPermission />


    </div>
  );
}
