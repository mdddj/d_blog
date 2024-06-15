import { AddOrUpdateFormByPermission } from './add';
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
