## INSTRUCTIONS

- Generate new csv map from below rules
- Think about placement, be creative don't just copy an example.
- Ensure and concise that walkable from entrance to exit, this is critical.

## RULES

- ➖: is walkable, must walkable from entrance to exit
- 🆕: is entrance for player spawn point, must have 1
- 🆒: is exit for player visit after clear stage, must have 1
- 🚪: place at the edge near entrance and exist point, must have 2
- 🌳: place at border, must have and optional randomly but walkable
- 💰: place randomly, must have, max 3
- 🪦: place randomly for enemies spawn point, max 2
- 🦀: as a NPC, max 1

## EXAMPLE_1

```csv
a,b,c,d,e,f,g,h
🌳,🚪,🌳,🌳,🌳,🌳,🌳,🌳
🌳,🆒,➖,➖,➖,➖,➖,🌳
🌳,🌳,🌳,➖,➖,➖,➖,🌳
🌳,💰,➖,➖,➖,🪦,➖,🌳
🌳,🌳,🌳,🌳,➖,➖,➖,🌳
🌳,🦀,➖,➖,➖,➖,➖,🌳
🌳,🌳,🌳,➖,➖,🆕,➖,🌳
🌳,🌳,🌳,🌳,🌳,🚪,🌳,🌳
```

## EXAMPLE_2

```csv
a,b,c,d,e,f,g,h
🌳,🌳,🌳,🌳,🌳,🚪,🌳,🌳
🌳,💰,➖,➖,➖,🆒,➖,🌳
🌳,➖,🌳,➖,➖,➖,🦀,🌳
🌳,🪦,➖,➖,➖,➖,➖,🌳
🌳,➖,🌳,➖,➖,➖,🪦,🌳
🌳,➖,🌳,🌳,➖,➖,🌳,🌳
🌳,🌳,🌳,🆕,➖,➖,➖,🌳
🌳,🌳,🌳,🚪,🌳,🌳,🌳,🌳
```
