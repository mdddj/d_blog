// 运行时配置

// 全局初始化数据配置，用于 Layout 用户信息和权限初始化
// 更多信息见文档：https://umijs.org/docs/api/runtime-config#getinitialstate
import { AxiosError, RequestConfig, RequestError } from '@@/plugin-request/request';

export async function getInitialState(): Promise<{ name: string }> {
  return { name: '@umijs/max' };
}

export const layout = () => {
  return {
    logo: 'https://img.alicdn.com/tfs/TB1YHEpwUT1gK0jSZFhXXaAtVXa-28-27.svg',
    menu: {
      locale: false,
    },
  };
};

export const request: RequestConfig = {
  timeout: 3000,
  baseURL: 'http://127.0.0.1:5800',
  errorConfig: {
    errorHandler(err: RequestError, opt) {
      console.error(`error:${err}`)
      if (err.name === "AxiosError") {
        let status = (err as AxiosError).response?.status
        if (status === 401) {
          window.location.href = '/user/login'
        }

      }
    },
    errorThrower() {
    }
  },
  requestInterceptors: [],
  responseInterceptors: []
};