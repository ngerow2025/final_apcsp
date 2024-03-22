# Equation Analizyer

This Rust based project takes in user input in the form of a mathematical expression or equation and simplifies it.

## How to run

### interactive mode
Run
* `cargo run --release` if you downloaded the code or
* `./final_apcsp` if you downloaded the execuable file

To input this equation in interactive mode:

$$\frac{\sin(2x)}{\tan(x)}$$

You would input ```sin(2x)/tan(x)``` into the program and get ```2cos^2(x)``` as the output.

You can also specify `--web` to launch a web server to access a web interface at https://localhost:3000 by default. `--port` can optionally be specified to change the default port for the web interface.

### command line interface
Run
* `cargo run --release -- <input>` or
* `./final_apcsp <input>`

Either will give the output on a single line in stdout.



# bibliography

Rust Clap