# FILLER

## Description

Filler is a game, where two players / algorithm are fighting on a map.

A virtual machine organizes the game :

- calls the players alternatively
- give each a random piece
- On each turn, the current player have to put his piece on the map, and try to bother the ennemy. To put a piece, the player have to write its coordinates in the "Y X\n" format on the standard input (stdin). If the piece cannot be placed, the player skip his turn.
- The game appears on the standard input.
- The aim of the game is to put more pieces on the map than the ennemy.

## Usage:

1. Use buildDocker to build the docker image and launch into a shell inside the docker
   ```bash
    solution/buildDocker.sh
    ```

2. Use build.sh inside the docker to build the project
   ```bash
    solution/buildBot.sh
    ```


You can now perform the tests manually, or use our script to run them.

```bash
python solution/autotest.py 10  "./m1_game_engine -f maps/map00 -p1 solution/r2_d2 -p2 m1_robots/wall_e"
```

```bash
python solution/autotest.py 10  "./m1_game_engine -f maps/map01 -p1 solution/r2_d2 -p2 m1_robots/h2_d2"
```

```bash
python solution/autotest.py 10  "./m1_game_engine -f maps/map02 -p1 solution/r2_d2 -p2 m1_robots/bender"
```

```bash
python solution/autotest.py 5  "./m1_game_engine -f maps/map01 -p1 solution/r2_d2 -p2 m1_robots/terminator"
```

```bash
python solution/autotest.py 5  "./m1_game_engine -f maps/map01 -p2 solution/r2_d2 -p1 m1_robots/terminator"
```

