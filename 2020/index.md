---
layout: post
tags: year
title: Advent of Code 2020
date: 2020-12-01
---
This is my set of entries for [advent of code 2020](https://www.adventofcode.com/2020).

## Completed days
  So far I've completed the following days:

{% for post in collections.aoc2020 %}
  * [ {{ post.title | default: post.fileSlug }} ]({{post.url}})
{% endfor %}

Cheers