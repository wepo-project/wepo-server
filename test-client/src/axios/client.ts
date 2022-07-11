import axios, { AxiosInstance } from "axios";

const Authorization = "Authorization";

const client = axios.create({
  baseURL: "http://127.0.0.1:8080/v1",
}) as NetClient;

const fontStyle = {
  request: 'color:orange;font-size:10px;',
  response: 'color:blue;font-size:10px;'
}

client.interceptors.request.use((config) => {
  console.log(`%c${(new Date()).toLocaleString()} [${config.method}]%o`, fontStyle.request, config.url, config.data);
  return config;
}, (err) => {
  return Promise.reject(err)
})

client.interceptors.response.use((resp) => {
  console.log(`%c${(new Date()).toLocaleString()} [${resp.config.method}(${resp.status})]%o`, fontStyle.response, resp.config.url, resp.data)
  return resp;
}, (err) => {
  if (err.response.status == 401) {
    console.log("登录失效")
  }
  return Promise.reject(err)
})

interface NetClient extends AxiosInstance {
  isLogined(): boolean
  loginWithAccount(nick: string, pwd: string): Promise<boolean>
  loginWithToken(): Promise<boolean>
}

export default client;

const getSavedToken = () => localStorage.getItem('_t');

const saveToken = (token: string): boolean => {
  if (typeof token !== 'string') {
    console.error("Setting token failed!", token);
    return false;
  }
  client.defaults.headers.common[Authorization] = token;
  localStorage.setItem('_t', token);
  console.log("登录成功");
  return true;
}

/**
 * 
 * @returns 是否登录
 */
client.isLogined = () => client.defaults.headers.common[Authorization] != null;

/**
 * 账号登录
 * @param nick 
 * @param pwd 
 */
client.loginWithAccount = async (nick: string, pwd: string): Promise<boolean> => {
  const resp = await client.post("/user/login", { nick, pwd });
  return saveToken(resp.data["token"]);
}

/**
 * token登录
 */
client.loginWithToken = async (): Promise<boolean> => {
  const token = getSavedToken();
  if (token) {
    const resp = await client.get('/token/login', {
      headers: {
        [Authorization]: token
      }
    });
    return saveToken(resp.data);
  }
  return false;
}

(window as any).client = client;