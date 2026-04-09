## Maxwell Equations, Vector Potential and Wave Equations

### Introduction

In order to understand the algorithm behind the Method of Moments (MoM) for antenna simulation,
and thus how MiniNEC works, it is essential to understand how we can express induction and radiation mathematically.
Applying some vector calculus identities on the Maxwell equations, one can derive the wave equations for 
the electric and magnetic fields, which describe how these fields propagate in space and time. Unfortunately, 
the inhomogeneous wave equations for the electric and magnetic fields obtained directly from the electric and magnetic field curls
contain a source term that depends on the charge distribution gradient. Such gradient i not readily available in the 
antenna simulation scenario, and to overcome this issue, the vector potential and scalar potential are introduced. 
By expressing the electric and magnetic fields in terms of such potentials, under a Lorenz gauge condition
the wave equations become decoupled and homogeneous, allowing us to solve for the potentials independently.


### Vector Calculus Basic Identities

Before diving into the Maxwell's equations manipulations, it's important to remember some basic understanding 
of vector calculus, which is the mathematical framework used to express these equations.

* A scalar field is a function that assigns a single value to every point in space, such as temperature or pressure. 

* A vector field is a mathematical representation of a physical quantity that has both magnitude and direction at every point in space. In the context of electromagnetism, the electric field ($\mathbf{E}$) and magnetic field ($\mathbf{B}$) are examples of vector fields.

For a scalar field $\phi$, its **gradient** ($\nabla \mathbf{\phi}$) is a vector field points in the direction of the greatest rate of increase of the field and has a magnitude equal to that rate of increase.
Its **laplacian** ($\nabla^2 \phi$) is a scalar field that gives the local curvature of the scalar field.

For some vector field $\mathbf{V}$, its **divergence** ($\nabla \cdot \mathbf{V}$) measures the net flow of the field out of an infinitesimal volume. 
A positive divergence indicates a source, while a negative divergence indicates a sink.
The **curl** of a vector field ($\nabla \times \mathbf{V}$) measures the rotation or swirling of the field around a point. A non-zero curl indicates that the field has a rotational component.

Those operators keep some important properties which are crucial in the context of electromagnetism:
* $\nabla \cdot (\nabla \phi) = \nabla^2 \phi$: The divergence of the gradient of a scalar field is equal to the Laplacian of that field.
* $\nabla \times (\nabla \phi) = 0$: The curl of the gradient of any scalar field is always zero.
* $\nabla \cdot (\nabla \times \mathbf{V}) = 0$: The divergence of the curl of any vector field is always zero.
* $\nabla \times (\nabla \times \mathbf{V}) = \nabla (\nabla \cdot \mathbf{V}) - \nabla^2 \mathbf{V}$: This identity relates the curl of the curl of a vector field to the divergence and the Laplacian of the field.

With these identities in mind, we can now proceed to manipulate Maxwell's equations and derive the wave equations for the electric and magnetic fields, as well as the vector and scalar potentials.

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

By manipulating the Maxwell's equations using vector calculus, we can derive the wave equations for the electric and magnetic fields.
First it is useful to define the magnetic field strenght $$\mathbf{H}=\frac{1}{\mu_0}\mathbf{B},$$ then, by applying the curl operator to Faraday's Law of Induction and using the vector calculus identities, we can derive the Helmholtz equations for the electric and magnetic fields:
```math
\nabla \times (\nabla \times \mathbf{E}) = \nabla \times( -\mu_0\frac{\partial \mathbf{H}}{\partial t})
```

The time derivative and curl operator commute, so we can rewrite the equation as:
```math
\nabla \times (\nabla \times \mathbf{E}) = -\mu_0 \frac{\partial}{\partial t} (\nabla \times \mathbf{H})
```

Now we substitute Ampère's Law with Maxwell's Addition into the equation:
```math
\nabla \times (\nabla \times \mathbf{E}) = -\mu_0 \frac{\partial}{\partial t}\mathbf{J} - \mu_0 \varepsilon_0 \frac{\partial^2 \mathbf{E}}{\partial^2 t}
```

