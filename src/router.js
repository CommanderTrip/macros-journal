import { createMemoryHistory, createRouter } from 'vue-router';

import App from "./App.vue";
import UserInfoView from "./components/UserInfo.vue";
import GettingStartedView from './components/GettingStarted.vue';

const routes = [
    { path: "/GettingStarted", component: GettingStartedView },
    { path: "/user", component: UserInfoView },
];

const router = createRouter({
    history: createMemoryHistory(),
    routes,
});

export default router;