import Guide from '@/components/Guide';
import { trim } from '@/utils/format';
import { PageContainer } from '@ant-design/pro-components';
import { useModel } from '@umijs/max';
import styles from './index.less';
import { Button } from 'antd';
import React from 'react';

const HomePage: React.FC = () => {
  const { name } = useModel('global');
  return (
    <PageContainer ghost>
      <div className={styles.container}>
        <Guide name={trim(name)} />
        <Button onClick={()=>{
          window.location.href = "/user/login";
        }}>登录</Button>
      </div>
    </PageContainer>
  );
};

export default HomePage;
