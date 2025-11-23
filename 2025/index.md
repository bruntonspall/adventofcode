---
layout: post
tags: 
- year
- adventofcode
- aoc2025
title: Advent of Code 2025
date: 2025-11-22
---

It's 2025, and I've had a heck of a year, including not managing to write my weekly newsletter for a number of months now.  One sign that things are slowly returning to normal is that I'm considering doing the Advent of Code.

Last year, I did it in Rust and I really enjoyed it.  I've of course, not touched Rust all year, so I figured with a little over a week to go, whether I should try getting everything setup.  As a challange, I thought I'd give a few of the infamous 2019 `intcode` puzzles a try.

These were a series of puzzles throughout the 2019 advent of code that required you to build an increasingly complex computer, starting with a couple of very simple opcodes and an in memory array, you could make a computer that executed intcode and see what the result was.  I was doing it in Go in 2019, and I got utterly derailed about half way through because I got stuck understanding the blocking and non-blocking aspects of channels.

I'm going to give the first one or two a go from 2019 under the "intcode" module, and also I'm going to revisit the old aoc-runner library that was used before.  I think I'd prefer to build each week with a library and a set of tests, and then a single main that runs each days solution on the respective days input.  Reducing the external dependencies should make it much cleaner (I hope).

I'm keeping most of the utils that I wrote last year, some simple stuff like a coordinate and a grid is something that is very likely to come up.

## Completed days
  So far I've completed the following days:

{% for post in collections.aoc2025 %}
  * [ {{ post.data.title | default: post.fileSlug }} ]({{post.url}})
{% endfor %}

Cheers