# To the Temple of Doom!

This is an adaptation of the game by that name by Storybrewers Roleplaying available for free [here](https://storybrewersroleplaying.com/temple-of-doom/) to the web.

It allows a group of people to play the game online (voicechat is also necessary but not provided). So, this is not a game by itself, just a tool that manages the hidden information needed for playing, and also keeping track of the character stats and conditions.

## How to install

There is no convenient way to deploy the system currently, but that might come later. The application comes in two parts, the client side and the server side. Both are written in Rust, but the client side is compiled to Web Assembly, so it can be run in any modern web browser.

### Server Side

You need to install the Rust environment, for example via [rustup](https://rustup.rs). Then navigate to the root directory of this project and enter

```sh
cargo run
```

It will take care of downloading all dependencies, building the binary for the current platform and running it. Note that there is a config file called `config.yaml` you might need to edit. The program also accepts a limited amount of command line flags, you can find out more by running

```sh
cargo run -- --help
```

### Client Side

In addition to the Rust environment mentioned in the previous section, you also need to install `npm` for building the frontend. After you have done so, navigate to the `tttod_frontend` directory and enter

```sh
npm install
npm start
```

This will run the development environment (and also install the dependencies and compile the Rust part). Creating the files for web server deployment can be done with

```sh
npm run-script build
```

## Using the System

Most of the things should be self-explanatory and most of the text is included on the web pages anyways, but it helps to read the original rules in full to know what's going on (it's not a long document anyways). If you navigate to the main path of the web site, you can enter a game name. Every player who enters the same name will participate in the same game. You need 3 to 5 players to proceed, and the server does verify this number.

### Licenses & References

The sourcecode (and only the sourcecode!) in this repository is licensed under the AGPLv3. See [COPYING.txt](COPYING.txt) in this directory for more information.

Most of the text visible to the user has been copied from the original game linked in the first section. Permission has been kindly granted under the limitation that the software author earns no money from this work and that the original source is referenced. These terms have been complied with to the best of the ability of the software author. However, these parts are thus *not* licensed under the AGPLv3!

The images and fonts have been taken from various online sources under different licenses. Notably, the file named `explosion.jpg` is also only available for non-commercial purposes. You can find all of them here:

### Images

* https://www.pexels.com/photo/horse-with-trailer-in-front-of-petra-1631665/
* https://www.maxpixels.net/Cavern-Light-Escape-Cave-Ocean-Exit-View-Travel-4971298
* world explosion: https://wallpaperaccess.com/faq#personaluse

### Fonts

* https://www.ffonts.net/Adventure-Normal.font
* https://fonts.google.com/specimen/Piazzolla
