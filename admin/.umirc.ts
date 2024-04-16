import { defineConfig } from "@umijs/max";

export default defineConfig({
  antd: {},
  access: {},
  model: {},
  initialState: {},
  request: {
    dataField: 'data'
  },
  layout: {
    title: "@umijs/max",
  },
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
      name: "权限演示",
      path: "/access",
      component: "./Access",
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
      name: "登录",
      layout: false,
      path: "/user/login",
      component: "./Login",
    }
  ],
  npmClient: "pnpm",
  tailwindcss: {},
  proxy: {
    '/api': {
      'target': 'http://127.0.0.1:5800/api/',
      'changeOrigin': true,
    },
  },
});
