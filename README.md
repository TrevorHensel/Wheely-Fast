# Wheely Fast

- Trevor Hensel
    - hensel@pdx.edxu
- Ryan Campbell     
    - cam28@pdx.edu
- Brooks Russell    
    - bru2@pdx.edu

For our term project, We are going to be creating a small arcade racing game. Where
the user can move a small car around a race track. We would like the
game to use graphics, but we have no previous experience with grpahics so
we are still unsure of exactly how we going to accomplish this goal in such
a short amount of time. We will try our best to get a functioning game by
the end of term.

The game that we came up with simulates driving a car on a straight road
where you have to avoid the randomly generated barriers that you are approaching.
If you hit a barrier the game ends and your score will pop up on the screen.
The controls are very simple, you can move left or right depending where you are
on the road.

START GAME:
- To start the game simply press enter on your keyboard and then begin using the 
left and right arrow keys to move the car.
- Also recommened to use "cargo run --release" in order to run the game as we 
noticed some games run better with the "--release" option.

END GAME;
- If you want to end the game early press the esc key.
- If you hit a barrier the game will end to exit hit the esc key.

We used the GGEZ library for graphics. 
- https://github.com/ggez/ggez
Their repository has a lot of useful examples and explinations on how to implement
the code to make a game.
    - We also used their examples to learn how to use GGEZ and so our code may
        reflect that in its' final form. So we owe a lot of our understanding of
        GGEZ to the examples they provided and we are citing them here because we
        used many of their resources throughout the project in order to help us
        learn how to make a simple game in GGEZ.

[LICENSE](./LICENSE)
