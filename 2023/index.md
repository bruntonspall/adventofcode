---
layout: post
tags: year
title: Advent of Code 2023
date: 2023-12-01
---
This is my set of entries for [advent of code 2023](https://www.adventofcode.com/2023).

## Completed days
  So far I've completed the following days:

{% for post in collections.aoc2023 %}
  * [ {{ post.title | default: post.fileSlug }} ]({{post.url}})
{% endfor %}

Cheers