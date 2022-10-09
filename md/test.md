# テスト

``` rust
fn chmin(a: &mut i32, b: i32) {

    // choose min value.
    // a: vector element (mutable reference)
    // b: reference number

    if *a > b {
        *a = b;
    }

}
```