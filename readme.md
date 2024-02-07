# Polymesh smart contract with terminate_contract function

when deploying smart contract code to the blockchain, it is important to ensure that the code is secure and that it can be terminated if necessary. 
This is why the terminate_contract function is important. It allows the owner of the smart contract to terminate the contract and remove it from the blockchain. 
This is useful if the contract is no longer needed, or if there is a security issue that needs to be addressed.

Code with active smart contracts attached to it can not be removed.
Therefor it is also a good practise to limit who can instantiate the new code and prevent unauthorized usage of the code.

The terminate_contract function will terminate the contract, and send all value in the contract to the designated address (in this example that is the current admin).

Onchain this will trigger an event: balances (Unreserved), and your storage reservation will be available again.

## opgradable contracts

when combining this example with the set_code_hash function, you could replace the code hash, keep the storage and then remove the old code from the blockchain.

## developer note on cost optimization

Now also important to note is that this contract has a size of 17.5k (+/-2500 POLYX), when build with the command

```
cargo contract build
```

But only 3.9k (+/- 500 POLYX) when build with the command

```
cargo contract build --release
```
