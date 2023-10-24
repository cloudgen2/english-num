# english-num
An english number exercise written in Rust. This project uses the online-downloader script in https://github.com/cloudgen2/online-installer

## Online Installation
```
curl -fsSL https://dl.leolio.page/english-num/ | python3
```
For instance, if you are using mac, the download address should be:
```
https://dl.leolio.page/english-num/aarch64-clang/0.2/english-num.tar.gz
```

## Run the source code
```
cargo run
```

## Build release
```
cargo build --release
rm -rf ~/.local/bin/english-num
cp target/release/english-num ~/.local/bin/
```

## Execution example

```
############################
#
# english-num v.0.4.0
# Updated at: 2023-10-14
#
############################

Type 'exit' to terminate the program!

== First Level, L1 ==
1) What is the number 7 in English? seven
» That's correct!
2) What is the number 10 in English? ten
» That's correct!
3) What is the number 7 in English? seven
» That's correct!
4) What is the number 10 in English? 10
» '10'.
» The correct answer is 'ten'.
 ** 4) What is the number 10 in English? four
» 'four'.
» The correct answer is 'ten'.
5) What is the number 7 in English? seven
» That's correct!
6) What is the number 4 in English? four
» That's correct!
7) What is the number 8 in English? eight
» That's correct!
8) What is the number 7 in English? seven
» That's correct!
9) What is the number 19 in English? nineteen
» That's correct!
10) What is the number 12 in English? twelve
» That's correct!
11) What is the number 11 in English? eleven
» That's correct!
12) What is the number 14 in English? fourteen
» That's correct!
13) What is the number 18 in English? eighteen
» That's correct!
14) What is the number 13 in English? thirteen
» That's correct!
15) What is the number 15 in English? fifteen
» That's correct!
16) What is the number 4 in English? four
» That's correct!
17) What is the number 7 in English? seven
» That's correct!
18) What is the number 17 in English? seventeen
» That's correct!
19) What is the number 4 in English? four
» That's correct!
20) What is the number 16 in English? sixteen
» That's correct!
== Level 2 L2 ==
21) What is it? ( 1🍌 )? This is a banana.
» That's correct!
22) What is it? ( 10🍌 )? 
```

Happy Programming!
