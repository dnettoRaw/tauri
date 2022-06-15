/*      #######                                               */
/*   ###       ###                                            */
/*  ##   ## ##   ##   F: App.tsx                              */
/*       ## ##                                                */
/*                    C: 2022/06/15 16:15:07 by:dnettoRaw     */
/*  ##   ## ##   ##   U: 2022/06/15 16:38:19 by:dnettoRaw     */
/*    ###########                                             */

import { useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import logo from './logo.svg'
import './App.css'


function App() {
  const [count, setCount] = useState(0);
  const invoke = window.__TAURI__.invoke;

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>Hello Vite + React!</p>
        <p>
          <button type="button" onClick={() => setCount((count) => count + 1)}>
            count is: {count}
          </button>
          <button type="button" onClick={() => invoke('my_button')}> let's print in terminal</button>
        </p>
        <p>
          Edit <code>App.tsx</code> and save to test HMR updates. p'
        </p>
        <p>
          <a
            className="App-link"
            href="https://reactjs.org"
            target="_blank"
            rel="noopener noreferrer"
          >
            Learn React
          </a>
          {' | '}
          <a
            className="App-link"
            href="https://vitejs.dev/guide/features.html"
            target="_blank"
            rel="noopener noreferrer"
          >
            Vite Docs
          </a>
        </p>
      </header>
    </div>
  )
}

export default App
