# NCD.L1--Chess

A chess game for the NEAR network. The pieces move properly, but I still need to implement some kind of authentication for the players. Also need to properly implement testing/better documentation. But it's still useful as a study material though. 



## Recommendations (How I'm building this project)
For testing the project, I usually create a subaccount, deploy to it, run my functions and then delete the subaccount.

Call "near login" to enter your main account. Let's say I have a testnet account called "a-test-account.testnet". In that case, I call "near create-account rust-tests.a-test-account.testnet --masterAccount a-test-account.testnet --initialBalance 100" to create a subaccount with name rust-tests and a deposit of 100 NEAR. When I'm done using the deployed project, I would call "near delete rust-tests.a-test-account.testnet a-test-account.testnet", which means the subaccount gets deleted and all the remaining NEAR is transferred back to a-tests-account.testnet.


## Compiling and deploying this chess game.

Bring a terminal to this folder.

cargo +nightly build --target wasm32-unknown-unknown --release

Might need to add wasm32-unknown-unknown before doing the above. A message will show up saying it.

cd ./target/wasm32-unknown-unknown/release

To reach the folder with the compiled wasm.

near deploy --accountId rust-tests.a-test-account.testnet --wasmFile ./chess.wasm

If everything goes well, the subaccount now contains the compiled project. We can run it's functions remotely.

## Functions available

Check lib.rs to see available functions. Functions with &self can be called with view. Functions with &mut self can be called with call.

### Example for view:

near view rust-tests.a-tests-account.testnet get_board '{}'

Will print a very ugly version of the board. It's stored as 64 bytes, so we can't expect much from it. Numbers go from 0 to 12, here's what they mean:

 - 0: Empty piece;
 - 1: White Pawn;
 - 2: White Rook;
 - 3: White Knight;
 - 4: White Bishop;
 - 5: White Queen;
 - 6: White King;
 - 7: Black Pawn;
 - 8: Black Rook;
 - 9: Black Knight;
 - 10: Black Bishop;
 - 11: Black Queen;
 - 12: Black King

### Another example for view:

near view rust-tests.a-tests-account.testnet get_piece_name '{"col": 7, "row": 7}'

Will get the piece in the bottom right corner of the board. A "White Rook". Columns and rows go from 0 to 7, get_piece_name_index does the same thing but you use the index of the entire board instead. For the same position the index would be 63.

### An example for call:

near call rust-tests.a-tests-account.testnet move_to '{"current_col": 0, "current_row": 6, "target_col": 0, "target_row": 4}' --account-id a-tests-account.testnet

Will make the bottom white pawn move forward two steps (If it's the white player's turn).

There's also a function to reset the game and get the game status. Please check them out.
