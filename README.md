# Rust Mario

This is an example of a state machine implemented in Rust.

It resembles the Mario classic game, in which there is a character (Mario or Luigi, but there's no distinction in this simple app), and he has 4 states:
* Dead
* Small
* Large
* Star (invencible)

When a character is **hit()**, he may change it's state from *Small* to *Dead*, of from *Large* to *Small*, or keep in *Star*.

When a character is **mushroom()**, he may go from *Small* to *Large*.


If you have any comments, please send an email to leonardo at vilelapinheiro.com
