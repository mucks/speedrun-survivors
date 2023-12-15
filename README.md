## Speedrun Survivors

A game for true crypto degens. Play as Pepe and punish hordes of enemies with your mighty arsenal of weapons. If it is called Speedrun Survivors, it may make senes to make this like an area game where there are waves of enemies and one can kill them in more or less time... This could in turn affect score points (in-game gold) rewards gained per game.



### Inspiration
- Vampire Survivors
- Worms


### Player Character
- Pepe
- Bonk Inu
- could be different NFT hero characters added over time
  - with different base stats
  - theoretical collaboration with any NFT collection


### Character stats
- Health
- Health regen
- Movement speed
- weapon cooldown +/- x %


### Weapons
- possible Manual Aim (Primary; ACTIVE)
    - CONSIDER THIS: manual aim -> complex REPLAY check
        - fire on click -> with rotation in 256 increments (u8)
        - this whole game is about attack spamming
            - not realistic to constantly update rotation
        -  manuual aim main attack must be slow (cooldown) and insanely powerful (fun) if added
    - BEAM rifle
    - RPG
- Passive Attacks
    - Grenade launcher
        - Napalm AoE
        - Explosive AoE
    - Nova attacks
        - Electric shock
        - Fire
    - Flame Thrower
    - Drones
        - Man-Hack (Half Life 2)
        - Shooting
    - Laser beams


### Passive Buffs
- Invincibility on DMG
- More XP
- More Gold
- AoE boost
- DMG boost
- Cooldown boost
- Health boost
- Health regen boost
- Movement speed boost
- Magnet for gold / item Pickup
- some "debuffs" like faster enemy spawn or more enemy spawn
    - that way players can get higher scores if they survive it


### Undead system
- hero gains 100% attack speed
- set on timer of 10 - 30 sec
- will explode in huge blast if time runs out
- has to either
    - find a shrine of ressurrection in due time
    - defeat some Death boss enemy and drain/posess their soul or whatever for a ressurrection
    - this could possibly influence the look of their NFT
        - becomes more death-ish (ghoul like) looking over time? (how to do that; need upgradeable NFTs)


### Tech thoughts


#### Replays
- generate some u256 random number to derive (seed) all "randomness" from
- this is the replay GUID
- to prevent replay attacks, each GUID should be globally unique forever
- all user input is logged.
- Server replays should arrive at the same outcome
    - award NFTs and Tokens
    - requires single threaded RNG access I suppose
- Protocol thoughts
    - 1 byte action OP code
    - 3 byte tick (= 60 tickrate)
        - 3 bytes allow for 4660 minutes = 77 hours
    - = 4 byte per WASD action (@5APS -> 9000 actions -> 36000 byte -> ~35 KB)
    - Could compress by using several OPCODEs
        - W_ON_2 +2 byte
        - W_ON_3 +3 byte
        - A_ON_2 +2 byte
        - A_ON_3 +3 byte
        - ... S, D (_ON_ and _OFF_)(=16 OpCodes)
        - FIRE +1 byte (rotation 256 degrees then -> 1.4° increments)


### TODO
- render a map using Tiled +ldk
    - https://github.com/Trouv/bevy_ecs_ldtk/tree/main
- render player sprite (or bone 2d animation if that exists)
    - https://bevyengine.org/examples/2D%20Rendering/sprite-sheet/
    - https://bevyengine.org/examples/Stress%20Tests/many-animated-sprites/
- add player movement
- add player animation
- render enemy
- add spawn system for enemies
- move enemy in straight line to player (no collisions; no pathfinding)
- damage taking system
- death, undead system
- add weapons
- add power ups
- UI
    - DEBUG
        - FPS
    - TIME
    - LEVEL PROGRESS
    - KILL COUNT
    - GOLD ACQUIRED
    - HEALTH BAR
    - WEAPON & PASSIVE icons
    - DMG texts (japanese style)
    - https://bevyengine.org/examples/UI%20(User%20Interface)/grid/
    - https://bevyengine.org/examples/UI%20(User%20Interface)/ui-scaling/
- recording system
    - playback system
    - headless server playback
- API to upload game replay for processing
- SFX
    - https://bevyengine.org/examples/Audio/spatial-audio-2d/
- VFX
    - https://bevyengine.org/examples/2D%20Rendering/bloom-2d/


### NFTs
What would be a resale value? What would make them tradeable?

- Cool looks
- heros are NFTs with various base stats
    - i.e. Pepe is fast but weak, ...
    - must not be required to play (can always play mint NFT after then game to "save state")
        - always a free mint of base hero
- NFTs as powerups; equipped inventory style at the start of a game
- NFTs as weapons; equipped inventory style at the start of a game


How to not flood the market and make them worthless?

- Derived & bind to ladder mode / season / cohort
- lost on death?
- maybe some "craft" logic that lets users combine / upgrade their NFTs but there is some chance it fails and all NFTs get deleted?
    - Lineage enchanting style


### Token
Launch without a token but keep track of player scores (in-game gold) - try to gain followers, generate hype and then do an airdrop to players and or twitter followers or galxe quest etc.

This token could then be used to:
- buy NFTs, upgrades etc.
- unlock new maps, game modes, ...
- join ranked ladders ("pro play")