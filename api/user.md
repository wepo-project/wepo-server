
## 注册用户
curl -i -d '{"nick":"curl","pwd":"123123"}' -H 'Content-Type: application/json' http://127.0.0.1:8080/v1/add_user

## 登录用户
curl -i -d '{"nick":"curl","pwd":"123123"}' -H 'Content-Type: application/json' http://127.0.0.1:8080/v1/login