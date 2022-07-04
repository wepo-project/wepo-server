import axios from "axios";

const client = axios.create({
  baseURL: "http://127.0.0.1:8080/v1",
});

export default client;

export const setToken = (token: string) => {
  if (typeof token !== 'string') {
    console.error("Setting token failed!", token);
  }
  client.defaults.headers.common["Authorization"] = token;
  localStorage.setItem('_t', token);
}

export const removeToken = (token: string) => {
  delete client.defaults.headers.common["Authorization"];
  localStorage.removeItem('_t');
}