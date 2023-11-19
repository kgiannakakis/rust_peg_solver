# Peg Solitaire Solver

This program solves the [English peg solitaire puzzle](http://en.wikipedia.org/wiki/Peg_solitaire)
and can produce image and gif solutions. The solver and input puzzle format is based on a
[Golang example](https://go.dev/test/solitaire.go). The code extends the existing sample so that it can 
work with other variations of the puzzle, allowing the user to define a custom board.

![Peg Solitaire solution](images/solution.gif)

You can run the solver as follows:

`cargo run -- games/english.txt images solutions`

- The first argument is the path of the input file
- The second argument is the solution output mode (text, images, gif). Default is text.
- The third argument is the output folder for the images. Default is solutions.

You can create your own board and try to solve it. Every position of the board is represented by a character:
- ● represents a peg in the position
- ○ represents a hole
- ◎ denotes the center position, starting with a hole
- ◉ denotes the center position, starting with a peg
- . represents an unreachable position

The board must be surrounded by 2 dots (.) in each direction so that we don't need to check
the board boundaries while examining valid moves.

The classic English peg solitaire board is represented by the following board:

``...........``  
``...........``  
``....●●●....``  
``....●●●....``  
``..●●●●●●●..``  
``..●●●◎●●●..``  
``..●●●●●●●..``  
``....●●●....``  
``....●●●....``  
``...........``  
``...........``  

You can find some examples in [games](games) folder.

**Note**: The solver algorithm is a simple, non-optimized brute-force mechanism. It will solve small boards fast enough, 
but it will fail to handle bigger and more complex boards. No solvability checks are performed.