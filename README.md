# `product-with-repeat`

Provides the trait `ProductWithRepeat`, which has a single method:
`product_with_repeat`.

```rust
use product_with_repeat::ProductWithRepeat;

let numbers = vec![0, 1, 2, 3];

assert_eq!(
  numbers.product_with_repeat(3).collect::<Vec<_>>,
  vec![
    vec![&0, &0, &0], vec![&0, &0, &1], vec![&0, &0, &2], vec![&0, &0, &3],
    vec![&0, &1, &0], vec![&0, &1, &1], vec![&0, &1, &2], vec![&0, &1, &3],
    vec![&0, &2, &0], vec![&0, &2, &1], vec![&0, &2, &2], vec![&0, &2, &3],
    vec![&0, &3, &0], vec![&0, &3, &1], vec![&0, &3, &2], vec![&0, &3, &3],

    vec![&1, &0, &0], vec![&1, &0, &1], vec![&1, &0, &2], vec![&1, &0, &3],
    vec![&1, &1, &0], vec![&1, &1, &1], vec![&1, &1, &2], vec![&1, &1, &3],
    vec![&1, &2, &0], vec![&1, &2, &1], vec![&1, &2, &2], vec![&1, &2, &3],
    vec![&1, &3, &0], vec![&1, &3, &1], vec![&1, &3, &2], vec![&1, &3, &3],

    vec![&2, &0, &0], vec![&2, &0, &1], vec![&2, &0, &2], vec![&2, &0, &3],
    vec![&2, &1, &0], vec![&2, &1, &1], vec![&2, &1, &2], vec![&2, &1, &3],
    vec![&2, &2, &0], vec![&2, &2, &1], vec![&2, &2, &2], vec![&2, &2, &3],
    vec![&2, &3, &0], vec![&2, &3, &1], vec![&2, &3, &2], vec![&2, &3, &3],
    
    vec![&3, &0, &0], vec![&3, &0, &1], vec![&3, &0, &2], vec![&3, &0, &3],
    vec![&3, &1, &0], vec![&3, &1, &1], vec![&3, &1, &2], vec![&3, &1, &3],
    vec![&3, &2, &0], vec![&3, &2, &1], vec![&3, &2, &2], vec![&3, &2, &3],
    vec![&3, &3, &0], vec![&3, &3, &1], vec![&3, &3, &2], vec![&3, &3, &3],
  ]
);
```
