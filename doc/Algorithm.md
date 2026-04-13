# MiniNEC Algorithm

## Introduction

[MiniNEC](https://apps.dtic.mil/sti/html/tr/ADA121535/index.html) is a software developed in 1981 by researchers at the Naval Ocean Systems Center (NOSC), USA. The project
aimed to investigate whether it was possible to adapt techniques to create a reduced version of the NEC (Numerical 
Electromagnetics Code) antenna modeling software, enabling it to solve small problems on computers with limited 
resources. The resulting code was written in BASIC and could run on both an Apple II and a UNIVAC, each with only 64 
kilobits of data memory available. 

Its algorithm is based on the Method of Moments (MoM), a numerical technique used to solve integral equations 
in electromagnetics. The main idea is to discretize the antenna structure into small segments and assume that the 
current distribution along each segment can be approximated by a simple function. This leads to a system of linear 
equations that can be solved to find the current distribution on the antenna, which in turn allows for the calculation 
of various antenna parameters such as input impedance, radiation pattern, and gain.

## Maxwell Equations, Vector Potential and Wave Equations

In order to understand the algorithm behind the Method of Moments (MoM) for antenna simulation,
and thus how MiniNEC works, it is essential to understand how we can express induction and radiation mathematically.
By applying some vector calculus identities on the Maxwell equations, one can derive the wave equations for
the electric and magnetic fields, which describe how these fields propagate in space and time. Unfortunately,
the inhomogeneous wave equations for the electric and magnetic fields obtained directly from the electric and magnetic
field curls contain a source term that depends on the charge distribution gradient. Such gradient i not readily 
available in the antenna simulation scenario, and to overcome this issue, the vector potential and scalar potential 
are introduced. By expressing the electric and magnetic fields in terms of such potentials, under a Lorenz gauge 
condition the wave equations become decoupled and homogeneous, allowing us to solve for the potentials independently.

### Vector Calculus Basic Identities

Before diving into the Maxwell's equations manipulations, it's important to remember some basic understanding
of vector calculus, which is the mathematical framework used to express these equations.

* A scalar field is a function that assigns a single value to every point in space, such as temperature or pressure.

* A vector field is a mathematical representation of a physical quantity that has both magnitude and direction at 
every point in space. In the context of electromagnetism, the electric field ($\mathbf{E}$) and 
magnetic field ($\mathbf{B}$) are examples of vector fields.

For a scalar field $\phi$, its **gradient** ($\nabla \mathbf{\phi}$) is a vector field points in the 
direction of the greatest rate of increase of the field and has a magnitude equal to that rate of increase.
Its **laplacian** ($\nabla^2 \phi$) is a scalar field that gives the local curvature of the scalar field.

For some vector field $\mathbf{V}$, its **divergence** ($\nabla \cdot \mathbf{V}$) measures the net flow of 
the field out of an infinitesimal volume. A positive divergence indicates a source, while a negative divergence 
indicates a sink. The **curl** of a vector field ($\nabla \times \mathbf{V}$) measures the rotation or swirling 
of the field around a point as a vector field orthogonal to the plane of rotation. A non-zero curl indicates 
that the field has a rotational component.

Those operators keep some important properties which are crucial in the context of electromagnetism:
* $\nabla \cdot (\nabla \phi) = \nabla^2 \phi$: The divergence of the gradient of a scalar field is equal to 
the Laplacian of that field.
* $\nabla \times (\nabla \phi) = 0$: The curl of the gradient of any scalar field is always zero.
* $\nabla \cdot (\nabla \times \mathbf{V}) = 0$: The divergence of the curl of any vector field is always zero.
* $\nabla \times (\nabla \times \mathbf{V}) = \nabla (\nabla \cdot \mathbf{V}) - \nabla^2 \mathbf{V}$: This identity
relates the curl of the curl of a vector field to the divergence and the Laplacian of the field.

With these identities in mind, we can now proceed to manipulate Maxwell's equations and derive the wave equations for 
the electric and magnetic fields, as well as the vector and scalar potentials.

### Maxwell's Equations

Let's start by recalling Maxwell's equations, defining $\mathbf{E}$ as the electric field strength,
$\mathbf{B}$ as the magnetic flux density, $\rho$ as the charge density, and $\mathbf{J}$ as
the current density.

The Gauss's Laws for electricity and magnetism are expressed as the divergence of such fields:
```math
\nabla \cdot \mathbf{E} = \frac{\rho}{\varepsilon_0},
```
```math
\nabla \cdot \mathbf{B} = 0.
```
Their physical interpretation is that the electric field lines begin and end on electric charges,
and that there is no magnetic monopoles, thus there is no sink or source for magnetic field lines.

Faraday's Law of Induction is expressed as the curl of the electric field:
```math
\nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t},
```
and it describes how a time-varying magnetic field induces a circulating electric
field in its perpendicular plane.

Finally, Ampère's Law with Maxwell's Addition is expressed as the curl of the magnetic field:
```math
\nabla \times \mathbf{B} = \mu_0 \mathbf{J} + \mu_0 \varepsilon_0 \frac{\partial \mathbf{E}}{\partial t},
```
and it describes how an electric current density $\mathbf{J}$ induce a circulating magnetic field
in their perpendicular plane. Maxwell's Addition term includes the circulating magnetic field induced by a
time-varying electric field.


### Wave Equations

By manipulating the Maxwell's equations using vector calculus, we can derive the wave equations for the electric and 
magnetic fields. First it is useful to define the magnetic field strenght 
```math
\mathbf{H}=\frac{1}{\mu_0}\mathbf{B},
``` 
then, by applying the curl operator to Faraday's Law of Induction and using the vector calculus identities, 
we can derive the wave equations for the electric and magnetic fields:
```math
\nabla \times (\nabla \times \mathbf{E}) = \nabla \times\left( -\mu_0\frac{\partial \mathbf{H}}{\partial t}\right)
```

The time derivative and curl operator commute, so we can rewrite the equation as:
```math
\nabla \times (\nabla \times \mathbf{E}) = -\mu_0 \frac{\partial}{\partial t} (\nabla \times \mathbf{H})
```

Now we substitute Ampère's Law with Maxwell's Addition into the equation:
```math
\nabla \times (\nabla \times \mathbf{E}) = -\mu_0 \frac{\partial}{\partial t}\mathbf{J} - 
\mu_0 \varepsilon_0 \frac{\partial^2 \mathbf{E}}{\partial^2 t}
```

Using the vector calculus identity 
$\nabla \times (\nabla \times \mathbf{E}) = \nabla (\nabla \cdot \mathbf{E}) - \nabla^2 \mathbf{E}$ we get:
```math
\nabla (\nabla \cdot \mathbf{E}) - \nabla^2 \mathbf{E} = -\mu_0 \frac{\partial}{\partial t}\mathbf{J} - 
\mu_0 \varepsilon_0 \frac{\partial^2 \mathbf{E}}{\partial^2 t}
```

We then rearrange the equation and apply Gauss's Law for Electricity to express the divergence of the electric field in 
terms of the charge density:
```math
\nabla^2 \mathbf{E} - \mu_0 \varepsilon_0 \frac{\partial^2 \mathbf{E}}{\partial^2 t} = 
\nabla \left(\frac{\rho}{\varepsilon_0}\right) + \mu_0 \frac{\partial}{\partial t}\mathbf{J}
```

This is called the non-homogenous wave equation for the electric field. In the antenna simulation scenario,
this equation poses a challenge as we need, beside segment currents, the charge distribution gradient over space.

### Vector Potential

As a means to solve the problem of needing the charge distribution gradient, we can introduce
the vector potential **A** and scalar potential **φ** to express the electric and magnetic fields.
The vector potential is defined such that:
```math
\mathbf{B} = \nabla \times \mathbf{A}
```

This definition is justified by Gauss's Law for Magnetism, which states that $\nabla \cdot \mathbf{B} = 0$.
Since the divergence of any curl is always zero, we can express the magnetic field as the curl of a vector
field without violating this law. This is a powerful mathematical insight, since instead of working directly
with the magnetic field, we can work with the vector potential, which often simplifies the equations
and calculations involved in electromagnetic problems.

Also, by using the vector potential in Faraday's Law of Induction
```math
\nabla \times \mathbf{E} = -\frac{\partial}{\partial t} (\nabla \times \mathbf{A}),
```
and collecting all terms inside the curl operator, we get the expression
```math
\nabla \times \left(\mathbf{E} + \frac{\partial \mathbf{A}}{\partial t}\right) = 0.
```

The curl of a gradient is always zero, the expression inside the curl can be represented by the gradient
of a scalar potential:
```math
\mathbf{E} + \frac{\partial \mathbf{A}}{\partial t} = -\nabla \phi,
```
where the negative sign is a convention to ensure that the electric field points in the direction of
decreasing potential. This equation shows how the electric field can be expressed in terms of the
vector potential and the scalar potential, which is particularly useful in solving electromagnetic
problems, such as those involving antennas. Thus the electric field can be expressed as:
```math
\mathbf{E} = -\nabla \phi - \frac{\partial \mathbf{A}}{\partial t}.
```

Now, applying those two definitions in the Ampère's Law with Maxwell's Addition, we can derive the
wave equations for the vector and scalar potentials. The resulting equation is known as the inhomogeneous
wave equation for the potentials, and it is given by:
```math
\nabla \times (\nabla \times \mathbf{A}) = 
\mu_0 \mathbf{J} + \mu_0 \varepsilon_0 \frac{\partial}{\partial t}\left(-\nabla \phi 
- \frac{\partial \mathbf{A}}{\partial t}\right)
```

Applying the curl of the curl identity and rearranging the equation, we get:
```math
\nabla (\nabla \cdot \mathbf{A}) - \nabla^2 \mathbf{A} = \mu_0 \mathbf{J} 
- \mu_0 \varepsilon_0 \nabla \frac{\partial \phi}{\partial t} 
- \mu_0 \varepsilon_0 \frac{\partial^2 \mathbf{A}}{\partial^2 t}
```

Next, by collecting the terms under the gradient operator, we can express the equation as:
```math
\nabla \left(\nabla \cdot \mathbf{A} + \mu_0 \varepsilon_0 \frac{\partial \phi}{\partial t}\right) 
- \nabla^2 \mathbf{A} - \mu_0 \varepsilon_0 \frac{\partial^2 \mathbf{A}}{\partial^2 t} = \mu_0 \mathbf{J}
```

Here we have again an inhomogeneous wave equation for the vector potential, and it is clearly coupled with
the scalar potential through the term $\nabla \cdot \mathbf{A} + \mu_0 \varepsilon_0 \frac{\partial \phi}{\partial t}$.
To decouple these equations, we can choose a specific gauge condition, such as the Lorenz gauge,
which is defined by:
```math
\nabla \cdot \mathbf{A} + \mu_0 \varepsilon_0 \frac{\partial \phi}{\partial t} = 0.
```
This gauge condition simplifies the vector potential equation to:
```math
\nabla^2 \mathbf{A} - \mu_0 \varepsilon_0 \frac{\partial^2 \mathbf{A}}{\partial^2 t} = -\mu_0 \mathbf{J}
```
which is a homogeneous wave equation for the vector potential.

The scalar potential can be derived from the Gauss's Law for Electricity by substituting the expression for the 
electric field in terms of the potentials:
```math
\nabla \cdot \left(-\nabla \phi - \frac{\partial \mathbf{A}}{\partial t}\right) = \frac{\rho}{\varepsilon_0}
```
Rearranging the equation, we get:
```math
\nabla^2 \phi + \frac{\partial}{\partial t} (\nabla   \cdot \mathbf{A}) = -\frac{\rho}{\varepsilon_0}
```
Applying the Lorenz gauge condition, we can simplify this equation to:
```math
\nabla^2 \phi - \mu_0 \varepsilon_0 \frac{\partial^2 \phi}{\partial^2 t} = -\frac{\rho}{\varepsilon_0}
```
which is a homogeneous wave equation for the scalar potential.

Note that we have the relation between the speed of light in vacuum $c$,
the permeability ($\mu_0$) and permittivity ($\varepsilon_0$) of free space:
```math 
c = \frac{1}{\sqrt{\mu_0 \varepsilon_0}}
```
and thus we can rewrite the wave equations for the vector and scalar potentials in terms of the speed of light as:
```math
\nabla^2 \mathbf{A} - \frac{1}{c^2}\frac{\partial^2 \mathbf{A}}{\partial^2 t} = -\mu_0 \mathbf{J}
```
```math
\nabla^2 \phi - \frac{1}{c^2} \frac{\partial^2 \phi}{\partial^2 t} = -\frac{\rho}{\varepsilon_0}
```
Those wave equations represent respectively, the induction and radiation components of the electromagnetic
field, and they can be solved independently due to the Lorenz gauge condition.

The Lorenz gauge ensures relativistic invariance of the equations and allows us to treat the vector and scalar 
potentials independently, making it easier to solve electromagnetic problems. The resulting wave equations for the 
potentials can be solved using various methods, including the Green's function approach, which provides a systematic 
way to find the solutions for arbitrary source distributions.

### Green's Function Solution for the Wave Equations
The Green's function is a powerful mathematical tool used to solve inhomogeneous differential equations,
such as the wave equations for the vector and scalar potentials. The Green's function represents the
response of the system to a point impulse source, and it can be used to construct the solution for any arbitrary
source distribution by means of convolution operations.

The Green's function for the wave equation in three-dimensional space is given by:
```math
G(\mathbf{r}, t; \mathbf{r}', t') = \frac{\delta\left(t - t' - \frac{|\mathbf{r} - \mathbf{r}'|}{c}\right)}
{4\pi |\mathbf{r} - \mathbf{r}'|}   
```
where $c$ is the speed of light in vacuum, and $\delta$ is the Dirac delta function. This Green's function represents 
the response of the system at position $\mathbf{r}$ and time $t$ due to a point source located at position 
$\mathbf{r}'$ that emits an impulse at time $t'$. The delta function ensures that the response occurs only when the 
time difference matches the time it takes for a signal to travel from the source to the observation point at the 
speed of light.

Using the Green's function, we can express the solutions for the vector potential as an integral over the source 
distribution:
```math
\mathbf{A}(\mathbf{r}, t) = \mu_0 \int \int G(\mathbf{r}, t; \mathbf{r}', t') \mathbf{J}(\mathbf{r}', t') d^3r' dt'
```
or
```math
\mathbf{A}(\mathbf{r}, t) = \mu_0 \int \int \frac{\delta\left(t - t' - \frac{|\mathbf{r} 
- \mathbf{r}'|}{c}\right)}{4\pi |\mathbf{r} 
- \mathbf{r}'|} \mathbf{J}(\mathbf{r}', t') d^3r' dt'
```

The time integral can easily be evaluated by defining the retarded time
$t' = t - \frac{|\mathbf{r} - \mathbf{r}'|}{c}$,
which accounts for the time it takes for the signal to propagate from the source to the observation point.
Then, by the shifting property of the Dirac delta distribution, this leads to the expression:
```math
\mathbf{A}(\mathbf{r}, t) = \frac{\mu_0}{4\pi} \int \frac{\mathbf{J}\left(\mathbf{r}', t 
- \frac{|\mathbf{r} - \mathbf{r}'|}{c}\right)}{|\mathbf{r} - \mathbf{r}'|} d^3r'.
```

