# WordVault
A small CLI utility for helping you learn japanese words made in rust 🦀

# How does it work?

This is supposed to be used in conjunction with [10ten japanese reader](https://github.com/birchill/10ten-ja-reader) browser extension, so you can hover over words in the browser to see the definition.

WordVault works with the concept of ✨**discovered words**✨, when you find a new word on the internet you had to use 10ten japanese reader because you didn't know it, you can copy it to wordvault and then review the words later using **spaced repetition**.

WordVault has 4 basic commands:
- `wv add <word>`: Adds a new ✨**discovered word**✨
- `wv list`: Lists your ✨**discovered words**✨ with the successes, failures and success rate in reviews
- `wv remove <word>`: Remove a ✨**discovered word**✨
- `wv review`: Starts a new review

## How the reviews work?

WordVault will ask the meaning or the reading of all the words in your vault, the order and frequency of which they appear will be sorted by the success rate, if you ever used something like duolingo before this will feel familiar.