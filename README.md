# relaks
A CLI Music player for playing lofi songs for studying, productivity and relaxation after a tireful day.

### Required dependencies
The application requires the alsa package, a sound driver available for linux to be installed so that the program can run.

### Getting started
Once you have installed the alsa package with the package manager depending on your linux distro, the next steps are these:
If you have rust and cargo already installed and you are at the root folder of the project you can build and run the binary with the following terminal command: `cargo run --release` which plays automatically a playlist of predefined mp3 songs or you can type `cargo run --release menu` to build and run the binary and open the menu to select a specific song you want to play. 
Otherwise type either `./relaks`or `./relaks menu` to run the compiled binary directly.
To exit the program while a song is playing (or even paused) you can hit the "q" key or if you are in the menu hit the escape key. To pause a song you hit "enter" and to skip a song, assuming you are playing the playlist you can press the right arrow key, otherwise if you have chosen to play a specific song it will default to exit the program after you have pressed the key. Lastly you can type the binary name with the -h or --help tag to display helpful commands on the screen.