In most practical applications, such as antenna simulations, the current distribution $\mathbf{J}$ is assumed
to be time-harmonic, meaning it varies sinusoidally with time. In this case, we can express the current as:
```math
\mathbf{J}(\mathbf{r}', t') = \mathbf{J}(\mathbf{r}') e^{j\omega t'} = \mathbf{J}(\mathbf{r}') e^{j\omega 
\left(t - \frac{|\mathbf{r} - \mathbf{r}'|}{c}\right)} = \mathbf{J}(\mathbf{r}') e^{j\omega t} 
e^{-j\omega \frac{|\mathbf{r} - \mathbf{r}'|}{c}}.
```
where $\omega$ is the angular frequency of the current. Substituting this into the expression for
the vector potential, we get:
```math
\mathbf{A}(\mathbf{r}, t) = \frac{\mu_0}{4\pi} e^{j\omega t} \int \frac{\mathbf{J}(\mathbf{r}') 
e^{-j\omega \frac{|\mathbf{r} - \mathbf{r}'|}{c}}}{|\mathbf{r} - \mathbf{r}'|} d^3r'.
```

Introducing the wavenumber $k = \frac{\omega}{c}$, and assuming $t=0$, we can rewrite the expression in its final form:
```math
\mathbf{A}(\mathbf{r}) = \frac{\mu_0}{4\pi} \int \frac{\mathbf{J}(\mathbf{r}') e^{-jk|\mathbf{r} 
- \mathbf{r}'|}}{|\mathbf{r} - \mathbf{r}'|} d^3r'.
```

