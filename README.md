# A Crow's Commute
## author: 
Hayden Blanchard
## description:
A Crow's Commute is a simple 2D game built in Rust using the Bevy crate. The game is inspired by the way that crows will nest together in large groups during winter, often in the urban center of cities. While these crows will spend the night together they are often more seclusive in the day hours, commuting into the city in search for food either by themselves or in smaller groups. When deciding what to do for this project, I thought that this behavior in crows was interesting and something I wanted to base my game off of. The goal of the game is simple, fly the 2D crow over the city map and look for worms, while avoiding the hawk who is following the crow. 
## building program:
Warning: compiling this program may take a few minutes and will use over a GB of storage.   
cargo run --release
## comments:
Project is a WIP
## controls:
Forward: W or Up_Arrow  
Left: A or Left_Arrow  
Right: D or Right_Arrow  
Back: S or Down_Arrow 
## testing:
I am pretty new to Bevy and Rust in general, so my limited knowledge of testing tools and limited time to work on the project meant that I had to do most of my testing in the form of playtesting rather than assertion based testing.  
I tried my best to do my playtests in a unit testing kind of way, that being that I would create an individual function and then extensively test inputs or events for the expected result in game, before moving on to the next function. 
## results:
Overall I am pretty happy with how this project came out. Like I said in the testing section, Bevy and Rust are fairly new environments for me, so building this project was all around a learning experience.

