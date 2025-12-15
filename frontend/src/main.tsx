import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { Toaster } from 'react-hot-toast'
import { DynamicContextProvider } from '@dynamic-labs/sdk-react-core'
import { EthereumWalletConnectors } from '@dynamic-labs/ethereum'
import App from './App'
import './index.css'

/**
 * Dynamic Labs Environment ID from environment variables
 * Get this from: https://app.dynamic.xyz/ → Your Project → Settings → API Keys
 */
const DYNAMIC_ENVIRONMENT_ID = import.meta.env.VITE_DYNAMIC_ENVIRONMENT_ID

if (!DYNAMIC_ENVIRONMENT_ID) {
  console.error(
    '❌ Missing VITE_DYNAMIC_ENVIRONMENT_ID in .env file\n' +
    'Please create a .env file with: VITE_DYNAMIC_ENVIRONMENT_ID=your-id-here\n' +
    'Get your ID from: https://app.dynamic.xyz/'
  )
}

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <DynamicContextProvider
      settings={{
        environmentId: DYNAMIC_ENVIRONMENT_ID,
        appName: 'Linera Poker',
        initialAuthenticationMode: 'connect-only',
        walletConnectors: [EthereumWalletConnectors],
      }}
    >
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
    </DynamicContextProvider>
  </StrictMode>,
)
