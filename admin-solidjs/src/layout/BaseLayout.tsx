import { A, RouteSectionProps } from '@solidjs/router';

const navigation = [
  { name: '首页', href: '/'},
  { name: '用户', href: '/user' },
  { name: '权限', href: '/permission' },
  { name: '角色', href: '/role' },
]

const BaseLayout = (props: RouteSectionProps) => {
  return <div>
    <header class="navbar bg-base-100">
      <div class="navbar-start">
        <div class="dropdown">
          <div tabindex="0" role="button" class="btn btn-ghost lg:hidden">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                 stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h8m-8 6h16" />
            </svg>
          </div>
          <ul tabindex="0" class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52">
            {
              navigation.map(value => <li><A href={value.href}>{value.name}</A></li>)
            }
          </ul>
        </div>
        <a class="btn btn-ghost text-xl">典典后台</a>
      </div>
      <div class="navbar-center hidden lg:flex">
        <ul class="menu menu-horizontal px-1">
          {
            navigation.map(value => <li><A href={value.href}>{value.name}</A></li>)
          }
        </ul>
      </div>
      <div class="navbar-end">
        <a class="btn">登录</a>
      </div>
    </header>
    <div class={'container p-2 max-auto'}>
      {props.children}
    </div>
  </div>
}

export default BaseLayout

