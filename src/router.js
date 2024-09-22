import { createMemoryHistory, createRouter } from 'vue-router';

import App from "./App.vue";
import UserInfoView from "./views/UserInfoView.vue";
import HomeView from './views/HomeView.vue';

const routes = [
    { path: "/home", component: HomeView },
    { path: "/user", component: UserInfoView },
];

const router = createRouter({
    history: createMemoryHistory(),
    routes,
});

export default router;