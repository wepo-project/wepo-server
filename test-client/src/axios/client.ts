import axios, { Method, AxiosPromise, AxiosRequestConfig, AxiosResponse } from "axios";
import JsonBigInt from "json-bigint"
import router from "../pageRouter";
import store from "../store";

interface NetClient {
  send(method: Method, model: string, func: string, config?: AxiosRequestConfig): AxiosPromise
  post(model: string, func: string, config?: AxiosRequestConfig): AxiosPromise
  get(model: string, func: string, config?: AxiosRequestConfig): AxiosPromise
  put(model: string, func: string, config?: AxiosRequestConfig): AxiosPromise
  delete(model: string, func: string, config?: AxiosRequestConfig): AxiosPromise
  isLogined(): boolean
  loginWithAccount(nick: string, pwd: string): Promise<boolean>
  loginWithToken(): Promise<boolean>
  isLoging: boolean;
  waitingQueue: [DelayResolver, Method, string, string, AxiosRequestConfig?][]
  getWaitingPromise(method: Method, model: string, func: string, config: AxiosRequestConfig | undefined): AxiosPromise;
  logout(): void;
}

interface DelayResolver {
  (value: AxiosResponse | PromiseLike<AxiosResponse>): void;
}

const client: NetClient = {} as NetClient;

client.isLoging = false;
client.waitingQueue = [];

const Authorization = "Authorization";

const axiosInstance = axios.create({
  baseURL: import.meta.env.VITE_HOST,
});

const fontStyle = {
  request: 'color:orange;font-size:10px;',
  response: 'color:blue;font-size:10px;'
}

axiosInstance.interceptors.request.use((config) => {
  console.log(`%c${(new Date()).toLocaleString()} [${config.method}]%o`, fontStyle.request, config.url, config.method == "get" ? config.params : config.data);
  return config;
}, (err) => {
  return Promise.reject(err)
})

axiosInstance.interceptors.response.use((resp) => {
  console.log(`%c${(new Date()).toLocaleString()} [${resp.config.method}(${resp.status})]%o`, fontStyle.response, resp.config.url, resp.data)
  return resp;
}, (err) => {
  if (err.response && err.response.status == 401) {
    console.log("登录失效")
    router.push('/login');
  }
  return Promise.reject(err)
})

// 转换big int
axiosInstance.defaults.transformResponse = [data => {
  if (data) {
    try {
      return JsonBigInt.parse(data)
    } catch (e) {
      console.error(e)
      return data
    }
  }
  return data
}]

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
  if (client.isLoging) {
    console.log('Waiting for Login');
    return client.getWaitingPromise(method, model, func, config);
  }
  return axiosInstance({
    ...config,
    method: method,
    url: `/${model}/${func}`,
  });
}

client.get = client.send.bind(client, 'GET');
client.post = client.send.bind(client, 'POST');
client.put = client.send.bind(client, 'PUT');
client.delete = client.send.bind(client, 'DELETE');

client.getWaitingPromise = (method: Method, model: string, func: string, config: AxiosRequestConfig | undefined) => {
  return new Promise<AxiosResponse>((resolve, _reject) => {
    client.waitingQueue.push([resolve, method, model, func, config]);
  })
}

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
client.loginWithAccount = async (
  nick: string,
  pwd: string
): Promise<boolean> =>
  wrapLoginCall(() => client.post('user', 'login', {
    data: { nick, pwd },
    headers: {
      [Authorization]: "Bearer 123"
    }
  }));

/**
 * token登录
 */
client.loginWithToken = async (): Promise<boolean> => {
  const token = getSavedToken();
  if (token) {
    return wrapLoginCall(() => client.get('user', 'token_refresh', {
      headers: {
        [Authorization]: token
      }
    }));
  }
  return false;
}

/**
 * 包装两种登录的方法
 * @param acitonCall
 * @returns 
 */
const wrapLoginCall = async (
  acitonCall: () => AxiosPromise
): Promise<boolean> => {
  try {
    const request = acitonCall();
    client.isLoging = true
    const resp = await request;
    client.isLoging = false
    let token = resp.data["token"];
    let user = resp.data["user"];
    if (!token || !user) {
      if (client.waitingQueue.length) {
        client.waitingQueue = [];
      }
      return false;
    }
    let result = saveToken(token);
    store.commit('changeUser', user);
    
    while(client.waitingQueue.length) {
      let data = client.waitingQueue.shift()!;
      let [resolve, method, model, func, config] = data;
      try {
        resolve(client.send(method, model, func, config))
      } catch (e) {
        console.error(e);
      }
    }

    if (router.currentRoute.value.name == "login") {
      router.push("/")
    }

    return result;
  } catch (e) {
    console.error(e)
    return false
  } finally {
    client.isLoging = false
  }
}

client.logout = () => {
  delete axiosInstance.defaults.headers.common[Authorization]
  localStorage.removeItem('_t');
  store.commit('logout')
  router.push('/login')
  window.location.reload()
}

(window as any).client = client;