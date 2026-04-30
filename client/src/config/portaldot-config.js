/**
 * Portaldot Configuration
 * Polkadot blockchain configuration for WordPot
 */

export const PORTALDOT_CONFIG = {
  // Portaldot RPC endpoints
  RPC_ENDPOINT: import.meta.env.VITE_PORTALDOT_RPC || 'wss://rpc.portaldot.xyz',
  
  // API base URL for backend
  API_BASE_URL: import.meta.env.VITE_API_BASE_URL || 'http://localhost:4000/api',
  
  // Portaldot chain info
  CHAIN: {
    id: 'portaldot',
    name: 'Portaldot',
    nativeToken: 'POT',
  },
  
  // Game configuration
  GAME: {
    ENTRY_FEE: 0.1, // POT
    ROUND_DURATION: 60, // seconds
    TREASURY_FEE_PERCENTAGE: 10,
    REWARD_POOL_PERCENTAGE: 90,
  },
  
  // App URL for callback
  APP_URL: import.meta.env.VITE_APP_URL || 'http://localhost:5173',
};

export default PORTALDOT_CONFIG;
