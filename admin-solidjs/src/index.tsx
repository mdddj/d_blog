import { render } from 'solid-js/web'

import './index.css'
import { Route, Router } from '@solidjs/router';
import BaseLayout from './layout/BaseLayout.tsx';
import HomePage from './page/home/HomePage.tsx';

const root = document.getElementById('root')

render(() => <Router root={BaseLayout}>
  <Route component={HomePage} path={'/'} />
</Router>, root!)
