# Command Line Rust
This repo is to deepen my understanding of how to program in Rust from the book Command Line Rust; which challenges you to implement common bash commands.
https://www.amazon.com/Command-Line-Rust-Project-Based-Primer-Writing/dp/1098109430

# Notes / Things I Learned
- bash command, `echo $?` will print the result of the last program run in
bash
- using `{:#?}` in a `println!` will pretty print output
- match can take a guard, after the pattern match, ie: 

```match val.parse() {
    Ok(n) if n > 0 => Ok(n),
    _ => Err(From::from(val)),
}
```
        
