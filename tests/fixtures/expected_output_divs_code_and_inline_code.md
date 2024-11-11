# Heading

## Details Boxes

This is some text

this latent will be preserved:
:::

<div role="alert" class="alert alert-info">

Some content

</div>

A trailing block:

:::

Some other text


<div class="alert alert-info">

More content

<div class="tip">

This is nested

</div>

</div>

A latent block
:::

## Code Blocks

### Hidden


### Display

<div class="rhai-display">

```rust
let t = "";
for i in 1..s {
    t += i;
    if i != (s-1) {
        t += " + ";
    }
}
t
```
<div class="rhai-out">

```
Error: Function not found: + (&str | ImmutableString | String, i64) (line 3, position 7)
```
</div>
</div>

### Inline Code

The sum of the first 10 numbers is 45 all together that is: 