LPDL (Linear Programming Definition Language)
#

## Note: this repo, along with the rest if JAM-C are still under development and they are subject to change drastically.
#

This repo contains the parser and the definition language specification for the linear programming solver by JAM-C
#

## Language Specification

A problem file consist of the objective function along with whether it should be minimized or maximized on, always, the first non-empty line.

The rest lines are assumed to be the constrains.

You can write the cost function along with the constraints as typical  equations 

Restring a variable to be an integer could be done by 

making a new line with var_name: int

// are used for comments
All spaces are ignored

An example is provided in example.txt

You do not need to define the variables, which seems redundant in LP problem format

## Parser
The main.ts file contains the source code that convert a problem file into a format that can be used to the solver in https://github.com/JAM-C/lp-solverlib

The parser could be used with cargo run main.ts -- example.txt
