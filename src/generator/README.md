
# Parser Documentation

See the [abc standard](https://abcnotation.com/wiki/abc:standard:v2.1#the_tune_body)!

Each note in `.abc` follows a specific pattern (we are ignoring some parts for simplicity at this time):

```
[accidentals][pitch][octave][note length]
```

## Symbols

### Accidentals

- `^` = sharp (can be repeated, i.e. double sharp is `^^`)
- `=` = natural
- `_` = flat (can be repeated, i.e. double flat is `__`)

### Pitch 

- `a` - `g` = low A through G
- `A` - `G` = high A through G
- `x`, `z`, `Z` = rest (the difference is only in how it should be displayed) 

### Octave

- `'` = rise an octave
- `,` = lower an octave

Note: there can be any number of octave symbols

### Note Lengths

(these are exclusive options)
- some amount of `0` - `9` = length
- `/` followed by some amount of `0` - `9` = fractional length
- `/` = shorthand for `/2`

## Other Important Things

- notes only come after a line starting with `K:` (which is a header field, more specfically the one for the key, but for now we are ignoring it)
- `%` = the rest of the line is a comment and thus should be ignored
- `|` = a visual bar, we ignore this
- `[` and `]` = indicate a chord, but we will not support this for now
- `\n` = empty lines are technically allowed, though some programs that I've used don't like them