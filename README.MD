## Run Once
Code inside the closure (of the same value*) will only run once.
```rust
r.run_once(1, || {
  // do stuff here!
});
```

Two closures of different values will both run once.
```rust
r.run_once(1, || {});
r.run_once(2, || {});
```

## Example
```rust
fn test() {
  let mut data = RunOnce::default();
  let mut value = 0;

  for _ in 0..5 {
    data.run_once(1, || {
      value += 1;
    });
  }
  assert_eq!(value, 1);
}
```

