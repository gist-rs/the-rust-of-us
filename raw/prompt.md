# INSTRUCTION:

Play the game by create timeline for the man's action, start at entrance (🚪) ensuring he attack skeleton (💀) moves towards and collect the treasure (💰) while avoiding obstacles (🦀).

# RULES:

- Top-left is (1,1) = a1
- Bottom-right is (8,8) = h8
- man can walk only on "1"

# INPUT EXAMPLE:

```csv
a,b,c,d,e,f,g,h
🌳,🌳,🌳,🌳,🌳,🚪,🌳,🌳
🌳,🌳,🌳,🌳,🌳,1,🌳,🌳
🌳,1,1,🦀,1,1,1,🌳
🌳,🌳,🌳,🌳,1,1,1,🌳
🌳,💰,1,1,💀,1,1,🌳
🌳,🌳,🌳,🌳,1,1,1,🌳
🌳,1,1,1,1,1,1,🌳
🌳,⛩️,🌳,🌳,🌳,🌳,🌳,🌳
```

# OUTPUT EXAMPLE:

```csv
sec,id,act,at,to
1,man_0,idle,f1,f1
1,skeleton_0,idle,d5,e5
2,man_0,walk,f1,e5
5,man_0,attack,e5,d5
5,skeleton_0,hurt,d5,d5
6,skeleton_0,die,d5,d5
7,man_0,walk,d5,c5
9,man_0,open,c5,b5
10,man_0,idle,c5,d5
```

# OUTPUT EXPLAIN:

- `1,man_0,idle,f1,f1` = man idle at d5
- `1,skeleton_0,idle,d5,e5` = skeleton idle at d5 look at e5
- `2,man_0,walk,f1,e5` = man walk from f1 and to e5
- `5,man_0,attack,e5,d5` = man stand at e5 and attack d5
- `5,skeleton_0,hurt,d5,e5` = skeleton stay at d5 and hurt while look at e5
- `6,skeleton_0,die,d5,e5` = skeleton die at d5 while look at e5
- `7,man_0,walk,d5,c5` = man walk from d5 and to c5
- `9,man_0,open,c5,b5` = man stand at c5 and open chest at b5
- `10,man_0,idle,c5,d5`= man idle at c5 and look at d5

# INPUT:

```csv
a,b,c,d,e,f,g,h
🌳,🌳,🌳,🌳,🌳,🚪,🌳,🌳
🌳,🌳,🌳,🌳,🌳,1,🌳,🌳
🌳,🦀,1,1,1,1,1,🌳
🌳,🌳,🌳,🌳,1,1,1,🌳
🌳,💰,1,💀,1,1,1,🌳
🌳,🌳,🌳,🌳,1,1,1,🌳
🌳,1,1,1,1,1,1,🌳
🌳,⛩️,🌳,🌳,🌳,🌳,🌳,🌳
```

# OUTPUT:
