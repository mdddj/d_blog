// 创建 Axios 实例
import Axios, { AxiosError, AxiosResponse } from 'axios';
import { getAccessToken,  } from '@/utils/token';

interface ResponseBase {
  code: number,
  msg: string
}

const axiosInstance = Axios.create({
  baseURL: "http://localhost:5800", // 你的 API 地址
  timeout: 5000, // 请求超时时间
});

// 响应拦截器
axiosInstance.interceptors.response.use(
  (response : AxiosResponse<ResponseBase>) => {
    console.log(response)
    const {data : {msg,code},status,statusText} = response
    if (status !== 200) {
      throw new BizException(msg)
    }
    return response;
  },
  (error: AxiosError<ResponseBase>) => {
    const response = error.response
    if(response){
      const {status,statusText,data: {code,msg}} = response
      if(status === 401 || status === 403) {
        window.location.href = "/user/login"
        return
      }
      if(status !== 200 && code !== 0) {
        let fullMsg =  `${statusText}:${msg}`
        return Promise.reject(fullMsg)
      }
    }
    return Promise.reject(error);
  }
);

// 添加请求头
axiosInstance.interceptors.request.use(value => {
  value.headers.Authorization = `Bearer ${getAccessToken() ?? ""}`
  return value
})

class BizException extends Error {
  constructor(message: string) {
    super(message);
    this.name = this.constructor.name
    Error.captureStackTrace(this, this.constructor)
  }

  toString(): string {
    return this.message
  }
}




export default axiosInstance;