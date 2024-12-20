---
layout: post
tags: year
title: Advent of Code 2021
date: 2021-12-01
---
[Advent of Code 2021](https://www.adventofcode.com/2021) was probably one of my best years.  I took the 2020 idea of writing these in Jupyter notebooks, whcih let me alternate between solving problems and explaining my thinking.  I've always enjoyed explaining why code is the way it is, and this enables me to think out loud.

A couple of tricky ones towards the end, but mostly it was just time that did for me in the end.

## Completed days
  So far I've completed the following days:

{% for post in collections.aoc2021 %}
  * [ {{ post.title | default: post.fileSlug }} ]({{post.url}})
{% endfor %}

Cheers