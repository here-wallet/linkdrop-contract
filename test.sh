

near call l.herewallet.testnet send_near '{"request_id":"HugsAPcjSdRRQJp2vfcKns7VTh244nE3RMPvAnNxFxam"}' --gas 242794783120800 --accountId petr4.testnet --deposit 0.1


near call l.herewallet.testnet receive_transfer '{"phone":"test3"}' --gas 242794783120800 --accountId petr4.testnet
near call l.herewallet.testnet allocate_phone '{"phone":"test3", "account_id":"petr4.testnet"}' --gas 242794783120800 --accountId herewallet.testnet --depositYocto 1

NEAR_ENV=mainnet near call l.herewallet.near receive_transfer '{"request_id":"BYdMhfekq4gW6mafbYd4Tr19FL3hLaoxZHmVXto2RXWK", "key":"C9s5FbaqTGxptYpUYxa4kR", "kind":1, "account_id":"mydev.near"}' --gas 242794783120800 --accountId mydev.near 
