# Bidirectional payment channel
Inspired by the Bitcoin lightning network and the solidity contract this framework presents a method for two participants to exchange SOL off chain any number of times without incurring any transaction fees other than the first and the last transaction.

Note: If the participants choose to update the channel in between then they might have more than two transactions. 

CREATING THE CHANNEL

1. Funding 

- Create a multisig account (2 of 2)
- Create transaction instructions:
1) To fund a wallet (PDA of the program) with the initial amount from Alice and Bob (ex. Alice put in 5 SOL & Bob put in 5 SOL).
2) Initialize the Struct : Payment Channel (PDA of the program) :  Maintains Alice balance and pubkey, Bob balance and pubkey, expiration date
- Both Alice and Bob invoke the approve instructions.
- Anyone can execute the transaction once both have signed.

2. Creating Initial transaction

- Both Alice and Bob try to send instructons to the program by signing them offline. On similar lines to https://spl.solana.com/token#example-offline-signing-with-multisig  
1) Alice’s transaction instructions
- Send the amount of 5 SOL she put into the wallet back to herself 
- Send the remaining 5 SOL to a special account (PDA of the program). Bob can withdraw from this account after a certain time or Alice can withdraw using Bob’s private key #1.
2) Bob’s transaction instructions (exactly the opposite of Alice’s)
- Send the amount of 5 SOL he put into the wallet back to himself
- Send the remaining 5 SOL to a special account (PDA of the program). Alice can withdraw from this account after a certain time or Bob can withdraw using Alice’s private key #1.

- Both Alice and Bob sign their own transactions offline and exchange the transaction details (signature) with each other.
- Now the other person can choose to sign and broadcast the transaction or not

If Alice choose to sign and broadcast Bob’s instructions then Bob immediately gets back the 5 SOL. Alice has to wait for a certain time get the remaining 5 SOL since she does not know Bob’s private key. Bob cannot withdraw the remaining 5 SOL since he does not know Alice’s private key. Similar case for Bob.

Therefore, uptill this point anyone can withdraw the money by just signing and broadcasting the transaction sent by the other person. 

3. Creating second transaction

Now lets say Alice wants to send Bob 2 SOL. 

Both of them create a new private key #2 for themselves. 

1) Alice’s transaction instructions
- Send the amount of 3 SOL she put into the wallet back to herself 
- Send the remaining 7 SOL to a special account (PDA of the program). Bob can withdraw from this account after a certain time or Alice can withdraw using Bob’s private key #2.
2) Bob’s transaction instructions (exactly the opposite of Alice’s)
- Send the amount of 7 SOL he put into the wallet back to himself
- Send the remaining 3 SOL to a special account (PDA of the program). Alice can withdraw from this account after a certain time or Bob can withdraw using Alice’s private key #2.

- Both Alice and Bob sign their own transactions offline and exchange the transaction details (signature) with each other.
- They also exchange the private keys #1

If any of them choose to sign and broadcast the transaction then their balances will be settled up. 

So far so good. Let’s see what happens when Alice tries to game the system and send an old transaction (5 SOL Alice and 5 SOL Bob). When Bob signs he immediately gets his 5 SOL and Alice has to wait for a certain period of time. If during this period, Bob realizes that he should have gotten 7 SOL and he has been cheated then he can use the private key #1 of Alice to withdraw the complete amount from the wallet. 

Hence when Alice and Bob exchanged their old private keys they made sure that no one can game the system or they risk losing all of their money. 

Now they can make any number of transactions they want without broadcasting to the network. 

CLOSING THE CHANNEL - If anyone wants to settle up or cash out they can then choose to broadcast any transaction. This will lock the funds one of the participants for a certain period of time. To avaoid this, If both parties agree then they can send a different transaction which does not have the conditions on the special account, both of them can approve and execute to receive money instantly. 


IF PARTIES GET NON COOPERATIVE i.e. refusing to sign each others transactions - Then post expiration, delete the payment channel/wallet and send the money back to Alice and Bob by referring to balances in the payment channel account

UPDATE THE PAYMENT CHANNEL - Payment channel can be updated after some transactions  so that if any party becomes uncooperative then the program refers to a more recent ledger for settlement. 
