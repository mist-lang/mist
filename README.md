```haskell
doSomething :: (String, Int) -> Int
doSomething (numString, num) = parse numString + num

doSomething' :: String -> Int -> Int
doSomething' numString num = parse numString + num

doSomething ("1", 1) -- legal
doSomething' "1" -- legal

doSomething "1" -- illegal
doSomething' ("1", 1) -- illegal
```

```rust
fn doSomething(numString: String, num: Int) -> Int {
	numString.parse() + num
}
```
