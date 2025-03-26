import { StrictMode } from 'react'
import ReactDOM from 'react-dom/client'
import { RouterProvider } from "react-router"
import router from './router'
import { Theme } from '@chakra-ui/react'
import { Provider } from './components/ui/provider'
import { Toaster } from './components/ui/toaster'
import "./lib/state/error"

const rootElement = document.getElementById('root')!
if (!rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement)
  root.render(
    <StrictMode>
      <Provider>
        <Theme appearance='light'>
          <Toaster />
          <RouterProvider router={router} />
        </Theme>
      </Provider>
    </StrictMode>,
  )
}