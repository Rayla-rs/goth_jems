# Goth Jems #
- CGRA252
- Assignment 01
- Created by: Rayla Wass
- (TODO: VIDEO)
## Description
The main game mechanic is swapping tile positions with vertically and horizontally adjacent tiles. This mechanic allows the player to cause cascading effects that a;ter the board state and score points. If the swap does not create a match immediately, it is undone.

The hardest part of creating this game was refreshing the board after resolving matches. I went through a few different models, the first of which required more refreshes as it only dropped tiles if there was an empty space below it. For example if a tile at row 2 was removed, the tile in row one would drop, then the tile in row 0 would drop, and finally, a new tile would be introduced and dropped into row 0. Visually this behaviour was very mechanical. My new model iterates from the bottom of the board and drops tiles by the total number of hollow tiles below it. Using the same example above, this implementation would drop the tile in row 1 and 0 and instance and drop a new tile into row 0 at the same time. Both methods were valid, however I much prefer the new model.

The most interesting part of developing this game was expanding on my stack machine model to manage behaviours and make my code more modular. This game is perfect for stack based execution because actions have cascading effects that are resolved sequentially.

## Reflection
One of the things I worked on in this project was implementing more gdscript into my code base. I have been trying different languages for game development trying to find what suits my skills best. In this game, I expanded on my knowledge by mixing more gdscript into my rust code. Rust allows for more control over development than gdscript, however, it can also be a hindrance at times, especially when trying to do tasks like tweening. In cases like these, using gdscript makes sense. Gdscript also made user interface design a breeze and then I could always use the bindings with rust if I ever needed to transfer information.

In the future, I am going to continue to balance my gdscript and rust code so I can make the most out of both languages. 

## Known Problems
- The board can reach an unplayable state where there are no valid matches. This can be solved by altering the randomly generated tiles if there are no matches yet so that there will be at least one new match when the board is resolved.


