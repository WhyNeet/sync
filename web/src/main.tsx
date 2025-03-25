import { StrictMode } from 'react'
import ReactDOM from 'react-dom/client'
import { RouterProvider } from "react-router"
import router from './router'
import { Theme } from '@chakra-ui/react'
import { Provider } from './components/ui/provider'

const rootElement = document.getElementById('root')!
if (!rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement)
  root.render(
    <StrictMode>
      <Provider>
        <Theme appearance='light'>
          <RouterProvider router={router} />
        </Theme>
      </Provider>
    </StrictMode>,
  )
}