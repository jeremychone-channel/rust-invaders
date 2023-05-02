NEW YouTube full tutorial - [Rust Bevy - Full Tutorial - Game Dev](https://www.youtube.com/watch?v=j7qHwb7geIM&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q) 

CODE UPDATED to **Bevy 0.10** - Thanks to [ehasnain](https://github.com/ehasnain)

Rust [Bevy](https://bevyengine.org/) - Game Dev tutorials

### Latest Bevy Version updates

- 2023-05-02 - Updated to Bevy 0.10. Thanks to [ehasnain](https://github.com/ehasnain)
- 2022-12-13 - Updated to Bevy 0.9. Thanks to [GiulianoCTRL](https://github.com/GiulianoCTRL)
- 2022-08-07 - Main branch updated to Bevy 0.8. Thanks to [DomagojRatko](https://github.com/DomagojRatko)

### MacOS Setup

- Ensure [Rust and Cargo are installed](https://www.rust-lang.org/tools/install)
- Delete the Cargo.lock, as it may otherwise raise an error the first time: `rm Cargo.lock`
- Install Cmake with [Homebrew](https://brew.sh/): `brew install cmake`
- Install Cargo Watch: `cargo install cargo-watch`

### Fedora (Linux) requirements

Ensure to have on your system with a package manager of your choice

```
dnf install rust-alsa-sys-devel
dnf install rust-libudev-devel
```
Thanks to [janpauldahlke](https://github.com/janpauldahlke)

### Development

For rapid development: 
- `cargo run --features bevy/dynamic`
- `cargo watch -q -c -x 'run --features bevy/dynamic'`

- Other Rust videos:
    - Weekly Rust Videos at [Jeremy Chone](https://www.youtube.com/jeremychone) channel
    - [Rust Game Development Tutorials](https://youtube.com/playlist?list=PL7r-PXl6ZPcCB_9zZFU0krBoGK3y5f5Vt)

### Change log

- 2022-12-13 - Updated to Bevy 0.9. Thanks to [GiulianoCTRL](https://github.com/GiulianoCTRL)
- 2022-08-07 - Main branch updated to Bevy 0.8. Thanks to [DomagojRatko](https://github.com/DomagojRatko)
- 2022-08-07 - Main branch updated to Bevy 0.8 thanks to [@DomagojRatko](https://github.com/DomagojRatko)
- 2022-05-09 - Updated to new tutorial for v0.7. See [Rust Bevy - Full Tutorial - Game Dev](https://www.youtube.com/watch?v=j7qHwb7geIM&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q) 
- 2022-01-28 - Code has been updated to **Bevy v0.6** !!!
- 2021-06-25 - Initial (for old tutorial [Episode 1](https://youtu.be/Yb3vInxzKGE), [Episode 2](https://youtu.be/Dl4PJG0eRhg), [Episode 3](https://youtu.be/4nEUX2hf2ZI))


## Rust & Bevy & Other Resources

Topics learned and used:

- Rust Programming for Game Development
- Game ECS Engine Bevy
- Bevy System, Bevy Components, Bevy Resources
- Bevy Plugins
- Bevy entities spawn and despawn
- Bevy SpriteBundle, Sprite Sheet (SpriteAtlas)
- Bevy Timesteps, Time, and Timer
- Bevy custom system criteria
- Indirection strategy to spawn explosions
- Sprite Sheet atlas for sprite animations
- Basic Rust Programming
- Rust module
- Rust closure
- Rust matches

Resources: 

- Rust: 
    - [Rust Book](https://doc.rust-lang.org/book/)
    - [Rust Video Course](https://www.youtube.com/playlist?list=PL7r-PXl6ZPcB4jn1_VR3D8tSK9DxOaiQE)
- Bevy: 
    - [Official Bevy Web Site](https://bevyengine.org/)
    - [Official Bevy Book](https://bevyengine.org/learn/book/introduction/)
    - [Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/)
    - [Official Bevy API Doc](https://docs.rs/bevy/latest/bevy/index.html)
    - [Official Bevy Assets](https://bevyengine.org/assets/)
    - [Offical GitHub examples](https://github.com/bevyengine/bevy/tree/latest/examples)
    - [Great Blog Post - snake game](https://mbuffett.com/posts/bevy-snake-tutorial/)
- Assets: 
    - [Player, Laser, Enemy Sprites](https://opengameart.org/content/space-shooter-redux)
    - [Explosion](https://opengameart.org/content/explosion)    


<br /><br /><br />
[This repo](https://github.com/jeremychone-channel/rust-invaders)