# particle-life

A(nother) port of HackerPoet/Particle-Life to the web, optimised for performance
using WebAssembly.

Particle Life is actually quite CPU intensive, meaning a JS implementation of it
is bound to be slow. This uses WebAssembly to run as fast as possible, plus a
few other optimisations since it's still not as fast as native code.

It's hosted at https://particle-life.netlify.app.

## Extra Features

- The camera can wrap as well as particles
- Particles are rendered on both sides when wrapping
- Not locked to 30fps

## Optimisations

- The step algorithm has been optimised slightly. The most expensive operation
  is to calculate the distance between two particles, but distance is the same
  in both directions - so instead of calculating it twice, iterate over pairs of
  particles and do the calculations for both.
- Run calculations on another thread. The step algorithm isn't very
  parallelisable, but the calculations can be run on another thread and then
  sent back to the main thread to render.

## Running Natively

Since this is written in Rust with a generic graphics library (`quicksilver`),
it can also run faster natively. No builds are published right now, but you can
build it from source:

- [Install Rust](https://www.rust-lang.org/learn/get-started#installing-rust)
- `cargo install --git=https://github.com/Liamolucko/particle-life.git`
- `particle-life`
