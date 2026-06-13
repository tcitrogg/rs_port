# TODO

`<leading>` - `cargo run --` or `todo` as binary file _if you cargo build_

```
$ <leading> add "Write Rust notes"
(Added: 1) Write Rust notes

$ <leading> add "Read chapter 4"
(Added: 2) Read chapter 4

$ <leading> mark 1
(Marked) x ID:1 as completed

$ <leading> list
(TODO List)
[x] 1 — Write Rust notes
[ ] 2 — Read chapter 4

$ <leading> mark 1
(Unmarked) - ID:1 as uncompleted

$ <leading> list
(TODO List)
[ ] 1 — Write Rust notes
[ ] 2 — Read chapter 4

$ <leading> delete 1
(Deleted) - ID:1

$ <leading> list
(TODO List)
[ ] 2 — Read chapter 4

$ <leading> clear
(Cleared) Todo List

$ <leading> list
(TODO List)
(Empty) Add a task & it'll show up here.
```

