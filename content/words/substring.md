---
title: Custom AMQ Filter for Fast Substring Search
description: using a character-based AMQ to optimize substring searches over millions of elements
date: 2025-06-08
tags:
    - algorithms
    - performance
    - rust
    - data-structures
---

## The Problem

Recently, my friend [Yui](https://zptr.cc/) asked me to help find a way to search for all elements that included a given word on her site [infinibrowser.wiki](https://infinibrowser.wiki/), which has ~10 million elements.
As part of the problem this substring search needed to [support all unicode characters](https://docs.google.com/spreadsheets/d/1PRtlXvjbHs4ulct6gSbYc6VYrQegU7HZ5SdhThkHuoY/edit?gid=965823706#gid=965823706) we could throw at it, with there being projects to uncover a recipe for every unicode character in Infinicraft.
Everything she tried was either slow (500ms+ per search) or didn't handle unicode properly with lots of solutions stripping away various parts of unicode.

## Enter AMQ Filters

I'd heard of Approximate Membership Query (AMQ) filters before but never had a good excuse to build one.
Turns out this was perfect - an AMQ can quickly tell you "might this element be in the set?" 
You get one of two answers: "definitely not" (which is always right) or "maybe" (which might be wrong).

What makes AMQs so useful here is that they never lie when they say no.
If an AMQ says "nope, not there", you can trust it completely.
But if it says "maybe", well, you gotta double-check because it might be getting excited over nothing.
They're also tiny compared to storing everything, and checking them is super fast.

So here's the plan: use the AMQ as a bouncer.
If it says "definitely not", we skip that string entirely.
If it says "maybe", we do the actual substring search to be sure.

## Our Custom AMQ Design

For substring search, we need an AMQ that can answer: "could this string contain this substring?"

The trick is pretty simple: if a character shows up in a substring, it has to show up in the full string too.
So we can make a little fingerprint for each string based on what characters it has.

I decided to use a 64-bit number as the fingerprint.
For each character in a string, we set a bit:

```rust
for each character c in string:
    bit_position = (c as u32) % 64
    amq_filter |= (1 << bit_position)
```

## How the Query Works

When someone searches for something, we make a fingerprint for their query the same way.
We generate a fingerprint for every element in our dataset and iterate over each of them, checking to see if the query fingerprint passes the check:

```rust
if (element_filter & query_filter) == query_filter {
    // All query characters are present in the string
    // Might contain substring - do full string search
    if element.contains(query) {
        results.push(element);
    }
} else {
    // Query contains characters not in the string
    // Definitely doesn't contain substring - skip entirely
}
```

## Why This Works

The whole thing hinges on a simple fact: you can't have a substring without having all its characters.
If a string is missing even one character from your query, there's no way it can contain that substring.

That bitwise check `(element_filter & query_filter) == query_filter` is doing all the heavy lifting.
Instead of checking every character in every string, we just do one 64-bit operation and know immediately if we can skip it.

## Results

It worked way better than I expected.
Searches went from taking hundreds of milliseconds down to single digits.
The AMQ filter threw out most strings without doing a costly substring, leaving just a tiny handful that needed the full substring check.
