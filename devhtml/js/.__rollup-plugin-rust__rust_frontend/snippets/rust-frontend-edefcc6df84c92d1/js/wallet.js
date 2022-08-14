
(function(l, r) { if (!l || l.getElementById('livereloadscript')) return; r = l.createElement('script'); r.async = 1; r.src = '//' + (self.location.host || 'localhost').split(':')[0] + ':35729/livereload.js?snipver=1'; r.id = 'livereloadscript'; l.getElementsByTagName('head')[0].appendChild(r) })(self.document);
import '../../../../_virtual/_rollup-plugin-inject-process-env.js';
import Onboard from '../node_modules/@web3-onboard/core';
import walletConnectModule from '../node_modules/@web3-onboard/walletconnect';

let onboard;
const MUMBAI_RPC_URL = `${window.process.env.POLYGON_TEST_URL}/${process.env.POLYGON_MATICVIGIL_API_KEY}`;
const walletConnect = walletConnectModule();

function _init() {
    console.log("Hello, World!!");
    onboard = Onboard({
        wallets: [walletConnect],
        chains: [
            {
                id: "0x13881",
                token: "MATIC",
                namespace: "evm",
                label: "Polygon Mumbai Testnet",
                rpcUrl: MUMBAI_RPC_URL,
            },
            {
                id: "0x89",
                token: "MATIC",
                namespace: "evm",
                label: "Polygon Mainnet",
                rpcUrl: POLYGON_RPC_URL,
            },
        ],
        appMetadata: {
            name: "halalanft",
            icon: "https://upload.wikimedia.org/wikipedia/commons/a/a7/React-icon.svg",
            logo: "https://upload.wikimedia.org/wikipedia/commons/a/a7/React-icon.svg",
            description: "My app using Onboard",
            // recommendedInjectedWallets: [
            //   { name: "MetaMask", url: "https://metamask.io" },
            // ],
        },
    });
}

// export async function _requestAccount() {
//     await window.ethereum.request({ method: 'eth_requestAccounts' });
// }

async function _connect() {
    const connectedWallet = await onboard.connectWallet();
    console.log(connectedWallet);
    // const { accounts, chains, provider } = wallets[0];
    // const providerEthers = new ethers.providers.Web3Provider(provider);
    // const signer = providerEthers.getSigner();
    // onboard = { providerEthers, signer, accounts, chains };
    return JSON.stringify(connectedWallet)
}


function _myAddress() {
    return onboard.accounts[0].address
}

// export async function _detectMetamask() {
//     const ethereumProvider = await detectEthereumProvider();
//     if (ethereumProvider) {
//         return true;
//     }
//     return false;
// }

export { _connect, _init, _myAddress };
//# sourceMappingURL=wallet.js.map
