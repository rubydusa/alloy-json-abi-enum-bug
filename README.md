# Alloy JSON-ABI solidity enum bug

It seems that enum type generation doesn't work with JSON-ABI specifications.

In `main.rs` there are two examples: one with inline solidity and one with a JSON-ABI import.

Here is the source for `Contract.json`:

```
pragma solidity ^0.8.0;

contract Contract {
    enum Enum {
        A,
        B,
        C
    }
}
```