Using the vector calculus identity $\nabla \times (\nabla \times \mathbf{E}) = \nabla (\nabla \cdot \mathbf{E}) - \nabla^2 \mathbf{E}$ we get:
```math
\nabla (\nabla \cdot \mathbf{E}) - \nabla^2 \mathbf{E} = -\mu_0 \frac{\partial}{\partial t}\mathbf{J} - \mu_0 \varepsilon_0 \frac{\partial^2 \mathbf{E}}{\partial^2 t}
```

We then rearrange the equation and apply Gauss's Law for Electricity to express the divergence of the electric field in terms of the charge density:
```math
\nabla^2 \mathbf{E} - \mu_0 \varepsilon_0 \frac{\partial^2 \mathbf{E}}{\partial^2 t} = \nabla (\frac{\rho}{\varepsilon_0}) + \mu_0 \frac{\partial}{\partial t}\mathbf{J}
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
\nabla \times (\mathbf{E} + \frac{\partial \mathbf{A}}{\partial t}) = 0.
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
\nabla \times (\nabla \times \mathbf{A}) = \mu_0 \mathbf{J} + \mu_0 \varepsilon_0 \frac{\partial}{\partial t}(-\nabla \phi - \frac{\partial \mathbf{A}}{\partial t})
```

Applying the curl of the curl identity and rearranging the equation, we get:
```math
\nabla (\nabla \cdot \mathbf{A}) - \nabla^2 \mathbf{A} = \mu_0 \mathbf{J} - \mu_0 \varepsilon_0 \nabla \frac{\partial \phi}{\partial t} - \mu_0 \varepsilon_0 \frac{\partial^2 \mathbf{A}}{\partial^2 t}
```

Next, by collecting the terms under the gradient operator, we can express the equation as:
```math
\nabla (\nabla \cdot \mathbf{A} + \mu_0 \varepsilon_0 \frac{\partial \phi}{\partial t}) - \nabla^2 \mathbf{A} - \mu_0 \varepsilon_0 \frac{\partial^2 \mathbf{A}}{\partial^2 t} = \mu_0 \mathbf{J}
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

The scalar potential can be derived from the Gauss's Law for Electricity by substituting the expression for the electric field in terms of the potentials:
```math
\nabla \cdot (-\nabla \phi - \frac{\partial \mathbf{A}}{\partial t}) = \frac{\rho}{\varepsilon_0}
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

The Lorenz gauge ensures relativistic invariance of the equations and allows us to treat the 
vector and scalar potentials independently, making it easier to solve electromagnetic problems. 
The resulting wave equations for the potentials can be solved using various methods, 
including the Green's function approach, which provides a systematic way to find the solutions for 
arbitrary source distributions.

## Green's Function Solution for the Wave Equations
The Green's function is a powerful mathematical tool used to solve inhomogeneous differential equations, 
such as the wave equations for the vector and scalar potentials. The Green's function represents the 
response of the system to a point impulse source, and it can be used to construct the solution for any arbitrary 
source distribution by means of convolution operations.

The Green's function for the wave equation in three-dimensional space is given by:
```math
G(\mathbf{r}, t; \mathbf{r}', t') = \frac{\delta(t - t' - \frac{|\mathbf{r} - \mathbf{r}'|}{c})}{4\pi |\mathbf{r} - \mathbf{r}'|}   
```
where $c$ is the speed of light in vacuum, and $\delta$ is the Dirac delta function. 
This Green's function represents the response of the system at position $\mathbf{r}$ and time $t$ due to 
a point source located at position $\mathbf{r}'$ that emits an impulse at time $t'$. The delta function ensures that the response 
occurs only when the time difference matches the time it takes for a signal to travel from the source to the 
observation point at the speed of light.

