---
description: >
  this is a long
  paragraph value spread
  across multiple lines
nested:
  key1: value1
  key2: value2
title: Malformed Sample
---

Body. The frontmatter above mixes a folded scalar and a nested mapping —
shapes the line-based heuristic cannot fully understand. It skips the folded
continuation lines (which have no separator), but it cannot tell that `key1` /
`key2` are nested under `nested`, so it lifts them to top-level keys. The point
of this fixture is that the heuristic degrades gracefully and the document does
not break — not that the summary is semantically perfect.
