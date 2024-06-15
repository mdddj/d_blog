export let MOOSE_REACT_LEARN_ACCESS_TOKEN = 'token';
// 保存
export const saveAccessToken = (token: string) => {
  localStorage.setItem(MOOSE_REACT_LEARN_ACCESS_TOKEN, token);
};

// 获取
export const getAccessToken = (): string | null => {
  return localStorage.getItem(MOOSE_REACT_LEARN_ACCESS_TOKEN);
};

// 移除
export const removeAccessToken = () => {
  localStorage.removeItem(MOOSE_REACT_LEARN_ACCESS_TOKEN);
};
// authorization
// 封装参数放到请求头中
export const getAuthorization = (): {} => {
  let accessToken = getAccessToken();
  return accessToken == null || accessToken === ''
    ? {}
    : {
      'authorization': 'Bearer ' + accessToken,
    };
};