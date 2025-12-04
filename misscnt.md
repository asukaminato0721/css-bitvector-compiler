| Folder | MISS\_CNT | TRI MISS\_CNT | QUAD MISS\_CNT | bit vs tmp | tri vs tmp | quad vs tmp |
|---|---:|---:|---:|:---:|:---:|:---:|
| a_to_b | 2 | 2 | 2 | OK | OK | OK |
| amazon | 2448 | 2448 | 2448 | OK | OK | OK |
| bilibili | 5591 | 5591 | 5456 | OK | OK | DIFF |
| bing | 310 | 310 | 300 | OK | OK | OK |
| bootstrap | 9513 | 9481 | 3236 | OK | OK | OK |
| google | 3712 | 3704 | 3569 | OK | OK | OK |
| netflix | 2527 | 2479 | 2113 | OK | OK | OK |
| reddit | PSEUDO_SKIPPED[bit]    eg .even\:z-1:nth-child(2n) | thread 'main' (2442166) panicked at src/tri.rs:977:10: | thread 'main' (2442176) panicked at src/quad.rs:1105:10: | DIFF | DIFF | DIFF |
| testcase | 10 | 7 | 7 | OK | OK | OK |
| tiktok | 2634 | 2634 | 2365 | DIFF | DIFF | DIFF |
| whatsapp | 1607 | 1607 | 1595 | OK | OK | OK |
| wikipedia | 8778 | 8771 | 2820 | OK | OK | DIFF |
| yahoo | 489 | 488 | 477 | OK | OK | OK |
| youtube | 17402 | 17402 | 17216 | OK | OK | OK |
