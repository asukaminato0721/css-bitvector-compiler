# CSS Bitvector Compiler

launch ladybird, visit google.com, type something

get csslog and trace file

rename csslog to css, trace to google.trace

put into css-gen-op folder


```
./run.sh google
```


or

```
./run.sh amazon
```

run > 2 times to stablize the test.

## Project Overview

steps

1. from css, generate 3 version of code, 1 is no optimization, 1 is only bitVector, 1 is `Vec<IState>` which has 3 states
2. run them on html changes, get 3 results. diff them to see all these 3 versions are correct.
3. run benchmark, see the diff between bitVector and IState

