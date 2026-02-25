# LDraw `.ldr`

## Specification

For more information as described here see the official [LDraw Specification](https://ldraw.org/article/218.html).

### File Structure

```
# Header
# First Step
0 STEP
# Secound Step
0 STEP
...
```

### Header

```
0 title_text
0 Name: filename.ldr
0 Author: author_name
```

### Step

A Step contains the information about the parts and their information in the build. A line for one Piece is specified as follows.

```
1 COLOR POS_X POS_Y POS_Z a b c d e f g h i ID.dat
```

Where `a b c d e f g h i` represents the rotational matrix:

```
/ a d g 0 \    / a b c x \
| b e h 0 |    | d e f y |
| c f i 0 | or | g h i z |
\ x y z 1 /    \ 0 0 0 1 /
```

#### Color

For color definition see at the [LDraw Colors](https://ldraw.org/article/547.html).

## Example

This example is from [Wikipedia](https://en.wikipedia.org/wiki/LDraw#Specification).

```
0 Example Pyramid for Demonstration of LDRAW Library
0 Name: pyramid.ldr
0 Author: James Jessiman

1 1 -40 -24 60 1 0 0 0 1 0 0 0 1 3001.dat
1 1 40 -24 60 1 0 0 0 1 0 0 0 1 3001.dat
1 1 60 -24 0 0 0 1 0 1 0 -1 0 0 3001.dat
1 1 40 -24 -60 1 0 0 0 1 0 0 0 1 3001.dat
1 1 -40 -24 -60 1 0 0 0 1 0 0 0 1 3001.dat
1 1 -60 -24 0 0 0 1 0 1 0 -1 0 0 3001.dat

0 STEP

1 4 -20 -48 40 1 0 0 0 1 0 0 0 1 3001.dat
1 4 40 -48 20 0 0 1 0 1 0 -1 0 0 3001.dat
1 4 20 -48 -40 1 0 0 0 1 0 0 0 1 3001.dat
1 4 -40 -48 -20 0 0 1 0 1 0 -1 0 0 3001.dat

0 STEP

1 14 0 -72 20 1 0 0 0 1 0 0 0 1 3001.dat
1 14 0 -72 -20 1 0 0 0 1 0 0 0 1 3001.dat

0 STEP

1 0 0 -96 0 1 0 0 0 1 0 0 0 1 3003.dat

0 STEP
```
