#Note Shadowing

##Shadowing
```rust    
    let x = 5;

    let x = x + 1;

    let x = x * 2;
```
> By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
> The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name