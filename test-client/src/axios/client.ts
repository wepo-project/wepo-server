import axios from "axios";

const Authorization = "Authorization";

const client = axios.create({
  baseURL: "http://127.0.0.1:8080/v1",
});

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
  console.log("--------")
  console.log(err.response.status);
  console.log("--------")
  return Promise.reject(err)
})

export default client;

export const setToken = (token: string) => {
  if (typeof token !== 'string') {
    console.error("Setting token failed!", token);
  }
  client.defaults.headers.common[Authorization] = token;
  localStorage.setItem('_t', token);
}

export const setTokenFromLocalStorage = () => {
  const token = localStorage.getItem('_t');
  const haveToken = token != null && token != '';
  if (haveToken) {
    setToken(token);
  }
  return haveToken;
}

export const removeToken = (token: string) => {
  delete client.defaults.headers.common[Authorization];
  localStorage.removeItem('_t');
}

export const isAuth = () => client.defaults.headers.common[Authorization] != null;

(window as any).client = client;