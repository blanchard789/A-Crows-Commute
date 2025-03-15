# A Crow's Commute
## Author: 
Hayden Blanchard

## Description:
A Crow's Commute is a simple 2D game built in Rust using the Bevy crate. The game is inspired by the way that crows will nest together in large groups during winter, often in the urban center of cities. While these crows will spend the night together they are often more secluded in the day hours, commuting into the city in search for food either by themselves or in smaller groups. When deciding what to do for this project, I thought that this behavior in crows was interesting and something I wanted to base my game around. 

The goal of the game is simple: fly the 2D crow over the city and look for 10 worms, while avoiding the hawk who follows you.

## Building program:
`cargo run --release`  

Warning: compiling this program may take a few minutes and will use over a GB of storage.  
If you are having issues with dependencies checkout: https://bevyengine.org/learn/quick-start/getting-started/setup/

## Controls:
Forward: `W` or `Up_Arrow`  
Left: `A` or `Left_Arrow`  
Right: `D` or `Right_Arrow`  
Back: `S` or `Down_Arrow`

## Testing:
I am pretty new to Bevy and Rust in general, so my limited knowledge of testing tools and limited time to work on the project meant that I had to do most of my testing in the form of playtesting rather than assertion based testing.  
My testing philosophy was to build out one function at a time, periodically testing portions of it as I built the function and then thoroughly testing it before moving on to the next function. Then in between developing new functions I would test the function against the rest of the code base to make sure that it was working alongside the other functions and to make sure that there weren't any unforeseen conflicts arising. I also made a habit to run cargo clippy and cargo fmt frequently to keep the code base clean. 

## Results:
Overall I am pretty happy with how this project came out. Like I said in the testing section, Bevy and Rust are fairly new environments for me, so building this project was all around a learning experience. That said, I think that most of my original vision for the project remained and made its way into the final product. I think the decision to stick to 2D was a smart choice, while it requires a more artistic approach to pull off well, I think code wise it paid off.

Like most first time experiences, starting the project was slow and not without its frustrations. But as the development progressed, I started to understand Bevy better and was able to go back and apply new knowledge on the older code. That said, I did have to shrink the scope of my project a bit compared to the original proposal to reach a deliverable project in time. But I was anticipating this and the things I cut were things I said I would only do if I had time. 

I don’t really have anything bad to say about this project. I am happy with the results and see it more as a complete prototype, so with that mindset I see it as a success. Of course like with many first attempts or really any project there are things you wished you did differently or wish you had more time for. But overall I think this was a good project to learn Rust and Bevy with, and I had a lot of fun doing it.

If I had more time to improve this project more. I would start with adding in the originally planned 3D obstacles that need to be avoided. I think the hawk chasing the crow is a good start, but at times the game can feel too easy and I think other obstacles would be a nice touch. I would also add a pause and settings menu, but I never planned for this to begin with given the time scale for this project.


## License: [LICENSE](LICENSE)

