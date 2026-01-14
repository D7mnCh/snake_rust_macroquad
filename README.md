# Playing the game via web
https://d7mnch.github.io/snake_rust_macroquad/
# Build the game from source
- you need to install rust via this commend 
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
- clone the rep with `git clone`
- then cargo run

# snake_rust_macroquad
building a snake game to learn more about rust using the macroquad library, and farming some greens to satisfy myself and feel like a real programmer

## Control the snake

| Keys | action                |
| ---- | --------------------- |
| `j`  | move snake down       |
| `k`  | move snake Up         |
| `h`  | move snake right      |
| `l`  | move snake left       |

> [!note] why not arrow keys ?
> the answer is.. i like using vim motion, i don't really care if you don't like it...

## Settings
| Keys          | action                  |
| ------------- | ----------------------- |
| `Esc` or `q`  | quit the game           |
| `Space`       | suspend the game        |


## What i learn as a begginer while building this project

- make every struct a mod
- don't put a struct as a parameter on other struct like food have snake parameter on it's field, instead make a struct that group them (App), inside it (App) make the method there that need multiple struct


## TODO

- [ ] make Screenshots
- [ ] make a licence
