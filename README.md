![Alt text](/screenshots/snake_man.png)
# Playing the game via web
https://d7mnch.github.io/snake_rust_macroquad/
# Build the game from source
- you need to install rust via this commend 
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
- clone the rep with `git clone`
- then `cd` to that direcotry that you were cloned (if you cd to a subdirectory and not the root one, it will crash), then `cargo run`

# snake_rust_macroquad
building a snake game to learn more about rust using the `macroquad` library, and farming some greens to satisfy myself and feel like a real programmer

## Control the snake

| Keys | action                |
| ---- | --------------------- |
| `j`  | move snake down       |
| `k`  | move snake Up         |
| `h`  | move snake right      |
| `l`  | move snake left       |


## Settings
| Keys          | action                  |
| ------------- | ----------------------- |
| `Esc` or `q`  | quit the game           |
| `Space`       | suspend the game        |


## What i learn as a beginner while building this project
- make every struct a mod
- don't put another struct inside a struct as a parameter, instead use a struct them group them all (App)
- i kinda feel why people choose game engine especially for ui...
- fields should be owned by the struct (usually), but you can make sorta of a wrapper to that reference, that can represent identity(new information) of the struct !
    - like sprite, instead of having reference to the texture, i make spirte use an enum that represent that texture
- if you want to introduce references, pass them as arguments
- if fields are sub of other fields, just pick one field (no duplication)
- if a function deponds on multiple entities, make it a system not a method of a struct
- if a function/method identifier have `checking` that means it will return a bool

## TODO
- [ ] when restting, food will not spawn randomly
- [ ] food spawing early when the new spawned food collided with the snake body
- [ ] make Screenshots
- [ ] make a licence
- [ ] fix the [!note] thing
- [ ] watch other peoples code on how they build snake game (using GitHub and not Youtube)
- [ ] check update, then draw (remove snake being drawn when she hits it's tail)

