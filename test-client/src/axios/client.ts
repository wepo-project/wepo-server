import axios, { Method, AxiosPromise, AxiosRequestConfig } from "axios";
import JsonBigInt from "json-bigint"

interface NetClient {
  send(method: Method, model: string, func: string, config?: AxiosRequestConfig): AxiosPromise
  post(model: string, func: string, config?: AxiosRequestConfig): AxiosPromise
  get(model: string, func: string, config?: AxiosRequestConfig): AxiosPromise
  isLogined(): boolean
  loginWithAccount(nick: string, pwd: string): Promise<boolean>
  loginWithToken(): Promise<boolean>
}

const client: NetClient = {} as NetClient;

const Authorization = "Authorization";

const axiosInstance = axios.create({
  baseURL: "http://127.0.0.1:8080/v1",
});

const fontStyle = {
  request: 'color:orange;font-size:10px;',
  response: 'color:blue;font-size:10px;'
}

axiosInstance.interceptors.request.use((config) => {
  // config.transformRequest = [data => data]
  console.log(`%c${(new Date()).toLocaleString()} [${config.method}]%o`, fontStyle.request, config.url, config.method == "get" ? config.params : config.data);
  return config;
}, (err) => {
  return Promise.reject(err)
})

axiosInstance.interceptors.response.use((resp) => {
  console.log(`%c${(new Date()).toLocaleString()} [${resp.config.method}(${resp.status})]%o`, fontStyle.response, resp.config.url, resp.data)
  return resp;
}, (err) => {
  // console.log(err);
  if (err.response && err.response.status == 401) {
    console.log("登录失效")
  }
  return Promise.reject(err)
})

// 转换big int
axiosInstance.defaults.transformResponse = [data => {
  if (data) {
    try {
      return JsonBigInt.parse(data)
    } catch(e) {
      console.error(e)
      return data
    }
  }
  return data
}]

// axiosInstance.defaults.transformRequest = [(data, headers) => {
//   console.log(typeof data?.origin);
//   return JSON.stringify(data)
// }]

export default client;

const getSavedToken = () => localStorage.getItem('_t');

const saveToken = (token: string): boolean => {
  if (typeof token !== 'string') {
    console.error("Setting token failed!", token);
    return false;
  }
  axiosInstance.defaults.headers.common[Authorization] = token;
  localStorage.setItem('_t', token);
  console.log("登录成功");
  return true;
}

/**
 * 发送命令
 * @param method 
 * @param model 
 * @param func 
 * @param config 
 * @returns 
 */
client.send = (method: Method, model: string, func: string, config?: AxiosRequestConfig) => {
  return axiosInstance({
    ...config,
    method: method,
    url: `/${model}/${func}`,
  });
}

client.get = client.send.bind(client, 'GET');
client.post = client.send.bind(client, 'POST');

/**
 * 
 * @returns 是否登录
 */
client.isLogined = () => axiosInstance.defaults.headers.common[Authorization] != null;

/**
 * 账号登录
 * @param nick 
 * @param pwd 
 */
client.loginWithAccount = async (nick: string, pwd: string): Promise<boolean> => {
  const resp = await client.post('user', 'login', {
    data: { nick, pwd }
  });
  return saveToken(resp.data["token"]);
}

/**
 * token登录
 */
client.loginWithToken = async (): Promise<boolean> => {
  const token = getSavedToken();
  if (token) {
    const resp = await client.get('token', 'login', {
      headers: {
        [Authorization]: token
      }
    });
    return saveToken(resp.data);
  }
  return false;
}

(window as any).client = client;