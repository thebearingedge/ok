ok
--

👌 Alright JSON Object validation.

`ok` is a simple JSON Object schema builder and validation library with an ergonomic API. An `ok` schema is a tree of structs implementing the `OkSchema` trait to validate `serde` JSON values. The validation result will either contain the validated JSON payload or an error listing the unsatisfied validation tests.

### Example

```rust
use ok::{OkSchema, object};

let user_schema = object()
    .string("username", |string| {
        string
            .min_length(1)
            .max_length(20)
    })
    .integer("luckyNumber", |integer| {
        integer
            .not_one_of(vec![2, 3, 5, 7, 11, 13, 17])
    });
```

In the above example a `user_schema` is created to validate an entire JSON Object modeling a `User`. The `object()` function returns an `ObjectSchema` with `string` and `integer` methods to register subschemas at the properties `"username"` and `"luckyNumber"`, respectively.
