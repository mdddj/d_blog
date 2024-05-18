import React from 'react';

const HomePage: React.FC = () => {
  return (
    <div>
      <div >
        <button className={'btn'} onClick={()=>{
          window.location.href = "/user/login";
        }}>登录</button>
      </div>
    </div>
  );
};

export default HomePage;
