use buildDocker to build the docker image
Then use build.sh inside the docker to build the project


You can now perform the tests manually, or use our script to run them.

```bash time python solution/autotest.py 10  "./m1_game_engine -f maps/map00 -p1 solution/r2_d2 -p2 m1_robots/wall_e"
```bash time python solution/autotest.py 10  "./m1_game_engine -f maps/map01 -p1 solution/r2_d2 -p2 m1_robots/h2_d2"
```bash time python solution/autotest.py 10  "./m1_game_engine -f maps/map02 -p1 solution/r2_d2 -p2 m1_robots/bender"
```bash time python solution/autotest.py 5  "./m1_game_engine -f maps/map01 -p1 solution/r2_d2 -p2 m1_robots/terminator"
```bash time python solution/autotest.py 5  "./m1_game_engine -f maps/map01 -p2 solution/r2_d2 -p1 m1_robots/terminator"
