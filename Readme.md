# perms

Generate all permutations of a list

## Installation

### Build from source

- Clone the repo
  `git clone https://github.com/Skyppex/perms.git`
  `gh repo clone Skyppex/perms`

- Build with cargo
  `cargo build --release`

## Usage

`perms [k]`

`k` is the number of elements in each permutation

```
[a, b, c] | perms 2

will produce

a;b
a;c
b;a
b;c
c;a
c;b
```
