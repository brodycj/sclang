# SCLang - a toy LISP-like circular data storage language with deterministic auto memory management overhead (NO NORMAL GC OVERHEAD REQUIREMENT)

LICENSE: MIT OR APACHE 2.0

USING REFERENCE MANAGEMENT OBJECTS WITH HELP FROM STRONG & WEAK REFERENCES TO AVOID THE NEED FOR PERIODIC OR RANDOM GC OVERHEAD

STATUS: EXPERIMENTAL WITH NO GUARANTEES WHATSOEVER NEEDS MASSIVE CLEANUP & LIKELY NEEDS MASSIVE OPTIMIZATION, MULTI-THREADING NOT EXPECTED TO WORK

HOW: Using data record cell objects with strong (A)RC references & weak (A)RC references to manage data record & data record linkage lifetimes - XXX TODO NEED TO DOCUMENT & EXPLAIN THIS

XXX TODO DOCUMENT programmatic `sclang` & `sc_data_record_manager` APIs

MAJOR TODO ITEM IS TO SUPPORT `no_std` environment for EMBEDDED SYSTEMS

BENCHMARKS ARE VERY ROUGH & LIKELY AFFECTED BY ADDITIONAL SCLang parsing overhead

TO RUN TEST:

```sh
cargo test
```

TO RUN INTERACTIVE CLI DEMO:

```sh
cargo run --example i-cli
```

TO RUN BENCHMARKS WITH VERBOSE BENCHMARK INFO IN THE END:

```sh
cargo bench --bench bench-1 -- --verbose
cargo bench --bench bench-2 -- --verbose
```

TO RUN CI-friendly iai benchmarks - requires valgrind:

```sh
cargo bench --bench iai-bench-1 -- --verbose
cargo bench --bench iai-bench-2 -- --verbose
```

---

SAMPLE INTERACTIVE SESSION THAT DEMONSTRATES STORING CIRCULAR-LINKED DATA CELLS WITH DEBUG PRINT OUTPUT INCLUDED, ABLE TO REMOVE & CLEAN UP UNREACHABLE CELLS IN THE END

STARTUP:

```sh
% cargo run --example i-cli
--> 
```

ENABLE DEBUG OUTPUT:

```sh
--> (enable-feature debug)
DEBUG ENABLED
ENABLE FEATURE: debug

--> 
```

STORE SOME DATA:

```sh
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

--> 
```

UPDATE SOME DATA WITH A CIRCULAR REFERENCE:

```sh
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

--> 
```

ADD & UPDATE WITH SOME MORE CIRCULAR DATA:

```sh
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

--> 
```

STORE ANOTHER 2 NODES OF DATA:

```sh
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

--> 
```

MAKE THIS EVEN MORE CIRCULAR:

```sh
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

--> 
```

CHECK THE FIRST NODE:

```sh
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

--> 
```

DROP A SYMBOL FOR DATA NODE B AND CHECK THAT NODE B IS STILL NOT GONE:

```sh
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

--> 
```

SHOW SOME MORE DATA - SHOULD SEE THAT NODE B IS STILL THERE:

```sh
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

--> 
```

DROP FIRST SYMBOL (FOR NODE A):

```sh
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

--> 
```

NOTE that while a couple "middle cell" wrappers are dropped & cleaned up, no data should be removed at this point.

TAKE A QUICK PEEK INTO NODE D - THIS DATA DUMP SHOWS ALL 5 NODES WITH DATA STILL PRESENT:

```sh
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

--> 
```

TRY DROPPING SYMBOL FOR DATA NODE B AGAIN - DROP FAILURE IS EXPECTED IN THIS CASE:

```sh
--> (drop-symbol data-node-b)
DROP FAILURE - SYMBOL NOT FOUND: data-node-b
--> 
```

DROP SYMBOL FOR NODE C & TAKE ANOTHER PEEK INTO DATA NODE D - SHOULD STILL SHOW ALL DATA IS STORED AT THIS POINT:

```sh
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

--> 
```

DROP SYMBOL FOR NODE D & PEEK INTO NODE E - SHOULD STILL SHOW ALL DATA NODES REACHABLE AT THIS POINT:

```sh
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

--> 
```

DROP SYMBOL FOR NODE E - DEBUG OUTPUT SHOULD SHOW CLEAN-UP OF ALL STORED DATA AT THIS POINT:

```sh
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
