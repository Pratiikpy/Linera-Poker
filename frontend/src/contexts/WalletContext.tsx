
import React, { createContext, useContext, ReactNode, useEffect, useState } from 'react'
import {
    DynamicContextProvider,
    useDynamicContext,
    DynamicConnectButton as RealDynamicConnectButton
} from '@dynamic-labs/sdk-react-core'
import { EthereumWalletConnectors } from '@dynamic-labs/ethereum'
import { Wallet } from 'lucide-react'

// Environment Config
const DYNAMIC_ENVIRONMENT_ID = import.meta.env.VITE_DYNAMIC_ENVIRONMENT_ID
const IS_LOCAL_DEMO = !DYNAMIC_ENVIRONMENT_ID || import.meta.env.VITE_LOCAL_DEMO === 'true'

// Wallet Interface
export interface WalletAccount {
    address: string
    connector?: any
}

interface WalletContextType {
    primaryWallet: WalletAccount | null
    isAuthenticated: boolean
    isLoading: boolean
    connect: () => void
    disconnect: () => void
}

const WalletContext = createContext<WalletContextType | undefined>(undefined)

export const useWallet = () => {
    const context = useContext(WalletContext)
    if (!context) {
        throw new Error('useWallet must be used within a WalletProvider')
    }
    return context
}

// Internal bridge component to extract values from Dynamic Context
const DynamicBridge = ({ children }: { children: ReactNode }) => {
    const { primaryWallet, handleLogOut, setShowAuthFlow } = useDynamicContext()

    // Transform Dynamic wallet to our interface
    const wallet: WalletAccount | null = primaryWallet ? {
        address: primaryWallet.address,
        connector: primaryWallet.connector
    } : null

    return (
        <WalletContext.Provider value={{
            primaryWallet: wallet,
            isAuthenticated: !!primaryWallet,
            isLoading: false,
            connect: () => setShowAuthFlow(true),
            disconnect: handleLogOut
        }}>
            {children}
        </WalletContext.Provider>
    )
}

// Provider Component
export const WalletProvider = ({ children }: { children: ReactNode }) => {
    // Demo Mode State
    const [demoWallet, setDemoWallet] = useState<WalletAccount | null>(null)

    if (IS_LOCAL_DEMO) {
        // Local Demo Implementation
        const mockContext: WalletContextType = {
            primaryWallet: demoWallet,
            isAuthenticated: !!demoWallet,
            isLoading: false,
            connect: () => {
                // Simulate connection delay
                setTimeout(() => {
                    setDemoWallet({
                        address: '0xfba350BD9c9bD18866936bB807E09439ba976cCe' // Consistent mock address
                    })
                }, 500)
            },
            disconnect: () => setDemoWallet(null)
        }

        return (
            <WalletContext.Provider value={mockContext}>
                {children}
            </WalletContext.Provider>
        )
    }

    // Real Dynamic Provider
    return (
        <DynamicContextProvider
            settings={{
                environmentId: DYNAMIC_ENVIRONMENT_ID || '', // Safe fallback, main.tsx checks validity
                appName: 'Linera Poker',
                initialAuthenticationMode: 'connect-only',
                walletConnectors: [EthereumWalletConnectors],
            }}
        >
            <DynamicBridge>
                {children}
            </DynamicBridge>
        </DynamicContextProvider>
    )
}

// Unified Connect Button Component
export const ConnectButton = ({ className, style }: { className?: string, style?: any }) => {
    const { connect, isAuthenticated, primaryWallet, disconnect } = useWallet()

    if (IS_LOCAL_DEMO) {
        if (isAuthenticated) {
            return (
                <button
                    onClick={disconnect}
                    className={className}
                    style={style}
                >
                    DISCONNECT {primaryWallet?.address.slice(0, 6)}...
                </button>
            )
        }
        return (
            <button
                onClick={connect}
                className={className}
                style={style}
            >
                CONNECT WALLET (DEMO)
            </button>
        )
    }

    // In real mode, we wrap our button or use theirs
    // But App.tsx customizes the button significantly using DynamicConnectButton children
    // So we probably want to expose the wrapper
    return (
        <RealDynamicConnectButton>
            {/* Use default or pass children? App.tsx passes a custom button inside. */}
            {/* To handle custom children correctly, we might need a render prop or similar. */}
            {/* For now, just render the Real button wrapper which handles the click logic for children */}
            <div className="dynamic-connect-wrapper">
                {/* This is tricky because RealDynamicConnectButton expects to wrap the trigger */}
                {/* We will let App.tsx use ConnectWalletWrapper instead */}
            </div>
        </RealDynamicConnectButton>
    )
}

// Wrapper for the custom button in App.tsx
export const ConnectWalletWrapper = ({ children }: { children: ReactNode }) => {
    const { connect, isAuthenticated, disconnect } = useWallet()

    if (IS_LOCAL_DEMO) {
        return (
            <div onClick={isAuthenticated ? disconnect : connect}>
                {children}
            </div>
        )
    }

    return (
        <RealDynamicConnectButton>
            {children}
        </RealDynamicConnectButton>
    )
}
