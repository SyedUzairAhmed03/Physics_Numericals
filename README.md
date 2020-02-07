# Phy-Numerical
It is the **0.1.4** version of **Phy-Numerical** crate 
Here you can solve ***Phy-Numericals*** of 
* **First Equation of Motion** 
* **Second Equation of Motion**
* **Third Equation of Motion**

Add **phy_numerical = "0.1.4"** in **cargo.toml** file in  **[dependencies]**
like 
```Rust
[dependencies]
phy_numerical = "0.1.4"
```
- For solving the **Numerical** releated to **First Equation of motion**
You just write **phy_numerical::First_equation_of_motion();** in Main.rs File like this 
```Rust
use std::io;

extern  crate phy_numerical;

fn main () {

//For First equation of motion

phy_numerical::First_equation_of_motion();

}
```
- For **Second Equation of motion**.
You write **phy_numerical::Second_equation_of_motion();** in Main.rs File like this 
```Rust
use std::io;

extern  crate phy_numerical;

fn main () {

//For Second equation of motion

phy_numerical::Second_equation_of_motion();

}
```
> **Note** : You cannot find time in second Equation of Motion Because it is impossible to separate *t* on the right side of equation.
- For **Third Equation of motion**.
You write **phy_numerical::Third_equation_of_motion();** in Main.rs File like this 
```Rust
use std::io;

extern  crate phy_numerical;

fn main () {

//For Third equation of motion

phy_numerical::Third_equation_of_motion();

}
```
### Variables and their meanings
* s = Distance.
* h = Height.
* g = Acceleration due to gravity  
  > Constant 9.8m/s
* a =  Acceleration.
* v = Velocity.
* v*i* = Initial Velocity.
*  v*f* = Final Velocity.
* m/s = Meter per second
* km/h = Kilometer per hour
