# Notes and new things I learned
- `echo $?` will print the result of the last program run in
bash
- using `{:#?}` in a `println!` will pretty print output
- match can take a guard, after the pattern match, ie: 

```match val.parse() {
    Ok(n) if n > 0 => Ok(n),
    _ => Err(From::from(val)),
}
```
        
