# dop

>The anticipation of most types of rewards increases the level of **dop**amine in the brain.

## Usage

Null case:

```
$ dop
No actions registered.

For more info:
  dop -h
```

Do a new thing:

```
$ dop doomscroll
New action registered: "doomscroll".

If this is something you want to do more often:
  dop -g doomscroll

If this is something you want to do less often:
  dop -b doomscroll

For more info:
  dop -h
```

Set the valence of a thing:

```
$ dop -b doomscroll
Valence of action "doomscroll" set to "bad".

Dop will discourage you from taking this action.
```

Set the valence of a new thing:

```
$ dop -g meditate
New action registered: "meditate".

Valence of action "meditate" set to "good".

Dop will encourage you to take this action.
```

Show status:

```
$ dop

| Action     | Last   | Frequency   |
| ---------- | ------ | ----------- |
| meditate   | 1m ago | no data yet |
| doomscroll | 1m ago | no data yet |

For more info:
  dop -h
```

## Configuration

Configuration is stored in `~/.config/dop/dop.toml`,
in the following format:

```toml
[actions.meditate]
good = true

[actions.doomscroll]
bad = true
```

Data is stored in `~/.local/share/dop/<ACTION>.ndjson`
as one timestamp per line.
