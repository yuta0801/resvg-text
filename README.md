resvg skips text elements even though fonts loaded

no texts are rendered in the output png file by the example

## steps to reproduce

### with docker

1. build: `docker build -t resvg-text .`
2. run: `docker run --rm -it -v $(pwd):/workspace -w /workspace resvg-text bash`
   ```
   cargo run image.svg image.png
   ```
3. cleanup: `docker rmi resvg-text`

### on local

it requires clang++ to build

`cargo run image.svg image.png`
