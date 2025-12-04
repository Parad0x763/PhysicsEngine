# Phyiscs Engine

## Radians and Degrees

- degrees = radians * 180 / PI
- radians = degrees * PI / 180

## Contact Resolution

- using an iterative approach to handling contact, or the way in which touching objects are processed
  - this approach has the advantage of speed to resolve contacts quickly
  - disadvantage in the contacts can affect each other, sometimes these interactions are significant
- *Impulse* is when a change is velocity is caused by a force that acts on it for a small fraction of a second
  - think a ball bouncing on the ground then going back up in the air
- This engine will be *impulse* driven, like many other physics game engines

## Vectors and Direction

- A change in position, given as a vector, `a = dn`
  - where `d` is a straight-line distance of the change, *magnitude* and `n` is the direction of the change
  - the vector `n` represents a change whose straight-line distance is always 1, same direction as vector `a`, `n` is often called the *unit vector* since it's magnitude is always 1
  - Solve for `d` using the 3D version of the *Pythagoras Theorem*
  `d = |a| = sqrt(x^2 + y^2 + z^2)`
    - where x, y, and z are the components of the vector and `|a|` is the magnitude
  - Solve for `n`:
  ^<sub>a</sub> = n = 1/d * a
    - where <sub>a</sub> = a / |a|
  - the process of finding just the direction `n` for a vector is called *normalizing* and the result of decomposing a vector into its two components is called the normal form of the vector
    - i.e. `dn` is the normal form of `a`
    
### Vector Multiplication

- *Scalar Product* `(a * b)` where * represents a dot
  + `= axbx + ayby + azbz`
- *Vector Product* `(a X b)`
- *Component Product* `a o b = [ ax ay az] o [bx by bz] = [ axbx ayby azbz]`

#### Trigonometry of the Scalar Product

- `a * b = axbx + ayby + azbz = |a||b| cos(theta)`
- if we have to normalized vecotrs, ^<sub>a</sub> and ^<sub>b</sub>, then the angle between them:
  + `theta = cost^-1 (` ^<sub>a</sub> * ^<sub>b</sub> `)`
  + If not normalized: `theta = cost^-1 (` ^<sub>a</sub> * ^<sub>b</sub> `\ |a||b| )`

#### Geometry of the Scalar Product

- ![geometry-of-scalar-1](/home/parad0x763/Pictures/Screenshots/geometry-of-scalar-product-1-of-2.png)
- ![geometry-of-scalar-2](/home/parad0x763/Pictures/Screenshots/geometry-of-scalar-product-2-of-2.png)

#### Trigonometry of the Vector Product

- `|a X b| = |a||b|sin(theta)` where theta is the angle between the vectors
- We can write `|a X b| = |a||b|sqrt( 1 - (a * b)^2 )` using the famous trig relationship between cosine and sine
  - `cos^2(theta) + sin^2(theta) = 1`
  - This could be used to find the angle, but it would be much more performant to just use the Scalar Product for that perpose

#### Communtativity of the Vector Product

- `a X b != b X a` which is different than `a o b = b o a` and `a * (dot) b = b * a`, `a X b = -b X a`

#### The Geometry of the Vector Product

- ![geometry-of-vector-product](/home/parad0x763/Pictures/Screenshots/geometry-of-the-vector-product.png)

#### The Orthonormal Basis

- *Orthonormal Basis* is a triple vector that is both normalized and orthogonal
- The algorithm process
  1. Normalize the starting vector *a*
  2. Find vector *c* by performing the cross-product `c = a X b`
  3. If vector *c* has a zero magnitude, then give up: *a* and *b* are parallel
  4. Normalize vector *c*
  5. Ensure that *a* and *b* are at right angles. Recalculate *b* based on *a* and *c* using the *cross-product*
    - `b = c X a`

## Calculus

### Differential Calculus

- We can view the differential of a quantity as being the rate that it is changing

#### Velocity

- If velocity represents a moving object, we can work out the velocity the object is moving by looking at two posiitons and waiting for a short time to pass
  + `v = (p^| - p) / (delta_time) = (delta_P) / (delta_time)
- To get an exact, or very accurate calculation you need to use: v = lim<sub>delta_time -> 0</sub> (delta_P / delta_time)
  + `dp / dt` is the more common notation
  + simplified further with *<sub>p</sub>, where * represents a dot
  
#### Acceleration

- Represents the rate the velocity is changing
  + a = lim<sub>delta_time -> 0</sub> (delta_V) / (delta_time) = dv / dt
- Velocity is the First Differential and Acceleration is the Second Differential
  + a = dv / dt = (d / dt) (dp / dt) = d^2 p / dt ^2
- ![acceleration-notation](/home/parad0x763/Pictures/Screenshots/acceleration-notation.png)
- ![acceleration-notation-continued](/home/parad0x763/Pictures/Screenshots/acceleration-notation-continued.png)

#### Vector Differential Calculus

- ![vector-differential-calculus](/home/parad0x763/Pictures/Screenshots/vector-differential-calculus.png)

#### Velocity, Direction, and Speed

- *Velocity* is a vector giving the rate that its position is changing
- *Speed* is the magnitude of this velocity vector
  + .<sub>x</sub> = s^<sub>d</sub>
    - where *s* is the speed of the object and ^<sub>d</sub> is its direction of movement
- using equations for magnitude and direction of any vector the speed is given by: s = | .<sub>x</sub> |
  + the direction by: ^<sub>d</sub> = .<sub>x</sub> / | .<sub>x</sub> |
  + both speed and direction can be calculated with the *magnitude* and *normalize* methods

### Vector Integral Calculus

- p^i = p + .<sub>p</sub>t
  + matrix1x3(
    p<sub>x</sub> + .<sub>p<sub>x</sub></sub>t
    p<sub>y</sub> + .<sub>p<sub>y</sub></sub>t
    p<sub>z</sub> + .<sub>p<sub>z</sub></sub>t
  )
  + `position += velocity * t;`
  + `position.addScaledVector(velocity, t);`
