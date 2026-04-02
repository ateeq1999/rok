## Closes

Closes #

## What changed

<!-- One paragraph. What was added, changed, or removed and why. -->

## Affected crates

<!-- List every crate whose public API or behaviour changed. -->

- [ ] `rok-`

## Acceptance gates

Paste the output of `./scripts/dev.sh gates` or link to the CI run.

```
>> Formatting     [ok]
>> Lints          [ok]
>> Tests          [ok]
>> Documentation  [ok]
```

## Checklist

- [ ] Gates are green (`./scripts/dev.sh gates`)
- [ ] Every new/changed `pub` item has a `///` doc comment
- [ ] New behaviour is covered by at least one test
- [ ] No `unwrap()` in library code outside of tests
- [ ] `CHANGELOG.md` updated if this is a user-visible change
- [ ] Spec in the linked issue is fully satisfied (nothing extra added)