Due to the symmetry established by the Lorenz gauge, the scalar potential $\Phi$ obeys a wave equation
identical in form to that of the vector potential $\mathbf{A}$. Consequently, its solution follows the
same Green's function approach, substituting the current density $\mathbf{J}$ for the charge density
$\rho$ and the permeability $\mu_0$ for the inverse of the permittivity $1/\epsilon_0$.
Thus the scalar potential can be expressed as:
```math
\Phi(\mathbf{r}) = \frac{1}{4\pi \varepsilon_0} \int \frac{\rho(\mathbf{r}') 
e^{-jk|\mathbf{r} - \mathbf{r}'|}}{|\mathbf{r} - \mathbf{r}'|} d^3r'.
```

### Solving the Fields From the Potentials

Having the scalar and vector potentials solutions at hand, the electric field strength is obtained from the fundamental
relation between the potentials and the electric field:
```math
\mathbf{E}(\mathbf{r},t)=-\frac{\partial \mathbf{A}(\mathbf{r},t)}{\partial t}-\nabla\Phi(\mathbf{r},t),
```
which in the frequency domain becomes:
```math
\mathbf{E}(\mathbf{r})=-j\omega\mathbf{A}(\mathbf{r})-\nabla\Phi(\mathbf{r}).
```

Although not used in the method of moments for antenna simulation, the magnetic flux intensity is readily available 
from the defining property of the vector potential:
```math
\mathbf{B}(\mathbf{r})=\nabla\times\mathbf{A}(\mathbf{r}).
```
Expanding the expression using the previously calculated $\mathbf{A}(\mathbf{r})$, we have:
```math
\mathbf{B}(\mathbf{r}) = \frac{\mu_0}{4\pi} \int_{V} \nabla \times \left( \mathbf{J}(\mathbf{r}') 
\frac{e^{-jk|\mathbf{r}-\mathbf{r}'|}}{|\mathbf{r}-\mathbf{r}'|} \right) d^3r'.
```
This integral can be simplified by the relation 
$\nabla \times (f \mathbf{V}) = (\nabla f) \times \mathbf{V} + f\cdot(\nabla \times \mathbf{V})$, 
which is valid for any scalar function $f$. Defining:
```math
f(\mathbf{r}) =  \left( \frac{e^{-jk|\mathbf{r}-\mathbf{r}'|}}{|\mathbf{r}-\mathbf{r}'|} \right),
```
we can rewrite:
```math
\nabla \times \left( \mathbf{J}(\mathbf{r}') f(\mathbf{r})\right)=
\nabla \left(f(\mathbf{r})\right) \times \mathbf{J}(\mathbf{r}')+f(\mathbf{r})\cdot(\nabla\times\mathbf{J}(\mathbf{r}')). 
```
Since the curl operator $\nabla$ acts upon the observation coordinates $\mathbf{r}$, and the current density 
$\mathbf{J}(\mathbf{r}')$ is a function only of the source coordinates 
$\mathbf{r}'$, the term $\nabla \times \mathbf{J}(\mathbf{r}')$ is identically zero, and the second term on the right 
hand side vanishes:
```math
\nabla \times \left( \mathbf{J}(\mathbf{r}') f(\mathbf{r})\right)=
\nabla \left(f(\mathbf{r})\right) \times \mathbf{J}(\mathbf{r}'). 
```

