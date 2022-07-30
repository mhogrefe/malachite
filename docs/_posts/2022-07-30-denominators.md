---
layout: post
title: Which denominators does an interval contain?
author: Mikhail Hogrefe
---

## Introduction

In this post I'm going to talk a bit about a problem that came up when I was figuring out how to generate rational numbers exhaustively. Malachite generates all positive rationals using the [Calkin-Wilf sequence](https://en.wikipedia.org/wiki/Calkin%E2%80%93Wilf_tree#Breadth_first_traversal):

1, 1/2, 2, 1/3, 3/2, 2/3, 3, 1/4, 4/3, 3/5, 5/2, 2/5, 5/3, 3/4, 4, 1/5, 5/4, 4/7, 7/3, 3/8, ...,

and by manipulating this sequence a bit it can also generate all rationals, all negative rationals, and so on. How about generating all rationals in a specified interval? This has many uses: for example, when testing conversion from rationals to floats, we might want to generate rationals that fall into the 64-bit subnormal float range.

For the remainder of this post I'm going to consider closed intervals only.

## First try

The most straightforward approach is this:
1. Generate all rationals in $$[0, 1]$$.
2. Scale those rationals to bring them into the target interval.

Step 1 is easy: we can generate 0 and 1, and then select every other rational from the above sequence:

$$0, 1, 1/2, 1/3, 2/3, 1/4, 3/5, 2/5, 3/4, 1/5, 4/7, 3/8, 5/7, 2/7, 5/8, 3/7, 4/5, 1/6, 5/9, 4/11, \ldots$$.

Step 2 is also easy. If our target interval is $[a, b]$, we transform each rational using x \to (b - a)x + a$. For example, here are the rationals in $[1/3, 1/2]$:

$$1/3, 1/2, 5/12, 7/18, 4/9, 3/8, 13/30, 2/5, 11/24, 11/30, 3/7, 19/48, 19/42, 8/21, 7/16, 17/42, 7/15, 13/36, 23/54, 13/33, \ldots$$.

For a simple interval this approach works fine. But what if we generate intervals in $$[268876667/98914198, 245850922/78256779]$$? The endpoints of this interval are the simplest rationals that round to the best 64-bit float representations of $$e$$ and $$\pi$$, respectively. Our algorithm gives us

$$268876667/98914198, 245850922/78256779, 45359568684866149/15481413065696484, 33200495296270871/11611059799272363, 69677715462056705/23222119598544726, 87442412500217335/30962826131392968, 19172880691153809/6450588777373535, 111760559277407891/38703532664241210, 31331954079749087/10320942043797656, 54241917203946464/19351766332120605, \ldots$$.

This is not so nice. Our interval contains nice rationals like $$3$$, $$11/4$$, $$14/5$$, and $$17/6$$, but they are nowhere to be seen (they will appear eventually, but much later in the sequence). We want an algorithm that gives preferential treatment to rationals with small denominators. Here's what Malachite does:

An improved algorithm
---------------------

Malachite's algorithm needs three things:
1. To be able to determine which denominators occur in an interval;
2. To be able to generate all rationals with a given denominator in an interval;
3. Given infinitely many iterators, each generating rationals with different denominators, to be able to interleave the iterators into a single iterator.

