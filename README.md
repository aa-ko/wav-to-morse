# wav-to-morse

CLI tool to extract Morse code beeps from a .wav file and translate it to ASCII.

## TODO
* Correctly insert spaces when LongPause is detected
* Use clap to build a real command line interface
* It might be beneficial for performance to not collect all iterators all the time, but I might be wrong about this
* Move parsing logic into a library and seperate it from the CLI wrapper
* Run FFT on each sample, ideally in parallel