If we define the distance function $R = |\mathbf{r} - \mathbf{r}'|$,
we can evaluate the gradient of the scalar function by using the chain rule
```math
\nabla f(R) = \frac{df}{dR} \nabla R.
```
Then we have
```math
\frac{\partial f(R)}{\partial R}=\frac{\partial}{\partial R}\left( \frac{e^{-jkR}}{R} \right)= 
-\left( jk + \frac{1}{R} \right) \frac{e^{-jkR}}{R},
```
and
the gradient of the distance can be evaluated to:
```math
\nabla |\mathbf{r} - \mathbf{r}'| = \frac{\mathbf{r} - \mathbf{r}'}{|\mathbf{r} - \mathbf{r}'|} = \hat{\mathbf{R}}.
```

Combining the expressions, we have:
```math
\mathbf{B}(\mathbf{r}) = \frac{\mu_0}{4\pi} \int_{V} \mathbf{J}(\mathbf{r}') \times \left(\frac{\mathbf{r} 
- \mathbf{r}'}{|\mathbf{r} - \mathbf{r}'|} \right) \left( jk + \frac{1}{|\mathbf{r} 
- \mathbf{r}'|} \right) \frac{e^{-jk|\mathbf{r} 
- \mathbf{r}'|}}{|\mathbf{r} - \mathbf{r}'|} d^3r'.
```
Rearranging the terms:
```math
\mathbf{B}(\mathbf{r}) = jk\frac{\mu_0}{4\pi} \int_{V} \mathbf{J}(\mathbf{r}') \times 
\left(\frac{\mathbf{r} - \mathbf{r}'}{|\mathbf{r} - \mathbf{r}'|}\right)
\frac {e^{-jk|\mathbf{r} - \mathbf{r}'|}}{|\mathbf{r} - \mathbf{r}'|} d^3r' + 
\frac{\mu_0}{4\pi} \int_{V} \mathbf{J}(\mathbf{r}') \times 
\left(\frac{\mathbf{r} - \mathbf{r}'}{|\mathbf{r} - \mathbf{r}'|}\right)
\frac{e^{-jk|\mathbf{r} - \mathbf{r}'|}}{|\mathbf{r} - \mathbf{r}'|^2} d^3r'.
```
The second term in this expression is the Biot-Savart equation for magnetic fields, and corresponds to the near-field 
magnetic propagation. The first term is the far-field radiation expression.

