# SCLang - a toy LISP-like circular data storage language with deterministic auto memory management overhead (NO NORMAL GC OVERHEAD REQUIREMENT)

LICENSE: MIT OR APACHE 2.0

USING REFERENCE MANAGEMENT CELLS WITH HELP FROM STRONG & WEAK REFERENCES TO AVOID THE NEED FOR PERIODIC OR RANDOM GC OVERHEAD

STATUS: EXPERIMENTAL WITH NO GUARANTEES WHATSOEVER NEEDS MASSIVE CLEANUP & LIKELY NEEDS MASSIVE OPTIMIZATION, MULTI-THREADING NOT EXPECTED TO WORK

HOW (DON"T ASK): Using cell-like data structures with strong RC references & weak RC references to manage data cell & data cell linkage lifetime - XXX TODO NEED TO DOCUMENT & EXPLAIN THIS

MAJOR TODO ITEM IS TO SUPPORT `no_std` environment for EMBEDDED SYSTEMS

TO RUN TEST:

```sh
cargo test
```

TO RUN INTERACTIVE CLI DEMO:

```sh
cargo run --example i-cli
```

SAMPLE INTERACTIVE SESSION THAT DEMONSTRATES STORING CIRCULAR-LINKED DATA CELLS WITH DEBUG PRINT OUTPUT INCLUDED, ABLE TO REMOVE & CLEAN UP UNREACHABLE CELLS IN THE END

```sh
% cargo run --example i-cli
--> (store-data data-node-a ("a-text-1" "a-text-2"))
STORED DATA FOR SYMBOL - data-node-a
- text 1: "a-text-1"
- text 2: "a-text-2"
- link 1 - empty
- link 2 - empty

--> (store-data data-node-b ("b-text-1" "b-text-2" (data-node-a data-node-a)))
STORED DATA FOR SYMBOL - data-node-b
- text 1: "b-text-1"
- text 2: "b-text-2"
- link 1 info:
  link 1 info - text 1: "a-text-1"
  link 1 info - text 2: "a-text-2"
  - link 1 -> link 1 - empty
  - link 1 -> link 2 - empty
- link 2 info:
  link 2 info - text 1: "a-text-1"
  link 2 info - text 2: "a-text-2"
  - link 2 -> link 1 - empty
  - link 2 -> link 2 - empty

--> (update-data data-node-a ("a-text-1" "a-text-2" (data-node-b data-node-b)))
UPDATED DATA FOR SYMBOL - data-node-a
- text 1: "a-text-1"
- text 2: "a-text-2"
- link 1 info:
  link 1 info - text 1: "b-text-1"
  link 1 info - text 2: "b-text-2"
  - link 1 -> link 1 info - text only:
    link 1 -> link 1 info - text 1: "a-text-1"
    link 1 -> link 1 info - text 2: "a-text-2"
  - link 1 -> link 2 info - text only:
    link 1 -> link 2 info - text 1: "a-text-1"
    link 1 -> link 2 info - text 2: "a-text-2"
- link 2 info:
  link 2 info - text 1: "b-text-1"
  link 2 info - text 2: "b-text-2"
  - link 2 -> link 1 info - text only:
    link 2 -> link 1 info - text 1: "a-text-1"
    link 2 -> link 1 info - text 2: "a-text-2"
  - link 2 -> link 2 info - text only:
    link 2 -> link 2 info - text 1: "a-text-1"
    link 2 -> link 2 info - text 2: "a-text-2"

--> (store-data data-node-c ("c-text-1" "c-text-2" (data-node-a data-node-b)))
STORED DATA FOR SYMBOL - data-node-c
- text 1: "c-text-1"
- text 2: "c-text-2"
- link 1 info:
  link 1 info - text 1: "a-text-1"
  link 1 info - text 2: "a-text-2"
  - link 1 -> link 1 info - text only:
    link 1 -> link 1 info - text 1: "b-text-1"
    link 1 -> link 1 info - text 2: "b-text-2"
  - link 1 -> link 2 info - text only:
    link 1 -> link 2 info - text 1: "b-text-1"
    link 1 -> link 2 info - text 2: "b-text-2"
- link 2 info:
  link 2 info - text 1: "b-text-1"
  link 2 info - text 2: "b-text-2"
  - link 2 -> link 1 info - text only:
    link 2 -> link 1 info - text 1: "a-text-1"
    link 2 -> link 1 info - text 2: "a-text-2"
  - link 2 -> link 2 info - text only:
    link 2 -> link 2 info - text 1: "a-text-1"
    link 2 -> link 2 info - text 2: "a-text-2"

--> (update-data data-node-b ("b-text-1" "b-text-2" (data-node-a data-node-c)))
UPDATED DATA FOR SYMBOL - data-node-b
- text 1: "b-text-1"
- text 2: "b-text-2"
- link 1 info:
  link 1 info - text 1: "a-text-1"
  link 1 info - text 2: "a-text-2"
  - link 1 -> link 1 info - text only:
    link 1 -> link 1 info - text 1: "b-text-1"
    link 1 -> link 1 info - text 2: "b-text-2"
  - link 1 -> link 2 info - text only:
    link 1 -> link 2 info - text 1: "b-text-1"
    link 1 -> link 2 info - text 2: "b-text-2"
- link 2 info:
  link 2 info - text 1: "c-text-1"
  link 2 info - text 2: "c-text-2"
  - link 2 -> link 1 info - text only:
    link 2 -> link 1 info - text 1: "a-text-1"
    link 2 -> link 1 info - text 2: "a-text-2"
  - link 2 -> link 2 info - text only:
    link 2 -> link 2 info - text 1: "b-text-1"
    link 2 -> link 2 info - text 2: "b-text-2"

--> (store-data data-node-d ("d-text-1" "d-text-2" (data-node-a data-node-c)))
STORED DATA FOR SYMBOL - data-node-d
- text 1: "d-text-1"
- text 2: "d-text-2"
- link 1 info:
  link 1 info - text 1: "a-text-1"
  link 1 info - text 2: "a-text-2"
  - link 1 -> link 1 info - text only:
    link 1 -> link 1 info - text 1: "b-text-1"
    link 1 -> link 1 info - text 2: "b-text-2"
  - link 1 -> link 2 info - text only:
    link 1 -> link 2 info - text 1: "b-text-1"
    link 1 -> link 2 info - text 2: "b-text-2"
- link 2 info:
  link 2 info - text 1: "c-text-1"
  link 2 info - text 2: "c-text-2"
  - link 2 -> link 1 info - text only:
    link 2 -> link 1 info - text 1: "a-text-1"
    link 2 -> link 1 info - text 2: "a-text-2"
  - link 2 -> link 2 info - text only:
    link 2 -> link 2 info - text 1: "b-text-1"
    link 2 -> link 2 info - text 2: "b-text-2"

--> (store-data data-node-e ("e-text-1" "e-text-2" (data-node-a data-node-d)))
STORED DATA FOR SYMBOL - data-node-e
- text 1: "e-text-1"
- text 2: "e-text-2"
- link 1 info:
  link 1 info - text 1: "a-text-1"
  link 1 info - text 2: "a-text-2"
  - link 1 -> link 1 info - text only:
    link 1 -> link 1 info - text 1: "b-text-1"
    link 1 -> link 1 info - text 2: "b-text-2"
  - link 1 -> link 2 info - text only:
    link 1 -> link 2 info - text 1: "b-text-1"
    link 1 -> link 2 info - text 2: "b-text-2"
- link 2 info:
  link 2 info - text 1: "d-text-1"
  link 2 info - text 2: "d-text-2"
  - link 2 -> link 1 info - text only:
    link 2 -> link 1 info - text 1: "a-text-1"
    link 2 -> link 1 info - text 2: "a-text-2"
  - link 2 -> link 2 info - text only:
    link 2 -> link 2 info - text 1: "c-text-1"
    link 2 -> link 2 info - text 2: "c-text-2"

--> (update-data data-node-a ("a-text-1" "a-text-2" (data-node-b data-node-e)))
UPDATED DATA FOR SYMBOL - data-node-a
- text 1: "a-text-1"
- text 2: "a-text-2"
- link 1 info:
  link 1 info - text 1: "b-text-1"
  link 1 info - text 2: "b-text-2"
  - link 1 -> link 1 info - text only:
    link 1 -> link 1 info - text 1: "a-text-1"
    link 1 -> link 1 info - text 2: "a-text-2"
  - link 1 -> link 2 info - text only:
    link 1 -> link 2 info - text 1: "c-text-1"
    link 1 -> link 2 info - text 2: "c-text-2"
- link 2 info:
  link 2 info - text 1: "e-text-1"
  link 2 info - text 2: "e-text-2"
  - link 2 -> link 1 info - text only:
    link 2 -> link 1 info - text 1: "a-text-1"
    link 2 -> link 1 info - text 2: "a-text-2"
  - link 2 -> link 2 info - text only:
    link 2 -> link 2 info - text 1: "d-text-1"
    link 2 -> link 2 info - text 2: "d-text-2"

--> (show-data data-node-a)
DATA FOR SYMBOL - data-node-a
- text 1: "a-text-1"
- text 2: "a-text-2"
- link 1 info:
  link 1 info - text 1: "b-text-1"
  link 1 info - text 2: "b-text-2"
  - link 1 -> link 1 info - text only:
    link 1 -> link 1 info - text 1: "a-text-1"
    link 1 -> link 1 info - text 2: "a-text-2"
  - link 1 -> link 2 info - text only:
    link 1 -> link 2 info - text 1: "c-text-1"
    link 1 -> link 2 info - text 2: "c-text-2"
- link 2 info:
  link 2 info - text 1: "e-text-1"
  link 2 info - text 2: "e-text-2"
  - link 2 -> link 1 info - text only:
    link 2 -> link 1 info - text 1: "a-text-1"
    link 2 -> link 1 info - text 2: "a-text-2"
  - link 2 -> link 2 info - text only:
    link 2 -> link 2 info - text 1: "d-text-1"
    link 2 -> link 2 info - text 2: "d-text-2"

--> (drop-symbol data-node-b)
DROPPED SYMBOL: data-node-b

--> (show-data data-node-b)
SYMBOL NOT FOUND: data-node-b

--> (show-data data-node-a)
DATA FOR SYMBOL - data-node-a
- text 1: "a-text-1"
- text 2: "a-text-2"
- link 1 info:
  link 1 info - text 1: "b-text-1"
  link 1 info - text 2: "b-text-2"
  - link 1 -> link 1 info - text only:
    link 1 -> link 1 info - text 1: "a-text-1"
    link 1 -> link 1 info - text 2: "a-text-2"
  - link 1 -> link 2 info - text only:
    link 1 -> link 2 info - text 1: "c-text-1"
    link 1 -> link 2 info - text 2: "c-text-2"
- link 2 info:
  link 2 info - text 1: "e-text-1"
  link 2 info - text 2: "e-text-2"
  - link 2 -> link 1 info - text only:
    link 2 -> link 1 info - text 1: "a-text-1"
    link 2 -> link 1 info - text 2: "a-text-2"
  - link 2 -> link 2 info - text only:
    link 2 -> link 2 info - text 1: "d-text-1"
    link 2 -> link 2 info - text 2: "d-text-2"

--> (show-data data-node-c)
DATA FOR SYMBOL - data-node-c
- text 1: "c-text-1"
- text 2: "c-text-2"
- link 1 info:
  link 1 info - text 1: "a-text-1"
  link 1 info - text 2: "a-text-2"
  - link 1 -> link 1 info - text only:
    link 1 -> link 1 info - text 1: "b-text-1"
    link 1 -> link 1 info - text 2: "b-text-2"
  - link 1 -> link 2 info - text only:
    link 1 -> link 2 info - text 1: "e-text-1"
    link 1 -> link 2 info - text 2: "e-text-2"
- link 2 info:
  link 2 info - text 1: "b-text-1"
  link 2 info - text 2: "b-text-2"
  - link 2 -> link 1 info - text only:
    link 2 -> link 1 info - text 1: "a-text-1"
    link 2 -> link 1 info - text 2: "a-text-2"
  - link 2 -> link 2 info - text only:
    link 2 -> link 2 info - text 1: "c-text-1"
    link 2 -> link 2 info - text 2: "c-text-2"

--> (show-data data-node-e)
DATA FOR SYMBOL - data-node-e
- text 1: "e-text-1"
- text 2: "e-text-2"
- link 1 info:
  link 1 info - text 1: "a-text-1"
  link 1 info - text 2: "a-text-2"
  - link 1 -> link 1 info - text only:
    link 1 -> link 1 info - text 1: "b-text-1"
    link 1 -> link 1 info - text 2: "b-text-2"
  - link 1 -> link 2 info - text only:
    link 1 -> link 2 info - text 1: "e-text-1"
    link 1 -> link 2 info - text 2: "e-text-2"
- link 2 info:
  link 2 info - text 1: "d-text-1"
  link 2 info - text 2: "d-text-2"
  - link 2 -> link 1 info - text only:
    link 2 -> link 1 info - text 1: "a-text-1"
    link 2 -> link 1 info - text 2: "a-text-2"
  - link 2 -> link 2 info - text only:
    link 2 -> link 2 info - text 1: "c-text-1"
    link 2 -> link 2 info - text 2: "c-text-2"

--> (drop-symbol data-node-a)
DROP MIDDLE CELL WRAPPER for CELL DATA with info
- text 1: "a-text-1"
- text 2: "a-text-2"
--- --- ---
DROP MIDDLE CELL WRAPPER for CELL DATA with info
- text 1: "b-text-1"
- text 2: "b-text-2"
--- --- ---
DROPPED SYMBOL: data-node-a

--> (show-data data-node-d)
DATA FOR SYMBOL - data-node-d
- text 1: "d-text-1"
- text 2: "d-text-2"
- link 1 info:
  link 1 info - text 1: "a-text-1"
  link 1 info - text 2: "a-text-2"
  - link 1 -> link 1 info - text only:
    link 1 -> link 1 info - text 1: "b-text-1"
    link 1 -> link 1 info - text 2: "b-text-2"
  - link 1 -> link 2 info - text only:
    link 1 -> link 2 info - text 1: "e-text-1"
    link 1 -> link 2 info - text 2: "e-text-2"
- link 2 info:
  link 2 info - text 1: "c-text-1"
  link 2 info - text 2: "c-text-2"
  - link 2 -> link 1 info - text only:
    link 2 -> link 1 info - text 1: "a-text-1"
    link 2 -> link 1 info - text 2: "a-text-2"
  - link 2 -> link 2 info - text only:
    link 2 -> link 2 info - text 1: "b-text-1"
    link 2 -> link 2 info - text 2: "b-text-2"

--> (drop-symbol data-node-b)
DROP FAILURE - SYMBOL NOT FOUND: data-node-b

--> (drop-symbol data-node-c)
DROPPED SYMBOL: data-node-c

--> (show-data data-node-d)
DATA FOR SYMBOL - data-node-d
- text 1: "d-text-1"
- text 2: "d-text-2"
- link 1 info:
  link 1 info - text 1: "a-text-1"
  link 1 info - text 2: "a-text-2"
  - link 1 -> link 1 info - text only:
    link 1 -> link 1 info - text 1: "b-text-1"
    link 1 -> link 1 info - text 2: "b-text-2"
  - link 1 -> link 2 info - text only:
    link 1 -> link 2 info - text 1: "e-text-1"
    link 1 -> link 2 info - text 2: "e-text-2"
- link 2 info:
  link 2 info - text 1: "c-text-1"
  link 2 info - text 2: "c-text-2"
  - link 2 -> link 1 info - text only:
    link 2 -> link 1 info - text 1: "a-text-1"
    link 2 -> link 1 info - text 2: "a-text-2"
  - link 2 -> link 2 info - text only:
    link 2 -> link 2 info - text 1: "b-text-1"
    link 2 -> link 2 info - text 2: "b-text-2"

--> (drop-symbol data-node-d)
DROPPED SYMBOL: data-node-d

--> (show-data data-node-e)
DATA FOR SYMBOL - data-node-e
- text 1: "e-text-1"
- text 2: "e-text-2"
- link 1 info:
  link 1 info - text 1: "a-text-1"
  link 1 info - text 2: "a-text-2"
  - link 1 -> link 1 info - text only:
    link 1 -> link 1 info - text 1: "b-text-1"
    link 1 -> link 1 info - text 2: "b-text-2"
  - link 1 -> link 2 info - text only:
    link 1 -> link 2 info - text 1: "e-text-1"
    link 1 -> link 2 info - text 2: "e-text-2"
- link 2 info:
  link 2 info - text 1: "d-text-1"
  link 2 info - text 2: "d-text-2"
  - link 2 -> link 1 info - text only:
    link 2 -> link 1 info - text 1: "a-text-1"
    link 2 -> link 1 info - text 2: "a-text-2"
  - link 2 -> link 2 info - text only:
    link 2 -> link 2 info - text 1: "c-text-1"
    link 2 -> link 2 info - text 2: "c-text-2"

--> (drop-symbol data-node-e)
DROP MIDDLE CELL WRAPPER for CELL DATA with info
- text 1: "e-text-1"
- text 2: "e-text-2"
--- --- ---
DROP MIDDLE CELL WRAPPER for CELL DATA with info
- text 1: "d-text-1"
- text 2: "d-text-2"
--- --- ---
DROP MIDDLE CELL WRAPPER for CELL DATA with info
- text 1: "c-text-1"
- text 2: "c-text-2"
--- --- ---
DROP MIDDLE CELL WRAPPER for CELL DATA with info
- text 1: "a-text-1"
- text 2: "a-text-2"
--- --- ---
DROP MIDDLE CELL WRAPPER for CELL DATA with info
- text 1: "a-text-1"
- text 2: "a-text-2"
--- --- ---
DROP MIDDLE CELL WRAPPER for CELL DATA with info
- text 1: "b-text-1"
- text 2: "b-text-2"
--- --- ---
DROP MIDDLE CELL WRAPPER for CELL DATA with info
- text 1: "a-text-1"
- text 2: "a-text-2"
--- --- ---
DROP CELL DATA with info:
- text 1: "a-text-1"
- text 2: "a-text-2"
DROP CELL COUNT: 1
--- --- ---
DROP MIDDLE CELL WRAPPER for CELL DATA with info
- text 1: "b-text-1"
- text 2: "b-text-2"
--- --- ---
DROP CELL DATA with info:
- text 1: "b-text-1"
- text 2: "b-text-2"
DROP CELL COUNT: 2
--- --- ---
DROP MIDDLE CELL WRAPPER for CELL DATA with info
- text 1: "c-text-1"
- text 2: "c-text-2"
--- --- ---
DROP CELL DATA with info:
- text 1: "c-text-1"
- text 2: "c-text-2"
DROP CELL COUNT: 3
--- --- ---
DROP MIDDLE CELL WRAPPER for CELL DATA with info
- text 1: "d-text-1"
- text 2: "d-text-2"
--- --- ---
DROP CELL DATA with info:
- text 1: "d-text-1"
- text 2: "d-text-2"
DROP CELL COUNT: 4
--- --- ---
DROP MIDDLE CELL WRAPPER for CELL DATA with info
- text 1: "e-text-1"
- text 2: "e-text-2"
--- --- ---
DROP CELL DATA with info:
- text 1: "e-text-1"
- text 2: "e-text-2"
DROP CELL COUNT: 5
--- --- ---
DROPPED SYMBOL: data-node-e

--> 
```