Using the Green's function, we can express the solutions for the vector potential as an integral over the source distribution:
```math
\mathbf{A}(\mathbf{r}, t) = \mu_0 \int \int G(\mathbf{r}, t; \mathbf{r}', t') \mathbf{J}(\mathbf{r}', t') d^3r' dt'
```
or
```math
\mathbf{A}(\mathbf{r}, t) = \mu_0 \int \int \frac{\delta(t - t' - \frac{|\mathbf{r} - \mathbf{r}'|}{c})}{4\pi |\mathbf{r} - \mathbf{r}'|} \mathbf{J}(\mathbf{r}', t') d^3r' dt'
```

The time integral can easily be evaluated by defining the retarded time 
$t' = t - \frac{|\mathbf{r} - \mathbf{r}'|}{c}$, 
which accounts for the time it takes for the signal to propagate from the source to the observation point.
Then, by the shifting property of the Dirac delta distribution, this leads to the expression:
```math
\mathbf{A}(\mathbf{r}, t) = \frac{\mu_0}{4\pi} \int \frac{\mathbf{J}(\mathbf{r}', t - \frac{|\mathbf{r} - \mathbf{r}'|}{c})}{|\mathbf{r} - \mathbf{r}'|} d^3r'.
```

In most practical applications, such as antenna simulations, the current distribution $\mathbf{J}$ is assumed 
to be time-harmonic, meaning it varies sinusoidally with time. In this case, we can express the current as:
```math
\mathbf{J}(\mathbf{r}', t') = \mathbf{J}(\mathbf{r}') e^{j\omega t'} = \mathbf{J}(\mathbf{r}') e^{j\omega (t - \frac{|\mathbf{r} - \mathbf{r}'|}{c})} = \mathbf{J}(\mathbf{r}') e^{j\omega t} e^{-j\omega \frac{|\mathbf{r} - \mathbf{r}'|}{c}}.
```
where $\omega$ is the angular frequency of the current. Substituting this into the expression for 
the vector potential, we get:
```math
\mathbf{A}(\mathbf{r}, t) = \frac{\mu_0}{4\pi} e^{j\omega t} \int \frac{\mathbf{J}(\mathbf{r}') e^{-j\omega \frac{|\mathbf{r} - \mathbf{r}'|}{c}}}{|\mathbf{r} - \mathbf{r}'|} d^3r'.
```

Introducing the wavenumber $k = \frac{\omega}{c}$, and assuming $t=0$, we can rewrite the expression in its final form:
```math
\mathbf{A}(\mathbf{r}) = \frac{\mu_0}{4\pi} \int \frac{\mathbf{J}(\mathbf{r}') e^{-jk|\mathbf{r} - \mathbf{r}'|}}{|\mathbf{r} - \mathbf{r}'|} d^3r'.
```

Due to the symmetry established by the Lorenz Gauge, the scalar potential $\Phi$ obeys a wave equation 
identical in form to that of the vector potential $\mathbf{A}$. Consequently, its solution follows the 
same Green's function approach, substituting the current density $\mathbf{J}$ for the charge density 
$\rho$ and the permeability $\mu_0$ for the inverse of the permittivity $1/\epsilon_0$.
Thus the scalar potential can be expressed as:
```math
\Phi(\mathbf{r}) = \frac{1}{4\pi \varepsilon_0} \int \frac{\rho(\mathbf{r}') e^{-jk|\mathbf{r} - \mathbf{r}'|}}{|\mathbf{r} - \mathbf{r}'|} d^3r'.
```

### Solving the Fields From the Potentials

Having the scalar and vector potentials solutions at hand, the electric field strength is obtained from the fundamental relation between the potentials and the electric field:
```math
\mathbf{E}(\mathbf{r},t)=-\frac{\partial \mathbf{A}(\mathbf{r},t)}{\partial t}-\nabla\Phi(\mathbf{r},t),
```
which in the frequency domain becomes:
```math
\mathbf{E}(\mathbf{r})=-j\omega\mathbf{A}(\mathbf{r})-\nabla\Phi(\mathbf{r}).
```

