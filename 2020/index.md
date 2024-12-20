---
layout: post
tags: year
title: Advent of Code 2020
date: 2020-12-01
---
[Advent of code 2020](https://www.adventofcode.com/2020) was the first year that I tried using Jupyter labs to explain my code and alternate blogging with actual coding.  It took a lot of setting up, and I had a very hacky shell script to turn it into a series of HTML documents using nbconvert.
Some work in 2024 converted them to markdown and then through 11ty into a nice static website that has some syntax highlighting and brings all the years together.

I only managed the first two weeks or so, although I don't remember now in 2024 just what delayed me so much.

## Completed days
  So far I've completed the following days:

{% for post in collections.aoc2020 %}
  * [ {{ post.title | default: post.fileSlug }} ]({{post.url}})
{% endfor %}

Cheers