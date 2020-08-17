# WIP: TODO App

The main idea is to added in your bash profile in order to keep track of all your pending tasks, as everybody knows, the terminal always goes first ;)


## Usage

As it is still in development you must change your MongoDB cluster URI manually => src/lib.rs#134

```bash
cargo run add "Call Alice and explain cryptos to her"
cargo run list fd
cargo run del 1
```

![alt text](https://github.com/rogercoll/todo/blob/master/img/todolist.png?raw=true)
