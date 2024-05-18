import { defineConfig } from "umi";

export default defineConfig({
  routes: [
    {
      path: "/",
      redirect: "/home",
    },
    {
      name: "首页",
      path: "/home",
      component: "./Home",
    },
    {
      name: "用户管理",
      path: "/user",
      component: "./User"
    },
    {
      name: "产品管理",
      path: "/product",
      component: "./Product"
    },
    {
      name: "角色",
      path: "/role",
      component: "./Role"
    },
    {
      name: "登录",
      layout: false,
      path: "/user/login",
      component: "./Login",
    }, {
      name: "权限管理",
      path: '/permission',
      component: './Permission'
    }
  ],
  npmClient: "pnpm",
  proxy: {
    '/api': {
      'target': 'http://127.0.0.1:5800/api/',
      'changeOrigin': true,
    },
  },
  tailwindcss: {
    tailwindCssFilePath: 'tailwind.css',
    tailwindConfigFilePath: "tailwind.config.js",
  },
  plugins: ["@umijs/plugins/dist/tailwindcss"],
  clientLoader: {}
});
