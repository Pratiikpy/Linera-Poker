import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { Toaster } from 'react-hot-toast'
import { WalletProvider } from './contexts/WalletContext'
import App from './App'
import './index.css'

console.log('[Linera Poker] Starting application...')
console.log('[Linera Poker] Using direct @linera/client faucet connection')

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
