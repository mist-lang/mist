```haskell
doSomething :: (String, Int) -> Int
doSomething (numString, num) = parse numString + num

doSomething' :: String -> Int -> Int
doSomething' numString = \num -> parse numString + num
```

```rust
fn doSomething(numString: String, num: Int) -> Int {
	numString.parse() + num
}
```

```ocaml
let doSomething (numString: String) (num: Int): Int = assert false

let doSomething': String -> Int -> Int =
	fun numString num ->
		assert false

fun doSomething (numString: String) -> (num: Int) -> Int = numString.parse() + num;
```

```kotlin
fun doSomething(numString: String) = fun(num: Int): Int = numString.parse() + num;
fun doSomething(numString: String) = {num: Int -> numString.parse() + num}

fun doSomething(numString: String): (num: Int): Int = numString.parse() + num;
```