Number 2 is easy to do and not very interesting. You can see the details [here](https://docs.rs/malachite-q/latest/malachite_q/exhaustive/fn.exhaustive_rationals_with_denominator_inclusive_range.html). Number 3 is more interesting, but already solved. I'll post about it in the future, but for now I'll just leave a link to the relevant function [here](https://docs.rs/malachite-base/latest/malachite_base/tuples/exhaustive/fn.exhaustive_dependent_pairs.html). That leaves number 1.

## Finding all denominators in an interval

Let me define the problem more explicitly.

*Problem:* Given a closed interval $$[a, b]$$ with $$a, b \in \Q$$ and $$a < b$$, for which $$d$$ in $$\N^+$$ is it the case that there exists an $$n \in \Z$$ with $$\gcd(n, d) = 1$$ and $$n/d \in [a, b]$$?

The simplest algorithm is to consider each denominator $$1, 2, 3, \ldots$$ in turn and determine whether some rational with the denominator exists in the interval. This works fine unless the diameter $$b - a$$ is very small. If the interval is $$[0, 2^{-100}]$$, it would take a very long time to find an admissible denominator greater than 1.

Luckily, Malachite knows how to find the simplest rational in an interval, ("simplest" meaning "having the lowest denominator"). It uses the continued fractions of the endpoints and follows the algorithm sketched out [here](https://en.wikipedia.org/wiki/Continued_fraction#Best_rational_approximations). This gives us the lowest denominator in the interval in $$O(n^2 \log n \log\log n)$$ time, $$n$$ being the maximum bit-length of the numerators and denominators of both endpoints. Once we've found the lowest denominator $$d$$, we can find the m rationals with that denominator in $$[a, b]$$ and then partition $$[a, b]$$ into $$m + 1$$ smaller intervals. Then we can repeat the process to get the second-lowest denominator, and so on.

This algorithm is efficient enough to be useful, but it's still a bit cumbersome. Our intuition suggests that $$[a, b]$$ contains every denominator in $$\N^+$$ except for finitely many exceptions: in other words, that for every interval $$[a, b]$$ there exists a threshold $$D$$ such that for all $$d >= D$$, $$[a, b]$$ contains a rational with denominator $$d$$. If we knew what $$D$$ was, then we could find start finding denominators using our cumbersome continued-fraction method, but once we reached $$D$$ we could simply generate $$D, D + 1, D + 2, \ldots$$ forever.

For the remainder of this post, I'll prove that a $$D$$ exists for any interval and give an efficient algorithm for finding it (though it generally won't be the lowest possible $$D$$).

## The relationship between an interval's diameter and the denominators it contains

Let $$s = b - a$$ be the diameter of $$[a, b]$$. If $$s \geq 1$$, then we can take $$D = 1$$: $$[a, b]$$ contains all denominators in $$N^+$$. (For any denominator $$d$$, $$k + 1/d$$ is in $$[a, b]$$ for some integer $$k$$.)

What if $$s < 1$$? We might think that if $$s \geq 1/d$$ then $$[a, b]$$ must contain some rational with denominator $$d$$, but this is not the case. For example, an interval with $$s < 2/3$$ might not contain any sixths:

<p align="center">
  <img width="500" src="/assets/denominators/sixths.svg" alt="The largest gap between sixths">
</p>

The largest gap between sixths is $$2/3$$. Let's define $$f(d)$$ to be the largest gap between fractions with denominator is $$d$$:

<p align="center">
  <img width="600" src="/assets/denominators/gaps.svg" alt="The largest gap between rationals with denominators 1 through 10">
</p>

<p align="center">
  <img width="600" src="/assets/denominators/gap-graph.svg" alt="A graph of the largest-gap function">
</p>

Any interval with $$s \geq f(d)$$ is, by definition, guaranteed to contain some rational with denominator $$d$$. If $$f$$ were monotonically decreasing, then we could use that to prove that $$D$$ exists. We'd simply need to find a $$D$$ such that $$f(D) \leq s$$, and then any $$f(d)$$ for $$d \geq D$$ would also be less than or equal to $$s$$. However, $$f$$ does not monotonically decrease since $$f(5) = 2/5$$ and $$f(6) = 2/3$$.

## The Jacobsthal function and primorials

I couldn't find any reference to $$f(n)$$ in the literature, but fortunately $$g(n) = n f(n)$$ has been studied: it's called the [Jacobsthal function](http://oeis.org/A048669) (not to be confused with the Jacobsthal numbers, which are something unrelated). $$g(n)$$ is the size of the maximal gap in the list of all integers relatively prime to $$n$$.

<p align="center">
  <img width="600" src="/assets/denominators/j-graph.svg" alt="A graph of the Jacobsthal function">
</p>

We can make use of a bound on $$g$$: $$g(n) \leq 2^w$, where $$w$$ is the number of distinct prime factors of $$n$$.

| constraint on $$n$$   | bound on $$w$$ | bound on $$g$$   | bound on $$f$$                |
|-----------------------|----------------|------------------|-------------------------------|
| $$1 \leq n < 2$$      | $$w \leq 0$$   | $$g(n) \leq 1$$  | $$f(n) \leq 1/n \leq 1$$      |
| $$2 \leq n < 6$$      | $$w \leq 1$$   | $$g(n) \leq 2$$  | $$f(n) \leq 2/n \leq 1$$      |
| $$6 \leq n < 30$$     | $$w \leq 2$$   | $$g(n) \leq 4$$  | $$f(n) \leq 4/n \leq 2/3$$    |
| $$30 \leq n < 210$$   | $$w \leq 3$$   | $$g(n) \leq 8$$  | $$f(n) \leq 8/n \leq 4/15$$   |
| $$210 \leq n < 2310$$ | $$w \leq 4$$   | $$g(n) \leq 16$$ | $$f(n) \leq 16/n \leq 8/105$$ |
| $$\ldots$$            | $$\ldots$$     | $$\ldots$$       | $$\ldots$$                    |

The sequence in the leftmost column, $$1, 2, 6, 30, 210, \ldots$$, is the sequence of [primorials](https://en.wikipedia.org/wiki/Primorial): they are the products of the first $$0, 1, 2, \ldots$$ primes and therefore the smallest integers with $$0, 1, 2, \ldots$$ distinct prime factors. The $$n$$th primorial is denoted $$p_n#$$. The sequence of bounds in the rightmost column, $$1, 1, 2/3, 4/15, 8/105, \ldots$$ is $$2^n/p_n#$$ and is weakly monotonically decreasing. This allows us to construct a weakly monotonically decreasing function $$h$$ that bounds $$f$$ from above:

$$h(n) = 2^k/p_k# \text{where} p_k# \leq n < p_{k+1}#$$.

Here are $$f$$ and $$h$$ plotted together:

<p align="center">
  <img width="400" src="/assets/denominators/gap-and-bound-graph.svg" alt="A graph of the largest-gap function and an upper bound">
</p>

$h$ is not a very tight bound. With more careful analysis, we could come up with a better one, perhaps by interpolating between the primorials or by making use of the bound $$g(h) \leq 2k^{2+2e\log k}$$.

We now have an algorithm or determining a denominator threshold $$D$$ for an interval $$[a, b]$$:
1. Find the diameter $$s = b - a$$.
2. Compute the sequence $$2^n/p_n#$$ until it is less than or equal to $$s$$: the sequence decreases as $$O((2/n)^n)$$, so this step doesn't take long.
3. Let $$n$$ be the value at which $$2^n/p_n#$$ is less than or equal to $$s$$. Then take $$D$$ to be $$p_n#$$.

## Results

Let's go back to our example interval, $$[268876667/98914198, 245850922/78256779]$$. Its diameter is about 0.42, so its $$D$$ is 30, meaning that it's guaranteed to contain all denominators greater than or equal to 30. This threshold is low enough that we can just test all the denominators 1 through 29, and we conclude that the denominators 1, 4, and all denominators greater than 4 are present.

Malachite generates the rationals contained in the interval in this order:

$$3, 14/5, 11/4, 23/8, 17/6, 25/8, 20/7, 35/12, 25/9, 29/10, 26/9, 30/11, 28/9, 31/11, 31/10, 41/15, 32/11, 37/12, 34/11, 39/14, \ldots$$.
