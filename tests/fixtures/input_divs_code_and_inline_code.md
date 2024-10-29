# Heading

## Details Boxes

This is some text

this latent will be preserved:
:::

:::alert

Some content

:::

A trailing block:

:::

Some other text


:::info

More content

:::tip

This is nested

:::

:::

A latent block
:::

## Code Blocks

### Hidden

```{rhai}
let s = 0;
for i in 1..10 {
    s += i;
}
```

### Display

```{rhai-display}
let t = "";
for i in 1..s {
    t += i;
    if i != (s-1) {
        t += " + ";
    }
}
t
```

### Inline Code

The sum of the first 10 numbers is λ#(s)# all together that is: λ#(t)#

