# Hex Editor
Apply a parsing pattern to a binary file, colourising the different sections.


## Pattern Language
A continuous run of a given data type. Optionally provide an identifier to reference this data later.
```
<data_type> <count> <identifier|_>
```

Repeatedly apply the pattern in the brackets until we run out of bytes.
```
TAKE_UNTIL {
    ...
}
```

### Data types
```
u8 u16 u32 u64 u128
```
