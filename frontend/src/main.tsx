import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { Toaster } from 'react-hot-toast'
import { WalletProvider } from './contexts/WalletContext'
import App from './App'
import './index.css'

// Check for Dynamic Environment ID mostly for dev feedback
const DYNAMIC_ENVIRONMENT_ID = import.meta.env.VITE_DYNAMIC_ENVIRONMENT_ID
const IS_LOCAL_DEMO = !DYNAMIC_ENVIRONMENT_ID || import.meta.env.VITE_LOCAL_DEMO === 'true'

if (!DYNAMIC_ENVIRONMENT_ID && !IS_LOCAL_DEMO) {
  console.error(
    '❌ Missing VITE_DYNAMIC_ENVIRONMENT_ID in .env file\n' +
    'Please create a .env file with: VITE_DYNAMIC_ENVIRONMENT_ID=your-id-here\n' +
    'Get your ID from: https://app.dynamic.xyz/'
  )
}

if (IS_LOCAL_DEMO) {
  console.log('🎮 Running in LOCAL DEMO MODE - wallet connection simulated')
}

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <WalletProvider>
      <Toaster
        position="top-right"
        toastOptions={{
          style: {
            background: '#1f2937',
            color: '#fff',
            border: '1px solid #374151',
          },
          success: {
            iconTheme: {
              primary: '#10b981',
              secondary: '#fff',
            },
          },
          error: {
            iconTheme: {
              primary: '#ef4444',
              secondary: '#fff',
            },
          },
        }}
      />
      <App />
    </WalletProvider>
  </StrictMode>,
)