Note that for direct current ($\omega = 0$), the far-field terms vanish entirely. The induced component of the 
electric field (proportional to $\partial \mathbf{A} / \partial t$) also becomes zero, leaving the system described by 
classic magnetostatics—represented by the magnetic near-field—and electrostatics, where the electric field arises 
solely from the stationary charge distribution ($\mathbf{E} = -\nabla\Phi$).



## Method of Moments in MiniNEC

To analyze complex antenna structures that lack closed-form analytical solutions, MiniNEC employs a numerical 
approach based on the Method of Moments (MoM). The fundamental strategy involves transforming the continuous
integro-differential equations governing the electromagnetic fields into a discrete system of linear algebraic 
equations.

MiniNEC also adopts the so-called thin-wire model, where all structures of the antenna are replaced by a wireframe of 
thin, perfectly electrically conductive wires, which in turn are divided in small segments.
By adopting a thin-wire model, MiniNEC assumes that the wire radius $a$ is significantly smaller than both the wavelength $\lambda$ and the physical length of the segments. This allows for two critical simplifications:
* the current is assumed to be strictly axially directed along the wire's contour; 
* and the angular variation of the current around the wire's circumference is neglected, reducing the problem to a one-dimensional distribution. 

Under these assumptions, the electromagnetic behavior of the antenna is governed by the interaction between the induced 
currents and the boundary conditions at the conductor's surface. Consequently, we can express the fundamental problem 
as follows:

### Problem statement

