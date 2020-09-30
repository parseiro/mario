# Rust Mario

This is an example of a state machine implemented in Rust.

It resembles the Mario classic game, in which there is a player (Mario or Luigi, but there's no distinction in this simple app), and he has 4 states:
* Dead
* Small
* Large
* Star (invencible)

When a player receives **hit()**, he may change it's state from *Small* to *Dead*, of from *Large* to *Small*, or keep in *Star*.

When a player receives **mushroom()**, he may go from *Small* to *Large*.

When a player receives **star()**, he goes to the *Star* state.


If you have any comments, please send an email to leonardo at vilelapinheiro.com
