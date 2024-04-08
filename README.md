# dop

>The anticipation of most types of rewards increases the level of **dop**amine in the brain

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
$ dop smoke
New action registered: "smoke".

If this is something you want to do more often:
  dop -g smoke

If this is something you want to do less often:
  dop -b smoke

For more info:
  dop -h
```

Set the valence of a thing:

```
$ dop -b smoke
Valence of action "smoke" set to "bad".

Dop will discourage you from taking this action.
```

Set the valence of a new thing:

```
$ dop -g eat
New action registered: "eat".

Valence of action "eat" set to "good".

Dop will encourage you to take this action.
```

Show status:

```
$ dop

| Action | Last   | Frequency   |
| ------ | ------ | ----------- |
| eat    | 1m ago | no data yet |
| smoke  | 1m ago | no data yet |

For more info:
  dop -h
```

## Configuration

Data is stored in `~/.local/share/dop/<ACTION>.ndjson`

Configuration is stored in `~/.config/dop/dop.toml`,
in the following format:

```toml
[actions.eat]
valence = "good"

[actions.smoke]
valence = "bad"
```
