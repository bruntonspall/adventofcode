---
layout: post
tags: year
title: Advent of Code 2023
date: 2023-12-01
---

2023 for [Advent of Code](https://www.adventofcode.com/2023) was not a good year for me.  I tried to follow on from the 2021 year and do this all in Jupyter labs, but work got the better of me, along with a very dispiritingly difficult puzzle on day 3, and I never got any further in year.
Anything past that point here is done much later.

## Completed days
  So far I've completed the following days:

{% for post in collections.aoc2023 %}
  * [ {{ post.title | default: post.fileSlug }} ]({{post.url}})
{% endfor %}

Cheers