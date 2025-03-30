# Pythagorean triplet

## Problem

Find all the solutions $x, y, z, a \in \mathbb{Z}$ such that:

1. $x, y, z, a > 0$
2. $x + y + z = a$
3. $x^2 + y^2 = z^2$
4. $x < y$

## Solving math constraints

5. By (1), (2) and (4):

$$0 < x < y < a$$

6. By (2) and (3):

$$y = \frac{a(a-2x)}{2(a-x)}$$

7. By (5) and (6):

$$0 < x < a \cdot \frac{2- \sqrt{2}}{2}$$

8. By (6) and parity:

$$y \in \mathbb{Z}+ \iff a \text{ is even}$$

## Brute force strategy

0. If $a$ is odd, does not exist a Pythagorean triplet such that its sum is $a$ (8).

1. Explore all the $x$ satisfying the bounds specified in (7).

2. Follow to next step only if $y \in \mathbb{Z}_+$ and $x^2 + y^2$ is a perfect square.

3. Check if the values satisfy (2).

