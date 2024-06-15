# relaks
A CLI Music player for playing lofi songs for studying, productivity and relaxation after a tireful day.

### Required dependencies
The application requires the alsa package, a sound driver available for linux to be installed so that the program can run.

### Getting started
Once you have installed the alsa package with the package manager depending on your linux distro, the next steps are these:
If you have rust and cargo already installed and you are at the root folder of the project you can run the binary with the following terminal command: `cargo run --release` or `cargo run --release menu`.
Otherwise you can type: `./target/release/relaks` which plays automatically a playlist of predefined mp3 songs or you can type: `./target/release/relaks menu`to open the menu and select a specific song you want to play. To exit the program while a song is playing (or even paused) you can hit the "q" key. To pause a song you hit "enter" and to skip a song, assuming you are playing the playlist you can press the right arrow key, otherwise if you have chosen to play a specific song it will default to exit the program after you have pressed the key.