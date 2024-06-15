import React from 'react';
import useAxios from 'axios-hooks';
import { Loading } from '@/components/loading';
import { Link } from 'umi';
///博客页面
const PostListPage: React.FC = () => {

  const [{data,loading,error}] = useAxios("/api/post")
  console.log(data)
  if(loading) return <Loading />
  if(!data || error) return <span>load data failed</span>
  return <div>
    <div>
      <Link className={'btn btn-primary'} to={'/post/add'}>添加博客22</Link>
    </div>
  </div>
}
export default PostListPage