Although not used in the method of moments for antenna simulation, the magnetic flux intensity is readily available from the defining property of the vector potential:
```math
\mathbf{B}(\mathbf{r})=\nabla\times\mathbf{A}(\mathbf{r}).
```
Expanding the expression using the previously calculated $\mathbf{A}(\mathbf{r})$, we have:
```math
\mathbf{B}(\mathbf{r}) = \frac{\mu_0}{4\pi} \int_{V} \nabla \times \left( \mathbf{J}(\mathbf{r}') \frac{e^{-jk|\mathbf{r}-\mathbf{r}'|}}{|\mathbf{r}-\mathbf{r}'|} \right) d^3r'.
```
This integral can be simplified by the relation $\nabla \times (f \mathbf{V}) = (\nabla f) \times \mathbf{V} + f\cdot(\nabla \times \mathbf{V})$, which is valid for any scalar function $f$. Defining:
```math
f(\mathbf{r}) =  \left( \frac{e^{-jk|\mathbf{r}-\mathbf{r}'|}}{|\mathbf{r}-\mathbf{r}'|} \right),
```
we can rewrite:
```math
\nabla \times \left( \mathbf{J}(\mathbf{r}') f(\mathbf{r})\right)=
\nabla \left(f(\mathbf{r})\right) \times \mathbf{J}(\mathbf{r}')+f(\mathbf{r})\cdot(\nabla\times\mathbf{J}(\mathbf{r}')). 
```
Since the curl operator $\nabla$ acts upon the observation coordinates $\mathbf{r}$, and the current density $\mathbf{J}(\mathbf{r}')$ is a function only of the source coordinates $\mathbf{r}'$, the term $\nabla \times \mathbf{J}(\mathbf{r}')$ is identically zero, and the second term on the right hand side vanishes:
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
\frac{\partial f(R)}{\partial R}=\frac{\partial}{\partial R}\left( \frac{e^{-jkR}}{R} \right)= -\left( jk + \frac{1}{R} \right) \frac{e^{-jkR}}{R},
```
and 
the gradient of the distance can be evaluated to:
```math
\nabla |\mathbf{r} - \mathbf{r}'| = \frac{\mathbf{r} - \mathbf{r}'}{|\mathbf{r} - \mathbf{r}'|} = \hat{\mathbf{R}}.
```

Combining the expressions, we have:
```math
\mathbf{B}(\mathbf{r}) = \frac{\mu_0}{4\pi} \int_{V} \mathbf{J}(\mathbf{r}') \times \frac{\mathbf{r} - \mathbf{r}'}{|\mathbf{r} - \mathbf{r}'|} \left( jk + \frac{1}{|\mathbf{r} - \mathbf{r}'|} \right) \frac{e^{-jk|\mathbf{r} - \mathbf{r}'|}}{|\mathbf{r} - \mathbf{r}'|} d^3r'.
```
Rearranging the terms:
```math
\mathbf{B}(\mathbf{r}) = jk\frac{\mu_0}{4\pi} \int_{V} \mathbf{J}(\mathbf{r}') \times 
\frac{\mathbf{r} - \mathbf{r}'}{|\mathbf{r} - \mathbf{r}'|}
\frac {e^{-jk|\mathbf{r} - \mathbf{r}'|}}{|\mathbf{r} - \mathbf{r}'|} d^3r' + 
\frac{\mu_0}{4\pi} \int_{V} \mathbf{J}(\mathbf{r}') \times 
\frac{\mathbf{r} - \mathbf{r}'}{|\mathbf{r} - \mathbf{r}'|}  \frac{e^{-jk|\mathbf{r} - \mathbf{r}'|}}{|\mathbf{r} - \mathbf{r}'|^2} d^3r'.
```
The second term in this expression is the Biot-Savart equation for magnetic fields, and corresponds to the near-field magnetic propagation. The first term is the far-field radiation expression. 

Note that for direct current ($\omega = 0$), the far-field terms vanish entirely. The induced component of the electric field (proportional to $\partial \mathbf{A} / \partial t$) also becomes zero, leaving the system described by classic magnetostatics—represented by the magnetic near-field—and electrostatics, where the electric field arises solely from the stationary charge distribution ($\mathbf{E} = -\nabla\Phi$).