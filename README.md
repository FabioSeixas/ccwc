# ccwc in Rust
Coding Challenges - wc tool

https://codingchallenges.fyi/challenges/challenge-wc

## Next steps

In this step your goal is to support being able to read from standard input if no filename is specified. 
If you’ve done it right your output should match this:

> cat test.txt | ccwc -l 
    7137


## Improvements as a way to learn

1. Define and Implement an object (Struct) to deal with user inputs. 
- Methods to validate and send to the correct flow according to inputs.
- Use std::fs to check if file exists

2. Define and Implement an object to deal with core flow
- file read
- buffer parse
- bytes count
- etc...

