
## 注册用户
curl -i -d '{"nick":"curl","pwd":"123123"}' -H 'Content-Type: application/json' http://127.0.0.1:8080/v1/user/add_user

## 登录用户
curl -i -d '{"nick":"curl","pwd":"123123"}' -H 'Content-Type: application/json' http://127.0.0.1:8080/v1/user/login


## 添加post
curl http://127.0.0.1:8080/v1/post/add_post -i -d '{"content":"a post"}' -H 'Content-Type: application/json' -H 'Authorization: Bearer ' 

## 删除post
curl http://127.0.0.1:8080/v1/post/del_post -i -X DELETE -d '{"id": "1c119422-db8b-47dc-afa4-232fdf12fc37"}' -H 'Content-Type: application/json' -H 'Authorization: Bearer ' 
