---
layout: post
tags: year
title: Advent of Code 2021
date: 2021-12-01
---
This is my set of entries for [advent of code 2021](https://www.adventofcode.com/2021).

## Completed days
  So far I've completed the following days:

{% for post in collections.aoc2021 %}
  * [ {{ post.title | default: post.fileSlug }} ]({{post.url}})
{% endfor %}

Cheers