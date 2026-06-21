# Scripting

You can filter LeetCode problems with custom Python scripts and pass the result to `list` or `pick` via `--plan`.

> Python scripting is gated behind the optional `pym` Cargo feature. A default `cargo install leetcode-cli` does **not** include it — install with `cargo install leetcode-cli --features pym`, otherwise `--plan` is silently ignored.

## Writing a plan

Scripts live in the `scripts` directory under your storage root (`~/.leetcode/scripts` by default; configurable via `[storage] scripts`).

```python
# ~/.leetcode/scripts/plan1.py
import json

def plan(sps, stags):
    # `print` works here — print the two args if you want to inspect their shape.
    problems = json.loads(sps)
    tags = json.loads(stags)

    tm = {}
    for tag in tags:
        tm[tag["tag"]] = tag["refs"]

    ret = []
    for i in problems:
        if i["level"] == 1 and str(i["id"]) in tm["linked-list"]:
            ret.append(str(i["id"]))

    # Return a List[str] of problem ids
    return ret
```

The module must define `plan(sps, stags)`:
- `sps` — JSON string of all problems.
- `stags` — JSON string of all tags.
- returns a `List[str]` of problem ids to keep.

## Running it

```sh
leetcode list -p plan1     # filter the list down to the plan's ids
leetcode pick -p plan1     # pick from the filtered set
```
