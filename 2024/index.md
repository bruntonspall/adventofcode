---
layout: post
tags: 
- year
- adventofcode
- aoc2024
title: Advent of Code 2024
date: 2024-12-01
---
This is my set of entries for [advent of code 2024](https://www.adventofcode.com/2024).

## Completed days
  So far I've completed the following days:

{% for post in collections.aoc2024 %}
  * [ {{ post.data.title | default: post.fileSlug }} ]({{post.url}})
{% endfor %}

Cheers