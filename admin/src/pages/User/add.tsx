import React, { JSX } from 'react';
import useAxios from 'axios-hooks';

type Prop = {
  trigger?: JSX.Element | undefined,
  initValues?: any | undefined
}

interface PropInitValue {
  username: string,
  password: string,
}





const AddOrUpdateForm: React.FC<Prop> = (props) => {


  const [_,execPost] = useAxios({url:'/api/users/',method: 'POST'})

  let isUpdate = props.initValues !== undefined;

  //提交数据
  const onFinish = async (values: PropInitValue) => {
    if (isUpdate) {
      // Todo! 修改
    } else {
      // ToDo! 新增
      let response = await execPost({data: values})
      console.log(response)
    }
    return true;
  };

  return (
    <div></div>
  );
};
export {
  AddOrUpdateForm,
};
