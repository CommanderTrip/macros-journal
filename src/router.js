import { createMemoryHistory, createRouter } from 'vue-router';

import App from "./App.vue";
import UserInfoView from "./views/UserInfoView.vue";
import GettingStartedView from './views/GettingStartedView.vue';

const routes = [
    { path: "/GettingStarted", component: GettingStartedView },
    { path: "/user", component: UserInfoView },
];

const router = createRouter({
    history: createMemoryHistory(),
    routes,
});

export default router;