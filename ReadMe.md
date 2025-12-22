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

- .<sub>p</sub> is the velocity of p measured a one instant in time
- ..<sub>p</sub> is the acceleration of p measured in the same way, one instant in time
- Second-Order Taylor expansion for the position of a particle
- `p` represents the initial position
- p^i = p + .<sub>p</sub>t
  + matrix1x3(
    p<sub>x</sub> + .<sub>p<sub>x</sub></sub>t
    p<sub>y</sub> + .<sub>p<sub>y</sub></sub>t
    p<sub>z</sub> + .<sub>p<sub>z</sub></sub>t
  )
  + `position += velocity * t;`
  + `position.addScaledVector(velocity, t);`

## Laws of Motion

- *Point Masses* is an object that has mass, but no size, can't rotate, but otherwise moves around normally
  - In game Physics, these are usually referred to as *Particals*
  
### The Particle

- Has a *position* but no orientation as in we only care where it is going
  - Think a bullet where we don't care about the direction the bullet is pointing, but rather we care about the direction it is traveling
- Each *Particle* has various properties: position, velocity, acceleration, for starters

### The First Two Laws

1. An object continues with a constant velocity unless a force acts unpon it.
2. A force acting on an object produces acceleration that is proportional to the object's mass.

#### The First Law

- The first law states that is no other force is acting on an object that is already in motion,
  - then it will continue moving forever
- *Damping* will be used to act as the drag without envolving complicated calculations and rounding errors

#### The Second Law

- *Velocity* and *Position* keep track of a quantity frame to frame
    - these values change indirectly
- *Acceleration* can be different from one moment to another due to forces that are applied

### The Force Equations

- f = ma = m..<sub>p</sub>
- This can be manipulated to give use the acceleration in terms of the force:
    - ..<sub>p</sub> = (1 / m) * f
        - Where ..<sub>p</sub> = ( (d^2) * p) / (d * (t^2) ) = a
        - Where a = lim<sub>delta_time -> 0</sub> * (dv) / (dt)

### Movmentum And Velocity

- *Momentum* is the product of velocity and mass
- mass is normally constant
- Rotating objects can change the way their mass is distributed, which can affect rotational speed

### The Force of Gravity

- f = G * ( (mass_one * mass_two) / (r^2) ) where r is the distance between the masses
- Assuming that r is such a large number that small changes in elevation to earth do not matter,
    - also that the mass of earth doesn't change
    - f = m*g
- Gravity can be defined by: g = G * ( (mass_earth) / r^2 )
    - approximated to 9.807 meters per second squared

### The Value of `g`

- Games often use a higher value for Gravity
- This engine will tune the `g` value on an object-by-object Basis
- g = [ 0 -g 0 ] because the Y axis represents up and down in the game world

### The Integrator

- Consists of two parts
    1. update the position of the object
        - depends on the velocity and acceleration
    2. update its velocity
        - depends only on the acceleration
- This is updated on a time schedule, common to use the delta time between frames,
    - better to decouple the physics from the frame rate!!!!
- Probably wise to use the System Clock to get a more consistent time
    - [Rust-Docs_Instant](https://doc.rust-lang.org/std/time/struct.Instant.html)
- Velocity: .i<sub>p</sub> = .<sub>p</sub>d^t + ..<sub>p</sub>t
    - where d is the damping for the object is now the proportion of the velocity retained each second rather than each frame


## Adding General Forces

### D'Alembert's Principle

- This prinicple imples that is we have a set of forces acting on an object, we can replace all forces with a single force
    - f = sum<subi</sub> (f<sub>i</sub>)
    - We simply add the forces together using vector addition and apply the single force results

### Force Generators

- Some forces asrise because of the behavior of an object, such as dedicated drag

#### Interfaces and Polymorphism

- An *Interface* is a specification of methods, constants, data types, and execeptions / errors that will be exposed, typically a *class*

##### Rust `Traits`

- [Rust-Programming-Language: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- A *trait* defines functionality a particular type has and can share with other types
- ```pub trait Summary { fn summarize(&self) -> String; }```
- ```impl Summary for SomeType { fn summarize(&self) -> String { return self.string_value; } }```

### A Drag Force Generator

- In game applications we use a simplified model of drag where the drag acting on the body depends on the speed of hte object and the square of its speed
    - f<sub>drag</sub> = ^<sub>.<sub>p</sub></sub> ( k<sub>1</sub> |^<sub>.<sub>p</sub></sub>| + k<sub>2</sub> |^<sub>.<sub>p</sub></sub>|^2)
    - where k<sub>1</sub> and k<sub>2</sub> are constants that characterize how strong the drag force is, usually called *drag coefficients*
- The formula syas that the force acts in the opposite direction to the velocity of the object 
    - ^<sub>.<sub>p</sub></sub> is the normalized velocity of the particle
    - the strenght depends both on the speed of the object and the square of the speed
- Drag that has a k<sub>2</sub> value will grow faster at higher speeds
