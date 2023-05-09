import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'
import init from '@rsw/planner_lib_wasm/planner_lib_wasm'

init()
    .catch(console.error)
    .then(() => {
    ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
        <React.StrictMode>
            <App/>
        </React.StrictMode>,
    )
})
