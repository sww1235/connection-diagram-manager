# connection-diagram-manager


Connection Diagram Manager is a program to view, edit and manage wiring
diagrams.

It can be used from the smallest wiring harness to the largest building
electrical system.

**Note:** it is not designed to be used to document a PCB wiring schematic, but
it can be used to document connections between PCBs.


It is still very much at a draft stage and should not be used for production.

This repository is a rewrite of my initial attempt at this same program in
[Go](https://github.com/sww1235/connection-diagram-manager). I migrated to Rust
because I liked the additional safety of Rust, along with some potential speed
advantages with the memory management (no GC). I also found the serialization
and deserialization of YAML to work much better, and be much better documented.

## Credits

The font resources in [pdf\_helper](./src/pdf_helper/) were copied from <https://github.com/chbrown/afm>.
They were released under a MIT licence and so have been copied and distributed here.
Please see their repository for more information.
