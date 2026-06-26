## **W. R.D.Wilson**

## **S. Sheu**

Mechanical and Nuclear Engineering Department, Northwestern University, Evanston, III. 60201

## Effect of Inlet Shear Heating Due to Sliding on Elastohydrodynamic Film Thickness

Recent experimental work **[1]** has shown that the semiempirical equation developed by Murch and Wilson [2] is a useful design tool for elastohydrodynamic contacts where the use of high speeds or high viscosity lubricants results in significant inlet shear heating with a consequent reduction in film thickness. The basis of Murch and Wilson's analysis is the "thermal Reynolds' equation" developed by Wilson and Wong [3]. Since this equation was developed for cases with equal surface velocities, Murch and Wilson's analysis is only valid for the case of pure rolling. However, reliable thermal Reynolds' equations which allow for sliding have now been developed by Aggarwal and Wilson [4]. The purpose of the present brief is to apply one of these to the elastohydrodynamic inlet zone problem and to generate a semi-empirical correction equation for situations with different degrees of sliding. It is hoped that this will be of value to the designers of gears, cams and other devices which involve sliding elastohydrodynamic contacts.

A Grubin inlet zone analysis of the type developed by Murch and Wilson was conducted using Aggarwal and Wilson's thermal Reynolds' equation

$$R = (1 + 0.5F^{0.85} + 0.2S^{0.9})^{-1}$$
 (1)

where

$$R = -\frac{p'h^3}{12\mu_s \bar{U}(h - h_o)}$$
 (2)

$$F = \frac{\mu_s \alpha \bar{U}^2 (h - h_o)^2}{kh^2}$$
 (3)

$$S = \frac{\mu_s \alpha (U_1 - U_2)^2}{k} \tag{4}$$

*p'* is the pressure gradient along the film, *h* the local film thickness, *fi<sup>s</sup>* the lubricant viscosity at the surface temperature and local pressure, *U* the mean speed of the surfaces, *h(>* the film thickness in the contact zone, *a* the lubricant temperature coefficient of viscosity, *k* its conductivity and *U]* and *U2* are the inlet surface velocities. The analytical methods were generally similar to those used by Murch and Wilson, with the exception that a transformation was used to simplify the handling of the upstream boundary condition (vanishing pressure at infinity).

The results of the present analysis can be expressed in terms of the variables used by Murch and Wilson plus a slide-roll ratio Z defined by

$$Z = (U_1 - U_2)/(U_1 + U_2)$$
 (5)

For design purposes it is convenient to use a thermal correction factor *C* defined as the ratio of the calculated thermal film thickness to that predicted by isothermal theory. The correction factor *C* is, for all practical purposes, solely a function of Z and the thermal loading parameter *L* defined by

$$L = \mu_o \alpha \bar{U}^2 / k \tag{6}$$

where /z" is the lubricant viscosity at the surface temperature and atmospheric pressure.

In many bearings where the lubricant viscosity and the surface velocities are low, the thermal loading parameter *L* is sufficiently small (L<0.1) that thermal effects are negligible in the inlet and conventional isothermal theory can be used. However with high surface speeds and/or high viscosity lubricants (JL>0.1 ) inlet heating can lead to substantial reductions in film thickness. Thus it is always advisable to calculate the value of *L* in a particular application to determine whether the use of the correction factor C is necessary.

Figure 1 shows the variation of the thermal correction factor *C* with thermal loading parameter *L* for different values of the slide roll ratio Z. At small values of *L* all the curves approach a value of *C* of unity corresponding to the isothermal solution. At larger values of *L,* there is a marked decrease in *C* with increasing *L* or Z. At high values of *L* the value of *C* for the simple sliding condition (Z=l) is only about 20 percent of that for the pure rolling condition (Z = 0).

![](_page_0_Figure_22.jpeg)

**Fig. 1 Effect of inlet shear heating on film thickness** 

Contributed by the Lubrication Division for publication in the JOURNAL OF LUBRICATION TECHNOLOGY. Manuscript received by the Lubrication Division, August 4, 1980.

Thus, inlet heating due to sliding can have a very dramatic influence on the film thickness.

The results of the present analysis can be represented by the semi-empirical equation

$$C = (1 + 0.241((1 + 14.8Z^{0.83})L^{0.64}))^{-1}$$
 (7)

with sufficient accuracy for design use. For the case of pure rolling equation (7) reduces to

$$C = (1 + 0.241L^{0.64})^{-1}$$
 (8)

which is close to the semi-empirical equation given by Wilson and Murch which can be written as

$$C = (1 + 0.254L^{0.62})^{-1} \tag{9}$$

The work described in this note was performed at the University of Massachusetts. The authors wish to thank the University of Massachusetts Computer Center for the use of its facilities and Paul Dargie, William Dvorak, Clement

Chang, Robert Proctor, Pei Tsao, Kendall Miller and Arun Wadhawan for their contributions to the work under the disguise of a homework assignment.

## **References**

- 1 Wilson, A. R., "An Experimental Thermal Correction For Oil Film Thickness in E.H.L.," Proc. Sixth Leeds-Lyon Symposium, Mechanical Engineering Publications Ltd., London, 1980.
- 2 Murch, L. E., and Wilson, W. R. D., "A Thermal Elastohydrodynamic Inlet Zone Analyses," ASME JOURNAL OF LUBRICATION TECHNOLOGY, Vol. 97 1975, pp. 212-216.
- 3 Wilson, W. R. D., and Wong, C. J., "Analysis of the Film Formation Process in Plane-Strain Forging," ASME JOURNAL OF LUBRICATION TECHNOLOGY, Vol. 96, 1974, pp. 605-610.
- 4 Aggarwal, B. B., and Wilson, W. R. D., "Improved Thermal Reynolds' Equations," *Proc. Sixth Leeds-Lyon Symposium,* Mechanical Engineering Publications Ltd., London, 1980.