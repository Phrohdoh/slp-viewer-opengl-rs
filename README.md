# `slp-viewer-opengl`

 An experiment in drawing SLPs using OpenGL via [glium](https://github.com/tomaka/glium)

## License

MIT

## Usage

Assuming you have extracted an slp and a palette graphics.drs:

```sh
$ cargo run -- --pal-path ~/Desktop/50500.bin --player 1 --slp-path ~/Desktop/00412.slp
```