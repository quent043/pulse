# pulse

A real-time system monitor: a Rust backend samples the machine's vitals and
streams them to a lightweight dashboard. Think `htop`, but with a live web view
you can leave open on a tablet.

> **Status: work in progress.** Currently at step 1 of the roadmap below.

## Why

<!-- TODO — write this last, in your own words. Two or three sentences:
     what itch this scratches, and why the interesting part is the backend,
     not the dashboard. -->

## Design decisions

<!-- TODO — this section is the point of the repo. One short subsection per
     decision actually made, each answering "what did I pick, what did I give
     up, and how do I know". Candidates, to fill in as they get decided:

     - How CPU usage is measured on macOS (no /proc, so what instead?)
     - Sampling interval, and how the monitor avoids measuring itself
     - Ring buffer: why fixed-size, what the memory footprint is, what
       happens on overrun
     - Which lock, and why (std::sync::Mutex vs parking_lot vs channels vs
       no lock at all)
     - Wire format and why serde serialises what it does

     Rule for this section: no claim you couldn't defend out loud. -->

## Roadmap

- [ ] 1. Sample one metric (CPU), single-threaded, print to stdout
- [ ] 2. Add memory
- [ ] 3. Fixed-size ring buffer for the last N seconds of history
- [ ] 4. Make the probes concurrent
- [ ] 5. Serialise (serde) and expose over the network
- [ ] 6. Minimal React/PWA dashboard in `web/`
- [ ] 7. Tests, benchmarks, and the design write-up above

## Running it

<!-- TODO — fill in once there is something to run. -->

```
cargo run
```

## Layout

```
src/     Rust backend: sampling, history, serving
web/     Dashboard (not started yet)
```
