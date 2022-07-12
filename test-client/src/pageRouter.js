import { createRouter, createWebHashHistory } from 'vue-router';

const routes = [
    {
        path: '/',
        component: () => import("./components/Home.vue"),
    },
    {
        path: '/login',
        component: () => import("./components/Login.vue"),
    },
    {
        path: '/reg',
        component: () => import("./components/Register.vue"),
    },
    {
        path: '/send',
        component: () => import("./components/SendPost.vue"),
    },
    {
        path: '/my_post',
        component: () => import("./components/MyPost.vue"),
    },
    {
        path: '/po/:id',
        component: () => import("./components/PostDetail.vue"),
    }
]

const router = createRouter({
    history: createWebHashHistory(),
    routes,
})

export default router;