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













```rs
trait Thing {
	fn doSomething(&self) -> usize;
}

struct MyThing {
	pub x: usize
}
impl Thing for MyThing {
	fn doSomething(&self) -> usize {
		*self.x
	}
}

fn getThing() -> impl Thing {
	MyThing()
}

fn main() {
	let my_thing: impl Thing = getThing();
	let val = goDoSomething(my_thing);
}

fn goDoSomething(thing: impl Thing) -> usize {
	thing.doSomething();
}
```
