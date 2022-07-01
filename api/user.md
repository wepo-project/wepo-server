
## 注册用户
curl -i -d '{"nick":"curl","pwd":"123123"}' -H 'Content-Type: application/json' http://127.0.0.1:8080/v1/user/add_user

## 登录用户
curl -i -d '{"nick":"curl","pwd":"123123"}' -H 'Content-Type: application/json' http://127.0.0.1:8080/v1/user/login


## 添加post
curl http://127.0.0.1:8080/v1/post/add_post -i -d '{"content":"a post"}' -H 'Content-Type: application/json' -H 'Authorization: Bearer ' 