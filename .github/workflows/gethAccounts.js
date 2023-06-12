// These are ripped from ganache-cli and also used in src/client/account_handler.rs
// They are used to test the client and are not used in production

const users = [
    {
        public: '0x40f2AC22966Ec049555FE7876c1803AFe49A717A',
        private: 'a584b5b2e4c973c4d90ff63f930df0c2938c4c5af029373881b35ea6ec1839a0'
    },
    {
        public: '0xA39f5AcC00c3Ba685133e2cb3067414eAAC69A43',
        private: 'a062ba0a7168f42b5cd45f05672cb2bdde0fe72a5f1056d5cc994bfd272005c3'
    },
    {
        public: '0xD39DBd7603ED1d9755151Fe6532d33D76Dc909D0',
        private: 'bfb34b71033d5103c25b9fbbcf0149f8badc06610969f4d13c6b1efb02c28951'
    },
    {
        public: '0xbE8cF79fa5bF19C0106023A0f3be6fd0e5b1D074',
        private: '5642314d8f1f9bfadbf52790bc68d288ddbabae8488c060b0f8871950f500677'
    },
    {
        public: '0x0e5Abeb462A67d7E499d95b4Ad777e0e8DCbF27d',
        private: '1d197f1c9d17cbbca2d1f746d73d8d04befdf4c22ef1e62c8024db2bec52ad5c'
    }
];

// Transfer ETH from the default address to each imported account
async function transferEth() {
    for (i = 0; i < users.length; i++) {
        var account = users[i];
        var to = account.public.toLowerCase();
        web3.personal.importRawKey(account.private, 'fhe');

        // Transfer ETH to the target account
        await web3.eth.sendTransaction({
            from: eth.accounts[0],
            to,
            value: web3.toWei('100', 'ether')
        });

        await web3.personal.unlockAccount(to, 'fhe', 0);

    }
}

// Run the transferEth function
transferEth()
    .then(() => {
        console.log('ETH transferred successfully');
    })
    .catch((error) => {
        console.error('Failed to transfer ETH:', error);
    });
