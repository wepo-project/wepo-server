import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router';
import Home from "./components/Home.vue";
import Login from "./components/Login.vue";
import Register from "./components/Register.vue";
import SendPost from "./components/SendPost.vue";
import MyPost from "./components/MyPost.vue";
import PostDetail from "./components/PostDetail.vue";

const routes: RouteRecordRaw[] = [
    {
        path: '/',
        component: Home,
    },
    {
        path: '/login',
        component: Login,
    },
    {
        path: '/reg',
        component: Register,
    },
    {
        path: '/send',
        component: SendPost,
    },
    {
        path: '/my_post',
        component: MyPost,
    },
    {
        path: '/po/:id',
        component: PostDetail,
    }
]

const router = createRouter({
    history: createWebHashHistory(),
    routes,
})

export default router;