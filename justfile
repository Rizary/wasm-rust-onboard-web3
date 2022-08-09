dev:
  npm run watch

release:
  npm run build
  
ethers:
  wget -P ./js https://cdn.ethers.io/lib/ethers-5.6.esm.min.js