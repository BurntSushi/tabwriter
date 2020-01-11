tabwriter-bin
=============
Simple example of command line utility

```bash
[andrew@Liger tabwriter] cat sample | sed 's/   /\\t/g'
a\tb\tc
abc\tmnopqrstuv\txyz
abcmnoxyz\tmore text

a\tb\tc
[andrew@Liger tabwriter] ./target/tabwriter < sample
a          b           c
abc        mnopqrstuv  xyz
abcmnoxyz  more text

a   b   c
```

Notice that once a column block is broken, alignment starts over again.
