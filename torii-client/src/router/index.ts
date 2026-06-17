import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import HomeView from '../views/HomeView.vue';

const routes: Array<RouteRecordRaw> = [
    {
        path: '/',
        name: 'home',
        component: HomeView,
    },
    {
        path: '/project',
        name: 'project',
        props: { projectPath: '' },
        component: () => import('../views/project/ProjectView.vue'),
    },
    {
        path: '/settings',
        redirect: '/settings/general',
        name: 'settings',
        // route level code-splitting
        // this generates a separate chunk (settings.[hash].js) for this route
        // which is lazy-loaded when the route is visited.
        component: () =>
            import(
                /* webpackChunkName: "settings" */ '../views/settings/SettingsView.vue'
            ),
        children: [
            {
                path: 'general',
                name: 'settings-general',
                component: () =>
                    import(
                        /* webpackChunkName: "settings-general" */ '../views/settings/GeneralSettingsView.vue'
                    ),
            },
            {
                path: 'theme',
                name: 'settings-theme',
                component: () =>
                    import(
                        /* webpackChunkName: "settings-theme" */ '../views/settings/ThemeSettingsView.vue'
                    ),
            },
        ],
    },
];

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes,
});

export default router;
