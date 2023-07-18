# Word Search App

This is a simple word search application written in Rust using the eframe and egui libraries. The app allows you to find words from a list based on the length and pattern you provide.
Dependencies

- eframe: The main application framework.
- egui: A GUI library used for creating the user interface.
- clipboard: A library for interacting with the system clipboard.
- csv: A library for reading CSV files.

# Usage

  Enter the word pattern you want to search in the text input.
  Press "Enter" or click outside the text input to trigger the search.
  The app will display the search time, the number of matching results, and a list of matching words.
  Click on a word from the list to copy it to the clipboard.
  Use the slider to adjust the length of the search pattern.

# How It Works

The app reads a list of words from the "Skribbl-words.csv" file, where each word is paired with its length. It then matches the search pattern against the words in the list based on the following criteria:

  - The length of the word must match the length of the search pattern.
  - The search pattern may contain question marks '?' as placeholders for characters to be matched in the word.

# Building and Running

To build and run the application, ensure you have Rust and Cargo installed on your system. Then, execute the following commands:
```
cargo run
```
For better performance, you can run the application in release mode:
```
cargo run --release
```

The application window will appear, and you can interact with it as described in the "Usage" section.

# Notes

  The initial window size is set to 320x240 pixels but can be adjusted as needed in the main function.
  The TEXT_SIZE constant determines the font size used in the user interface.

Enjoy exploring words with the Word Search App!
