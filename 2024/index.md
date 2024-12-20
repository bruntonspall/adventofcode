---
layout: post
tags: 
- year
- adventofcode
- aoc2024
title: Advent of Code 2024
date: 2024-12-01
---

In 2024, I set about trying Advent of Code using Rust.  I've tried it before, but it's not a language I'm terribly familiar with, but it's come a long way and since I almost never get to code professionally any more, this is my chance to try and explore new ecosystems.

I'm going to try to minimise the number of smart things needed, so I'm not going to be optimising particuarly well.  I know that Rust has some good functional practices, so I'm expecting to use iterators, zip, map, flatmap and so on.

My experiences with non pythonic langugaes is that that I really miss the batteries included standard library, so I'm going to make a bit more effort to use the rather excellent cargo crates system when needed as well.

Work has been insanely busy this year, so I'm going to consider myself happy if I manage about half the problems this year.

## Completed days
  So far I've completed the following days:

{% for post in collections.aoc2024 %}
  * [ {{ post.data.title | default: post.fileSlug }} ]({{post.url}})
{% endfor %}

Cheers