The objective is to determine the complex current distribution $\mathbf{I}(\mathbf{r}')$ induced on a structure of thin, 
perfectly conducting wires by an external excitation. In this formulation, $\mathbf{r}$ denotes the observation position 
vector and $\mathbf{r}'$ represents the source position vector along the wire's axis.

#### The Boundary Condition 
For a Perfect Electric Conductor (PEC), the total tangential electric field must vanish at every point $\mathbf{r}$ on 
the conductor's surface. We define the total tangential field $\mathbf{E}^{tan}(\mathbf{r})$ as the sum of the incident 
$\mathbf{E}_{in}^{tan}(\mathbf{r})$ and the scattered $\mathbf{E}_{sc}^{tan}(\mathbf{r})$ fields:
```math
\mathbf{E}^{tan}(\mathbf{r}) = \mathbf{E}_{in}^{tan}(\mathbf{r}) + \mathbf{E}_{sc}^{tan}(\mathbf{r}) = 0
``` 
Considering only the tangential component along the wire's unit vector $\hat{s}$ at the observation point $\mathbf{r}$:
```math
\hat{s} \cdot \mathbf{E}_{sc}^{tan}(\mathbf{r}) = -\hat{s} \cdot \mathbf{E}_{in}^{tan}(\mathbf{r})
```

**Note:** For sake of simplifying the notation we will deal only with tangential fields from now on, so the $tan$ 
superscript will be ommited.

#### The Electric Field Integral Equation (EFIE)
The scattered field $\mathbf{E}_{sc}(\mathbf{r})$ is generated by the induced currents and charges in the antenna structure. 
Expressing the field in terms of the magnetic vector potential $\mathbf{A}(\mathbf{r})$ and the electric scalar potential 
$\Phi(\mathbf{r})$, we have:
```math
\mathbf{E}_{sc}(\mathbf{r}) = -j\omega\mathbf{A}(\mathbf{r}) - \nabla \Phi(\mathbf{r}),
```
where the potentials are defined by the unknown current $\mathbf{I}(\mathbf{r}')$ via the free-space Green's function $G(\mathbf{r}, \mathbf{r}')$:
```math
\mathbf{A}(\mathbf{r}) = \frac{\mu_0}{4\pi} \int_L \mathbf{I}(\mathbf{r}') G(\mathbf{r}, \mathbf{r}') dr',
```
derived in the last section. Note that this integral is derived from the general volume integral:
```math
\mathbf{A}(\mathbf{r}) = \frac{\mu_0}{4\pi} \int \frac{\mathbf{J}(\mathbf{r}') e^{-jk|\mathbf{r} 
- \mathbf{r}'|}}{|\mathbf{r} - \mathbf{r}'|} d^3r',
```
where, for a thin-wire approximation, the volume current density is reduced to a filamentary current along the wire path, such that 
```math
\mathbf{J}(\mathbf{r}')d^3r'=\mathbf{I}(\mathbf{r}')dr'.
```

Substituting this into the boundary condition yields the EFIE for the wire structure:
```math
\hat{s} \cdot \left[ j\omega\mathbf{A}(\mathbf{r}) + \nabla \Phi(\mathbf{r}) \right] 
= \hat{s} \cdot \mathbf{E}_{in}(\mathbf{r})
```
In our problem, the incident field $\mathbf{E}_{in}(\mathbf{r})$ corresponds to the external excitation of the antenna. 
In a receiving context, this typically represents a plane wave originating from a distant source. In a transmitting context,
$\mathbf{E}_{in}(\mathbf{r})$ is modeled as the field produced by a potential differential (voltage source) applied across 
a specific gap in the antenna structure.

### Discretization and Basis Functions

Except for a few specialized cases, for a general antenna structure, even wire-based,
the EFIE offers no closed-form solution. Hence, the geometry must be discretized into a
numerically computable system. This is usually achieved by dividing each wire into a 
number of segments, connected at nodes.

I addition, certain assumptions must be made regarding the current 
$\mathbf{I}(\mathbf{r}')$ on each discrete segment, as well as the potential at each node. 

#### Geometry Segmentation and Current Approximation
The antenna wireframe structure is divided into $N$ segments, which are connected at nodes. However, a key distinction in the MiniNEC implementation is that the current is not defined as a single value per segment, but rather at the nodes.

For a structure with $N$ segments, we have $N+1$ nodes. If we consider a single open wire, the current at the two endpoints is enforced to zero, leaving us with $M = N-1$ unknown current values to be determined, while for closed loops all $M = N+1$ nodes will be used as unknowns. Once discretized, the unknown current distribution is approximated by a set of basis 
functions $f_j(\mathbf{r}')$:
```math
\mathbf{I}(\mathbf{r}') \approx \sum_{j=1}^{M} I_j f_j(\mathbf{r}')
```
In MiniNEC, $f_j(\mathbf{r}')$ is chosen as a unit pulse function centered at node $j$, which extends from the midpoint of the preceding segment to the midpoint of the succeeding segment.
This approximation for $\mathbf{I}(\mathbf{r}')$ allows the scattered field $\mathbf{E}_{sc}(\mathbf{r})$ to be represented as a linear operator over the unknown constant values $I_j$. 

The vector potential would then be approximated by
```math
\mathbf{A}(\mathbf{r}) \approx
\frac{\mu_0}{4\pi} \int_{L} \sum_{j=1}^{M} I_j \frac{f_j(\mathbf{r}') e^{-jk|\mathbf{r} 
- \mathbf{r}'|}}{|\mathbf{r} - \mathbf{r}'|} dr'.
```
Due to the linearity of the integral operator, the summation and the unknown constants can be moved outside the integral:
```math
 \mathbf{A}(\mathbf{r}) \approx
\sum_{j=1}^{M} I_j \left[\frac{\mu_0}{4\pi} \int_{L} \frac{f_j(\mathbf{r}') e^{-jk|\mathbf{r} 
- \mathbf{r}'|}}{|\mathbf{r} - \mathbf{r}'|} dr'\right],
```

To determine the scalar potential, we apply the continuity condition in the frequency domain,
```math
\nabla'\cdot\mathbf{I}(\mathbf{r}')=-j\omega\rho(\mathbf{r}').
```
In a thin-wire model, this divergence reduces to the derivative of the current along the path of the wire:
```math
\nabla'\cdot\mathbf{I}(\mathbf{r}')=\frac{\partial I(r')}{\partial r'}.
```

Thus, the Green's function solution for the scalar potential becomes:
```math
\Phi(\mathbf{r}) = \frac{1}{4\pi \varepsilon_0} \frac{j}{\omega} \int_L 
\frac{\partial I(r')}{\partial r'}\frac{
e^{-jk|\mathbf{r} - \mathbf{r}'|}}{|\mathbf{r} - \mathbf{r}'|} dr'.
```
Substituting the current approximation into this expression, we obtain:
```math
\Phi(\mathbf{r}) \approx \frac{1}{4\pi \varepsilon_0} \frac{j}{\omega} 
\int_L \left[ 
    \frac{\partial}{\partial r'} \sum_{j=1}^{M} I_j f_j(\mathbf{r}') 
    \right] 
    \frac{e^{-jk|\mathbf{r} - \mathbf{r}'|}}{|\mathbf{r} - \mathbf{r}'|} dr'.
```

Again, due to the linearity of both the derivative and integral operators, the summation and the unknown coefficients $I_j$ can be extracted:
```math
\Phi(\mathbf{r}) \approx 
\sum_{j=1}^{M} I_j 
\left[ \frac{j}{4\pi \varepsilon_0 \omega} \int_L \frac{\partial f_j(\mathbf{r}')}{\partial r'} \frac{e^{-jk|\mathbf{r} - \mathbf{r}'|}}{|\mathbf{r} - \mathbf{r}'|} dr' \right].
```

A key advantage of choosing the unit pulse as a basis function $f_j(\mathbf{r}')$ 
lies in the evaluation of the scalar potential. Since $f_j$ is defined as a constant value (1) over its support and zero elsewhere, its derivative in the sense of distributions results in two Dirac delta functions:
```math
\frac{\partial f_j(r')}{\partial r'} = \delta(r' - r'_{j-1/2}) - \delta(r' - r'_{j+1/2}),
```
where $r'_{j \pm 1/2}$ represents the midpoints of the segments adjacent to node $j$. Substituting this into the integral for the scalar potential $\Phi_j(\mathbf{r})$ yields:
```math
\Phi_j(\mathbf{r}) = \frac{j}{4\pi \varepsilon_0 \omega} \int_L \left[ \delta(r' - r'_{j-1/2}) - \delta(r' - r'_{j+1/2}) \right] \frac{e^{-jk|\mathbf{r} - \mathbf{r}'|}}{|\mathbf{r} - \mathbf{r}'|} dr'.
```
Due to the shifting property of the Dirac delta, the integral collapses into the difference of the Green's function evaluated at the two midpoints:
```math
\Phi_j(\mathbf{r}) = \frac{j}{4\pi \varepsilon_0 \omega} \left[ G(\mathbf{r}, \mathbf{r}_{j-1/2}) - G(\mathbf{r}, \mathbf{r}_{j+1/2}) \right],
```
where
```math
G(\mathbf{r}, \mathbf{r}') = \frac{e^{-jk|\mathbf{r} - \mathbf{r}'|}}{|\mathbf{r} - \mathbf{r}'|}.
```
This result is mathematically profound for the implementation: it proves that for pulse basis functions, the scalar potential contribution $\Phi_j$ is equivalent to the potential generated by two equal and opposite point charges (a numerical doublet) located at the centers of the segments flanking node $j$.

Both potentials now have the summation over $j$ as a common factor, and the scattered field can be written as:
```math
\mathbf{E}_{sc}(\mathbf{r}) = \sum_{j=1}^M I_j
\left[-j\omega\mathbf{A}_j(\mathbf{r}) - \nabla \Phi_j(\mathbf{r})\right].
```

The gradient of the potential can be approximated as the difference:
```math
\nabla \Phi_j(\mathbf{r}_i) \cdot \hat{\mathbf{s}}_i \approx \Phi_j(\mathbf{r}_{i+1/2}) - \Phi_j(\mathbf{r}_{i-1/2}),
```
which requires four evaluations of Green's function.

#### Point Matching and the Impedance Equation

To solve for the $M$ unknown currents, we enforce the boundary condition that the total tangential electric field must vanish on the surface of the conductor:
```math
\mathbf{E}_{in}(\mathbf{r}) + \mathbf{E}_{sc}(\mathbf{r}) = 0.
```

By applying the Point Matching technique, we evaluate this equation at $M$ discrete observation points $\mathbf{r}_i$, located at the nodes where the basis functions are centered. This transforms the continuous integral equation into a system of $M$ linear equations:
```math
\mathbf{E}_{in}(\mathbf{r}_i) = \sum_{j=1}^M I_j \left[ j\omega\mathbf{A}_j(\mathbf{r}_i) + \nabla \Phi_j(\mathbf{r}_i) \right].
```

We can now define the Impedance Matrix element $Z_{ij}$, which represents the electromagnetic interaction between a source at node $j$ and an observation point at node $i$:
```math
Z_{ij} = j\omega\mathbf{A}_j(\mathbf{r}_i) + \nabla \Phi_j(\mathbf{r}_i).
```
Finally, the entire problem is reduced to a compact matrix form:
```math
\mathbf{V} = \mathbf{Z}\mathbf{I}
```
where $\mathbf{V}$ is the excitation vector containing the incident field at each node, and $\mathbf{I}$ is the vector of the unknown current coefficients to be determined. 

## Implementation

While the previous section established the theoretical framework, the numerical stability and efficiency of the solver depend on how these operators are evaluated. Specifically, we must address the integration of the singular kernel in $\mathbf{A}_j$ and the discrete approximation of the gradient $\nabla \Phi_j$.

The transition from the theoretical Method of Moments (MoM) framework to a functional numerical solver is organized into four distinct computational stages. This modular approach ensures that the electromagnetic physics remain decoupled from the linear algebra and post-processing logic.

* Geometrical Discretization: The antenna wireframe is converted into a structured mesh of $N$ segments. This stage handles the mapping of nodes and segments, ensuring that connectivity and boundary conditions (such as open ends or closed loops) are properly established.
* Filling the Linear System: This is the core of the solver. For each pair of nodes $(i, j)$, the impedance matrix element $Z_{ij}$ is calculated by evaluating the vector and scalar potential contributions. Numerical integration via Gaussian quadrature is employed here to handle the kernels of the integral equations.
* Solving the Linear System: Once the matrix $\mathbf{Z}$ and the excitation vector $\mathbf{V}$ are populated, the complex system $\mathbf{V} = \mathbf{Z}\mathbf{I}$ is solved using high-performance linear algebra techniques (such as LU decomposition) to determine the unknown current coefficients $I_j$.
* Post-processing and Parameter Evaluation: With the current distribution known, the final stage calculates observable antenna parameters, including input impedance, SWR, and the far-field radiation patterns.

The following sections will detail each of the above steps. Later in the next chapter, we will implement 
other useful tools in the simulation, like ground models and multiple wire junctions.

### Geometrical Discretization


The discretization process transforms a high-level wireframe description into a numerical model suitable for the Method of Moments. This stage is responsible for defining the geometry, establishing connectivity, and identifying the unknown current variables.

#### Input Specification (The Wire List)
The antenna is defined as a collection of straight wire entities. Each wire is characterized by its start and end points ($\mathbf{P}_1, \mathbf{P}_2$), its physical radius ($a$), and the number of segments ($N_{seg}$) into which it will be divided. For a single wire, this creates a sequence of $N_{seg}+1$ nodes.

#### Node and Segment Mapping
A global list of nodes and segments is compiled from the input wires. To ensure electrical continuity, nodes from different wires that occupy the same spatial coordinates (within a small tolerance $\epsilon$) are merged into a single logical node. Each segment is then defined by its connectivity to two nodes, its midpoint $\mathbf{r}_{mid}$, and its tangential unit vector $\hat{\mathbf{s}}$.

#### Identifying Current Unknowns (Incidence)
In MiniNEC, the current is defined at the nodes. The implementation must distinguish between different types of nodes based on their degree of incidence (the number of segments connected to them):

* Degree 1 (Open Ends): The node is at the extremity of a wire with no further connections. Following the boundary condition for thin wires, the current is enforced to zero ($I = 0$). These nodes do not contribute as unknowns in the linear system.
* Degree 2 (Simple Junctions): The node connects two segments. This is the standard case for nodes along the body of a wire or at the corners of a loop. Each Degree 2 node is assigned a unique index $j$ corresponding to an unknown current $I_j$.

By iterating through the global node list and filtering for Degree 2 incidence, the solver determines the total number of unknowns $M$ and establishes the mapping for the impedance matrix $\mathbf{Z}$.

#### Pulse Basis Construction
For every node $j$ identified as an unknown, a unit pulse basis function $f_j(\mathbf{r}')$ is constructed. The support of this pulse spans the two half-segments adjacent to the node. 

Each pulse is defined by a set of implementation parameters:
* Observation Point: The node coordinate $\mathbf{r}_j$.
* Source Geometries: The vectors and lengths of the preceding ($S_j^-$) and succeeding ($S_j^+$) segments.
* Charge Points: The midpoints $\mathbf{r}_{j-1/2}$ and $\mathbf{r}_{j+1/2}$, used for the scalar potential calculation.



### Filling the Linear System

The goal of this stage is to populate the $M \times M$ impedance matrix $\mathbf{Z}$ and the $M \times 1$ excitation vector $\mathbf{V}$. Each entry $Z_{ij}$ represents the complex interaction between a source node $j$ and an observation node $i$.

#### The Matrix Element Structure

For every pair of nodes $(i, j)$, the element $Z_{ij}$ is calculated as the sum of the inductive and capacitive contributions:
```math
Z_{ij} = Z_{ij}^{A} + Z_{ij}^{\Phi}
```
where: 
```math
Z_{ij}^{A} = j\omega \mathbf{A}_j(\mathbf{r}_i) \cdot \hat{\mathbf{s}}_i
```
and
```math
Z_{ij}^{\Phi} = \nabla \Phi_j(\mathbf{r}_i) \cdot \hat{\mathbf{s}}_i.
```
The unit vector $\hat{\mathbf{s}}_i$ represents the tangential direction of the wire at the observation node $i$. The dot products ensure that only the component of each field aligned with the conductor contributes to the induced electromotive force, consistent with the thin-wire boundary condition. 

#### Numerical Integration of $Z_{ij}^{A}$

Since $f_j(\mathbf{r}')$ is a unit pulse, the vector potential term involves integrating the Green’s function over the two half-segments ($S_{j}^-$ and $S_{j}^+$) adjacent to node $j$, where $f_j(\mathbf{r}')$ is non-zero:
```math
\mathbf{A}_j(\mathbf{r}_i) = \frac{\mu_0}{4\pi} \left( \hat{\mathbf{s}}_j^- \int_{S_j^-} G(\mathbf{r}_i, \mathbf{r}') dr' + \hat{\mathbf{s}}_j^+ \int_{S_j^+} G(\mathbf{r}_i, \mathbf{r}') dr' \right),
```
where $\hat{\mathbf{s}}_j^-$ and $\hat{\mathbf{s}}_j^+$ are the unit vectors of the segments meeting at node $j$. For general terms where the observation point $\mathbf{r}_i$ is far from the source point $\mathbf{r}'$, the free-space Green's function is used:
```math
G(\mathbf{r}, \mathbf{r}') = \frac{e^{-jk|\mathbf{r} - \mathbf{r}'|}}{|\mathbf{r} - \mathbf{r}'|}
```

However, for the self-term ($i=j$), the distance $|\mathbf{r} - \mathbf{r}'|$ approaches zero, making the kernel singular. To resolve this, we employ the thin-wire approximation, where the current is assumed to flow on the axis of the wire while the boundary condition is enforced on its surface. This leads to the reduced kernel:
```math
G(\mathbf{r}, \mathbf{r}') = 
\frac{e^{-jk\sqrt{|\mathbf{r} - \mathbf{r}'|^2+a^2}}}{\sqrt{|\mathbf{r} - \mathbf{r}'|^2+a^2}}.
```
where $a$ is the wire radius. This modification removes the singularity, allowing the 5-point Gaussian quadrature to converge, provided the segment subdivision is sufficiently fine near the node. In a practical implementation the reduced kernel can be used everywhere, as the computation overhead is small, sometimes being less than a branch prediction hit.

#### Finite Difference Approximation of $Z_{ij}^{\Phi}$
Following the derivation in the previous chapter, the scalar potential $\Phi_j$ generated by the $j$-th node is equivalent to the potential produced by two equal and opposite point charges located at the centers of the adjacent segments, $S_j^-$ and $S_j^+$.

In the Point Matching scheme, we evaluate the gradient of this potential at the observation node $i$. To avoid the direct numerical differentiation of the Green's function, MiniNEC employs a finite difference approximation. The tangential component of the gradient at node $i$ is calculated by evaluating the difference in potential between the centers of its own adjacent segments:
```math
Z_{ij}^{\Phi} = 
\frac{j}{4\pi \varepsilon_0 \omega} 
\left[ 
    G(\mathbf{r}_{i+1/2}, \mathbf{r}_{j+1/2}) 
    - G(\mathbf{r}_{i+1/2}, \mathbf{r}_{j-1/2}) 
    - G(\mathbf{r}_{i-1/2}, \mathbf{r}_{j+1/2}) 
    + G(\mathbf{r}_{i-1/2}, \mathbf{r}_{j-1/2}) 
\right],
```
where: 
* $\mathbf{r}_{i \pm 1/2}$ are the midpoints of the segments connected to the observation node $i$, and
* $\mathbf{r}_{j \pm 1/2}$ are the midpoints of the segments connected to the source node $j$,
* $G(\mathbf{r}, \mathbf{r}')$ is the same reduced kernel used for the vector potential.

This formulation is remarkably efficient as it entirely bypasses numerical integration for the capacitive term. By evaluating the potential at the segment midpoints, the observation points are naturally displaced from the source "charges" by at least half a segment length, further enhancing the numerical stability of the $Z_{ii}$ terms.


### Solving the Linear System
### Post-processing and Parameter Evaluation