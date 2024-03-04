# noodlez

## Mentions

A improved attempt to graphs in rust. This repository tries to implement a fresh approach to graphs with the latest rust features available. It takes heavy inspiration from the following sources and we'd like to thank them:

- the general bevy community for bringing up the topic: https://github.com/bevyengine/bevy/discussions/6719#discussioncomment-4199393
- DasLixou for showing us that a new implementation is possible and for providing some groundwork (bevy PR) here https://github.com/bevyengine/bevy/pull/7130
- petgraph for showing us that big graph libraries in rust are possible and needed https://github.com/petgraph/petgraph
- prepona for showing us that a smoother API is possible with some more architectural care https://github.com/maminrayej/prepona
- graphlib for showing us some details that are useful to incorporate https://github.com/purpleprotocol/graphlib
- the boost graph library and the haskell Data.Graph implementation for showing us how other languages did it

## Investigations

Some interesting groundwork before we can start with this lib is:

- generally set up some benchmarks for petgraph for comparison
- roughly benchmark indexmap (petgraph) vs slotmap (new impl) with different sizes (probably boils down to HashMap vs Vec so might not make sense. Anyways, at least verify that the latter is comparably as fast as the former)
- checkout dashmap
- checkout bitflags
- investigate graph-rs a bit and look for further inspiration there
- investigate completely trait based approaches
  ```rust
  struct G;
  struct N;
  struct E;
  // ...
  impl Graph for G {
      type Node = N;
      type Edge = E;
  }
  // ...
  ```
- once there's enough implemented investigate dot integration
- once there's enough implemented create a visual bevy example
