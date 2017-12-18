# `slp-viewer-opengl`

 An experiment in drawing SLPs using OpenGL via [glium](https://github.com/tomaka/glium)

## License

MIT

## Usage

Assuming you have [extracted an SLP and a palette from graphics.drs](https://github.com/ChariotEngine/drs-studio/):

```sh
$ cargo run -- --pal-path 50500.bin --player 1 --slp-path 00412.slp --frame-index 34
```
