# chaosmarmoset

The pygmy marmoset is the smallest monkey.  (Not the smallest primate, though;
that'd be Madame Berthe's mouse lemur.)

This is intended to be a very small "unit of chaos" which can be injected into
a kubernetes cluster to answer questions like "what happens if we use all the
memory?" or "does etcd get angry if we saturate the EBS connection?"

# BEWARE

I'm using this project to help learn Rust which means the majority of it
will be really bad code.  You should probably not trust it and you should
*definitely* not try to learn Rust from it!

Basically I found myself needing pods that did various "stuff" in Kubernetes,
most of which could be hacked together with small bash scripts running netcat
and friends... but that collection was pretty clunky, and I wanted to learn
Rust, so here we are.

## Usage

```chaosmarmoset --mode {cpu,max-memory,network-sink,set-memory,web-client}```

## Building

```cargo build --release```

You'll need rustup; if you're just starting out with Rust check out at least the first
couple of chapters of https://doc.rust-lang.org/book/second-edition/ch01-00-introduction.html
You can skip the `--release` flag if you're looking for a quick compile rather than
an optimized binary.  The binary will end up as ./target/release/chaosmarmoset, or if
you skipped `--release` then it'll be ./target/debug/chaosmarmoset.

## Testing

No tests yet; please add some!
