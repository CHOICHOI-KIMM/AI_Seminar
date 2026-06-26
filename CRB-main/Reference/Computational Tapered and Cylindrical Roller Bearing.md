Hung Nguyen-Schäfer

# Computational Tapered and Cylinder Roller Bearings

![](_page_0_Picture_2.jpeg)

### Computational Tapered and Cylinder Roller Bearings

# Computational Tapered and Cylinder Roller Bearings

![](_page_2_Picture_2.jpeg)

Hung Nguyen-Schäfer Asperg, Germany

ISBN 978-3-030-05443-4 ISBN 978-3-030-05444-1 (eBook) https://doi.org/10.1007/978-3-030-05444-1

Library of Congress Control Number: 2018964242

### © Springer Nature Switzerland AG 2019

This work is subject to copyright. All rights are reserved by the Publisher, whether the whole or part of the material is concerned, specifically the rights of translation, reprinting, reuse of illustrations, recitation, broadcasting, reproduction on microfilms or in any other physical way, and transmission or information storage and retrieval, electronic adaptation, computer software, or by similar or dissimilar methodology now known or hereafter developed.

The use of general descriptive names, registered names, trademarks, service marks, etc. in this publication does not imply, even in the absence of a specific statement, that such names are exempt from the relevant protective laws and regulations and therefore free for general use.

The publisher, the authors, and the editors are safe to assume that the advice and information in this book are believed to be true and accurate at the date of publication. Neither the publisher nor the authors or the editors give a warranty, express or implied, with respect to the material contained herein or for any errors or omissions that may have been made. The publisher remains neutral with regard to jurisdictional claims in published maps and institutional affiliations.

This Springer imprint is published by the registered company Springer Nature Switzerland AG The registered company address is: Gewerbestrasse 11, 6330 Cham, Switzerland

### Preface

This monograph briefly deals with the computation of tapered and cylinder roller bearings using in automotive applications and other industries in which radial and axial loads and bending moments acting on the bearings are relatively large compared to ball bearings. At first, loads acting on gears are calculated from the driving torque on the rotor. The resulting loads on bearings computed from the gear loads are necessary for further calculations of the bearing characteristics.

Tapered roller bearings under large radial and axial loads and bending moment are modelled to compute the Hertzian pressures at the contact zones on the inner race (IR) and outer race (OR). Furthermore, the lifetime of the bearings is computed for load spectra of various driving cycles. Moreover, the oil-film thicknesses at the IR and OR are computed in the EHD contact zones. The limiting voltage at the Hertzian contact zone is calculated to avoid the possible electro-pitting. Additionally, frictions in the bearings are calculated depending on acting loads and bending moment on the bearings, oil temperature and rotor speed as well. Using degrees of freedom (DOF) of the inner and outer races and the rolling elements, the balances of loads and moments are written in a large strongly nonlinear coupled equation system that is numerically solved by the Levenberg and Marquardt's algorithm based on least squares method.

Compared to the tapered roller bearings are cylinder roller bearings much simpler due to the bearing geometry and distribution of loads on the rolling elements. The similar tasks are done for cylinder roller bearings in this book. Additionally, the same topics for ball bearings had been coped with in my other book Computational Design of Rolling Bearings at Springer International Publishing, Switzerland (2016).

Furthermore, using the best-known machine-learning method for clustering, the real load spectrum is clustered in k cluster means based on the invariant damage number to generate an accelerated load spectrum. In order to hasten the testing time and to reduce costs, the resulting accelerated load spectrum is applied to the testing of the bearings.

vi Preface

I am very grateful to Dr. Jan-Philip Schmidt and Mrs. Petra Jantzen at Springer Heidelberg for invaluable suggestions and fruitful cooperation to successfully publish this monograph.

Eventually, my special thanks go to my wife for her understanding patience and endless support for making it big.

Asperg, Germany Hung Nguyen-Schäfer

### Contents

| 1<br>Tapered<br>Roller<br>Bearings |          |                                                                               | 1  |  |  |  |
|------------------------------------|----------|-------------------------------------------------------------------------------|----|--|--|--|
|                                    | 1.1      | Components<br>of<br>Tapered<br>Roller<br>Bearings<br>                         | 1  |  |  |  |
|                                    | 1.2      | Geometry<br>of<br>Tapered<br>Roller<br>Bearings<br>2<br>                      |    |  |  |  |
|                                    | 1.3      | Setup<br>of<br>Bearings<br>in<br>X<br>and<br>O<br>Arrangement<br>             | 3  |  |  |  |
|                                    | 1.4      | Computational<br>Model<br>of<br>Tapered<br>Roller<br>Bearings<br>             | 4  |  |  |  |
|                                    | 1.5      | Computing<br>Minimum<br>Load<br>and<br>Preload<br>on<br>TRB<br>14<br>         |    |  |  |  |
|                                    | 1.6      | Computing<br>Centrifugal<br>Force<br>of<br>Rolling<br>Elements<br>16<br>      |    |  |  |  |
|                                    | 1.7      | Computing<br>Hertzian<br>Pressures<br>at<br>the<br>Contact<br>Zones<br>17<br> |    |  |  |  |
| 1.8                                |          | Computing<br>Oil<br>Film<br>Thickness<br>in<br>TRB<br>                        | 18 |  |  |  |
|                                    |          | 1.8.1<br>Oil-Film<br>Thicknesses<br>in<br>the<br>Contact<br>Area<br>          | 18 |  |  |  |
|                                    |          | 1.8.2<br>Computing<br>the<br>Oil-Film<br>Thicknesses                          |    |  |  |  |
|                                    |          | in<br>Roller<br>Bearings<br>                                                  | 22 |  |  |  |
|                                    | 1.9      | Computing<br>Bearing<br>Friction<br>in<br>TRB<br>                             | 23 |  |  |  |
|                                    | 1.10     | Computing<br>Lifetime<br>of<br>TRB<br>                                        | 25 |  |  |  |
|                                    | 1.11     | Computing<br>Bearing<br>Stiffness<br>of<br>TRB<br>                            | 28 |  |  |  |
|                                    | 1.12     | An<br>Example<br>for<br>Computational<br>TRB<br>                              | 29 |  |  |  |
| References                         |          |                                                                               | 38 |  |  |  |
| 2                                  | Cylinder | Roller<br>Bearings<br>                                                        | 41 |  |  |  |
|                                    | 2.1      | Geometry<br>of<br>Cylinder<br>Roller<br>Bearings<br>                          | 41 |  |  |  |
|                                    | 2.2      | Setup<br>of<br>Cylinder<br>Roller<br>Bearings<br>                             | 42 |  |  |  |
|                                    | 2.3      | Computational<br>Model<br>of<br>Cylinder<br>Roller<br>Bearings<br>            | 43 |  |  |  |
|                                    | 2.4      | Computing<br>Hertzian<br>Pressures<br>at<br>the<br>Contact<br>Zones<br>       | 53 |  |  |  |
|                                    | 2.5      | Computing<br>Oil<br>Film<br>Thickness<br>in<br>CRB<br>                        | 54 |  |  |  |
| 2.6                                |          | Computing<br>Bearing<br>Friction<br>in<br>CRB<br>                             | 55 |  |  |  |
|                                    | 2.7      | Computing<br>Lifetime<br>of<br>CRB<br>                                        | 57 |  |  |  |
|                                    | 2.8      | Computing<br>Bearing<br>Stiffness<br>of<br>CRB                                | 59 |  |  |  |
|                                    | 2.9      | An<br>Example<br>for<br>Computational<br>CRB<br>                              | 61 |  |  |  |
|                                    |          | References<br>                                                                | 70 |  |  |  |
|                                    |          |                                                                               |    |  |  |  |

viii Contents

| 3 | Loads<br>Acting<br>on<br>Gears<br>and<br>Bearings<br>                            | 73       |  |  |
|---|----------------------------------------------------------------------------------|----------|--|--|
|   | 3.1<br>Calculating<br>Loads<br>Acting<br>on<br>Gears<br>                         | 73       |  |  |
|   | 3.2<br>Calculating<br>Loads<br>Acting<br>on<br>Bearings<br>                      | 76       |  |  |
|   | References<br>                                                                   | 77       |  |  |
| 4 | Bearing<br>Endplay<br>Over<br>Operating<br>Temperatures<br>                      |          |  |  |
|   | 4.1<br>Calculating<br>the<br>Axial<br>Endplay<br>                                | 79<br>79 |  |  |
|   | 4.2<br>Computational<br>Examples                                                 | 82       |  |  |
|   | References<br>                                                                   | 86       |  |  |
| 5 | Accelerated<br>Load<br>Spectrum<br>                                              |          |  |  |
|   | 5.1<br>Calculating<br>the<br>Damage<br>Number<br>                                | 87<br>87 |  |  |
|   | 5.2<br>Calculating<br>the<br>Accelerated<br>Load<br>Spectrum<br>                 | 89       |  |  |
|   | 5.3<br>An<br>Example<br>for<br>an<br>Accelerated<br>Load<br>Spectrum<br>         | 90       |  |  |
|   | References<br>                                                                   | 94       |  |  |
| 6 | Solving<br>Nonlinear<br>Equation<br>Systems                                      |          |  |  |
|   | 6.1<br>Fundamental<br>of<br>Nonlinear<br>Equation<br>Systems<br>                 | 95<br>95 |  |  |
|   | 6.2<br>NL<br>Equation<br>Systems<br>with<br>Gauss-Newton<br>Algorithm<br>        | 97       |  |  |
|   | 6.3<br>NL<br>Equation<br>Systems<br>with<br>Levenberg-Marquardt<br>Algorithm<br> | 99       |  |  |
|   | 6.4<br>Solving<br>NL<br>Equation<br>Systems<br>with<br>MATLAB<br>                | 101      |  |  |
|   | References<br>                                                                   | 102      |  |  |
|   | Appendix<br>A:<br>Calculating<br>the<br>System<br>Lifetime<br>                   | 103      |  |  |
|   | Appendix<br>B:<br>Linear<br>Regression<br>Analysis<br>                           | 107      |  |  |
|   | Appendix<br>C:<br>Cluster-Weighting<br>Modelling<br>(CWM)<br>                    | 111      |  |  |
|   | Index<br>                                                                        | 113      |  |  |

### About the Author

Dr. Hung Nguyen-Schäfer (KIT, Ph.D. Karlsruhe in 1989) has more than 30 years of experience in the automotive industry at Robert Bosch GmbH, Bosch Mahle Turbo Systems and EM-motive. His various working areas are gasoline and diesel direct injection systems, fuel supply systems, anti-breaking systems, automotive turbochargers, fuel-cell vehicles, hybrid/electric vehicles and transmission system for e-Mobility.

He is author/co-author of many technical papers and professional books:

- Aero and Vibroacoustics of Automotive Turbochargers. Springer Berlin-Heidelberg (2013)
- Rotordynamics of Automotive Turbochargers, Second Ed. Springer Berlin-Heidelberg (2015)
- Computational Design of Rolling Bearings. Springer International Publishing Switzerland (2016)
- Tensor Analysis and Elementary Differential Geometry for Physicists and Engineers. Second Ed. Springer Berlin-Heidelberg (2017).

# <span id="page-9-0"></span>**Chapter 1 Tapered Roller Bearings**

![](_page_9_Picture_1.jpeg)

1

Tapered roller bearings (TRB) are normally used under large radial, axial loads, and bending moments at moderate shaft speeds and heavy-duty operations. Many applications of these bearings are found in the automotive industry (e.g. for front and rear wheels, differentials of trucks and buses), marine and aerospace industries (e.g. boats, ships, airplanes, and space shuttles), construction and mining industries (e.g. bulk conveyors, compact track loaders, concrete mixers, continuous miners, and tunnel drills), agricultural industries (e.g. mowers, tractors, and grain carts), machine tool spindles, and wind turbines.

### 1.1 Components of Tapered Roller Bearings

Figure 1.1 shows the main components of a TRB that are the cup, cone, tapered rolling elements, bearing cage, and lubricant (oil or grease). The rolling elements are kept in the bearing cage that locates between the cone (IR called inner race) and cup (OR called outer race). The cone (IR) is mounted on the rotor shaft and the cup (OR) on the bearing housing. A pair of TRB could be setup in an O or X arrangement depending on the application.

Generally, due to preload of the bearings in the X arrangement the cone is tightly fixed on the shaft with a fitting interference (e.g. k6) and the cup is quite loose on the bearing housing with a fitting clearance (e.g. H7); and vice versa in the O arrangement (e.g. a fitting clearance g6 for the shaft and a fitting interference P7 for the bearing housing), cf. Sect. 1.3. Obviously, where the preload is adjusted, there must be moveable, and the other part must be fixed.

<span id="page-10-0"></span>![](_page_10_Picture_2.jpeg)

Fig. 1.1 Components of a tapered roller bearing (TRB)

### 1.2 Geometry of Tapered Roller Bearings

Figure 1.2 shows the main geometries of a tapered roller bearing that are the cone bore diameter d, the cup outside diameter D, the bearing width T, the pitch diameter  $D_{pw}$ , the length  $L_{Re}$  of the rolling element, and the mean diameter of the rolling element  $D_m$  at its center  $O_{Re}$ .

![](_page_10_Picture_6.jpeg)

Fig. 1.2 Key geometrical parameters of a TRB

<span id="page-11-0"></span>The angle between the cone (IR) and the rotating axis x is defined as the half cone-angle  $\alpha_i$ . Similarly, the half cup-angle  $\alpha_o$  is for the cup (OR), the half roller center line angle  $\alpha_m$ , and the half roller-angle  $\alpha_{12}$  for the rolling element (RE). Finally, the kerb (or curb) angle  $\alpha_f$  is defined as the angle between the bearing kerb and the rotating axis x, where  $\alpha_f = \pi/2 - \alpha_m$ . A tapered roller bearing has Z rolling elements (called rollers) per row. All bearing geometries are necessary for the computation of tapered roller bearings that will be discussed in the next sections.

### 1.3 Setup of Bearings in X and O Arrangement

In general, there are two kinds of adjusting setup of TRB in the X and O arrangement, as shown in Figs. 1.3 and 1.4. At first, the X arrangement is displayed in Fig. 1.3. A couple of bearings are tightly fixed mounted on the shaft with a fitting interference and their outer races (cups) are quite loose in the covers of the bearing housing with a fitting clearance. A set collar at the left cup in the bearing housing is used to adjust the axial endplay. This process is called preloading the bearings. The thickness of the set collar defines the preload on the bearings that depends on the operating bearing temperature, as discussed in Chap. 4. Note that both TRB unconditionally need a certain preload on them. Without a resulting axial load between the axial preload and external axial load they do not operate in the optimum condition.

In this case, both bearings are set up in the X arrangement as the form of the load directions (dot lines) that are perpendicular to the centerlines of the rolling elements. Besides radial loads, the external axial load  $F_a$  could be alternatively acted on the shaft in both directions. The axial load is transmitted in the shaft through the opposite cone and rollers to the cup located in the bearing housing. The X arrangement is usually applied to gearboxes due to its simple assembly of the cups in the covers of the bearing housing. Note that the maximum misaligned angle for the X arrangement (called face-to-face arrangement) is about 3.5 min (0.058°).

![](_page_11_Picture_6.jpeg)

Fig. 1.3 Setup of bearings in the X arrangement

<span id="page-12-0"></span>![](_page_12_Picture_2.jpeg)

Fig. 1.4 Setup of bearings in the O arrangement

The O arrangement is shown in Fig. 1.4, in which a couple of bearings are mounted on the shaft with a fitting clearance and the outer races (cups) are tightly fixed in the bearing housing with a fitting interference. Both bearings are set up in the O arrangement as the form of the load directions (dot lines) that are perpendicular to the centerlines of the rolling elements. Note that the maximum misaligned angle for the O arrangement (called back-to-back arrangement) is about 1.5 min (0.025°).

Like the X arrangement, both bearings need a certain preload in axial direction to operate in the optimum condition. The axial preload is determined by adjusting the set collar on the left cone on the shaft. Besides radial loads, the external axial load  $F_a$  could be alternatively acted on the shaft in both directions. The axial load is transmitted in the shaft through the back cone and rollers to the cup located in the bearing housing.

### 1.4 Computational Model of Tapered Roller Bearings

Tapered roller bearings have Z rollers (rolling elements RE); each of them is divided into  $n_s$  circular slices along its length  $L_{Re}$  with an equal thickness  $\Delta x_k$ . The radial load  $F_r$ , axial load  $F_a$ , and bending moment  $M_b$  acting on the bearing (s. Fig. 1.5) are calculated from the torque on the rotor, cf. Chap. 3.

The inner race (IR) has three degrees of freedom (DOF) in which one DOF is caused by the bending moment. Additionally, each rolling element (RE) has also three DOF. In total, there are (3Z+3) DOF for the computation of the tapered roller bearing, s. Fig. 1.6.

At first, the radial load  $F_r$  acting on the bearing causes different loads on each rolling element in radial and axial direction. Without a certain load or preload in axial direction on the IR, the rolling elements and the IR would be thrown out to the right-hand side (kerb/curb side). Thus, a tapered roller bearing unconditionally needs a certain load in axial direction to operate in the optimum condition.

The radial load  $F_r$  distributes different normal loads  $Q_{ji}$  and  $Q_{jo}$  at the IR- and OR-contact surface of each roller #j (j = 1 to Z). These normal loads result from the

<span id="page-13-0"></span>![](_page_13_Picture_2.jpeg)

Fig. 1.5 Loads and bending moment on a tapered roller bearing

![](_page_13_Picture_4.jpeg)

Fig. 1.6 DOF of a tapered roller bearing

elastic deformations  $\delta_{ki}$  and  $\delta_{ko}$  of the slice k at the contact zones. Additionally, the reaction force  $Q_f$  on the bearing kerb is computed from the deformation  $\delta_{fj}$  at the contact area between the IR and the bearing kerb.

In case of a stationary outer race (OR) in the bearing housing, the IR has two unknowns DOF of  $\delta_{r,IR}$  and  $\delta_{a,IR}$  in radial and axial direction, respectively which are fixed to the inertial coordinate system (x, y, z), as shown in Fig. 1.6.

Similarly, each rolling element j has also three unknowns DOF of  $\delta_{yj}$  and  $\delta_{xj}$  in radial and axial direction, and the tilting angle  $\psi_j$  in the direction z, which are also fixed to the RE #j. In addition, the bending angle  $\theta_b$  between the IR and OR is caused from the bending moment between them.

To consider the local Hertzian pressures at various positions along the length of rolling elements at the tilting position. These DOF cause the elastic deformations  $\delta_{ki}$ ,  $\delta_{ko}$ , and  $\delta_{fj}$  at the relating contact areas between the rolling element *RE #j* and the *IR*, *OR*, and bearing kerb, respectively.

The elastic deformation  $\delta_{ko}$  of the rolling element #j at the contact zone of the stationary OR is calculated from the DOF of the rolling element in the directions y and x as

$$\delta_{ko}\cos\alpha_o + \delta y_j = 0$$
$$\delta_{ko}\sin\alpha_o + \delta x_i = 0$$

Solving the deformation  $\delta_{ko}$  from both equations, one obtains

$$\delta_{ko}(j) = -(\delta y_j \cos \alpha_o + \delta x_j \sin \alpha_o) \tag{1.1}$$

Similarly, the elastic deformation  $\delta_{ki}$  of the rolling element #j at the contact zone of rotating IR is calculated from the DOF of the IR and the rolling element in the directions y and x. In this case, the calculation is invariant for any position angle  $\varphi_j$  when the deformation vector  $\delta_{r,IR}$  would rotate with the angle  $\varphi_j$  in the same direction, as shown in Fig. 1.6. As a result, the new deformation  $\delta_{r,IR}$  becomes  $\delta_{r,IR}$  cos  $\varphi_j$  in the rotating radial direction.

$$-\delta_{ki}\cos\alpha_i + \delta y_j + \delta_{r,IR}\cos\varphi_j = 0$$
  
$$-\delta_{ki}\sin\alpha_i + \delta x_i + \delta_{a,IR} = 0$$

Substituting above equations the deformation  $\delta_{ki}$  is calculated as

$$\delta_{ki}(j) = (\delta_{a,IR} + \delta x_j) \sin \alpha_i + (\delta_{r,IR} \cos \varphi_j + \delta y_j) \cos \alpha_i$$
 (1.2)

The elastic deformation  $\delta_{jj}$  of the rolling element #j at the bearing kerb results from the DOF of the IR and the rolling element in the directions y and x as

$$\delta_{ff} \sin \alpha_m^* + \delta y_j + \delta_{r,IR} \cos \varphi_j = 0$$
$$-\delta_{ff} \cos \alpha_m^* + \delta x_j + \delta_{a,IR} = 0$$

Thus,

$$\delta_{fj}(j) = (\delta_{a,IR} + \delta x_j) \cos \alpha_m^* - (\delta_{r,IR} \cos \varphi_j + \delta y_j) \sin \alpha_m^*$$
 (1.3)

<span id="page-15-0"></span>in which the tilting angle  $\psi_i$  of the RE #j is used to calculate

$$\alpha_m^*(j) = \alpha_m - \psi_j(j);$$

$$\alpha_m = \frac{1}{2}(\alpha_i + \alpha_o); \quad \varphi_j = \frac{2\pi j}{Z}$$
(1.4)

In the following sections, the relating equations of DOF are generated from the balance of loads and moments acting on Z rolling elements. In case of a stationary OR, the load balances in the directions y and x are used in the computation.

Based on the line contact, the normal loads acting on the IR and OR at the slice k of the RE #j are computed, respectively:

$$Q_{ki}(k,j) = \frac{C_L}{n_s} \hat{\delta}_{ki}^{10/9} f_k(k)$$

$$Q_{ko}(k,j) = \frac{C_L}{n_s} \hat{\delta}_{ko}^{10/9} f_k(k)$$
(1.5)

where

$$C_L = 2^{10/9} \times C_L' = 7.765 \times 10^4 L_{Re}^{8/9} \text{ in N/mm}^{10/9}$$

 $\widehat{\delta}_{ki}$ 

 $\widehat{\delta}_{ko}$ 

 $f_k$ 

is the contact stiffness coefficient for two-side deformation of the *RE* on the *IR* and *OR*, cf. Eq. (1.11);

is the modified deformation on the slice k of the IR of the RE # j; is the modified deformation on the slice k of the OR of the RE # j; is the Reusner's correction factor of the load on the slice k of the RE # j.

The Reusner's correction factor is used to modify the local load distribution along the length  $L_{Re}$ . The factor  $f_k$  relating to the slice k is computed as [1], as shown in Fig. 1.7.

$$f_k(k) = 1 - \frac{10^{-2}}{\ln\left[1.985 \times \left\|\frac{k - n_{S,12}}{n_S - 1}\right\|\right]}, \quad \forall k = 1, ..., n_s$$
 (1.6)

The parameter  $n_{S,12}$  is defined for an odd number  $n_S$  of the slices in each RE as

$$n_{S,12}\equiv\frac{n_S+1}{2}$$

Using the contour profile  $P(x_k)$  of the contact area, the modified deformation on the slice k is calculated under the influence of the tilting angle  $\psi_j$  and the position angle  $\varphi_j$  of the  $RE \ \# j$ .

<span id="page-16-0"></span>![](_page_16_Figure_2.jpeg)

Fig. 1.7 Reusner's correction factor  $f_k$  along the roller length

$$\widehat{\delta}_{ki}(k,j) = (\delta_{ki} - e_r/2) - 2P(x_k) + x_k \cos \varphi_j \tan \psi_j; 
\widehat{\delta}_{ki}(k,j) = \max \left(0, \widehat{\delta}_{ki}(k,j)\right) \ge 0 
\widehat{\delta}_{ko}(k,j) = (\delta_{ko} - e_r/2) - 2P(x_k) - x_k \cos \varphi_j \tan \psi_j; 
\widehat{\delta}_{ko}(k,j) = \max \left(0, \widehat{\delta}_{ko}(k,j)\right) \ge 0$$
(1.7)

where  $e_r$  is the diametral bearing clearance. Note that the modified deformation on the slice k of the IR and OR must be positive or zero; otherwise, the acting load on it is not generated because there is no reaction without deformation.

The contour profile is computed according to ISO/TS 16281:2008 (E) for tapered roller bearings as [2], as shown in Fig. 1.8.

$$P(x_k) = -4.5 \times 10^{-4} D_m \ln \left[ 1 - \left( \frac{2x_k}{L_{Re}} \right)^2 \right]$$
 (1.8)

The coordinate  $x_k$  is defined as the distance from the *RE* center  $O_{Re}$  to the center of the slice k; it is calculated according to Eq. (1.13).

In the following section, the required equations for all DOF are derived using the balance of loads and moment on the bearing. Firstly, three equations for the DOF  $\delta_{r,IR}$ ,  $\delta_{a,IR}$ , and  $\theta_b$  result from the load balance of  $F_r$ ,  $F_a$ , and  $M_b$ , respectively

<span id="page-17-0"></span>![](_page_17_Figure_2.jpeg)

Fig. 1.8 Contour profile function  $P(x_k)$  along the roller length of TRB

(s. Fig. 1.6). Note that the DOF  $\theta_b$  for bending angle is related to the displacement  $\delta_{kM}$  that is caused by the bending moment  $M_b$  as an optional computation.

The sum of the normal loads on the slices at the contact area between the OR and rolling elements in the direction y equals the radial load  $F_r$  acting on the bearing. As a result, the nonlinear equation relating to  $\delta_{r,IR}$  is written as

$$F_r - \frac{C_L}{n_s} \sum_{j=1}^{Z} \sum_{k=1}^{n_s} \hat{\delta}_{ko}^{10/9} f_k(k) \cos \alpha_o \cos \varphi_j = 0$$
 (1.9)

Analogously, the sum of the normal loads on the slices at the contact area between the OR and rolling elements in the direction x equals the axial load  $F_a$  acting on the bearing. Therefore, the nonlinear equation relating to  $\delta_{a,IR}$  results as

$$F_a - \frac{C_L}{n_s} \sum_{i=1}^{Z} \sum_{k=1}^{n_s} \hat{\delta}_{ko}^{10/9} f_k(k) \sin \alpha_o = 0$$
 (1.10)

The bending moment  $M_b$  on the bearing in the direction z acts different moments  $M_{bj}$  on each rolling element. However, the sum of all moments on the rolling elements in the direction z is equal to the given bending moment. As a result, the nonlinear equation relating to  $\delta_{kM}$  relating to the DOF  $\theta_b$  is written as

$$M_b - \frac{C_L'}{n_s} \sum_{i=1}^{Z} \sum_{k=1}^{n_s} l_{kM} \hat{\delta}_{kM}^{10/9} f_k(k) \cos \varphi_j = 0$$
 (1.11)

<span id="page-18-0"></span>where the modified bending deformation on the slice k of the RE #j is computed as

$$\widehat{\delta}_{kM}(k,j) = (\delta_{kM}\cos\varphi_j - e_r/2) - 2P(x_k) + x_k\cos\varphi_j\tan\psi_j; 
\widehat{\delta}_{kM}(k,j) = \max\left(0, \widehat{\delta}_{kM}(k,j)\right) \ge 0$$
(1.12)

Note that the modified bending deformation on the slice k must be positive or zero; otherwise, the acting load on it is not generated because there is no reaction without deformation. In Eq. (1.11), the contact stiffness coefficient  $C_L' = 3.5948 \times 10^4 L_{Re}^{8/9}$  in N/mm<sup>10/9</sup> is used in case of one-side deformation at the kerb contact zone

The moment arm  $l_{kM}$  at  $M_b \ge 0$  ( $M_b < 0$ ) is defined as the distance that is from the right-end (left-end) center of the RE to the load on the slice k of the IR.

$$l_{kM} = + \left[ \frac{1}{2} L_{Re} - x_k(k) \right] \cos \alpha_{12} - \frac{1}{2} D_k \sin \alpha_{12} : M_b \ge 0;$$
  
$$l_{kM} = - \left[ \frac{1}{2} L_{Re} + x_k(k) \right] \cos \alpha_{12} - \frac{1}{2} D_k \sin \alpha_{12} : M_b < 0$$

The distance  $x_k$  from the RE center  $O_{Re}$  to the slice center is calculated as

$$x_k(k) = (k - n_{S,12})\Delta x_k \tag{1.13}$$

where all slices have the constant thickness of

$$\Delta x_k = \frac{L_{\text{Re}}}{n_S} \tag{1.14}$$

Figure 1.9 shows the normal loads on the rolling elements of TRB. The total normal load  $Q_{ji}$  on the contact area between the IR und the rolling element RE # j is obviously the sum of all relating normal loads on its slices.

$$Q_{ji}(j) = \sum_{k=1}^{n_S} Q_{ki}(k,j) = \frac{C_L}{n_S} \sum_{k=1}^{n_S} \hat{\delta}_{ki}^{10/9} f_k(k)$$
 (1.15)

Similarly, the total normal load  $Q_{jo}$  on the contact area between the OR und the rolling element  $RE \ #j$  is the sum of all relating loads on its slices.

$$Q_{jo}(j) = \sum_{k=1}^{n_S} Q_{ko}(k,j) = \frac{C_L}{n_s} \sum_{k=1}^{n_S} \widehat{\delta}_{ko}^{10/9} f_k(k)$$
 (1.16)

<span id="page-19-0"></span>![](_page_19_Picture_2.jpeg)

Fig. 1.9 Normal loads on the rolling element of a TRB

Based on the deformation  $\delta_{fj}$ , the normal load  $Q_{fj}$  at the bearing kerb acting on the rolling element #j is calculated as

$$Q_{fj}(j) = C'_L \delta_{fj}^{10/9}(j); \delta_{fj} = \max(0, \delta_{fj}(j)) \ge 0$$
(1.17)

Note that the deformation on the kerb must be positive or zero; otherwise, the acting load on it is not generated because there is no reaction without deformation.

Using the balance of loads of the *RE* in the directions y and x, one obtains two sets of Z nonlinear equations for the DOF of  $\delta_{yj}$  and  $\delta_{xj}$ :

$$-Q_{ji}\cos\alpha_i + Q_{jo}\cos\alpha_o + Q_f\sin\alpha_m^* - F_c = 0$$
 (1.18)

$$-Q_{ji}\sin\alpha_i + Q_{jo}\sin\alpha_o - Q_f\cos\alpha_m^* = 0$$
 (1.19)

Similarly, the set of Z nonlinear equations for the DOF  $\psi_j$  results the balance of moments of the RE in the direction z:

$$\left(-\sum_{k=1}^{< n_{S,12}} l_{kL} Q_{ko} + \sum_{k \ge n_{S,12}}^{n_S} l_{kR} Q_{ko}\right) \cos \varphi_j + \left(\sum_{k=1}^{< n_{S,12}} l_{kL} Q_{ki} - \sum_{k \ge n_{S,12}}^{n_S} l_{kR} Q_{ki}\right) \cos \varphi_j 
- F_c l_c \cos \varphi_j + Q_f h_{Qf} \cos \varphi_j + M_{bj}(j) = 0$$
(1.20)

<span id="page-20-0"></span>**Fig. 1.10** Moment arms of normal loads on the slice k of RE # j

![](_page_20_Picture_3.jpeg)

In Eq. (1.20), the moment arms  $O_{Re}P$  about the RE center are computed for each normal load acting on the slice k at the left and right halves of the RE, respectively, s. Fig. 1.10.

$$\begin{split} l_{kL} &= \left( |x_k| - \frac{D_k}{2} \tan \alpha_{12} \right) \cos \alpha_{12} = |x_k| \cos \alpha_{12} - \frac{D_k}{2} \sin \alpha_{12}; \\ l_{kR} &= \left( |x_k| + \frac{D_k}{2} \tan \alpha_{12} \right) \cos \alpha_{12} = |x_k| \cos \alpha_{12} + \frac{D_k}{2} \sin \alpha_{12} \end{split}$$

in which the diameter of the slice k is calculated as

$$D_k(k) = D_m + 2x_k \tan \alpha_{12}$$

The moment arm of the kerb load about the RE center is calculated as

$$h_{Qf} = \frac{(D_{M2} - d_1)}{2\cos\alpha_m^*}$$

where the diameter of the right center of the RE is calculated as

$$D_{M2} = D_{pw} + L_{Re} \sin \alpha_m^*.$$

The bending moment on each RE # j results from Eq. (1.11) as

$$M_{bj}(j) = \frac{C_L'}{n_s} \sum_{k=1}^{n_s} l_{kM} \hat{\delta}_{kM}^{10/9} f_k(k) \cos \varphi_j$$
 (1.21)

In summary, one obtains a nonlinear equation system of (3Z + 3) equations written in Eqs. (1.9, 1.10, 1.11, 1.18, 1.19) and (3Z + 3) equational

model for tapered roller bearings under the normal loads, kerb load, and a bending moment. The computational model enables computations of the loads acting on the rolling elements and their slices at the contact areas on the IR and OR.

To solve the DOF of the computational model with a large number of strongly nonlinear equations, the Levenberg and Marquardt method based on Least Squares Method (LSM) is applied, cf. Chap. [6.](#page-100-0)

This LSM is adapted to the solver fsolve in MATLAB® as follows:

```
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%%%
% Solving the equation system of (3*Z+3) unknowns X(i)
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%%%
Function NLES_TRB
% NonLinear Equation Systems for TRB
%
% Input for iteration
iter1 = 1000;
iter2 = 1000;
epsX = 1E-6;
%
% Input for initial values of unknowns
dr_IR0 = 0.1; % mm
da_IR0 = 0.1; % mm
del_M0 = 0.1; % mm
del_xj0 = 0.05; % mm
del_yj0 = -0.05; % mm
psi_0min = 0.1; % min
%
% Initial values for unknowns X(i), i = 1,...,(3*Z+3):
X0(1) = dr_IR0;
X0(2) = da_IR0;
X0(3) = del_M0;
X0(4:1:Z+3) = del_xj0;
X0(Z+4:1:2*Z+3) = del_yj0;
X0(2*Z+4:1:3*Z+3) = psi_0;
%
% @Eq_System of unknowns X(i)
% Levenberg-Marquardt method
Options = optimoptions ('fsolv
e', 'Display', 'Iter-detailed',…
         'TolFun', epsX, 'TolX', epsX, 'MaxFunctionEvaluations',…
          iter1,'MaxIterations', iter2, 'Algorithm',…
         'levenberg-marquardt','StepTolerance', 1e-12);
```

```
%
% Solving the @Eq_System(X)
[X,fval,exitflag,output] = fsolve(@Eq_System,X0,options);
fval;
exitflag;
output;
%
Function [fX] = Eq_System(X)
% Unknowns X(i)
dr_IR = X(1);
da_IR = X(2);
del_M = X(3);
del_xj = X(4:1:Z+3);
del_yj = X(Z+4:1:2*Z+3);
psi_j = X(2*Z+4:1:3*Z+3);
%
% fX = Eq_System(X) with 3*Z+3 nonlinear equations
fX(1) = F_r - RHS_Fr; % cf. Eq. (1.9)
fX(2) = F_a - RHS_Fa; % cf. Eq. (1.10)
fX(3) = M_b - RHS_Mb; % cf. Eq. (1.11)
for j = 1:1:Z
fX(j+3) = -Q_ji(j)*sin(alfa_irad) + Q_jo(j)*sin(alfa_orad)-...
               Q_f(j)*cos(alfa_mstar(j)); % cf. Eq. (1.18)
fX(j+Z+3) = -Q_ji(j)*cos(alfa_irad) + Q_jo(j)*cos(alfa_orad)+...
               Q_f(j)*sin(alfa_mstar(j)) - Fc; % cf. Eq. (1.19)
fX(j+2*Z+3) = (RHS_OR + RHS_IR - Fc*hFc(j) + Q_f(j)*hQf(j))*...
               cos(phi_j(j)) + Mb_j(j); % cf. Eq. (1.20)
end
return
end
```

### 1.5 Computing Minimum Load and Preload on TRB

According to FAG (Schaeffler), the minimum load on the TRB in radial direction is about 2% of the dynamic load rating of the bearing.

$$F_{r,\min} = 0.02C_r \tag{1.22}$$

At a given radial load Fr and a bending moment Mb on the bearing, the minimum load in axial direction results from solving the nonlinear equation system of <span id="page-23-0"></span>(3Z + 2) equations written in Eqs. (1.9, 1.11, 1.18, 1.19 and 1.20). As a result, the minimum load on the TRB in axial direction is computed using Eq. (1.16) as

$$F_{a,\min} = \sum_{j=1}^{Z} Q_{jo}(j) \sin \alpha_o = \frac{C_L}{n_s} \sum_{j=1}^{Z} \sum_{k=1}^{n_s} \widehat{\delta}_{ko}^{10/9} f_k(k) \sin \alpha_o$$
 (1.23)

Therefore, the minimum axial load on the bearing is necessary to prevent from throwing the *IR* out of the bearing under the radial load and bending moment.

As an example, using Eqs. (1.22, 1.23) a tapered roller bearing type FAG 32007-X-XL with a bore diameter d=35 mm needs a minimum load in axial direction of about 11 kN at a radial load of 30 kN and a bending moment of 50 Nm; i.e.,  $F_{a,min}$  is approximately one-third of the radial load. In this case, the minimum radial load  $F_{r,min}$  is computed about 1.08 kN, that is much less than the minimum axial load.

The axial preload on TRB should be appropriate to keep the bearings operating in the optimum condition. On the one side, an axial preload with highly negative axial endplays leads to overheating, high friction power, low bearing efficiency, and strongly reducing the lifetime of the bearings. In worst case, they would fail in a very short time of just a few operating hours. On the other side, TRB at largely positive axial endplays have at first a high bearing efficiency and low friction in the bearings. However, they confront with some serious problems, such as NVH (noise vibration harshness), large bearing clearances, reduction of the load angle, small bearing stiffness, micro-oscillations, and slip vibrations that cause slip-related wear and damage in the bearings leading to the reduction of the bearing lifetime, s. Fig. 4.4. Furthermore, the operating conditions such as operating bearing temperature, materials of bearings and bearing housing should be considered at determining the right axial preload, as computed in Chap. 4. This task is like the correct dosing of medicaments for patients.

In practice, the axial preload on TRB in the X arrangement is the load on the bearings in axial direction at the assembly temperature of 20 °C so that the preloaded axial endplay  $\delta_{a,pl}$  of the bearings is reduced by about 0.100 and 0.200 mm for small and large bearing sizes, respectively; i.e.,  $\delta_{a,pl} = -0.100$  to -0.200 mm. As a result, the operating radial bearing endplay  $\delta_r$  depends on the operating axial endplay  $\delta_a$ , cf. Chap. 4:

$$\delta_r = \begin{cases} \delta_a \tan \alpha_o \text{ for X arrangement} \\ \delta_a \tan \alpha_i \text{ for O arrangement} \end{cases}$$
 (1.24)

As an example, for the O arrangement with the axial preloading on the IR and a half cone-angle  $\alpha_i=12.6^\circ$  at  $\delta_{a,pl}=-0.100$  to -0.200 mm, the radial bearing endplay  $\delta_{r,pl}$  at the preload is about -0.022 and -0.044 mm. For the X arrangement with the axial preloading on the OR and a half cup-angle  $\alpha_o=16.7^\circ$  at the same  $\delta_{a,pl}$ , the radial bearing endplay  $\delta_{r,pl}$  at the preload is nearly -0.030 and -0.060 mm for small and large bearing sizes, respectively, s. Figs. 1.3 and 1.4.

### <span id="page-24-0"></span>1.6 Computing Centrifugal Force of Rolling Elements

At higher rotor speeds the centrifugal force acting on the rolling elements becomes much larger. However, this centrifugal force is usually smaller than the bearing loads. In the section, the centrifugal force for the *RE* is calculated that is taken into account in the balance of loads and moments as well.

The centrifugal force lies at the mass center G of the RE. At first, its position  $x_G$  locates in the axial direction x of the RE, as shown in Fig. 1.11. The mass center position  $x_G$  results from

$$x_G V_{\text{Re}} = \int_0^{V_{\text{Re}}} x dV = \int_0^{L_{\text{Re}}} x \pi \xi^2(x) dx = \int_0^{L_{\text{Re}}} x \pi (r + x \tan \alpha_{12})^2 dx$$
$$= \pi L_{\text{Re}}^2 \left[ \frac{r^2}{2} + \frac{2r}{3} (R - r) + \frac{1}{4} (R - r)^2 \right]$$

where the volume of the RE is calculated as

$$V_{\rm Re} = \frac{\pi L_{\rm Re}}{3} (R^2 + Rr + r^2)$$

Substituting both equations, one obtains the position of the mass center G

$$x_G = \frac{L_{\text{Re}}}{4} \left( \frac{3R^2 + 2Rr + r^2}{R^2 + Rr + r^2} \right)$$

Thus, the centrifugal force on the RE is computed as

$$F_{c} = \frac{1}{2} m_{\text{Re}} D_{pw} \omega_{R}^{2}$$

$$= \frac{1}{2} \left[ \frac{\pi}{3} \rho_{\text{Re}} L_{\text{Re}} (R^{2} + Rr + r^{2}) \right] D_{pw} \omega_{R}^{2}$$

$$= \frac{\pi}{6} \left( \frac{2\pi}{60} \right)^{2} \rho_{\text{Re}} L_{\text{Re}} (R^{2} + Rr + r^{2}) D_{pw} N_{R}^{2}$$
(1.25)

**Fig. 1.11** Mass center G of the rolling element (RE)

![](_page_24_Picture_13.jpeg)

<span id="page-25-0"></span>where the mass of the *RE* results from its volume  $V_{Re}$  and the density of steel  $\rho_{Re} = 7900 \text{ kg/m}^3$  in

$$m_{\rm Re} = \rho_{\rm Re} V_{\rm Re} = \rho_{\rm Re} \frac{\pi L_{\rm Re}}{3} (R^2 + Rr + r^2)$$
  
 $\approx 8.273 \times 10^{-6} L_{\rm Re} (R^2 + Rr + r^2)$  (1.26)

where the mass is in kg and all lengths in mm.

For steel rolling elements, using Eq. (1.25) the centrifugal force can be approximately calculated as

$$F_c \approx 4.54 \times 10^{-11} L_{\text{Re}}(R^2 + Rr + r^2) D_{pw} N_R^2$$
 (1.27)

in which the force is in N, the geometrical dimensions in mm, and the rotor speed in rpm.

### 1.7 Computing Hertzian Pressures at the Contact Zones

The individual Hertzian contact width of the slice k is calculated on the IR and OR of the RE # j, cf. Fig. 1.12.

$$b_{ki;ko}(k) = \sqrt{\frac{8Q_{ki;ko}(k)}{\pi E'\left(\frac{\Delta x_k}{\cos \alpha_{12}}\right) \sum \rho_{IR;OR}}}$$
(1.28)

The effective elasticity module E' results from the elasticity modules  $E_1$  and  $E_2$  of rollers and races, respectively.

**Fig. 1.12** Hertzian contact zone on the slice k

![](_page_25_Picture_13.jpeg)

$$E' = \frac{2}{\left(\frac{1-\nu_1^2}{E_1} + \frac{1-\nu_2^2}{E_2}\right)} \tag{1.29}$$

<span id="page-26-0"></span>where  $v_1$  and  $v_2$  are the Poison's numbers of rollers and races, respectively. The curvature sums of the inner and outer races are computed as [3]

$$\sum \rho_{IR} = \frac{2}{D_k(x_k)} \left( \frac{A_i}{A_i - 1} \right);$$

$$\sum \rho_{OR} = \frac{2}{D_k(x_k)} \left( \frac{A_o}{A_o + 1} \right);$$

in which the auxiliary parameters are defined and calculated as

$$\begin{split} A_i &= \frac{D_{pwk}(x_k)}{D_k \cos \alpha_i}; \quad A_o = \frac{D_{pwk}(x_k)}{D_k \cos \alpha_o}; \\ D_{pwk}(x_k) &= D_{pw} + 2x_k \sin \alpha_m^*; \quad \alpha_m^* = \alpha_m - \psi_j \cos \phi_j; \\ D_k(x_k) &= D_m + 2x_k \tan \alpha_{12}; \quad \alpha_{12} = \frac{1}{2}(\alpha_o - \alpha_i). \end{split}$$

The Hertzian pressures  $p_{Hi}$  and  $p_{Ho}$  at the contact zone on the slice k on the IR and OR of the RE # j is computed from the contact widths in Eq. (1.28) as

$$p_{Hi;Ho}(k) = rac{2Q_{ki;ko}(k)}{\pi b_{ki;ko} \left(rac{\Delta x_k}{\coslpha_{12}}
ight)} \sqrt{1 - \left(rac{\zeta}{b_{ki;ko}}
ight)^2}$$

Obviously, the maximum Hertzian pressure on the slice k occurs at  $\zeta = 0$ :

$$p_{Hi;Ho,\text{max}} = \frac{2Q_{ki;ko}(k)}{\pi b_{ki;ko} \left(\frac{\Delta x_k}{\cos \alpha_{12}}\right)}$$
(1.30)

### 1.8 Computing Oil Film Thickness in TRB

### 1.8.1 Oil-Film Thicknesses in the Contact Area

The oil-film thickness at the Hertzian region is based on the theory of Hamrock and Dowson [4] that is implemented in Program TRBOFT using the MATLAB Code [5].

There are two oil-film thicknesses of  $h_{k,c}$  and  $h_{k,min}$  in the center and at the outflow of the contact area of the slice k, as shown in Fig. 1.13. Generally, the

<span id="page-27-0"></span>![](_page_27_Figure_2.jpeg)

Fig. 1.13 Oil-film thicknesses in a roller bearing

oil-film thickness between the rollers and raceways depends on the speed parameter of the rollers, material parameter of the rollers and raceways, load parameter of the rollers.

It is obvious that

- The faster the roller rotates, the larger the oil-film thickness is created between the rollers and raceways. In this case, more dissolved oil in grease is separated from grease due to the oil centrifugal force and increased oil temperature. Additionally, the hydrodynamic effect of the oil film is improved at high velocities of the oil inflow to build up the oil film. Thus, the oil-film thickness in the contact area increases with the *speed parameter*.
- The larger the oil viscosity is, the larger the oil-film thickness is induced in the
  contact area. The oil viscosity exponentially increases with the
  pressure-viscosity coefficient that relates to the *material parameter*.
- On the contrary, the higher the equivalent radial load acts on the roller, the smaller the oil-film thickness is in the contact area.
- If the width  $2b_k$  of the contact area in the rolling direction  $\zeta$  is much smaller than the contact length  $L_{Re}$  in the axial direction x, the oil-film breaks down in the contact area due to a small contact area leading to very high Hertzian pressure, s. Fig. 1.12.

At first, the oil-film-thicknesses related dimensionless parameters are defined in the following section. They are used in the semi-empirical equations for computing the oil-film thickness in the contact area.

The curvature radius  $R_{IR,\zeta}$  of the roller (r) and inner raceway (IR) in the *rolling direction*  $\zeta$  is defined as

$$R_{IR,\zeta} = \frac{1}{\rho_{r/IR,\zeta}}$$

<span id="page-28-0"></span>The curvature  $\rho_{r/IR,\zeta}$  for the roller/inner raceway is written in [3]

$$\rho_{r/IR,\zeta} = (\rho_{11} + \rho_{21})_{IR} = \frac{2}{D_m} + \frac{2}{D_m} \left(\frac{1}{A-1}\right) = \frac{2}{D_m} \left(\frac{A}{A-1}\right)$$

where the calculating factor A is defined as

$$A = \frac{D_{pw}}{D_m \cos \alpha}$$

Thus, the curvature radius  $R_{IR,\zeta}$  in the rolling direction  $\zeta$  results in

$$R_{IR,\zeta} = \frac{D_m}{2} \left( \frac{A-1}{A} \right)$$

Similarly, the curvature radius  $R_{OR,\zeta}$  of the curvature of the roller (r) and outer raceway (OR) in the *rolling direction*  $\zeta$  is defined as

$$R_{OR,\zeta} = \frac{1}{\rho_{r/OR,\zeta}}$$

The curvature  $\rho_{r/OR,\zeta}$  for the roller/outer raceway is written as

$$\rho_{r/OR,\zeta} = (\rho_{11} + \rho_{21})_{OR} = \frac{2}{D_m} - \frac{2}{D_m} \left(\frac{1}{A+1}\right) = \frac{2}{D_m} \left(\frac{A}{A+1}\right)$$

Thus, the curvature radius  $R_{OR,\zeta}$  in the rolling direction  $\zeta$  results as

$$R_{OR,\zeta} = \frac{D_m}{2} \left( \frac{A+1}{A} \right)$$

The dimensionless oil-film thickness H for the inner and outer raceways is defined as

$$H\equiv rac{h}{R_{\zeta}}; \quad R_{\zeta}\equiv R_{IR;OR,\zeta}$$

The dimensionless speed parameter  $U^*$  of the roller at the inner and outer raceways is defined as

$$U^* = rac{\mu_0 U}{E' R_{\zeta}}; \quad R_{\zeta} \equiv R_{IR;OR,\zeta}$$

<span id="page-29-0"></span>where  $\mu_0$  is the dynamic oil viscosity at the ambient pressure.

The circumferential mean velocity U in m/s at the roller-raceways contact is calculated as [3]

$$U = \frac{\pi N_R D_{pw}}{120} \times \left[ 1 - \left( \frac{D_m}{D_{pw}} \cos \alpha_o \right)^2 \right] \times 10^{-3}$$

where  $N_R$  is the rotor speed in rpm;  $D_{pw}$  is the pitch diameter in mm; and  $D_m$  is the mean tapered roller diameter in mm.

The dimensionless *load parameter W\** of the rollers is defined as

$$W^* = \frac{W}{E'L_{\mathrm{Re}}R_{\zeta}}; \quad R_{\zeta} \equiv R_{IR;OR,\zeta}$$

where W is the maximum equivalent normal load of  $Q_{ki}$  and  $Q_{ko}$  acting upon the Hertzian contact area of the slice k;  $L_{Re}$  is the length of the roller bearing.

Finally, the dimensionless material parameter  $G^*$  is defined as

$$G^* = \alpha_{EHL}E'$$

where  $\alpha_{EHL}$  is the pressure-viscosity coefficient (*Barus coefficient*) in the regime of elastohydrodynamic lubrication (EHL).

The Barus coefficient is empirically calculated according to [4] as

$$\alpha_{EHL} \approx 5.1 \times 10^{-9} Z' \times (\ln \mu_0 + 9.67)$$

where  $\alpha_{EHL}$  in 1/Pa,  $\mu_0$  in Pas at the ambient relative pressure (p = 0).

The Roeland's pressure-viscosity index Z' is given in [3, 4]

$$Z' \approx [7.81 \times (H_{40} - H_{100})]^{1.5} \times F_{40}$$

where

$$H_{40} = \log_{10}(\log_{10}\mu_{40} + 1.2)$$
 at 40 °C;  
 $H_{100} = \log_{10}(\log_{10}\mu_{100} + 1.2)$  at 100 °C;  
 $F_{40} = 0.885 - 0.864H_{40}$ 

with the dynamic viscosity  $\mu_{40}$  and  $\mu_{100}$  in mPa of the lubricating oil at 40 and 100 °C, respectively.

Usually, the Roeland's pressure-viscosity index Z' is about 0.60 for mineral oils, and between 0.40 and 0.80 for synthetic oils.

## <span id="page-30-0"></span>1.8.2 Computing the Oil-Film Thicknesses in Roller Bearings

The minimum oil-film thickness  $h_{min}$  for a hard EHL regime of roller bearings at the contact area with a rectangular footprint results from solving the coupled Reynolds und elasticity equations [3]. Next, using the least squares fit method the dimensionless minimum oil film thickness is calculated at the inner and outer raceways according to [4] as, cf. Fig. 1.13

$$H_{\min} \equiv \frac{h_{\min}}{R_{\zeta}} = 1.714 \times U^{*0.694} \cdot G^{*0.568} \cdot W^{*-0.128}$$
 (1.31)

Usually, the empirical value  $5.007 \times 10^3$  is used for the dimensionless material parameter  $G^*$  in the roller bearings.

To analyze the influence parameters on the minimum oil-film thickness  $h_{min}$  that is rewritten in

$$h_{\min} = \frac{1.806 \times (\mu_0 U)^{0.694} \cdot \alpha_{EHL}^{0.568} \cdot R_{\zeta}^{0.434}}{E'^{(-0.002)} \cdot \left(\frac{W}{L_{Re}}\right)^{0.128}}$$
(1.32)

Equation (1.32) shows that the parameters  $\mu_0$ , U,  $\alpha_{EHL}$ , and  $R_{\zeta}$  have a strong influence on the minimum oil-film thickness. On the contrary, the bearing load W has a slight influence on it since its exponent is very small compared to the other exponents. Furthermore, the minimum oil-film thickness is quasi-independent of the effective modulus of elasticity E' because its exponent nearly equals zero.

The center oil-film thickness  $h_c$  for a hard EHL regime at a rectangular contact area is semi-empirically calculated at the inner and outer raceways according to [4] as

$$H_c \equiv \frac{h_c}{R_{\zeta}} = 2.922 \times U^{*0.692} \cdot G^{*0.470} \cdot W^{*-0.166}$$
 (1.33)

In similar to the minimum oil-film thickness  $h_{min}$ , to analyze the influence parameters on the center oil-film thickness  $h_c$ , one rewrites Eq. (1.33) in

$$h_c = \frac{2.922 \times (\mu_0 U)^{0.692} \cdot \alpha_{EHL}^{0.47} \cdot R_{\zeta}^{0.474}}{E'^{0.056} \cdot \left(\frac{W}{L_{Re}}\right)^{0.166}}$$
(1.34)

Equation (1.34) shows that the parameters  $\mu_0$ , U,  $\alpha_{EHL}$ , and  $R_{\zeta}$  have a strong influence on the center oil-film thickness. On the contrary, the bearing load W has a

<span id="page-31-0"></span>slight influence on it since its exponent is very small compared to the other exponents. Furthermore, the center oil-film thickness is nearly independent of the reduced elastic modulus E' because its exponent is very small.

### 1.9 Computing Bearing Friction in TRB

The total friction torque acting on the bearing is caused by the bearing loads, viscous friction of oil, and kerb friction of the bearing.

$$M_f = M_l + M_v + M_a (1.35)$$

where

 $M_l$  is the load torque that is caused by radial and thrust loads acting on the bearing;

 $M_{\nu}$  is the viscous torque that is caused by the oil viscous friction in the bearing;

 $M_a$  is the kerb torque that is caused by the friction between the bearing kerb and rolling elements.

The load torque  $M_l$  (N × mm) of bearings is computed using the empirical formula of Palmgren [6, 7] as

$$M_l = f_1 F_\beta D_{pw}; f_1 = 4 \times 10^{-4} \text{ to } 5 \times 10^{-4}$$
 (1.36)

in which  $D_{pw}$  (mm) is the pitch diameter of the bearing.

The second factor  $F_{\beta}$  (N) depends on the radial and thrust loads and it is written [8]:

$$F_{\beta} = \max(2YF_a, F_r)$$
 for  $X/O$ -single bearing;  $F_{\beta} = \max\left(1.21\frac{F_a}{e}, F_r\right)$  for  $X/O$ -bearing set

where

Y and e are the bearing parameters of a certain type (cf. bearing catalogues);  $F_r$  and  $F_a$  are the radial and resulting axial load (N) on the bearing, respectively.

The resulting axial load  $F_{a,A}$  on the bearing A is calculated from the external axial load  $F_{ax}$  on the shaft and the radial load  $F_{r,B}$  on the opposite bearing B in an O or X arrangement and vice versa [8]:

$$F_{a,A} = F_{ax} + 0.47 \frac{F_{r,B}}{Y_B};$$

$$F_{a,B} = 0.47 \frac{F_{r,A}}{Y_A} - F_{ax}$$
(1.37)

<span id="page-32-0"></span>Note that the bearing A in the O or X arrangement is defined as the bearing where the external axial load  $F_{ax}$  acts on it directly.

The viscous torque  $M_{\nu}$  (N × mm) between oil and rollers is computed using the empirical formula of Palmgren [6] as

$$M_{\nu} = 160 \times 10^{-7} f_o D_{pw}^3$$
 for  $\nu N_R < 2000$ ;  
=  $10^{-7} f_o (\nu N)^{2/3} D_{pw}^3$  for  $\nu N_R \ge 2000$  (1.38)

where  $v \text{ (mm}^2\text{/s)}$  is the oil kinematic viscosity;  $N_R$  is the rotor speed (rpm).

The factor  $f_o$  in Eq. (1.38) depends on the bearing types such as

$$f_o = 3.0$$
 for TRB types 302, 303, 313;  
= 4.5 for TRB types 320, 322, 323, 329;  
= 6.0 for TRB types 330, 331, 332.

The additional torque  $M_a$  (N × mm) is caused by the friction due to the resulting load  $F_a$  (N) in axial direction [8]:

$$M_a = 0.06 f_a F_a D_{pw} (1.39)$$

The factor  $f_a$  in Eq. (1.39) is experimentally calculated based on the parameter Y as

$$f_a = 0.15$$
 for  $0.5 \le Y < 1$   
=  $(0.17267 - 2.267 \times 10^{-2} Y)$  for  $1 \le Y \le 7$   
=  $0.14$  for  $7 < Y \le 40$ 

The dimensionless parameter Y is defined by

$$Y = f_b D_{pw} v N_R (D^2 - d^2) / F_a^2$$

where  $f_b = 0.0048$ ,  $D_{pw}$  (mm), v (mm<sup>2</sup>/s),  $N_R$  (rpm), the bore diameter d (mm), the outside diameter D (mm), and the resulting axial load  $F_a$  (N) in Eq. (1.37).

<span id="page-33-0"></span>In total, the frictional power  $P_f$  (W) in the bearing results from Eqs. (1.36 and 1.38, 1.39) in

$$P_f = M_f \omega_R$$
  
=  $(M_l + M_v + M_a) \times \left(\frac{2\pi N_R}{60}\right) \times 10^{-3}$  (1.39)

### 1.10 Computing Lifetime of TRB

Using the Palmgren-Miner's damage rule, the extended fatigue lifetime  $Lh_m(h)$  of the bearing results from the individual lifetimes  $Lh_{m,ij}(h)$  of all operating points with the individual percentages of revolution  $t_{ij}$  in the load spectrum as [3].

$$\frac{1}{Lh_m} = \sum_{i=1}^{N} \sum_{j=1}^{M} \frac{t_{ij}}{Lh_{m,ij}} \Rightarrow Lh_m = \frac{1}{\sum_{i=1}^{N} \sum_{j=1}^{M} \frac{t_{ij}}{Lh_{m,ij}}}$$
(1.40)

The individual lifetime  $Lh_{m,ij}$  (h) is calculated from its dynamic load rating  $C_r$  (N), dynamic equivalent load  $P_{m,ij}$  (N), rotor speed  $N_{R,ij}$  (rpm), and the extended lifetime factor  $a_{ISO,ij}$  according to the DIN/ISO 281 [9]:

$$Lh_{m,ij} = \frac{10^6}{60N_{R,ij}} a_1 a_{ISO,ij} L_{10,ij}$$

$$= \frac{10^6}{60N_R} a_1 a_{ISO,ij} \left(\frac{C_r}{P_{m,ij}}\right)^p$$
(1.41)

in which the lifetime factor p = 10/3 is chosen for all roller bearings;  $a_1$  depends on the failure probability m of the bearing (e.g.  $a_1 = 1$  for m = 10%). The dynamic equivalent load  $P_m$  on each individual bearing is calculated from the radial and axial load in the X or O arrangement [8, 10].

In case of considering the tilting rollers in the TRB, the Hertzian pressures increase at the roller ends compared to the pressures at the roller middle. Therefore, the modified lifetime for each slice of the rollers should be calculated at the given loads in the load spectrum according to the ISO/TS 16281:2008 (E) [2]. Then, the extended bearing lifetime results from Eqs. (1.40, 1.41).

The dynamic load ratings on a slice k of the roller on the IR and OR are calculated as

$$q_{kci} = Q_{ci} \left(\frac{1}{n_S}\right)^{7/9} \text{ for } IR;$$

$$q_{kco} = Q_{co} \left(\frac{1}{n_S}\right)^{7/9} \text{ for } OR.$$

$$(1.42)$$

<span id="page-34-0"></span>where  $Q_{ci}$  and  $Q_{co}$  are the dynamic load ratings of the roller on the *IR* and *OR*, respectively;  $n_S$  is the number of slices per roller.

The dynamic load ratings of the roller at the IR are computed as

$$Q_{ci} = \frac{1}{\lambda v} \cdot \frac{C_r}{0.378Z \cos \alpha_m i^{7/9}} \left[ 1 + \left\{ 1.038 \left( \frac{1 - \gamma}{1 + \gamma} \right)^{143/108} \right\}^{+9/2} \right]^{2/9}$$

in which the used parameters are defined for a single row (i = 1) by

$$\gamma = \frac{D_k \cos \alpha_m}{D_{pw}}; \quad \lambda v = 0.83; \quad i = 1$$

Similarly, the dynamic load ratings of the roller at the OR are computed as

$$Q_{co} = \frac{1}{\lambda v} \cdot \frac{C_r}{0.364Z \cos \alpha_m i^{7/9}} \left[ 1 + \left\{ 1.038 \left( \frac{1-\gamma}{1+\gamma} \right)^{143/108} \right\}^{-9/2} \right]^{2/9}$$

The lifetimes in  $10^6$  revolutions of the roller #j for the line contact on the IR and OR result from the modified reference lifetimes of the slices of the roller in

$$L_{ir,j} = \sum_{k=1}^{n_S} \left(\frac{q_{kci}}{q_{kei}}\right)^{4.0} \quad \text{for } IR;$$

$$L_{or,j} = \sum_{k=1}^{n_S} \left(\frac{q_{kco}}{q_{keo}}\right)^{4.5} \quad \text{for } OR.$$

$$(1.43)$$

where  $q_{kei}$  and  $q_{keo}$  are the dynamic equivalent loads on the slice k on the IR and OR, respectively.

The Reusner's factors are used to compute the increase of normal stresses along the roller length on the IR and OR:

$$f_{i}(j,k) = \left[ \left( \frac{p_{Hi}}{271} \right)^{2} D_{k}(k) \cdot (1-\gamma) \cdot \frac{L_{\text{Re}}}{n_{S}} \right] \times \frac{1}{Q_{ki}(j,k)};$$

$$f_{o}(j,k) = \left[ \left( \frac{p_{Ho}}{271} \right)^{2} D_{k}(k) \cdot (1-\gamma) \cdot \frac{L_{\text{Re}}}{n_{S}} \right] \times \frac{1}{Q_{ko}(j,k)}$$

$$(1.44)$$

<span id="page-35-0"></span>where  $p_{Hi}$  and  $p_{Ho}$  are the maximum Hertzian pressures (MPa) on the slice k of the IR and OR, s. Eq. (1.30);  $Q_{ki}$  and  $Q_{ko}$  are the normal loads (N) on the slice k of the IR and OR, respectively, s. Eq. (1.5).

The dynamic equivalent loads on the slice k based on Z rollers of the TRB are computed for the IR and OR, respectively:

$$q_{kei}(k) = \left(\frac{1}{Z} \sum_{j=1}^{Z} \left[ f_i(j,k) \cdot Q_{ki}(j,k) \right]^{4.0} \right)^{1/4};$$

$$q_{keo}(k) = \left(\frac{1}{Z} \sum_{j=1}^{Z} \left[ f_o(j,k) \cdot Q_{ko}(j,k) \right]^{4.5} \right)^{1/4.5}$$
(1.45)

Using the Palmgren and Miner's damage law for the *IR* and *OR*, the modified reference lifetime of the TRB results in, cf. App. A

$$\frac{1}{L_r^{\beta}} = \frac{1}{L_{ri}^{\beta}} + \frac{1}{L_{ro}^{\beta}} \Rightarrow L_r = \left(L_{ri}^{-\beta} + L_{ro}^{-\beta}\right)^{-1/\beta} \tag{1.46}$$

in which  $\beta = 9/8$  is the Weibull's slope for roller bearings.

Substituting Eqs. (1.42, 1.43, 1.44, 1.45) into Eq. (1.46), one obtains the modified reference lifetime of the TRB in  $10^6$  revolutions:

$$L_r(rev) = \left[ \sum_{k=1}^{n_S} \left\{ \left( \frac{q_{kci}(k)}{q_{kei}(k)} \right)^{-4\beta} + \left( \frac{q_{kco}(k)}{q_{keo}(k)} \right)^{-4.5\beta} \right\} \right]^{-1/\beta}$$

$$= \left[ \sum_{k=1}^{n_S} \left\{ \left( \frac{q_{kci}(k)}{q_{kei}(k)} \right)^{-4.5} + \left( \frac{q_{kco}(k)}{q_{keo}(k)} \right)^{-5.0625} \right\} \right]^{-8/9}$$
(1.47)

The extended lifetime in  $10^6$  revolutions for a 10% failure probability is computed as

$$L_{10,r}^{*}(rev) = a_{ISO}L_{r}(rev)$$

$$= a_{ISO} \times \left[ \sum_{k=1}^{n_{S}} \left\{ \left( \frac{q_{kci}(k)}{q_{kei}(k)} \right)^{-4.5} + \left( \frac{q_{kco}(k)}{q_{keo}(k)} \right)^{-5.0625} \right\} \right]^{-8/9}$$
(1.48)

Using the rotor speed  $N_R$  (rpm), the extended lifetime  $Lh^*_{10}$  (h) results in

$$Lh_{10}^{*}(h) = \frac{10^{6}L_{10,r}^{*}(rev)}{60N_{R}(rpm)}$$
(1.49)

### <span id="page-36-0"></span>1.11 Computing Bearing Stiffness of TRB

The radial bearing stiffness is calculated from the radial load on the bearing and the elastic deformation of the *IR* in radial direction as

$$K_{b,r} = \frac{\partial F_r}{\partial \delta_r} \approx \frac{F_r}{\delta_{r,IR}} \tag{1.50}$$

Analogously, the axial bearing stiffness is calculated from the axial load on the bearing and the elastic deformation of the IR in axial direction as

$$K_{b,a} = \frac{\partial F_a}{\partial \delta_a} \approx \frac{F_a}{\delta_{a,IR}} \tag{1.51}$$

It is quite complicated to calculate the bending stiffness of the TRB. Therefore, a computing model shown in Fig. 1.14 is used to compute it. Let the bending moment  $M_b$  act on the IR in the positive direction (counterclockwise) while the OR is stationary on the bearing housing. Due to the resistance of the bearing stiffness  $K_b$ , the IR deforms at a bending angle  $\theta_b > 0$  to the stationary OR.

The bending moment  $M_b$  on the IR is the sum of the resistant moments of Z rollers in the bearing, cf. Eq. (1.11). The resistant moment  $M'_{bj}$  of the roller #j is calculated from its bending stiffness  $K_{bj}$ , tilting angle  $\psi_j$ , and position angle  $\varphi_j$ . As a result, the bending stiffness of the roller #j results in

**Fig. 1.14** Computational model of the bearing stiffness

![](_page_36_Picture_10.jpeg)

<span id="page-37-0"></span>
$$M_{b} = \sum_{j=1}^{Z} M'_{bj}(j); M'_{bj}(j) = K_{bj}(j) \cdot \psi_{j} \cos \varphi_{j}$$

$$\Rightarrow K_{bj}(j) = \left| \frac{M'_{bj}(j)}{\psi_{j} \cos \varphi_{j}} \right| \ge 0$$
(1.52)

At a small bending angle  $\theta_b$  between the *IR* and *OR*, the bending moment  $M_b$  on the *IR* is calculated from the bending stiffness  $K_{bj}$  of the roller #j as

$$M_b = \sum_{j=1}^{Z} M_{bj}(j) \Leftrightarrow K_{b,M} \cdot \theta_b = \sum_{j=1}^{Z} K_{bj}(j) \cdot \theta_b$$
 (1.53)

Dividing both sides of Eq. (1.53) by  $\theta_b$ , one obtains the bending stiffness of the bearing from Eq. (1.52)

$$K_{b,M} = \sum_{j=1}^{Z} K_{bj}(j)$$

$$\Rightarrow K_{b,M} = \sum_{j=1}^{Z} \left| \frac{M'_{bj}(j)}{\psi_{j} \cos \varphi_{j}} \right| \ge 0$$
(1.54)

According to Eq. (1.54), the bending stiffness  $K_{b,M}$  is the sum of the bending stiffnesses of Z rollers. It is like the total electrical conductance (i.e. the reciprocal to resistance) of the parallel circuit of resistors, as shown in Fig. 1.14.

The bending angle of the bearing results from Eq. (1.53) in

$$\theta_b = \frac{M_b}{K_{bM}} \tag{1.55}$$

### 1.12 An Example for Computational TRB

In the following section, an example to compute a TRB of type FAG 32007-X-XL with a bore diameter of 35 mm and 20 rolling elements. Each roller with a mean diameter of 6.5 mm and a length of about 12.445 mm is divided into 65 circular slices.

Let a radial load of 30,000 N, an axial load of 10,000 N, and a positive bending moment of 50 Nm act on the bearing. Using the program COMTRB [11], the local Hertzian pressures on the slices at the contact zones of the *IR* and *OR* are computed. To compute the oil film thicknesses the program TRBOFT [5] is applied.

### Input data for MATLAB

### Case = 'TRB Type FAG 32007-X-XL';

```
F_r = 30,000; % radial load on TRB (N)
F_a = 10,000; % axial load on TRB (N)
M_b = 50; % bending moment on TRB (Nm)
Z = 20; % number of bearing rollers
ns_odd = 65; % number of circular slices per RE (always odd number)
e_r = 0E−3; % bearing diametral clearance (mm)
L_re = 12.445; % length of RE (mm)
Dm = 6.5; % mean diameter of RE (mm)
Dpw = 48; % mean pitch diameter of bearing (mm)
d_1 = 50; % cf. bearing catalog (mm)
alfa_i = 12.66; % half cone angle (°)
alfa_o = 16.70; % half cup angle (°)
i_cor = 1; % 0: without; 1: with the Reusner's correction
jfav = 1; % favorite roller # (jfav = 1 to Z)
%
% Material data of RE
Em_1 = 208E3; % elasticity modulus of RE (MPa)
Em_2 = 208E3; % elasticity modulus of races (MPa)
nu_1 = 0.3; % Poisson ratio of RE (−)
nu_2 = 0.3; % Poisson ratio of races (−)
%
% Iteration
iter1 = 5000; % maximum iteration steps
iter2 = 5000; % maximum function evaluations
epsX = 1E−6; % convergence tolerance
%
% Initial values for unknowns
dr_IR0 = 0.1; % initial radial deformation of dr_IR > 0 (mm)
da_IR0 = 0.1; % initial axial deformation of da_IR > 0 (mm)
del_M0 = 0.1; % initial radial deformation of del_M > 0 (mm)
del_xj0 = 0.05; % initial deformation of del_xj > 0 (mm)
del_yj0 = −0.05; % initial deformation of del_yj < 0 (mm)
psi_0 min = 0.1; % initial tilting angle of RE (min)
```

Due to strong nonlinearity of the equation system describing the TRB, the initial values of the unknowns should be appropriately chosen; otherwise, no converged solution is reached for this problem. The unknowns are iteratively solved beginning with these initial values. The convergence of solutions is reached if the norm of their residuals is less than the given tolerance epsX = 10−<sup>6</sup> . Additionally, the limits

<span id="page-39-0"></span>![](_page_39_Figure_2.jpeg)

Fig. 1.15 Distribution of the bending moment on the rollers

iter1 and iter2 of iteration steps and maximal evaluations are given at 5000 for each limit. Using the program fsolve in MATLAB optimized with the Levenberg-Marquardt algorithm, the solutions are converged after 10 iteration steps and about 700 function evaluations. The convergence reaches a tolerance of  $3.8 \times 10^{-7} < \text{epsX}$ .

Figure 1.15 shows the distribution of the bending moment of 50 Nm on the rollers of the bearing in the direction z. The result indicates that the maximum bending moment of 10.6 Nm acts on the roller Z # 20, which is the opposite roller to the radial load (s. Fig. 1.5). The individual bending moment on the roller in the direction z is computed from Eq. (1.21) and affects the distribution of the normal loads on the IR and OR of the RE by the balance of moments.

The normal loads acting on the IR and OR of each RE result from Eqs. (1.15, 1.16). They are shown in Figs. 1.16 and 1.17, respectively. The maximum loads of nearly 9184 N also occur on the roller Z #20. As a result, the maximum Hertzian pressures on the IR and OR of each RE are caused by the maximum loads on the roller Z #20. The normal loads at the bearing kerb (called kerb load) on the rollers are computed from Eq. (1.17) and are displayed in Fig. 1.18. The maximum kerb load of 647 N is on the roller Z #20.

All results show that the roller 5–15 are not loaded with the normal load, kerb load, and bending moment. The maximum loads and bending moment concentrate on the roller Z #20.

The normal loads of 9184 N on the IR and OR of the roller Z #20 are distributed in  $n_S = 65$  slices with a slice thickness of about 0.190 mm over the roller length, as shown in Fig. 1.19. The maximum load of about 175 N occurs on the IR at 2.3 mm;

<span id="page-40-0"></span>![](_page_40_Figure_2.jpeg)

Fig. 1.16 Distribution of normal loads in the IR of the rollers

![](_page_40_Figure_4.jpeg)

Fig. 1.17 Distribution of normal loads in the OR of the rollers

and on the OR at −2.3 mm from the roller center (xk = 0). Due to the rounded profile of the RE, no load occurs at the left end of the IR and the right end of the OR.

Using Eq. [\(1.28](#page-25-0)) the different widths bk at the EHD contact zone between the IR/ OR and the RE of Z #20 are computed over the roller length. The results in Fig. [1.20](#page-42-0) show the maximum contact widths of nearly 0.170 mm (IR) and 0.190 mm (OR) occur at the positions of ca. +3.45 and −2.50 mm from the RE center, respectively.

<span id="page-41-0"></span>![](_page_41_Figure_2.jpeg)

Fig. 1.18 Distribution of loads at the bearing kerb of the rollers

![](_page_41_Figure_4.jpeg)

Fig. 1.19 Normal loads over the length on the IR and OR of the roller Z #20

From the normal loads and the contact widths result the maximum Hertzian pressures on the IR and OR of the roller Z #20 according to Eq. [\(1.30](#page-26-0)). The maximum Hertzian pressure is about 3.4 GPa on the IR; 3.0 GPa on the OR at the positions of ca. +1.90 and −3.10 mm from the RE center, respectively, as shown in Fig. [1.21](#page-42-0).

<span id="page-42-0"></span>![](_page_42_Figure_2.jpeg)

Fig. 1.20 Contact widths over the length of the roller Z #20

![](_page_42_Figure_4.jpeg)

Fig. 1.21 Hertzian pressures over the length of the roller Z #20

The computational result shows that the tilting angle w<sup>j</sup> of the roller Z #20 is about 2.6 min in counterclockwise. As a result, it causes an increase of the Hertzian pressure on the IR to about 2.5 GPa at the rounded right end and no contact (i.e. pHi = 0) at the rounded left end of the roller. On the contrary, the Hertzian pressure on the OR of approximately 2.4 GPa is at the rounded left end and no contact (i.e.  $p_{Ho} = 0$ ) at the rounded right end of the roller.

The computational results give that the radial bearing stiffness  $K_{b,r}$  is about  $2.58 \times 10^5$  N/mm, the axial bearing stiffness  $K_{b,a}$  is about  $4.34 \times 10^4$  N/mm, and the bending stiffness  $K_{b,M}$  is about  $5.7 \times 10^4$  Nm/rad. From Eq. (1.55) the bending angle  $\theta_b$  between the *IR* and *OR* results in ca. 3 min.

In the following section, the bearing friction in two tapered roller bearings A and B is computed with the load spectrum of electric vehicles shown in Fig. 1.22. The positive torque is for the driving operation and the negative torque for the recuperation phase. The probabilities of each event that consists of the shaft torque and speed based on the revolutions are displayed in vertical direction. Obviously, the sum of all probabilities of the events in the load spectrum must be 100%. Note that the axial load direction in the recuperation phase changes oppositely compared to the axial load in the driving operation because the contact position at the gear flange changes in the opposite side of the gear tooth. However, the driving shaft rotates in the same direction in both operations.

In the computation, both bearings 32007-X-XL are preloaded with 7500 N in axial direction. For the bearing lubrication, Castrol oil type BOT 352 B1 with an additive is used at an oil temperature 90  $^{\circ}$ C. The cleanliness of oil quality -/15/12 according to ISO 4406 is chosen for the driving operation without filter.

The computation results of the program TRBLHF [10] show that the friction power A is about 116 W in the bearing and 82 W in the bearing B. In total, the friction of both bearings is nearly 0.2 kW with this load spectrum.

![](_page_43_Figure_7.jpeg)

Fig. 1.22 Load spectrum of an intermediate gear shaft with two TRB

![](_page_44_Figure_2.jpeg)

Fig. 1.23 Minimum oil film thickness  $h_{min}$  over the length of the roller Z #20

Additionally, the lifetimes  $Lh_{10}$  of the bearings are computed according to the DIN/ISO 281 [9], cf. Eq. (1.41). The extended lifetime  $Lh_{10}$  of the bearing A is nearly 2710 h and  $Lh_{10}$  of the bearing B is 260,000 h for the load spectrum.

The oil film thicknesses in a TRB are computed using the program TRBOFT [5] for an operation with a radial load of  $30,000~\rm N$ , an axial load of  $10,000~\rm N$ , and without bending moment on the bearing. The shaft speed is about  $4500~\rm rpm$  for this operation. The Castrol oil type BOT  $352~\rm B1$  at  $100~\rm ^{\circ}C$  with the same characteristics is applied to the computation.

The minimum oil film thicknesses on the IR and OR of the roller Z #20 are displayed over the roller length, s. Fig. 1.23. The minimum oil film thickness of 167 nm (1 nm =  $10^{-9}$  m) occurs on the IR at the position of about 4 mm left from the roller center. On the OR the minimum oil film thickness is ca. 187 nm at the same position on the left half. In this case, the corresponding dimensionless oil film thicknesses (called  $\lambda$  factor) are approximately 6.0 and 6.8 on the IR and OR, respectively (s. Fig. 1.24).

Similarly, the center oil film thicknesses on the IR and OR of the roller Z #20 are displayed over the roller length, s. Fig. 1.25. The center oil film thickness of 337 nm occurs on the IR at the position of about 4 mm left from the roller center. On the OR the center oil film thickness is ca. 373 nm at the same position on the left half part. In this case, the corresponding dimensionless oil film thicknesses (called  $\lambda$  factor) are approximately 11.9 and 13.5 on the IR and OR, respectively (s. Fig. 1.26).

<span id="page-45-0"></span>![](_page_45_Figure_2.jpeg)

**Fig. 1.24** Factor  $\lambda_{\min}$  for  $h_{\min}$  over the length of the roller Z #20

![](_page_45_Figure_4.jpeg)

Fig. 1.25 Center oil film thickness  $h_c$  over the length of the roller Z # 20

At a given the limiting voltage gradient of  $UG_{lim} = 35.5 \text{ kV/mm}$  for electro-pitting in the contact zone of the bearing, the maximum applied voltage of PWM signal (Pulse Width Modulation) results from the minimum oil film thickness  $h_{min} = 167 \text{ nm}$  (1 nm =  $10^{-9} \text{ m}$ ) on the IR in nearly 6 VAC. Thus, the

<span id="page-46-0"></span>![](_page_46_Figure_2.jpeg)

**Fig. 1.26** Factor  $\lambda_c$  for  $h_c$  over the length of the roller Z #20

electro-pitting could occur in the Hertzian contact zone at  $h_{min} = 167$  nm if the PWM voltage is higher than 6 VAC.

$$\frac{U_{PWM}}{h_{\min}} \le UG_{\lim}$$

$$\Rightarrow U_{PWM} \le UG_{\lim} \cdot h_{\min} \approx 6 \, VAC$$

### References

- Fritz, F.: Modellierung von Wälzlagern als generische Maschinenelemente einer Mehrkörpersimulation (in German). KIT Scientific Publishing, Karlsruhe (2011)
- Technical Specification ISO/TS 16281:2008(E).: Rolling bearings—methods for calculating the modified reference rating life for universally loaded bearings. ISO (2008)
- 3. Nguyen-Schäfer, H.: Computational Design of Rolling Bearings Springer International Publishing, Switzerland (2016)
- Hamrock, B., Schmid, S.R., Jacobson, B.O.: Fundamentals of Fluid Film Lubrication, 2nd edn. Marcel Dekker Inc., New York-Basel (2004)
- Nguyen-Schäfer, H.: Program TRBOFT to Compute the Oil-Film Thickness of Tapered Roller Bearings. Internal MATLAB code (2018)
- Harris, T.A., Kotzalas, M.N.: Essential Concepts of Bearing Technology, 5th edn. CRC Taylor & Francis Inc., Boca Raton (2006)
- Harris, T.A., Kotzalas, M.N.: Advanced Concepts of Bearing Technology, 5th edn. CRC Taylor & Francis Inc., Boca Raton (2006)

<span id="page-47-0"></span>References 39

8. Schaeffler: Wälzlagerpraxis (in German), 4. Auflage, Vereinigte Fachverlage GmbH, Mainz (2015)

- 9. DIN-Taschenbuch 24: DIN/ISO 281 Wälzlager 1 (in German), 9. Auflage, Verlag Beuth (2012)
- 10. Nguyen-Schäfer, H.: Program TRBLHF for Computing Lifetime and Friction of Tapered Roller Bearings. Internal code in MATLAB (2018)
- 11. Nguyen-Schäfer, H.: Program COMTRB for Computing Tapered Roller Bearings. Internal code in MATLAB (2018)

### <span id="page-48-0"></span>Chapter 2 Cylinder Roller Bearings

![](_page_48_Picture_1.jpeg)

Cylinder roller bearings (CRB) are much simpler than tapered roller bearings (TRB). They are used under large radial loads and quite small axial loads depending on the bearing type at moderate shaft speeds and heavy-duty operations. The induced friction of these bearings is much less than the friction of TRB. Many applications of these bearings are found in transmission systems for the automotive industry, multi-rolling mills, continuous casters, slurry and mud pumps, construction and mining industries (e.g. long-wall shears, front-end loaders, vibratory compactors, and coal pulverized wheels), and wind turbines.

### 2.1 Geometry of Cylinder Roller Bearings

Figure [2.1](#page-49-0) shows the main components of a CRB that are the inner and outer races, cylindrical rolling elements, bearing cage, and lubricant (oil or grease). The rolling elements are kept in the bearing cage that locates between the inner race (IR) and the outer race (OR). The IR is mounted on the rotor shaft and the OR on the bearing housing.

Depending on types of CRB, a relatively small axial load could act on the bearing in both axial directions. The bearing type shown in Fig. [2.1](#page-49-0) enables only the axial load on the IR from right to left.

The maximum misaligned angle between the races is normally about 3.5 min (0.058°) for the bearing width series 0–1 and nearly 1.5 min (0.025°) for the bearing width series 2. If the misaligned angle exceeds this limit, the rolling elements contact the IR and OR at their kerbs. It causes a huge adhesive or abrasive friction in the bearing and finally leads to a total failure of the bearing due to seizing at the kerbs.

<span id="page-49-0"></span>![](_page_49_Picture_2.jpeg)

Fig. 2.1 Components of a cylinder roller bearing (CRB)

### 2.2 Setup of Cylinder Roller Bearings

In some gearbox applications at a moderate rotational speed, a combination of cylinder roller and ball bearings is used where a large radial load on one bearing is required. In some cases, the radial load is too large so that ball bearings cannot operate with it, but cylinder roller bearings are very suitable for this case. The ball bearing A is setup as a fixed bearing that could take both radial and axial loads. On the contrary, the cylinder roller bearing B is used as a loose bearing that can support a much larger radial load without any thrust load or a relatively small axial load. In this case, the cylinder roller bearing B is applied to the larger radial load in case the acting loads on the gear 1 (or the park-locking wheel) are nearest to the bearing B with its center distance b < a, as shown in Fig. 2.2.

![](_page_49_Picture_6.jpeg)

Fig. 2.2 Loads on the bearings in a driven shaft of the gearbox

<span id="page-50-0"></span>To reduce the bearing friction in two single tapered roller bearings, the combination of ball and cylinder roller bearings is very appropriate to a driven shaft at higher torque in the gearbox where the driven shaft rotational speed  $\omega_1$  is quite small to moderate compared to the driving rotational speed  $\omega_2$ .

The axial load  $\vec{F}_A$  on the gear 1 is perpendicular to the radial load  $\vec{F}_R$  and the tangential load  $\vec{F}_T$  and is parallel to the product vector  $\vec{F}_R \times \vec{F}_T$ . However, the loads on the gear 1 do not lie on the axis of the shaft. Therefore, the axial load acts a counterclockwise moment on the shaft in the positive direction of z:

$$\vec{M}_P = \vec{r}_{pw} \times \vec{F}_A \Rightarrow M_{Pz} = r_{pw} \cdot F_A$$

in which  $r_{pw}$  is the pitch radius of the gear 1.

Using the balance of loads and moments on the shaft, one obtains after a few calculations the loads on the bearings A and B, cf. Chap. 3:

$$\vec{F}_A = \begin{bmatrix} F_{xA} = F_A \\ F_{yA} = \frac{b}{l} F_R + \frac{r_{pw}}{l} F_A \\ F_{zA} = -\frac{b}{l} F_T \end{bmatrix}; \quad \vec{F}_B = \begin{bmatrix} F_{xB} = 0 \\ F_{yB} = \frac{a}{l} F_R - \frac{r_{pw}}{l} F_A \\ F_{zB} = -\frac{a}{l} F_T \end{bmatrix}$$

The radial load on the bearing A results from both load components in the directions y and z and its axial load in the direction x is calculated as

$$F_{rA} = \sqrt{\left(\frac{b}{l}F_R + \frac{r_{pw}}{l}F_A\right)^2 + \left(\frac{b}{l}F_T\right)^2};$$
  
 $F_{aA} = F_A$ 

Similarly, the radial load on the bearing B results from both load components in the directions y and z and its axial load in the direction x is computed as

$$F_{rB} = \sqrt{\left(\frac{a}{l}F_R - \frac{r_{pw}}{l}F_A\right)^2 + \left(\frac{a}{l}F_T\right)^2};$$
  
 $F_{aB} = 0$ 

### 2.3 Computational Model of Cylinder Roller Bearings

Cylinder roller bearings have Z rollers (rolling elements RE); each of them is divided into  $n_s$  circular slices along its length  $L_{Re}$  with an equal thickness  $\Delta x_k$ . The radial load  $F_r$ , axial load  $F_a$ , and bending moment  $M_b$  acting on the bearing (s. Fig. 2.3) are calculated from the torque on the rotor, cf. Chap. 3.

The inner race (IR) has two degrees of freedom (DOF) in which one DOF is caused by the bending moment. Additionally, each rolling element (RE) has also

<span id="page-51-0"></span>![](_page_51_Picture_2.jpeg)

Fig. 2.3 Loads and bending moment on a cylinder roller bearing

![](_page_51_Picture_4.jpeg)

Fig. 2.4 DOF of a cylinder roller bearing

two DOF. In total, there are (2Z + 2) DOF for the computation of the cylinder roller bearing (s. Fig. 2.4).

At first, the radial load  $F_r$  acting on the bearing causes different loads on each rolling element in radial direction. Similarly, the axial load  $F_a$  on the bearing generates different loads on the rollers in axial direction.

The radial load  $F_r$  distributes normal loads  $Q_{ji}$  and  $Q_{jo}$  at the IR- and OR-contact surface of each roller #j (j=1-Z). The normal loads result from the elastic deformations  $\delta_{ki}$  and  $\delta_{ko}$  of the slice k at the contact zones. Additionally, the reaction force  $Q_f$  at the bearing kerb is computed from the deformation  $\delta_{fj}$  at the contact area between the IR and the bearing kerb.

<span id="page-52-0"></span>In case of a stationary outer race (OR) in the bearing housing, the IR has two unknowns DOF  $\delta_{r,IR}$  in radial direction and  $\theta_b$  in the direction z, which are fixed to the inertial coordinate system (x, y, z) in Fig. 2.4.

Similarly, each rolling element #j has also two unknowns DOF  $\delta_{yj}$  in radial direction and the tilting angle  $\psi_j$  in the direction z, which are also fixed to the RE #j. In addition, the bending angle  $\theta_b$  between the IR and OR is caused from the bending moment.

To consider the local Hertzian pressures over the length of rolling elements at the tilting position. These DOF cause the elastic deformations  $\delta_{ki}$ ,  $\delta_{ko}$ , and  $\delta_{fj}$  at the contact areas between the rolling element *RE* #*j* and the *IR*, *OR*, and bearing kerb, respectively.

The elastic deformation  $\delta_{ko}$  of the rolling element #j at the contact zone of the stationary OR is calculated from the DOF of the rolling element in the direction y as

$$\delta_{ko} + \delta y_j = 0 \Rightarrow \delta_{ko}(j) = -\delta y_j(j) \tag{2.1}$$

Similarly, the elastic deformation  $\delta_{ki}$  of the rolling element #j at the contact zone of rotating IR is calculated from the DOF of the IR and the rolling element in the direction y. In this case, the calculation is invariant for any position angle  $\varphi_j$  when the deformation vector  $\delta_{r,IR}$  would rotate with the angle  $\varphi_j$  in the same direction, as shown in Fig. 2.4. As a result, the new deformation  $\delta_{r,IR}$  becomes  $\delta_{r,IR}$  cos  $\varphi_j$  in the rotated radial direction.

$$-\delta_{ki} + \delta y_j + \delta_{r,lR} \cos \varphi_j = 0$$
  

$$\Rightarrow \delta_{ki}(j) = \delta y_j(j) + \delta_{r,lR} \cos \varphi_j$$
(2.2)

The axial load at the bearing kerb on the OR of the RE #j is written as

$$Q_{fj}(j) = C_L' \delta_{fj}^{10/9}(j) \tag{2.3}$$

in which the contact stiffness coefficient  $C_L' = 3.5948 \times 10^4 L_{Re}^{8/9}$  in N/mm<sup>10/9</sup> is used in case of one-side deformation of the *OR* at the bearing kerb.

Thus, the axial deformation on the OR of the RE #j results as

$$\delta_{ff}(j) = \left(\frac{Q_{ff}(j)}{C_L'}\right)^{9/10} \ge 0$$
 (2.4)

The axial deformation  $\delta_{xj}$  of the RE #j at the tilting position is calculated as

$$\delta x_j(j) = -\delta_{fj}(j) \cos \alpha_m^*; \alpha_m^*(j) = \psi_j(j)$$
 (2.5)

<span id="page-53-0"></span>The kerb loads in axial direction are assumed to be proportional to their corresponding normal loads on the RE. Using the balance of loads in axial direction on the OR of the RE #j, one obtains the following relation of loads, s. Fig. 2.3.

$$F_a = \sum_{j=1}^Z Q_{fj}(j) = F_a \sum_{j=1}^Z \frac{Q_{jo}(j)\cos\varphi_j}{F_r} \equiv F_a \sum_{j=1}^Z \xi_j(j)$$

where the proportional factor  $\xi_i$  for the RE #j is defined by

$$\xi_j(j) \equiv \frac{Q_{jo}(j)\cos\varphi_j}{F_r} \Rightarrow \sum_{j=1}^{Z} \xi_j(j) = 1$$

in which  $Q_{jo}$  is the normal load on the OR of the RE # j and  $\varphi_j = 2\pi j/Z$ . The axial deformation on the kerb of the RE # j results from Eq. (2.4) as

$$\delta_{fj}(j) = \max \left[ 0, \left( \frac{F_a}{C_L'} \cdot \xi_j(j) \right)^{9/10} \right] \ge 0$$
 (2.6)

Note that the axial deformation of the kerb must be positive or zero; otherwise, the acting load on it is not generated because there is no reaction without deformation.

Using Eq. (2.3), the kerb load on the RE #j is calculated as

$$Q_{fj}(j) = C'_L \delta_{fj}^{10/9}(j)$$

In the following sections, the relating equations of DOF are generated from the balance of loads and moments acting on the Z rolling elements. In case of a stationary OR, the load balances in the directions y and x are used in the computation.

Firstly, two equations for the DOF  $\delta_{r,IR}$  and  $\theta_b$  result from the load balance of  $F_r$  and  $M_b$ , respectively (s. Fig. 2.4). Note that the DOF  $\theta_b$  for bending angle is related to the displacement  $\delta_{kM}$  that is caused by the bending moment  $M_b$  for an optional computation.

The sum of the normal loads on the slices at the contact area between the OR and rolling elements in the direction y equals the radial load  $F_r$  acting on the bearing. Thus, the nonlinear equation relating to  $\delta_{r,IR}$  is written as, cf. Fig. 2.4

$$F_r - \frac{C_L}{n_s} \sum_{i=1}^{Z} \sum_{k=1}^{n_s} \hat{\delta}_{ko}^{10/9} f_k(k) \cos \varphi_j = 0$$
 (2.7)

<span id="page-54-0"></span>where

 $C_L = 2^{10/9} \times C_L' = 7.765 \times 10^4 L_{Re}^{8/9}$  in N/mm<sup>10/9</sup> is the contact stiffness coefficient for two-side deformation on the *IR* and *OR* of the *RE*, cf. Eqs. (2.8a–2.8b);

 $\widehat{\delta}_{ko}$  is the modified deformation on the slice k of the OR of the RE #j;

 $f_k$  is the Reusner's correction factor of the load on the slice k of the RE # j. The Reusner's correction factor is used to modify the local load distribution along the length  $L_{Re}$  of the RE. The factor  $f_k$  relating to each slice k is computed as [1], as shown in Fig. 2.5.

$$f_k(k) = 1 - \frac{10^{-2}}{\ln\left[1.985 \times \left\|\frac{k - n_{S,12}}{n_S - 1}\right\|\right]}, \forall k = 1, \dots, n_s$$

The parameter  $n_{S,12}$  is defined for an odd number  $n_S$  of the slices per RE as

$$n_{S,12} \equiv \frac{n_S + 1}{2}$$

Using the contour profile  $P(x_k)$  of the contact area, the modified deformation on the slice k is calculated under the influence of the tilting angle  $\psi_j$  and the position angle  $\varphi_j$  of the RE # j.

![](_page_54_Figure_10.jpeg)

**Fig. 2.5** Reusner's correction factor  $f_k$  along the roller length

$$\widehat{\delta}_{ko}(k,j) = (\delta_{ko} - e_r/2) - 2P(x_k) - x_k \cos \varphi_j \tan \psi_j;$$

$$\widehat{\delta}_{ko}(k,j) = \max \left(0, \widehat{\delta}_{ko}(k,j)\right) \ge 0$$

where  $e_r$  is the diametral bearing clearance. Note that the modified deformation on the slice k must be positive or zero; otherwise, the acting load on it is not generated because there is no reaction without deformation.

The contour profile is computed according to ISO/TS 16281:2008 (E) for cylinder roller bearings with a roller diameter  $D_m$  as [2], as shown in Fig. 2.6.

$$P(x_k) = -3.5 \times 10^{-4} D_m \ln \left[ 1 - \left( \frac{2x_k}{L_{Re}} \right)^2 \right]$$

The distance  $x_k$  from the RE center  $O_{Re}$  to the slice center is calculated as

$$x_k(k) = (k - n_{S.12}) \Delta x_k$$

where all slices have the constant thickness

$$\Delta x_k = \frac{L_{\text{Re}}}{n_{\text{S}}}$$

The bending moment  $M_b$  on the bearing in the direction z acts different moments  $M_{bj}$  on each rolling element. However, the sum of all moments on the rolling

![](_page_55_Figure_11.jpeg)

**Fig. 2.6** Contour profile function  $P(x_k)$  along the roller length of CRB

<span id="page-56-0"></span>elements in the direction z is equal to the given bending moment. As a result, the nonlinear equation relating to  $\delta_{kM}$  relating to the DOF  $\theta_b$  is written as, cf. Fig. 2.7

$$M_b - \frac{C_L'}{n_s} \sum_{j=1}^{Z} \sum_{k=1}^{n_s} l_{kM} \hat{\delta}_{kM}^{10/9} f_k(k) \cos \varphi_j = 0$$
 (2.8a)

The bending moment on each rolling element RE # j in the direction z results from Eq. (2.8a) as

$$M_{bj}(j) = \frac{C_L'}{n_s} \sum_{k=1}^{n_s} l_{kM} \hat{\delta}_{kM}^{10/9} f_k(k) \cos \varphi_j$$
 (2.8b)

In Eqs. (2.8a–2.8b) the contact stiffness coefficient  $C_L' = 3.5948 \times 10^4 L_{Re}^{8/9}$  in N/mm<sup>10/9</sup> is used in case of one-side deformation. The modified bending deformation on the slice k of the RE # j is computed as

$$\begin{split} \widehat{\delta}_{kM}(k,j) &= (\delta_{kM}\cos\varphi_j - e_r/2) - 2P(x_k) + x_k\cos\varphi_j\tan\psi_j; \\ \widehat{\delta}_{kM}(k,j) &= \max\left(0, \widehat{\delta}_{kM}(k,j)\right) \geq 0 \end{split}$$

where the modified bending deformation on the slice k must be positive or zero; otherwise, the acting load on it is not generated because there is no reaction without deformation.

Note that the moment arm  $l_{kM}$  at  $M_b \ge 0$  (or  $M_b < 0$ ) is defined as the distance that is from the right-end  $O_{REC}$  (or left-end  $O_{LEC}$ ) center of the RE to the load on the slice k of the IR, as shown in Fig. 2.7.

$$l_{kM} = + \left[ \frac{1}{2} L_{Re} - x_k(k) \right] : M_b \ge 0;$$
  
$$l_{kM} = -\left[ \frac{1}{2} L_{Re} + x_k(k) \right] : M_b < 0$$

**Fig. 2.7** Moment arms of normal loads on the slice k

![](_page_56_Picture_12.jpeg)

<span id="page-57-0"></span>The balance of loads on the RE #j in the normal direction results as, s. Fig. 2.8

$$Q_{jo}(j) - Q_{ji}(j) - F_c = 0 (2.9)$$

The normal load on the OR of the RE #j is computed as

$$Q_{jo}(j) = \sum_{k=1}^{n_S} Q_{ko}(k,j) = \frac{C_L}{n_S} \sum_{k=1}^{n_S} \hat{\delta}_{ko}^{10/9} f_k(k)$$
 (2.10)

In Eq. (2.10) the load on the slice k is

$$Q_{ko}(k,j) = \frac{C_L}{n_s} \widehat{\delta}_{ko}^{10/9} f_k(k)$$

Analogously, the normal load on the IR of the RE #j is computed as

$$Q_{ji}(j) = \sum_{k=1}^{n_S} Q_{ki}(k,j) = \frac{C_L}{n_S} \sum_{k=1}^{n_S} \hat{\delta}_{ki}^{10/9} f_k(k)$$
 (2.11)

in which the load on the slice k is

$$Q_{ki}(k,j) = \frac{C_L}{n_s} \widehat{\delta}_{ki}^{10/9} f_k(k)$$

The modified deformation on the slice k of the profile on the IR is calculated as

$$\widehat{\delta}_{ki}(k,j) = (\delta_{ki} - e_r/2) - 2P(x_k) + x_k \cos \varphi_j \tan \psi_j;$$

$$\widehat{\delta}_{ki}(k,j) = \max \left(0, \widehat{\delta}_{ki}(k,j)\right) \ge 0$$

**Fig. 2.8** Loads acting on the RE # j in both directions x and y

![](_page_57_Picture_15.jpeg)

Note that the modified deformation on the slice k must be positive or zero; otherwise, the acting load on it is not generated because there is no reaction without deformation.

Substituting Eqs. (2.10–2.12) into (Eq. 2.9), one obtains the set of Z nonlinear equations for the DOF  $\delta_{vj}$ 

$$\frac{C_L}{n_S} \left( \sum_{k=1}^{n_S} \hat{\delta}_{ko}^{10/9} f_k(k) - \sum_{k=1}^{n_S} \hat{\delta}_{ki}^{10/9} f_k(k) \right) - F_c = 0, \quad \forall j = 1, ..., Z$$
 (2.12)

The centrifugal force  $F_c$  (N) on the steel RE is computed as

$$F_c = \frac{1}{2} m_{\text{Re}} D_{pw} \omega_R^2$$

$$\approx 3.39 \times 10^{-11} D_m^2 L_{\text{Re}} D_{pw} N_R^2$$

Similarly, the set of Z nonlinear equations for the DOF  $\psi_j$  results the balance of moments of the RE #j in the direction z, s. Fig. 2.9:

$$\left(-\sum_{k=1}^{n_{S,12}} l_{kL}Q_{ko} + \sum_{k\geq n_{S,12}}^{n_{S}} l_{kR}Q_{ko}\right) \cos \varphi_{j} + \left(\sum_{k=1}^{n_{S,12}} l_{kL}Q_{ki} - \sum_{k\geq n_{S,12}}^{n_{S}} l_{kR}Q_{ki}\right) \cos \varphi_{j} + Q_{jj}(h_{1} + h_{2}) \cos \varphi_{j} + M_{bj}(j) = 0, \ \forall j = 1, \dots, Z$$
(2.13)

In summary, one obtains a nonlinear equation system of (2Z + 2) equations written in Eqs. (2.7-2.8a, 2.8b and 2.12-2.13). That describes a computing model for cylinder roller bearings under the normal loads, kerb load, and bending moment. This computing model enables computations of the loads acting on the rolling elements and their slices at the contact areas on the *IR* and *OR*. To solve the DOF of

**Fig. 2.9** Moment arm of loads on the *IR* and *OR* 

![](_page_58_Picture_11.jpeg)

the computing model with a large number of strongly nonlinear equations, the Levenberg and Marquardt solving algorithm based on Least Squares Method (LSM) is applied to the computation, cf. Chap. [6](#page-100-0).

This LSM is adapted to the solver fsolve in MATLAB® as follows:

```
%=============================================================== 
% Solving the equation system of (2*Z+2) unknowns X(i) 
%=============================================================== 
Function NLES_CRB % NonLinear Equation Systems for CRB 
%
% Input for iteration
iter1 = 1000; 
iter2 = 1000; 
epsX = 1E-6; 
%
% Input for initial values of unknowns
dr_IR0 = 0.05; % mm 
del_M0 = 0.05; % mm 
del_yj0 = -0.1; % mm 
psi_0min = 0.1; % min 
%
% Initial values for unknowns X(i), i = 1,...,(2*Z+2): 
X0(1) = dr_IR0; 
X0(2) = del_M0; 
X0(3:1:Z+2) = del_yj0; 
X0(Z+3:1:2*Z+2) = psi_0; 
% 
% @Eq_System of unknowns X(i) 
% Levenberg-Marquardt method
Options = optimoptions ('fsolve', 'Display', 'Iter-detailed',...
 'TolFun', epsX, 'TolX', epsX, 'MaxFunctionEvaluations',... 
 iter1,'MaxIterations', iter2, 'Algorithm',... 
    'levenberg-marquardt','StepTolerance', 1e-12); 
%
% Solving the @Eq_System(X)
[X,fval,exitflag,output] = fsolve(@Eq_System,X0,options); 
fval; 
exitflag; 
output; 
%
 Function [fX] = Eq_System(X) 
% Unknowns X(i) 
dr_IR = X(1); 
del_M = X(2); 
del_yj = X(3:1:Z+2); 
psi_j = X(Z+3:1:2*Z+2); 
% 
% fX = Eq_System(X) with 2*Z+2 nonlinear equations 
fX(1) = F_r - RHS_Fr; % cf. Eq. (2.7) 
fX(2) = M_b - RHS_Mb; % cf. Eq. (2.8a) 
for j = 1:1:Z 
 fX(j+2) = Q_jo(j) - Q_ji(j) - Fc % cf. Eq. (2.12) 
 fX(j+Z+2) = (RHS_OR +RHS_IR +Q_f(j)*hQf)*cos(phi_j(j))+ Mb_j(j); % cf. Eq. (2.13) 
end 
return 
end
```

### <span id="page-60-0"></span>2.4 Computing Hertzian Pressures at the Contact Zones

The individual Hertzian contact width of the slice k is calculated on the IR and OR of the RE # j, cf. Fig. 2.10.

$$b_{ki;ko}(k) = \sqrt{\frac{8Q_{ki;ko}(k)}{\pi E' \Delta x_k \sum \rho_{IR;OR}}}$$
 (2.14)

The effective elasticity module  $E^{'}$  results from the elasticity modules  $E_1$  and  $E_2$  of rollers and races, respectively.

$$E' = \frac{2}{\left(\frac{1-\nu_1^2}{E_1} + \frac{1-\nu_2^2}{E_2}\right)} \tag{2.15}$$

where  $v_1$  and  $v_2$  are the Poison's numbers of rollers and races, respectively.

The curvature sums of the inner and outer races are computed as [3]

$$\sum \rho_{IR} = \frac{2}{D_m} \left( \frac{A_i}{A_i - 1} \right); \sum \rho_{OR} = \frac{2}{D_m} \left( \frac{A_o}{A_o + 1} \right)$$

in which the auxiliary parameters are defined and calculated as

$$A_i = A_o = \frac{D_{pw}}{D_m \cos \alpha_0}; \alpha_0 = 0.$$

The Hertzian pressures  $p_{Hi}$  and  $p_{Ho}$  at the contact zone on the slice k on the IR and OR of the RE #j is computed from the contact widths in Eq. (2.14) as

**Fig. 2.10** Hertzian contact zone on the slice k

![](_page_60_Picture_14.jpeg)

$$p_{Hi;Ho}(k) = \frac{2Q_{ki;ko}(k)}{\pi b_{ki;ko} \Delta x_k} \sqrt{1 - \left(\frac{\zeta}{b_{ki;ko}}\right)^2}$$

<span id="page-61-0"></span>Obviously, the maximum Hertzian pressure on the slice k occurs at  $\zeta = 0$ :

$$p_{Hi;Ho,\max} = \frac{2Q_{ki;ko}(k)}{\pi b_{ki;ko} \Delta x_k}$$
 (2.16)

### 2.5 Computing Oil Film Thickness in CRB

The minimum oil-film thickness  $h_{min}$  for a hard EHL regime of roller bearings at the contact area with a rectangular footprint results from solving the coupled Reynolds und elasticity equations. Next, using the least squares fit method the dimensionless minimum oil film thickness is calculated at the inner and outer raceways according to [4], cf. Fig. 2.11

$$H_{\min} \equiv \frac{h_{\min}}{R_{\zeta}} = 1.714 \times U^{*0.694} \cdot G^{*0.568} \cdot W^{*-0.128}$$
 (2.17)

in which all dimensionless parameters are defined in Sect. 1.8.

Usually, the empirical value  $5.007 \times 10^3$  is used for the dimensionless material parameter  $G^*$  in the roller bearings.

To analyze the influence parameters on the minimum oil-film thickness  $h_{min}$  that is rewritten in

**Fig. 2.11** Oil-film thicknesses in a roller bearing

![](_page_61_Figure_12.jpeg)

$$h_{\min} = \frac{1.806 \times (\mu_0 U)^{0.694} \cdot \alpha_{EHL}^{0.568} \cdot R_{\zeta}^{0.434}}{E'^{(-0.002)} \cdot \left(\frac{W}{L_{Rc}}\right)^{0.128}}$$
(2.18)

<span id="page-62-0"></span>Equation (2.18) shows that the parameters  $\mu_0$ , U,  $\alpha_{EHL}$ , and  $R_{\zeta}$  have a strong influence on the minimum oil-film thickness. On the contrary, the bearing load W has a slight influence on it since its exponent is very small compared to the other exponents. Furthermore, the minimum oil-film thickness is quasi-independent of the effective modulus of elasticity E' because its exponent nearly equals zero.

The center oil-film thickness  $h_c$  for a hard EHL regime at a rectangular contact area is semi-empirically calculated at the inner and outer raceways according to [4] as

$$H_c \equiv \frac{h_c}{R_{\zeta}} = 2.922 \times U^{*0.692} \cdot G^{*0.470} \cdot W^{*-0.166}$$
 (2.19)

Similarly, to analyze the influence parameters on the center oil-film thickness  $h_c$ , one rewrites Eq. (2.19) in

$$h_c = \frac{2.922 \times (\mu_0 U)^{0.692} \cdot \alpha_{EHL}^{0.47} \cdot R_{\zeta}^{0.474}}{E'^{0.056} \cdot \left(\frac{W}{L_{Re}}\right)^{0.166}}$$
(2.20)

Equation (2.20) shows that the parameters  $\mu_0$ , U,  $\alpha_{EHL}$ , and  $R_{\zeta}$  have strong influences on the center oil-film thickness. On the contrary, the bearing load W has a slight influence on it since its exponent is very small compared to the other exponents. Furthermore, the center oil-film thickness is nearly independent of the reduced elastic modulus E' because its exponent is very small.

### 2.6 Computing Bearing Friction in CRB

The total friction torque acting on the bearing is caused by the bearing loads, viscous friction of oil, and kerb friction of the bearing.

$$M_f = M_l + M_v + M_a (2.21)$$

where

 $M_l$  is the load torque that is caused by radial and thrust loads acting on the bearing;  $M_v$  is the viscous torque that is caused by the oil viscous friction in the bearing;  $M_a$  is the kerb torque that is caused by the friction between the bearing kerb and rolling elements.

The load torque  $M_l$  (N × mm) of bearings is computed using the empirical formula of Palmgren [6, 7] as

$$M_l = f_1 P_m D_{pw}; f_1 = 2.5 \times 10^{-4} \text{ to } 3 \times 10^{-4}$$
 (2.22)

<span id="page-63-0"></span>in which  $D_{pw}$  (mm) is the pitch diameter of the bearing.

The dynamic equivalent load  $P_m$  in the bearing results from the radial and axial load:

$$\begin{aligned} \frac{F_a}{F_r} &\leq e : P_m = F_r; \\ \frac{F_a}{F_r} &> e : P_m = 0.92F_r + Y \cdot F_a \end{aligned}$$

The parameters e and Y depend on the bearing types such as [8]

e = 0.20; Y = 0.60 for NJ2, NJ3, NJ4, NUP2, NUP3;

e = 0.30; Y = 0.40 for NJ22, NJ23, NUP22, NUP23;

e = 0.24; Y = 0.50 for other types.

The viscous torque  $M_{\nu}$  (N  $\times$  mm) between oil and rollers is computed using the empirical formula of Palmgren [6] as

$$M_{\nu} = 160 \times 10^{-7} f_o D_{pw}^3 \text{ for } \nu N_R < 2000;$$
  
=  $10^{-7} f_o (\nu N)^{2/3} D_{pw}^3 \text{ for } \nu N_R \ge 2000$  (2.23)

where  $v \text{ (mm}^2\text{/s)}$  is the oil kinematic viscosity;  $N_R$  is the rotor speed (rpm); the factor  $f_0 = 2$  to 3 is used for all cylinder roller bearings.

The additional torque  $M_a$  (N × mm) is caused by the friction due to the resulting load  $F_a$  (N) in axial direction [8]:

$$M_a = 0.06 f_a F_a D_{pw} (2.24)$$

The factor  $f_a$  in Eq. (2.24) is experimentally calculated based on the parameter Y as

$$f_a = 0.15$$
 for  $0.5 \le Y < 1$ ;  
=  $(0.17267 - 2.267 \times 10^{-2}Y)$  for  $1 \le Y \le 7$ ;  
=  $0.14$  for  $7 < Y \le 40$ 

The dimensionless parameter Y is defined by

$$Y = f_b D_{pw} v N_R (D^2 - d^2) / F_a^2$$

where  $f_b = 0.0048$ ,  $D_{pw}$  (mm), v (mm<sup>2</sup>/s),  $N_R$  (rpm), the bore diameter d (mm), the outside diameter D (mm), and the resulting axial load  $F_a$  (N).

In total, the frictional power  $P_f(W)$  in the bearing results from Eqs. (2.22–2.24) in

$$P_f = M_f \omega_R$$
  
=  $(M_l + M_v + M_a) \times \left(\frac{2\pi N_R}{60}\right) \times 10^{-3}$  (2.25)

### <span id="page-64-0"></span>2.7 Computing Lifetime of CRB

Like the tapered roller bearings as discussed in Sect. 1.10, the extended fatigue lifetime  $Lh_m(h)$  of the bearing results from the individual lifetimes  $Lh_{m,ij}(h)$  of all operating points with the individual percentages of revolution  $t_{ij}$  in the load spectrum as [3].

$$\frac{1}{Lh_m} = \sum_{i=1}^{N} \sum_{j=1}^{M} \frac{t_{ij}}{Lh_{m,ij}} \Rightarrow Lh_m = \frac{1}{\sum_{i=1}^{N} \sum_{j=1}^{M} \frac{t_{ij}}{Lh_{m,ij}}}$$
(2.26)

The individual lifetime  $Lh_{m,ij}$  (h) is calculated from its dynamic load rating  $C_r$  (N), dynamic equivalent load  $P_{m,ij}$  (N), rotor speed  $N_{R,ij}$  (rpm), and the extended lifetime factor  $a_{ISO,ij}$ , according to the DIN/ISO 281 [9]:

$$Lh_{m,ij} = \frac{10^6}{60N_{R,ij}} a_1 a_{ISO,ij} L_{10,ij}$$

$$= \frac{10^6}{60N_R} a_1 a_{ISO,ij} \left(\frac{C_r}{P_{m,ij}}\right)^p$$
(2.27)

in which the lifetime factor p = 10/3 is chosen for all roller bearings;  $a_1$  depends on the failure probability m of the bearing (e.g.  $a_1 = 1$  for m = 10%). The dynamic equivalent load  $P_m$  on the bearing is calculated from the radial and axial load [8, 11].

In case of considering the tilting rollers in the CRB, the Hertzian pressures increase at the roller ends compared to the pressures at the roller middle. Therefore, the modified lifetime for each slice of the rollers should be calculated at the given loads in the load spectrum according to the ISO/TS 16281:2008 (E) [2]. Then, the extended bearing lifetime results from Eqs. (2.26–2.27).

The dynamic load ratings on a slice k of the roller on the IR and OR are calculated as

$$q_{kci} = Q_{ci} \left(\frac{1}{n_S}\right)^{7/9} \text{ for } IR;$$

$$q_{kco} = Q_{co} \left(\frac{1}{n_S}\right)^{7/9} \text{ for } OR.$$
(2.28)

<span id="page-65-0"></span>where  $Q_{ci}$  and  $Q_{co}$  are the dynamic load ratings of the roller on the *IR* and *OR*, respectively;  $n_S$  is the number of slices per roller.

The dynamic load ratings of the roller at the IR are computed as

$$Q_{ci} = \frac{1}{\lambda v} \cdot \frac{C_r}{0.378Z \cos \alpha_m i^{7/9}} \left[ 1 + \left\{ 1.038 \left( \frac{1 - \gamma}{1 + \gamma} \right)^{143/108} \right\}^{+9/2} \right]^{2/9}$$

in which the used parameters are defined for a single row (i = 1) by

$$\gamma = \frac{D_m \cos \alpha_0}{D_{pw}}; \ \lambda v = 0.83; \ i = 1; \ \alpha_0 = 0.$$

Similarly, the dynamic load ratings of the roller at the OR are computed as

$$Q_{co} = \frac{1}{\lambda v} \cdot \frac{C_r}{0.364Z \cos \alpha_m i^{7/9}} \left[ 1 + \left\{ 1.038 \left( \frac{1 - \gamma}{1 + \gamma} \right)^{143/108} \right\}^{-9/2} \right]^{2/9}$$

The lifetimes in  $10^6$  revolutions of the roller #j for the line contact on the IR and OR result from the modified reference lifetimes of the slices of the roller in

$$L_{ir,j} = \sum_{k=1}^{n_S} \left(\frac{q_{kci}}{q_{kei}}\right)^4 \text{ for } IR;$$

$$L_{or,j} = \sum_{k=1}^{n_S} \left(\frac{q_{kco}}{q_{keo}}\right)^{4.5} \text{ for } OR.$$

$$(2.29)$$

where  $q_{kei}$  and  $q_{keo}$  are the dynamic equivalent loads on the slice k on the IR and OR, respectively.

The Reusner's factors are used to compute the increase of normal stresses along the roller length on the *IR* and *OR*:

$$f_{i}(j,k) = \left[ \left( \frac{p_{Hi}}{271} \right)^{2} D_{k}(k) \cdot (1-\gamma) \cdot \frac{L_{\text{Re}}}{n_{S}} \right] \times \frac{1}{Q_{ki}(j,k)};$$

$$f_{o}(j,k) = \left[ \left( \frac{p_{Ho}}{271} \right)^{2} D_{k}(k) \cdot (1-\gamma) \cdot \frac{L_{\text{Re}}}{n_{S}} \right] \times \frac{1}{Q_{ko}(j,k)}$$

$$(2.30)$$

where  $p_{Hi}$  and  $p_{Ho}$  are the maximum Hertzian pressures (MPa) on the slice k of the IR and OR, s. Eq. (2.16);  $Q_{ki}$  and  $Q_{ko}$  are the normal loads (N) on the slice k of the IR and OR, respectively, cf. Sect. 2.3.

The dynamic equivalent loads on the slice k based on Z rollers of the CRB are computed for the IR and OR, respectively:

<span id="page-66-0"></span>
$$q_{kei}(k) = \left(\frac{1}{Z} \sum_{j=1}^{Z} \left[ f_i(j,k) \cdot Q_{ki}(j,k) \right]^4 \right)^{1/4};$$

$$q_{keo}(k) = \left(\frac{1}{Z} \sum_{j=1}^{Z} \left[ f_o(j,k) \cdot Q_{ko}(j,k) \right]^{4.5} \right)^{1/4.5}$$
(2.31)

Using the Palmgren and Miner's damage law for the *IR* and *OR*, the modified reference lifetime of the CRB results in, cf. App. A

$$\frac{1}{L_r^{\beta}} = \frac{1}{L_{ri}^{\beta}} + \frac{1}{L_{ro}^{\beta}} \Rightarrow L_r = \left(L_{ri}^{-\beta} + L_{ro}^{-\beta}\right)^{-1/\beta}$$
 (2.32)

in which  $\beta = 9/8$  is the Weibull's slope for roller bearings.

Substituting Eqs. (2.28-2.31) into Eq. (2.32), one obtains the modified reference lifetime of the CRB in  $10^6$  revolutions

$$L_{r}(rev) = \left[ \sum_{k=1}^{n_{S}} \left\{ \left( \frac{q_{kci}(k)}{q_{kei}(k)} \right)^{-4\beta} + \left( \frac{q_{kco}(k)}{q_{keo}(k)} \right)^{-4.5\beta} \right\} \right]^{-1/\beta}$$

$$= \left[ \sum_{k=1}^{n_{S}} \left\{ \left( \frac{q_{kci}(k)}{q_{kei}(k)} \right)^{-4.5} + \left( \frac{q_{kco}(k)}{q_{keo}(k)} \right)^{-5.0625} \right\} \right]^{-8/9}$$
(2.33)

The extended lifetime in  $10^6$  revolutions for 10% failure probability is computed as

$$L_{10,r}^{*}(rev) = a_{ISO}L_{r}(rev)$$

$$= a_{ISO} \times \left[ \sum_{k=1}^{n_{S}} \left\{ \left( \frac{q_{kci}(k)}{q_{kei}(k)} \right)^{-4.5} + \left( \frac{q_{kco}(k)}{q_{keo}(k)} \right)^{-5.0625} \right\} \right]^{-8/9}$$
(2.34)

Using the rotor speed  $N_R$  (rpm) the extended lifetime  $Lh^*_{10}$  (h) results in

$$Lh_{10}^{*}(h) = \frac{10^{6}L_{10,r}^{*}(rev)}{60N_{R}(rpm)}$$
(2.35)

### 2.8 Computing Bearing Stiffness of CRB

The radial bearing stiffness is calculated from the radial load on the bearing and the elastic deformation of the *IR* in radial direction as

<span id="page-67-0"></span>Fig. 2.12 Computational model of the bearing stiffness

![](_page_67_Picture_3.jpeg)

$$K_{b,r} = \frac{\partial F_r}{\partial \delta_r} \approx \frac{F_r}{\delta_{r,IR}} \tag{2.36}$$

Analogously, the axial bearing stiffness is calculated from the axial load on the bearing and the elastic deformation of the *IR* in axial direction as

$$K_{b,a} = \frac{\partial F_a}{\partial \delta_a} \approx \frac{F_a}{\delta_{a,Z}}; \ \delta_{a,Z} = \delta_{ff}(Z)$$
 (2.37)

It is quite complicated to calculate the bending stiffness of the TRB. Therefore, a computing model shown in Fig. 2.12 is used to compute it. Let the bending moment  $M_b$  act on the IR in the positive direction (counterclockwise) while the OR is stationary in the bearing housing. Due to the resistance of the bearing stiffness  $K_b$ , the IR deforms at a bending angle  $\theta_b > 0$  to the stationary OR.

The bending moment  $M_b$  on the IR is the sum of the resistant moments of Z rollers in the bearing, cf. Eq. (2.8a). The resistant moment  $M'_{bj}$  of the roller #j is calculated from its bending stiffness  $K_{bj}$ , tilting angle  $\psi_j$ , and position angle  $\varphi_j$ . As a result, the bending stiffness of the roller #j results in

$$M_{b} = \sum_{j=1}^{Z} M'_{bj}(j); M'_{bj}(j) = K_{bj}(j) \cdot \psi_{j} \cos \varphi_{j}$$

$$\Rightarrow K_{bj}(j) = \left| \frac{M'_{bj}(j)}{\psi_{j} \cos \varphi_{j}} \right| \ge 0$$
(2.38)

At a small bending angle  $\theta_b$  between the *IR* and *OR*, the bending moment  $M_b$  on the *IR* is calculated from the bending stiffness  $K_{bj}$  of the roller #j as

$$M_b = \sum_{j=1}^{Z} M_{bj}(j) \Leftrightarrow K_{b,M} \cdot \theta_b = \sum_{j=1}^{Z} K_{bj}(j) \cdot \theta_b$$
 (2.39)

<span id="page-68-0"></span>Dividing both sides of Eq. (2.39) by  $\theta_b$ , one obtains the bending stiffness of the bearing from Eq. (2.38)

$$K_{b,M} = \sum_{j=1}^{Z} K_{bj}(j)$$

$$\Rightarrow K_{b,M} = \sum_{j=1}^{Z} \left| \frac{M'_{bj}(j)}{\psi_{j} \cos \varphi_{j}} \right| \ge 0$$
(2.40)

According to Eq. (2.40), the bending stiffness  $K_{b,M}$  is the sum of the bending stiffnesses of Z rollers. It is like the total electrical conductance (i.e. the reciprocal to resistance) of the parallel circuit of resistors, as shown in Fig. 2.12.

The bending angle of the bearing results from Eq. (2.39) in

$$\theta_b = \frac{M_b}{K_{bM}} \tag{2.41}$$

### 2.9 An Example for Computational CRB

In the following section, an example to compute a CRB of type FAG NJ207-E-XL-TVP2 with 15 rolling elements. Each roller with a roller diameter of 10 mm and a length of about 9 mm is divided into 65 circular slices.

Let a radial load of 20,000 N, an axial load of 5000 N, and a bending moment of 30 Nm act on the bearing. Using the program COMCRB [10], the local Hertzian pressures on the slices at the contact zones of the *IR* and *OR* are computed. To compute the oil film thicknesses the program CRBOFT [5] is applied.

Input data for MATLAB

Case = 'CRB Type FAG NJ207-E-XL-TVP2';

 $F_r = 20000$ ; % radial load on TRB (N) – >  $F_r$  must be unequal zero.

 $F_a = 5000$ ; % axial load on TRB (N)

 $M_b = 30$ ; % bending moment on TRB (Nm)

F cent = 1; % = 0: without; = 1 with centrifugal force on RE;

 $N_br = 5000$ ; % shaft rotational speed (rpm) Z = 15; % number of bearing rollers

ns\_odd = 65; % odd number of slice per RE (ns\_odd = 33)

e\_r = 0E-3; % bearing clearance (mm) L\_re = 9.0; % effective length of RE (mm) Dw = 10; % diameter of the RE (mm)

```
Dpw = 53.5:
               % mean pitch diameter of bearing (mm)
               % bearing geometry, s. catalog (mm)
d 1 = 48:
D 1 = 61:
               % bearing geometry, s. catalog (mm)
               % 0: no Reusner correction; 1: with Reunsner correction
i cor = 1;
ifav = 1;
               % favorite roller (ifav = 1, ..., Z)
   %
   % Material data of RE
Em 1 = 208E3; % elasticity modulus of rolling element (RE) (MPa)
Em 2 = 208E3; % elasticity modulus of races (MPa)
nu 1 = 0.3;
                 % Poisson ratio of RE (-)
nu 2 = 0.3;
                 % Poisson ratio of races (-)
   %
   % Iteration
iter1 = 5000:
              % maximum iteration steps
iter2 = 5000; % maximum iteration steps
epsX = 1E-6; % convergence tolerance
   %
   % Initial values for unknowns
dr IR0 = 0.05:
                   % initial radial deformation of dr IR > 0 (mm)
del M0 = 0.05;
                   % initial radial deformation of del M > 0 (mm)
del vi0 = -0.10;
                   % initial deformation of del_y = 0 \text{ (mm)}
psi 0 min = 0.10; % initial tilting angle of RE (min)
```

Due to strong nonlinearity of the equation system describing the CRB, the initial values of the unknowns should be appropriately chosen; otherwise, no converged solution is reached for this problem. The unknowns are iteratively solved beginning with the initial values. The convergence of solutions is reached if the norm of their residuals is less than the given tolerance eps $X = 10^{-6}$ . Additionally, the limits iter1 and iter2 of iteration steps and maximal evaluations are given at 5000 for each limit. Using the program *fsolve* in MATLAB optimized with the Levenberg-Marquardt solving algorithm, the solutions are converged after 10 iteration steps and about 700 function evaluations. The convergence reaches a tolerance of  $6.6 \times 10^{-17} < \text{eps}X$ .

Figure 2.13 shows the distribution of the bending moment of 30 Nm on the rollers of the bearing in the direction z. The result indicates that the maximum bending moment of 8.36 Nm acts on the roller Z # 15, which is the opposite roller to the radial load (s. Fig. 2.3). The individual bending moment on the roller in the direction z is computed from Eq. (2.8b) and affects the distribution of the normal loads on the IR and OR of the RE by the balance of moments.

The normal loads acting on the IR and OR of each RE result from Eqs. (1.15 and 1.16). They are shown in Figs. 2.14 and 2.15, respectively. The maximum loads on the IR and OR of nearly 5746 N and 5787 N, respectively also occur on the roller Z #15. The discrepancy between these loads is exactly the centrifugal force on the

<span id="page-70-0"></span>![](_page_70_Figure_2.jpeg)

Fig. 2.13 Distribution of the bending moment on the rollers

![](_page_70_Figure_4.jpeg)

Fig. 2.14 Distribution of normal loads in the IR of the rollers

RE of 41 N at 5000 rpm. As a result, the maximum Hertzian pressures on the IR and OR of each RE are caused by the maximum loads on the roller Z #15. The normal loads at the bearing kerb (called kerb load) on the rollers are computed from Eq. ([2.3](#page-52-0)) and are displayed in Fig. [2.16](#page-71-0). The maximum kerb load of 1447 N is on the roller Z #15.

<span id="page-71-0"></span>![](_page_71_Figure_2.jpeg)

Fig. 2.15 Distribution of normal loads in the OR of the rollers

![](_page_71_Figure_4.jpeg)

Fig. 2.16 Distribution of loads at the bearing kerb of the rollers

All results show that the roller 4 to 11 are not loaded with the normal load, kerb load, and bending moment. The maximum loads and bending moment concentrate on the roller Z #15.

The normal loads of 5746 N and 5787 N on the IR and OR of the roller Z #15 are distributed in nS = 65 slices with a slice thickness of about 0.138 mm over the roller length, as shown in Fig. 2.17. The maximum load of about 152 N occurs on the IR at 3.2 mm; and 153 N on the OR at −3.1 mm from the roller center (xk = 0). Due to the rounded profile of the RE, no load occurs at the left end of the IR and the right end of the OR. Due to the counterclockwise bending moment of the IR, the roller Z #15 is misaligned in the positive direction. As a result, the load increases to about 140 N at the right end of the IR and the left end of the OR.

Using Eq. [\(2.14\)](#page-60-0) the widths bk at the EHD contact zone between the IR and OR and the RE of Z #15 are computed over the roller length. The results in Fig. [2.18](#page-73-0) show the maximum contact widths of nearly 0.223 mm (IR) and 0.270 mm (OR) occur at the positions of ca. +3.3 mm and −3.2 mm from the RE center, respectively.

From the normal loads and the contact widths result the maximum Hertzian pressures on the IR and OR of the roller Z #15 according to Eq. [\(2.16](#page-61-0)). The maximum Hertzian pressure is about 3.14 GPa on the IR; 2.60 GPa on the OR at the positions of ca. +3.2 mm and −3.3 mm from the RE center, respectively, as shown in Fig. [2.19](#page-73-0).

The computational result shows that the tilting angle w<sup>j</sup> of the roller Z #15 is about 13.5 min in counterclockwise. As a result, it causes an increase of the Hertzian pressure on the IR to about 3 GPa at the rounded right end and no contact

![](_page_72_Figure_7.jpeg)

Fig. 2.17 Normal loads over the length on the IR and OR of the roller Z #15

<span id="page-73-0"></span>![](_page_73_Figure_2.jpeg)

Fig. 2.18 Contact widths over the length of the roller Z #15

![](_page_73_Figure_4.jpeg)

Fig. 2.19 Hertzian pressures over the length of the roller Z #15

![](_page_74_Figure_2.jpeg)

Fig. 2.20 Load spectrum of an intermediate gear shaft with a CRB

(i.e.  $p_{Hi} = 0$ ) at the rounded left end of the roller. On the contrary, the Hertzian pressure on the OR of approximately 2.5 GPa is at the rounded left end and no contact (i.e.  $p_{Ho} = 0$ ) at the rounded right end of the roller, cf. Fig. 2.19.

The computational results give that the radial bearing stiffness  $K_{b,r}$  is about  $5.33 \times 10^5$  N/mm, the axial bearing stiffness  $K_{b,a}$  is about  $5.23 \times 10^5$  N/mm, and the bending stiffness  $K_{b,M}$  is about  $9.7 \times 10^3$  Nm/rad. From Eq. (2.40) the bending angle  $\theta_b$  between the *IR* and *OR* results in ca. 10.6 min.

In the following section, the bearing friction in the cylinder roller bearing is computed with the load spectrum of electric vehicles shown in Fig. 2.20. The positive torque is for the driving operation and the negative torque for the recuperation phase. The probabilities of each event that consists of the shaft torque and speed based on the revolutions are displayed in vertical direction. Obviously, the sum of all probabilities of the events in the load spectrum must be 100%. Note that the axial load direction in the recuperation phase oppositely changes compared to the axial load in the driving operation because the contact position at the gear flange changes in the opposite side of the gear tooth. However, the driving shaft rotates in the same direction in both operations.

A cylinder roller bearing of type FAG NJ207-E-XL-TVP2 is chosen for the computation. For the bearing lubrication, Castrol oil type BOT 352 B1 with an additive is used at an oil temperature 90  $^{\circ}$ C. The cleanliness of oil quality -/15/12 according to ISO 4406 is chosen for the driving operation without filter.

The computation results of the program CRBLHF [11] show that the friction power in the bearing is about 27 W with this load spectrum. An axial preload on the bearing is not necessary; therefore, the friction in CRB is quite less than in TRB, cf.

Sect. 1.12. Additionally, the lifetimes  $Lh_{10}$  of the bearing is computed according to the DIN/ISO 281 [9], cf. Equation (2.26). The extended lifetime  $Lh_{10}$  of the bearing is nearly 2810 h for the load spectrum.

The oil film thicknesses in a CRB are computed using the program CRBOFT [5] for a driving operation only with a radial load of 20,000 N on the bearing. In this computation, a centrifugal force of the RE is about 33 N at a shaft speed of 4500 rpm. The Castrol oil type BOT 352 B1 at 100 °C with the same characteristics is applied to the computation.

The minimum oil film thicknesses on the IR and OR of the roller Z #15 are displayed over the roller length, s. Fig. 2.21. The minimum oil film thickness of 215 nm (1 nm =  $10^{-9}$  m) occurs on the IR at the roller center. On the OR the minimum oil film thickness is ca. 253 nm also at the roller center. In this case, the corresponding dimensionless oil film thicknesses (called  $\lambda$  factor) are approximately 7.7 and 9.1 on the IR and OR, respectively (s. Fig. 2.22).

Similarly, the center oil film thicknesses on the IR and OR of the roller Z #15 are displayed over the roller length, s. Fig. 2.23. The center oil film thickness of 428 nm occurs on the IR at the roller center. On the OR the center oil film thickness is ca. 512 nm also at the roller center. In this case, the corresponding dimensionless oil film thicknesses (called  $\lambda$  factor) are approximately 15.4 and 18.5 on the IR and OR, respectively (s. Fig. 2.24).

At a given the limiting voltage gradient of  $UG_{lim} = 35.5 \text{ kV/mm}$  for electro-pitting in the contact zone of the bearing, the maximum applied voltage of PWM signal (Pulse Width Modulation) results from the minimum oil film thickness

![](_page_75_Figure_7.jpeg)

Fig. 2.21 Minimum oil film thickness  $h_{min}$  over the length of the roller Z #15

<span id="page-76-0"></span>![](_page_76_Figure_2.jpeg)

Fig. 2.22 Factor kmin for hmin over the length of the roller Z #15

![](_page_76_Figure_4.jpeg)

Fig. 2.23 Center oil film thickness hc over the length of the roller Z #15

<span id="page-77-0"></span>![](_page_77_Figure_2.jpeg)

**Fig. 2.24** Factor  $\lambda_c$  for  $h_c$  over the length of the roller Z #15

 $h_{min} = 215$  nm  $(1 \text{ nm} = 10^{-9} \text{ m})$  on the *IR* in nearly 7.6 VAC. Thus, the electro-pitting could occur in the Hertzian contact zone at  $h_{min} = 215$  nm if the PWM voltage is higher than 7.6 VAC.

$$\frac{U_{PWM}}{h_{\min}} \le UG_{\lim}$$
  
 $\Rightarrow U_{PWM} \le UG_{\lim} \cdot h_{\min} \approx 7.6 \text{ VAC}$ 

### References

- Fritz, F.: Modellierung von Wälzlagern als generische Maschinenelemente einer Mehrkörpersimulation (in German). KIT Scientific Publishing, Karlsruhe (2011)
- Technical Specification ISO/TS 16281:2008(E): Rolling Bearings—Methods for calculating the modified reference rating life for universally loaded bearings. ISO (2008)
- Nguyen-Schäfer, H.: Computational Design of Rolling Bearings. Springer International Publishing, Switzerland (2016)
- Hamrock, B., Schmid, S.R., Jacobson, B.O.: Fundamentals of Fluid Film Lubrication, 2nd edn. Marcel Dekker Inc., New York-Basel (2004)
- Nguyen-Schäfer, H.: Program CRBOFT to compute the Oil-Film Thickness of cylinder roller bearings. Internal MATLAB code (2018)

<span id="page-78-0"></span>References 71

6. Harris, T.A., Kotzalas, M.N.: Essential Concepts of Bearing Technology, 5th edn. CRC Taylor & Francis Inc., Boca Raton (2006)

- 7. Harris, T.A., Kotzalas, M.N.: Advanced Concepts of Bearing Technology, 5th edn. CRC Taylor & Francis Inc., Boca Raton (2006)
- 8. Schaeffler: Wälzlagerpraxis (in German), 4. Auflage, Vereinigte Fachverlage GmbH, Mainz (2015)
- 9. DIN-Taschenbuch 24: DIN/ISO 281 Wälzlager 1 (in German), 9. Auflage, Verlag Beuth (2012)
- 10. Nguyen-Schäfer, H.: Program COMCRB for computing tapered roller bearings. Internal code in MATLAB (2018)
- 11. Nguyen-Schäfer, H.: Program CRBLHF for computing Lifetime and Friction of tapered roller bearings. Internal code in MATLAB (2018)

### <span id="page-79-0"></span>Chapter 3 Loads Acting on Gears and Bearings

![](_page_79_Picture_1.jpeg)

This chapter deals with the calculations of loads acting on helical gears and bearings used in transmission systems for the automotive industry. The gear loads depend only on the gear geometries and the driving torque on the shaft. As a result, the loads on the bearings result from the gear loads and the setup geometries. The loads on the bearings change with the operating conditions, such as forwards and backwards driving cycles, and recuperation phase.

### 3.1 Calculating Loads Acting on Gears

Most gearbox applications use the X arrangement with two single floating tapered roller bearings, as shown in Fig. [3.1](#page-80-0). At a driving torque Ms three loads FT, FR, and FA are generated at the pitch circle of the helical gears. However, the forces on the bearings, not on the gears are required for further calculations in the computational model [\[1](#page-83-0)].

Using the theory of helical gears, the tangential force FT acting on the gear 1 results from the driving torque Ms and the pitch radius rpw in

$$F_T = \frac{M_s}{r_{pw}} \tag{3.1}$$

The axial force FA acting on the gear 1 results from the tangential force FT and the helical angle b as

$$F_A = F_T \tan \beta \tag{3.2}$$

<span id="page-80-0"></span>![](_page_80_Picture_2.jpeg)

Fig. 3.1 Loads on the bearings of a driving shaft in a gearbox

The axial force  $\overrightarrow{F}_A$  is parallel to the product vector  $\overrightarrow{F}_R \times \overrightarrow{F}_T$ , s. Figs. 3.2 and 3.3.

The radial force  $F_R$  acting on the gear 1 results from the tangential force  $F_T$  and the operating pressure angle  $\alpha_t$  in

$$F_R = F_T \tan \alpha_t$$

Using trigonometrical relations in Fig. 3.3, the operating pressure angle  $\alpha_t$  of the helical gear results in

![](_page_80_Picture_8.jpeg)

Fig. 3.2 Loads acting on the driving helical gear at a rotational speed  $\omega_1$ 

<span id="page-81-0"></span>![](_page_81_Figure_2.jpeg)

Fig. 3.3 Analysis of loads on the driving helical gear at the torque  $M_s$ 

$$\tan \alpha_t = \frac{F_R}{F_T}; \tan \alpha_n = \frac{F_R}{F_m}$$
  
 $\Rightarrow \tan \alpha_t = \frac{F_m}{F_T} \tan \alpha_n$ 

The tangential force  $F_{tn}$  is written in

$$F_{tn} = \frac{F_T}{\cos \beta}$$

Substituting two above equations, one obtains

$$\tan \alpha_t = \frac{\tan \alpha_n}{\cos \beta}$$

$$\Rightarrow \alpha_t = \tan^{-1} \left( \frac{\tan \alpha_n}{\cos \beta} \right) \equiv \arctan \left( \frac{\tan \alpha_n}{\cos \beta} \right)$$

As a result, the radial force  $F_R$  is written as

$$F_R = F_T \frac{\tan \alpha_n}{\cos \beta} \tag{3.3}$$

where  $\alpha_n$  is called the normal pressure angle of the helical gear.

<span id="page-82-0"></span>The tangential force  $F_{tn}$  on the gear results as

$$\overrightarrow{F}_{tn} \equiv \overrightarrow{F}_T + \overrightarrow{F}_A \Rightarrow F_{tn} = \frac{F_T}{\cos \beta} \tag{3.4}$$

Using Eq. (3.4), the normal force  $F_{bn}$  on the gear is calculated as

$$\overrightarrow{F}_{bn} \equiv \left(\overrightarrow{F}_T + \overrightarrow{F}_A\right) + \overrightarrow{F}_R = \overrightarrow{F}_{tn} + \overrightarrow{F}_R$$

$$\Rightarrow F_{bn} = \frac{F_m}{\cos \alpha_n} = \frac{F_T}{\cos \beta \cdot \cos \alpha_n}$$
(3.5)

The radial force  $F_{bt}$  on the shaft results as

$$\overrightarrow{F}_{bt} = \overrightarrow{F}_T + \overrightarrow{F}_R \Rightarrow F_{bt} = \frac{F_T}{\cos \alpha_t}$$
 (3.6)

For a given geometry of the helical gear with the pitch radius  $r_{pw}$ , the helical angle  $\beta$ , and the normal pressure angle  $\alpha_n$  at a driving torque  $M_s$ , all forces acting on the helical gear are computed according to Eqs. (3.1–3.6). They are used for the further computation of the bearing loads [2].

### 3.2 Calculating Loads Acting on Bearings

Using the balance of loads and moment on the driving shaft of the gear 1, the acting loads on the bearings A and B in the forwards driving cycle result as, s. Fig. 3.1.

$$\overrightarrow{F}_{A} = \begin{bmatrix} F_{xA} = F_{A} \\ F_{yA} = \frac{b}{l} F_{R} + \frac{r_{pw}}{l} F_{A} \\ F_{zA} = -\frac{b}{l} F_{T} \end{bmatrix}; \quad \overrightarrow{F}_{B} = \begin{bmatrix} F_{xB} = 0 \\ F_{yB} = \frac{a}{l} F_{R} - \frac{r_{pw}}{l} F_{A} \\ F_{zB} = -\frac{a}{l} F_{T} \end{bmatrix}$$
(3.7)

The radial load on the bearing A results from both load components in the directions y and z and its axial load in the direction x is calculated as

$$F_{rA} = \sqrt{\left(\frac{b}{l}F_R + \frac{r_{pw}}{l}F_A\right)^2 + \left(\frac{b}{l}F_T\right)^2}; \quad F_{aA} = F_A$$
 (3.8)

Similarly, the radial load on the bearing B results from both load components in the directions y and z and its axial load in the direction x is computed as

$$F_{rB} = \sqrt{\left(\frac{a}{l}F_R - \frac{r_{pw}}{l}F_A\right)^2 + \left(\frac{a}{l}F_T\right)^2}; \quad F_{aB} = 0$$
 (3.9)

<span id="page-83-0"></span>Obviously, the radial load on the bearing B is much larger than the radial load on the bearing A if the gear 1 is near to the bearing B in case of b < a, cf. Eqs. (3.8 and 3.9).

In the *recuperation phase* in hybrid vehicles (HV), a part of the kinetic energy of the combustion engine applied to in a long-range driving is used to recharge the batteries for electric motors. Similarly, the same process takes place in the recuperation phase in electric vehicles (EV) in which the kinetic energy of braking or the kinetic energy of the car during down-hill driving recharges the supply batteries.

In this case, the shaft of the gear 2 becomes the driving shaft while the contact position at the gear flange changes in the opposite side of the gear tooth. However, the driven shaft of the gear 1 rotates in the same clockwise direction in both operations, as shown in Fig. 3.1. As a result, only the directions of the gear loads  $F_T$  and  $F_A$  change opposite to the respective directions in the *forwards driving cycle*. Thus, the acting loads on the bearings A and B in the *recuperation phase* result as

$$\overrightarrow{F}_{A} = \begin{bmatrix} F_{xA} = 0 \\ F_{yA} = \frac{b}{l} F_{R} - \frac{r_{pw}}{l} F_{A} \\ F_{zA} = \frac{b}{l} F_{T} \end{bmatrix}; \quad \overrightarrow{F}_{B} = \begin{bmatrix} F_{xB} = -F_{A} \\ F_{yB} = \frac{a}{l} F_{R} + \frac{r_{pw}}{l} F_{A} \\ F_{zB} = \frac{a}{l} F_{T} \end{bmatrix}$$
(3.10)

The amplitudes of the bearing loads are calculated from Eq. (3.10) as

$$F_{rA} = \sqrt{\left(\frac{b}{l}F_R - \frac{r_{pw}}{l}F_A\right)^2 + \left(\frac{b}{l}F_T\right)^2}; \quad F_{aA} = 0$$
 (3.11)

$$F_{rB} = \sqrt{\left(\frac{a}{l}F_R + \frac{r_{pw}}{l}F_A\right)^2 + \left(\frac{a}{l}F_T\right)^2}; \quad F_{aB} = F_A$$
 (3.12)

The results show that the radial loads and the axial loads are changed, cf. Eqs. (3.8 and 3.9).

In case of the *backwards driving cycle*, the driving shaft of gear 1 rotates backwards with a counterclockwise rotational speed  $\omega_1$ . Therefore, the contact position at the gear flange changes in the opposite side of only the directions of the gear tooth. In this case, gear loads  $F_T$  and  $F_A$  change opposite to the respective directions in the *forwards driving cycle*. The amplitudes of the radial loads on the bearings A and B result from both load components in the directions y and z and the axial loads in the direction x are calculated like in the *recuperation phase*, s. Eqs. (3.11 and 3.12).

### References

- Nguyen-Schäfer, H.: Computational Design of Rolling Bearings. Springer International Publishing, Switzerland (2016)
- 2. Schlecht, B.: Maschinenelemente 2 (in German). Pearson Education, München (2010)

# <span id="page-84-0"></span>Chapter 4 Bearing Endplay Over Operating Temperatures

![](_page_84_Picture_1.jpeg)

In the following section, the axial endplay of two single tapered roller bearings (TRB) in the X and O setups is computed over the operating bearing temperatures.

### 4.1 Calculating the Axial Endplay

The axial endplay of the bearings varies with the axial and the radial extension of the bearings that are caused by changing the operating bearing temperature, as shown in Figs. 4.1 and 4.2.

The axial extension of the bearings results from the change of the distance l between them over the operating bearing temperatures. As a result, the change of the axial endplay  $\Delta J_1$  is calculated from the axial extension of the bearings as

$$\Delta J_1 = (\alpha_h - \alpha_b)l \cdot \Delta T \tag{4.1}$$

where

 $\alpha_h$  is the thermal expansion coefficient of the housing (e.g.  $\alpha_{Al} = 22 \times 10^{-6} \text{ K}^{-1}$ ); is the thermal expansion coefficient of the bearing (e.g.  $\alpha_{Steel} = 12 \times 10^{-6} \text{ K}^{-1}$ ); is the distance between two bearings (mm);

 $\Delta T$  is the temperature change of the bearings (K).

The temperature change  $\Delta T$  of the bearings is defined as the difference of the operating bearing temperature  $T_b$  (°C) and the assembly temperature  $T_0$  = 20 °C.

$$\Delta T \equiv T_b - T_0 \tag{4.2}$$

<span id="page-85-0"></span>![](_page_85_Picture_2.jpeg)

Fig. 4.1 Tapered roller bearings in the X setup

![](_page_85_Picture_4.jpeg)

Fig. 4.2 Tapered roller bearings in the O setup

The limiting temperatures  $T_{\text{lim}}$  of the bearings A and B are determined so that their average radial interferences or clearances  $I_f$  at the outer race (OR) in the bearing housing are eliminated. Therefore, they are computed as

$$T_{\text{lim},A} = \frac{I_{fA}}{(\alpha_h - \alpha_b)D_A} + T_0; \quad T_{\text{lim},B} = \frac{I_{fB}}{(\alpha_h - \alpha_b)D_B} + T_0$$
 (4.3)

The radial interference changes of the bearings A and B depend on the temperature change of the bearings:

$$\Delta I_{fA} = \begin{cases}
(\alpha_h - \alpha_b)D_A \cdot (T - T_A) & \text{if } T \leq T_{\text{lim},A}; \\
\max(I_{fA}, 0) & \text{if } T > T_{\text{lim},A}
\end{cases}$$

$$\Delta I_{fB} = \begin{cases}
(\alpha_h - \alpha_b)D_B \cdot (T - T_B) & \text{if } T \leq T_{\text{lim},B}; \\
\max(I_{fB}, 0) & \text{if } T > T_{\text{lim},B}
\end{cases}$$
(4.4)

where

$$T_A = \begin{cases} T_{\text{lim},A} & \text{if } I_{\text{fA}} \leq 0 \\ T_0 & \text{if } I_{\text{fA}} > 0 \end{cases}; \quad T_B = \begin{cases} T_{\text{lim},B} & \text{if } I_{\text{fB}} \leq 0 \\ T_0 & \text{if } I_{\text{fB}} > 0 \end{cases}$$

<span id="page-86-0"></span>The change of the radial interference at the OR causes the additional axial change  $\Delta J_2$  of the endplay in the bearings, which results from the changes of the axial endplay of both bearings in

$$\Delta J_2 = K_A \lambda_A \Delta I_{fA} + K_B \lambda_B \Delta I_{fB} \tag{4.5}$$

where

 $K_A$  and  $K_B$  are the transforming factors of the radial interference change of the bearings A and B, respectively;

 $\lambda_A$  and  $\lambda_B$  are the influence factors of the radial interference of the bearings A and B, respectively;

The transforming factors K of the bearings A and B are calculated from the values  $Y_A$  and  $Y_B$  that are found in bearing catalogues as

$$K_A = \frac{Y_A}{0.8}; \quad K_B = \frac{Y_B}{0.8}$$
 (4.6)

The influence factors  $\lambda$  for the *IR* (inner race) and *OR* (outer race) are defined as the ratio of the change of the radial bearing endplay to the average radial interference or clearance at the *IR* and *OR*, respectively.

$$\lambda_{IR} \equiv \frac{\Delta \delta_r}{I_{f,IR}}; \quad \lambda_{OR} \equiv \frac{\Delta \delta_r}{I_{f,OR}}$$
(4.7)

Some useful influence factors for the IR and OR are recommended in [1] (s. Table 4.1).

In summary, the total change of the axial endplay in both setups X and O is calculated as

$$\Delta J_t = \begin{cases} \Delta J_2 + \Delta J_1 \text{ for } X \text{ setup} \\ \Delta J_2 - \Delta J_1 \text{ for } O \text{ setup} \end{cases}$$
 (4.8)

Eventually, the operating axial endplay  $\delta_a$  of the bearings results from the axial endplay  $\delta_{a,pl}$  and the total change of the axial endplay  $\Delta J_t$  in

$$\delta_a = \delta_{a,pl} + \Delta J_t \tag{4.9}$$

**Table 4.1** Influence factors  $\lambda$  for various applications

| Inner race (IR) | Full shaft                 | $\lambda_{IR}=0.8$   |
|-----------------|----------------------------|----------------------|
|                 | Hollow shaft               | $\lambda_{IR}=0.6$   |
| Outer race (OR) | Steel/iron bearing housing | $\lambda_{OR} = 0.7$ |
|                 | Light-metal bearing        | $\lambda_{OR} = 0.5$ |
|                 | housing                    |                      |

### <span id="page-87-0"></span>4.2 Computational Examples

As a first example, two similar single TRB of type FAG 32007-X-XL in the X setup used for the bearings A and B operate under the following operating conditions:

```
D_A = 72 mm; D_B = 72 mm; l = 200 mm; Y_A = 1.32; Y_B = 1.32; \lambda_A = 0.5; \lambda_B = 0.5; \lambda_B = 0.5; \lambda_B = 0.5; \lambda_B = 0.5 (Aluminum alloy); \lambda_B = 12 × 10<sup>-6</sup> K<sup>-1</sup> (Steel); \lambda_B = -0.014 mm for the fitting clearance H7 at the OR (floating bearings); \lambda_B = -20 °C to +120 °C (operating range of the bearing temperature); \lambda_B = 20 °C (assembly temperature).
```

Using the program COMAEP [2] the axial changes  $\Delta J_1$ ,  $\Delta J_2$ , and  $\Delta J_t$  of the endplay of the bearings over the operating bearing temperatures are computed for the X setup and displayed in Fig. 4.3. The total change of the axial endplay  $\Delta J_t$  is necessary for calculating the operating axial endplay  $\delta_a$  of the bearings over the operating bearing temperatures  $T_b$ .

The preload with  $\delta_{a,pl}$  < 0 at the assembly temperature of 20 °C should be chosen so that the operating axial endplay  $\delta_a$  is little negative in the mostly operating temperature so that the bearing lifetime is near the optimum, s. Fig. 4.4. In the X setup, a negative  $\delta_{a,pl}$  at the preload from the line-to-line position (i.e. at the null axial endplay) is selected with an axial interference of about  $-165~\mu m$ . The computed total change of the axial endplay  $\Delta J_t$  results as  $+140~\mu m$  at the mostly

![](_page_87_Figure_7.jpeg)

Fig. 4.3 Change of the axial endplay over bearing temperatures in the X setup

<span id="page-88-0"></span>![](_page_88_Figure_2.jpeg)

Fig. 4.4 Lifetime and load angle versus operating axial endplay

operating bearing temperature Tb = 90 °C, s. Fig. [4.3](#page-87-0). According to Eq. ([4.9](#page-86-0)), the operating axial endplay d<sup>a</sup> of the bearings is also about −25 µm at the operating temperature Tb.

Figure 4.4 shows the real lifetime Lh of the bearing versus the operating axial endplay where Lh\* is the bearing lifetime at the null axial endplay where the load angle 2c is 180°. At slightly reducing the axial endplay from the line-to-line position, the load angle of the bearing increases. As a result, the external load is evenly distributed on the rollers leading to the increase of the lifetime of the bearing. After reaching the maximum lifetime, further preload with highly negative axial endplays leads to the large load angle, high friction power, overheating, low bearing efficiency, and strong reduction of the bearing lifetime. In worst case, they would fail in a very short time of just a few operating hours. On the contrary, TRB at largely positive axial endplays have at first a high bearing efficiency and low friction in the bearings. However, they confront with some serious problems, such as NVH (noise vibration harshness), large bearing clearances, reduction of the load angle, small bearing stiffness, micro-oscillations, and slip vibrations that cause slip-related wear and damage in the bearings leading to the reduction of the bearing lifetime.

![](_page_89_Figure_2.jpeg)

Fig. 4.5 Change of the axial endplay over bearing temperatures in the O setup

As a second example, both same single TRB of type FAG 32007-X-XL in the O setup used for the bearings A and B operate under the following operating conditions:

```
D_A = 72 mm; D_B = 72 mm; l = 200 mm; Y_A = 1.32; Y_B = 1.32; \lambda_A = 0.5; \lambda_B = 0.5; \alpha_h = 22 × 10<sup>-6</sup> K<sup>-1</sup> (Aluminum alloy); \alpha_b = 12 × 10<sup>-6</sup> K<sup>-1</sup> (Steel); I_{fA} = I_{fB} = +0.037 mm for the fitting interference P7 at the OR (fixed bearings); T_b = -20 °C to +120 °C (operating range of the bearing temperature); T_0 = 20 °C (assembly temperature).
```

Using the program COMAEP [2] the axial changes  $\Delta J_1$ ,  $\Delta J_2$ , and  $\Delta J_t$  of the endplay of the bearings over the operating bearing temperatures are computed for the O setup and displayed in Fig. 4.5. The total change of the axial endplay  $\Delta J_t$  is necessary for calculating the operating axial endplay  $\delta_a$  of the bearings over the operating bearing temperatures  $T_b$ .

At adjusting the bearings in the O setup, a positive axial endplay is chosen far from the line-to-line position (s. Fig. 4.2); e.g.,  $\delta_{a,pl}$  with an axial clearance of about +55  $\mu$ m. The computed total change of the axial endplay  $\Delta J_t$  results as -79  $\mu$ m at the mostly operating bearing temperature  $T_b = 90$  °C, s. Fig. 4.5. According to Eq. (4.9), the operating axial endplay  $\delta_a$  of the bearings is also about -24  $\mu$ m at the operating temperature  $T_b$  so that the lifetime of the bearings is near the optimum, cf. Fig. 4.4.

The program COMAEP written in MATLAB code is applied to compute the axial endplay over the operating bearing temperatures.

```
%========================================================= 
% Program COMAEP (Computing Axial Endplay for Tapered Roller Bearings) 
% Author: Dr. Nguyen-Schaefer (2018) 
%========================================================= 
Function COMAEP 
% Input data 
Case = 'Two single bearings FAG 32007-X-XL'; 
% Housing bore diameter D 
DA = 72; % mm 
DB = 72; % mm 
% Distance l 
l = 200; % mm 
% Bearing factors (s. bearing catalogue) 
YA = 1.32; 
YB = 1.32; 
% Bearing factors (s. bearing catalogue) 
lambda_A = 0.5; 
lambda_B = 0.5;
% Fitting interference or clearance: P7 
I_fA = 0.037; % mm 
I_fB = 0.037; % mm 
% Extension coefficients 
alfa_h = 22E-6; % bearing housing (1/K) 
alfa_b = 12E-6; % bearings (1/K) 
% Operating bearing temperature (°C) 
Tb_min = -20; 
Tb_max = 120; 
% Intervals of temperature for computing 
NtP = 50; 
% Setup of the X or O arrangement 
i_setup = 0; % = 0: O arrangement; = 1: X arrangement 
% Assembly temperature (°C) 
T_0 = 20; 
%
% Array 
NtP1 = NtP + 1; 
iarray = 1:1:NtP1; 
f_array = zeros(size(iarray)); % generating zero vectors 
T_op = f_array; % Vector with NtP1 elements 
DJ_1 = f_array; 
DJ_2 = f_array; 
DJ_t = f_array; 
%
% Computation 
for i = 1:1:NtP1 %=============== BEGIN of computation 
T_op(i) = Tb_min + (i-1)*(Tb_max-Tb_min)/NtP; 
DJ_1(i) = (alfa_h - alfa_b)*l*(T_op(i)- T_0); 
T_limA = I_fA/((alfa_h - alfa_b)*DA) + T_0; 
T_limB = I_fB/((alfa_h - alfa_b)*DB) + T_0; 
if (I_fA <= 0 || I_fB <= 0) 
 TA = T_limA; TB = T_limB;
```

```
else 
 TA = T_0; 
 TB = T_0; 
end 
if (T_op(i) <= T_limA) 
 DI_fA = (alfa_h - alfa_b)*DA*(T_op(i)-TA); 
else 
 DI_fA = max(I_fA,0); 
end 
if (T_op(i) <= T_limB) 
 DI_fB = (alfa_h - alfa_b)*DB*(T_op(i)-TB); 
else 
 DI_fB = max(I_fB,0); 
end 
KA = YA/0.8; KB = YB/0.8; 
DJ_2(i) = KA *lambda_A *DI_fA + KB *lambda_B *DI_fB; 
if (i_setup == 0) 
 DJ_t(i) = DJ_2(i) - DJ_1(i); % for the O arrangement 
else 
 DJ_t(i) = DJ_2(i) + DJ_1(i); % for the X arrangement 
end 
end % ================ END of computation 
return 
end
```

### References

- 1. NTN-SNR: Online documentation of tapered roller bearings (2018)
- 2. Nguyen-Schäfer, H.: Program COMAEP for Computing Axial Endplay for Tapered Roller Bearings in MATLAB (2018)

### <span id="page-92-0"></span>Chapter 5 Accelerated Load Spectrum

![](_page_92_Picture_1.jpeg)

In general, the load spectrum is too large and extensive to test the lifetime of a component. To hasten the testing time and to reduce costs, the load spectrum should be accelerated so that its much shorter testing time is equivalent to the real lifetime for the load spectrum. The equivalent load spectrum is called the accelerated load spectrum.

### 5.1 Calculating the Damage Number

The fatigue failure takes place after a certain period of time depending on the load spectrum. To generate an equivalent accelerated load spectrum, a basic number has to be determined so that the fatigue lifetimes of the component are equivalent in both load spectra. This basic number is defined as the damage number that is coped with the following section.

The extended lifetime Lhn(h) of a bearing as component is calculated as [\[1](#page-99-0)]

$$Lh_n = \frac{10^6}{60N_R} a_1 a_{ISO} \left(\frac{C_r}{P_m}\right)^p \tag{5.1}$$

where

NR is the rotational speed of the shaft (rpm);

a<sup>1</sup> is the dimensionless parameter for the failure probability (e.g. a<sup>1</sup> = 1 for 10% failure probability);

aISO is the dimensionless parameter for extended lifetime;

Cr is the dynamic load rating of the bearing (N);

Pm is the equivalent load on the bearing (N);

p is the dimensionless lifetime factor.

<span id="page-93-0"></span>In general, the lifetime factors are used for various applications such as

p = 3 for ball bearing;

p = 10/3 for roller bearing;

p = 6.6 for gear flange;

p = 8.7 for gear foot.

Rewriting Eq. (5.1), one obtains the new term involving the bearing lifetime, acting load, and rotor speed

$$Lh_n \cdot (60N_R P_m^p) = 10^6 \times (a_1 a_{ISO} C_r^p) \rightarrow invariant$$
 (5.2)

This RHS term of Eq. (5.2) only depends on the parameters  $a_1$ ,  $a_{ISO}$ , and  $C_r$ . Therefore, it is invariant in both load spectra.

Due to proportionality between torques and loads in the load spectrum, the other invariant term involving the lifetime, acting torque, and rotor speed results from Eq. (5.2) in

$$Lh_n \cdot \left(60N_R M_m^p\right) \propto Lh_n \cdot \left(60N_R P_m^p\right) \to invariant$$
  
 
$$\Rightarrow \left(60N_R Lh_n\right) M_m^p = invariant$$
(5.3)

The total number of revolutions in the load spectrum is calculated as

$$T_{rev} = 60N_R L h_n (5.4)$$

Substituting Eq. (5.4) into Eq. (5.3), the also invariant damage number DN in  $(Nm)^p \times revolutions$  is defined as

$$DN \equiv M_m^p T_{rev} \to invariant \tag{5.5}$$

For an equivalent lifetime of the component, the damage number must be invariant in both load spectra. Let the real load spectrum be assumed as a  $(M \times N)$  matrix with  $M_i$  the torques for i = 1,..., M and  $N_i$  the shaft speeds for j = 1,..., N. As a result, the real load spectrum could contain a large number of  $M \times N$  operational points where M and N are very large numbers. Using artificial intelligence (AI) and machine learning (ML), the real load spectrum is collected online from the fleet of electric vehicles in real-life conditions driven by drivers and autonomy operation, as the *new*  $carmaker Tesla^{@}$  did. The collected load spectrum is optimized and used to redesign its R&D processes. Instead of using the AI and ML to do this task like *Tesla*, the *old* carmakers have adapted the existing load spectrum of combustion vehicles (CV) for electric vehicles (EV). The adapted load spectrum of CV is quite inappropriate and too conservative compared to the real load spectrum for EV since the supply electric power of batteries continuously reduces with the driving time during a charged cycle and the number of recharging cycles, unlike conventional fuels. As a result, it leads to a very robust design of the product more than necessary and therefore higher costs, much heavier weight, and reducing the driving range as well.

<span id="page-94-0"></span>The individual damage number  $DN_i$  for the operational point with torque  $M_i$  and its relating shaft speeds  $N_i$  is computed as

$$DN_i \equiv \sum_{i=1}^{N} M_i^p T_{ij,rev} \tag{5.6}$$

The total damage number DN of the real load spectrum results from the individual damage numbers  $DN_i$  for all torques  $M_i$  in

$$DN = \sum_{i=1}^{M} DN_{i} = \sum_{i=1}^{M} \left( \sum_{j=1}^{N} M_{i}^{p} T_{ij,rev} \right)$$

$$= \sum_{i=1}^{M} \left( M_{i}^{p} \cdot \sum_{j=1}^{N} T_{ij,rev} \right)$$
(5.7)

where

M is the number of the acting torques  $M_i$ ;

 $T_{ij,rev}$  is the individual number of revolutions for the operational point with a torque  $M_i$  and a shaft speed  $N_i$  in the real load spectrum.

Analogous to Eq. (5.4), the individual number of revolutions of any operational point P(i, j) in the real load spectrum is computed as

$$T_{ij,rev} = 60N_i \tau_{ij} \tag{5.8}$$

where  $N_j$  is the shaft speed (rpm) and  $\tau_{ij}$  the time interval (h) for the operational point P.

### 5.2 Calculating the Accelerated Load Spectrum

To hasten the testing time of a component, an accelerated load spectrum is chosen so that its damage number equals the damage number of the real load spectrum. In this case, the damage number is invariant in the accelerated load spectrum for the same lifetime of the real load spectrum. However, it must be considered that the accelerated testing stress must be always less than the yield stress and the ultimate tensile stress of the component. Otherwise, the testing probe is plastically deformed or broken at once. In addition, the component temperature at the accelerated testing should be approximately equal to the temperature in the real load spectrum by cooling the lubricant in the testing component. In summary, the failure mechanism in the accelerated load spectrum must be unchanged in both cases [2].

In the following, an accelerated load spectrum of a  $(P \times Q)$  matrix with the torques  $M_k$  for k = 1,..., P and the shaft speeds  $N_l$  for l = 1,..., Q is used. The

<span id="page-95-0"></span>damage number  $DN_{acc}$  for the accelerated load spectrum is calculated using Eqs. (5.7 and 5.8) as

$$DN_{acc} = \sum_{k=1}^{P} \left( M_k^p \cdot \sum_{l=1}^{Q} T_{kl,rev} \right) = \sum_{k=1}^{P} \left( M_k^p \cdot \sum_{l=1}^{Q} 60 N_l \tau_{kl} \right)$$

In case of a diagonal matrix (i.e. P = Q), the individual time interval is calculated as

$$au_{kl} = au_k \delta_{kl} \equiv \left\{ egin{array}{ll} 0 & ext{if } l 
eq k; \ au_k & ext{if } l = k. \end{array} 
ight.$$

where  $\delta_{kl}$  is called the Kronecker delta.

Therefore, the accelerated damage number results as

$$DN_{acc} = \sum_{k=1}^{P} M_k^p \times (60N_k \tau_k) = \sum_{k=1}^{P} M_k^p T_{k,rev} = DN$$
 (5.9)

Three parameters of torque  $M_k$  (Nm), shaft speed  $N_k$  (rpm), and time interval  $\tau_k$  (h) are chosen in accordance with other components in the system (e.g. bearings and gears) so that its damage number should equal the damage number in the real load spectra, cf. Eq. (5.9). To reduce the testing time, appropriate higher torques should be applied to the accelerated testing because the damage number increases with torque to the power of p > 1 and is only linear with shaft speed and time interval. As a result, the higher the torque is applied to the testing, the larger the damage number becomes and vice versa. Thus, the shorter testing time is needed for the accelerated load spectrum.

The total testing time  $\tau_{testing}$  is the sum of the time intervals of P testing points in the accelerated load spectrum.

$$\tau_{testing}(h) = \sum_{k=1}^{P} \tau_k(h) \ll \tau_{real}(h)$$
 (5.10)

The purpose of the accelerated load spectrum is to hasten the testing time and to reduce costs for the testing lifetime that is equivalent to the required lifetime in the real load spectrum.

### 5.3 An Example for an Accelerated Load Spectrum

In the following section, an example of the accelerated load spectrum derived from a real load spectrum of a  $(M \times N)$  matrix for electric vehicles is demonstrated. The real load spectrum consists of 10,000 operational points for M = 100 and N = 100, as shown in Fig. 5.1.

<span id="page-96-0"></span>![](_page_96_Figure_2.jpeg)

Fig. 5.1 Real load spectrum of a driving shaft for electric vehicles

The positive torques occur in the forwards driving cycle; the negative torques in the recuperating cycle. The event probability  $t_{ij}$  is defined as the ratio of the individual number of revolutions to the total number of revolutions of the load spectrum. They are displayed for all operational points in the real load spectrum, as shown in Fig. 5.1. A null probability of an event means that no operation occurs in this operational condition.

Using the program COMALSX [3], the accelerated load spectrum of a  $(P \times Q)$  diagonal matrix for electric vehicles is computed from the real load spectrum. At first, the damage numbers of bearings and gears at flange and foot for the driving shaft are computed. The total damage number of ball bearings with p=3 on the driving shaft results as  $1.58 \times 10^{14} \, (\text{Nm})^3 \cdot \text{rev}$ . The spectrum of the damage number of the bearings for the real load spectrum is computed and displayed in Fig. 5.2. In this case, the lifetime Lh<sub>10</sub> of about 7000 h is required in the real load spectrum.

In summary, the real load spectrum requires a testing at 10,000 operational points in a testing time of 7000 h. That requires a more than 290-day testing without interruption. First, nobody can pay for this extremely intensive testing. Second, the task costs a lot of time that no customer wants to wait for. Third, time to market (TTM) lasts too long to lose the contract of the customer. For economic aspects, the question is how the testing could be hastened to overcome these handicaps. This hard request will be answered here.

The precondition for accelerating the testing time is that the damage number must be the same or invariant in both load spectra. In accordance with the lifetime and damage number of gears, an accelerated load spectrum of a  $(P \times Q)$  diagonal matrix

<span id="page-97-0"></span>![](_page_97_Figure_2.jpeg)

Fig. 5.2 Spectrum of the damage numbers of ball bearings

| <b>Table 5.1</b> Results of the accelerated load spectrum for ball bearings |
|-----------------------------------------------------------------------------|
|-----------------------------------------------------------------------------|

| Torque $M_k$ [Nm] | Shaft speed $N_k$ [rpm] | $ \begin{array}{c} DN_k \\ [(\text{Nm})^3 \text{ rev}] \end{array} $ | Time interval $\tau_k$ [h] | $DN_k$ percentage [%] |
|-------------------|-------------------------|----------------------------------------------------------------------|----------------------------|-----------------------|
| 50                | 12,000                  | $4.05 \times 10^{13}$                                                | 450                        | 25.6                  |
| 65                | 10,000                  | $5.52 \times 10^{13}$                                                | 335                        | 34.9                  |
| 80                | 8000                    | $4.18 \times 10^{13}$                                                | 170                        | 26.4                  |
| 100               | 6000                    | $7.20 \times 10^{12}$                                                | 20                         | 4.6                   |
| 120               | 5000                    | $7.78 \times 10^{12}$                                                | 15                         | 4.9                   |
| 140               | 3500                    | $5.76 \times 10^{12}$                                                | 10                         | 3.6                   |
| Total:            |                         | $1.58 \times 10^{14}$                                                | 1000                       | 100                   |

with P=Q=6 in six cluster means on its diagonal are selected by machine-learning methods for all necessary testing conditions, as shown in Table 5.1. The best-known machine-learning (ML) method for clustering is the k-means algorithm [2] in which the data points delivered by customers are clustered in k clusters, s. App. C. Each cluster has a mean value that represents the *cluster mean* in the accelerated load spectrum. Note that the sizes of k clusters are not necessary to be equal by using the k-means algorithm. In fact, the cluster sizes of data points vary from case to case with the real load spectrum supplied by the customers.

<span id="page-98-0"></span>In this case, the real load spectrum provides us with the data set of the customers. Here the cluster means are the testing points of different testing stresses in the Wöhler curve (SN curve) [[1\]](#page-99-0). The cluster means are selected for different operating conditions at high, middle, and low stresses on the component with the corresponding fatigue cycles in the SN curve. Obviously, the cluster means could change with different data sets of the customers depending on the operating cycles.

In the accelerated load spectrum, different cluster means are selected at low torques with high rotor speeds for the driving cycle on highways and country roads, at middle torques with relating rotor speeds for the traffic cycle in cities, and at high torques with low rotor speeds for the cycle of passing/accelerating maneuvers.

Using Eq. [\(5.10\)](#page-95-0), the total testing time amounts to 1000 h compared to the required lifetime of 7000 h, i.e. the time accelerating factor is seven and the total testing time is reduced by nearly 86% to get the same lifetime of the bearings.

The computational results show the individual damage numbers, the individual time intervals, and their percentages of damage numbers in the accelerated load spectrum of a (6 6) diagonal matrix for ball bearings, s. Table [5.1](#page-97-0) and Fig. 5.3. Furthermore, the testing time could be more hastened when the testing conditions are strengthened by much higher acting torques. However, if the failure mechanism of the testing component is changed, it will fail due to being overstressed or overheated instead of the material fatigue.

![](_page_98_Figure_6.jpeg)

Fig. 5.3 Accelerated load spectrum of a (6 6) diagonal matrix

### <span id="page-99-0"></span>References

- 1. Nguyen-Schäfer, H.: Computational Design of Rolling Bearings. Springer International Publishing, Switzerland (2016)
- 2. McCool, J.I.: Using the Weibull Distribution. Wiley, Hoboken (2012)
- 3. Nguyen-Schäfer, H.: Program COMALSX for Computing Accelerated Load Spectrum of Bearings and Gears. Internal code in MATLAB (2017)

# <span id="page-100-0"></span>**Chapter 6 Solving Nonlinear Equation Systems**

![](_page_100_Picture_1.jpeg)

This chapter deals with solving nonlinear equation systems that describe the computational models for tapered and cylinder roller bearings in Chaps. 1 and 2. In general, the computational models consisting of a large number of coupled equations are strongly nonlinear. It is not easy to get converged solutions for large strongly nonlinear coupled equation systems. Therefore, an appropriate algorithm is required to solve such nonlinear equation systems. In the following sections, the Gauss-Newton and the Levenberg-Marquardt algorithm based on least squares method are mathematically derived for solving the computational models of tapered and cylinder roller bearings.

### 6.1 Fundamental of Nonlinear Equation Systems

A linear function f(x) is a linear mapping f of the variable  $x \in \mathbf{R}$  to a real value  $f(x) \in \mathbf{R}$  in which the value of f(x) linearly changes with the variable x, i.e. the slope of the function f(x) is constant or invariant as x varies. On the contrary, a nonlinear function f(x) is a nonlinear mapping of the variable  $x \in \mathbf{R}$  to a real value  $f(x) \in \mathbf{R}$  in which the value of f(x) weakly or strongly changes with the variable x, i.e. the slope of the function f(x) is variant with the variable x. Note that both properties of additivity and homogeneity fail in the nonlinear mapping.

Let  $\mathbf{f}(\mathbf{x}) = \mathbf{0}$  be a nonlinear (*NL*) equation system consisting of *n* nonlinear functions  $f_i(x_1, x_2, ..., x_n)$  for i = 1, 2, ..., n. The *NL* equation system of *n* unknowns of  $x_i$  is written in vectorial form as

$$\begin{cases}
f_1(x_1, x_2, \dots, x_n) = 0 \\
f_2(x_1, x_2, \dots, x_n) = 0 \\
\dots \\
f_n(x_1, x_2, \dots, x_n) = 0
\end{cases}
\Leftrightarrow
\begin{cases}
\mathbf{f}(\mathbf{x}) = \mathbf{0} \in \mathbb{R}^n; \\
\mathbf{x} = (x_1 x_2 \dots x_n)^T \in \mathbb{R}^n
\end{cases}$$
(6.1)

<sup>©</sup> Springer Nature Switzerland AG 2019

<span id="page-101-0"></span>Using Taylor's series, the *i*th equation is expressed at the iteration step (k + 1) as

$$f_{i}(\mathbf{x}^{(k+1)}) = f_{i}(\mathbf{x}^{(k)}) + \sum_{j=1}^{n} \underbrace{\frac{\partial f_{i}(\mathbf{x}^{(k)})}{\partial x_{j}}}_{J_{f,ij}(\mathbf{x}^{(k)})} \underbrace{(x_{j}^{(k+1)} - x_{j}^{(k)})}_{\delta x_{j}^{(k+1)}} + \dots$$
(6.2)

for all i = 1, 2, ..., n.

The Jacobian  $J_f(x)$  of the functional vector f(x) is a  $(n \times n)$  matrix of which element is defined as [1]

$$\mathbf{J_f}(\mathbf{x}) = \left[J_{\mathbf{f},ij}(\mathbf{x})\right] \in \mathbb{R}^n \times \mathbb{R}^n;$$

$$J_{\mathbf{f},ij}(\mathbf{x}) \equiv \frac{\partial f_i(\mathbf{x})}{\partial x_i} \in \mathbb{R}, \forall i, j = 1, ..., n$$
(6.3)

The equation system can be written in vectorial form at the iteration step (k + 1)

$$\mathbf{f}(\mathbf{x}^{(k+1)}) = \mathbf{f}(\mathbf{x}^{(k)}) + \mathbf{J}_{\mathbf{f}}(\mathbf{x}^{(k)}) \delta \mathbf{x}^{(k)} + \dots$$
 (6.4)

where  $\mathbf{f}(\mathbf{x}^{(k)}) \in \mathbb{R}^n$ ;  $\mathbf{J}_{\mathbf{f}}(\mathbf{x}^{(k)}) \in \mathbb{R}^n \times \mathbb{R}^n$ ;  $\delta \mathbf{x}^{(k)} \in \mathbb{R}^n$ .

The computed solutions of the equation system converge at the iteration step (k) if the norm of  $\delta \mathbf{x}^{(k)}$  is less than a given tolerance  $\varepsilon$  of about  $10^{-6}$ .

$$\|\delta \mathbf{x}^{(k)}\| = \sqrt{\sum_{i=1}^{n} \left(\delta x_i^{(k)}\right)^2} \le \varepsilon$$
 (6.5)

Using the computing scheme, the NL equation system (6.1) is iteratively solved [2], as displayed in Fig. 6.1:

As a result, one obtains at the iteration step (k + 1)

$$\mathbf{f}(\mathbf{x}^{(k+1)}) = \mathbf{0}$$

Therefore, the approximation of the Jacobian of the vectorial function  $\mathbf{f}(x)$  results from Eq. (6.4) in

$$\mathbf{J}_{\mathbf{f}}(\mathbf{x}^{(k)})\delta\mathbf{x}^{(k)} = -\mathbf{f}(\mathbf{x}^{(k)}) 
\Rightarrow \delta\mathbf{x}^{(k)} = -\mathbf{J}_{\mathbf{f}}^{-1}(\mathbf{x}^{(k)})\mathbf{f}(\mathbf{x}^{(k)})$$
(6.6)

<span id="page-102-0"></span>**Fig. 6.1** Scheme of solving a NL equation system f(x) = 0

![](_page_102_Figure_3.jpeg)

### 6.2 NL Equation Systems with Gauss-Newton Algorithm

Let define the nonlinear equation system f(x) = 0 that consists of an approximate functional vector F(x) and a residual vector R(x) as

$$\mathbf{f}(\mathbf{x}) = \mathbf{F}(\mathbf{x}) + \mathbf{R}(\mathbf{x}) = \mathbf{0} \in \mathbb{R}^n;$$
  
$$\mathbf{x} = (x_1 x_2 \dots x_n)^T \in \mathbb{R}^n$$
(6.7)

The residual column vector is written as

$$\mathbf{R}(\mathbf{x}) = (\varepsilon_1 \, \varepsilon_2 \dots \varepsilon_n)^T \in \mathbb{R}^n$$

The functional vector  $\mathbf{F}(\mathbf{x})$  of the equation system could be expressed as the gradient of a residual function  $\phi(\mathbf{x})$ :

$$\mathbf{F}(\mathbf{x}) \equiv \nabla \phi(\mathbf{x}) = \mathbf{f}(\mathbf{x}) - \mathbf{R}(\mathbf{x});$$

$$\phi(\mathbf{x}) = \frac{1}{2} ||\mathbf{R}(\mathbf{x})||^2 = \frac{1}{2} \sum_{i=1}^{n} \varepsilon_i^2(\mathbf{x}) \le \varepsilon$$
(6.8)

Using least squares method (LSM) [3], cf. App. B, an unknown vector  $\mathbf{x}$  is iteratively solved at which the residual function  $\phi(\mathbf{x})$  or the norm of the residual

<span id="page-103-0"></span>vector  $\mathbf{R}(\mathbf{x})$  is minimized so that the functional vector  $\mathbf{F}(\mathbf{x})$  would converge to  $\mathbf{f}(\mathbf{x}) = \mathbf{0}$ , cf. Eq. (6.7).

The functional vector  $\mathbf{F}(\mathbf{x})$  in Eq. (6.8) is further calculated as [2]

$$\mathbf{F}(\mathbf{x}) = \nabla \phi(\mathbf{x}) = \mathbf{J}_R^T(\mathbf{x})\mathbf{R}(\mathbf{x}) \tag{6.9}$$

where  $J_R(x)$  is the Jacobian of the residual vector R(x).

The Hessian matrix  $\mathbf{H}(\mathbf{x})$  is defined as the Jacobian of  $\mathbf{F}(\mathbf{x})$  [1-4]:

$$\mathbf{H}(\mathbf{x}) \equiv \mathbf{J}_{\mathbf{F}}(\mathbf{x}) = \mathbf{J}_{\nabla \phi}(\mathbf{x}) = \mathbf{J} [\mathbf{J}_{\mathbf{R}}^{T}(\mathbf{x})\mathbf{R}(\mathbf{x})]$$
(6.10)

Using Taylor's series, the Hessian matrix in Eq. (6.10) is computed as

$$\mathbf{H}(\mathbf{x}) = \mathbf{J}_{\mathbf{R}}^{T}(\mathbf{x})\mathbf{J}_{\mathbf{R}}(\mathbf{x}) + \underbrace{\sum_{i=1}^{n} \frac{\partial^{2} \varepsilon_{i}(\mathbf{x})}{\partial x_{j} \partial x_{k}} \varepsilon_{i}(\mathbf{x})}_{\mathbf{S}(\mathbf{x})} + \dots, \forall j, k = 1, \dots, n$$

$$= \mathbf{J}_{\mathbf{R}}^{T}(\mathbf{x})\mathbf{J}_{\mathbf{R}}(\mathbf{x}) + \underbrace{\mathbf{S}(\mathbf{x})}_{=0} + \dots$$
(6.11)

The term  $\mathbf{S}(\mathbf{x})$  in RHS of Eq. (6.11) is relatively insignificant compared to the first term in case of weak nonlinearity. Thus, it is mostly neglected in the *Gauss-Newton algorithm* with  $\mathbf{S}(\mathbf{x}) \approx \mathbf{0}$  in solving *weakly NL* equation systems. However, this term becomes much larger in *strongly NL* equation systems. In the latter, the *Levenberg-Marquardt algorithm* with  $\mathbf{S}(\mathbf{x}) \neq \mathbf{0}$  is applied to get converged solutions that will be discussed in the next section.

Changing f(x) into F(x) in Eq. (6.6) and using Eqs. (6.9 and 6.11), one obtains for  $S(x) \approx 0$  the following equation:

$$\mathbf{J}_{\mathbf{F}}(\mathbf{x})\delta\mathbf{x} \equiv \mathbf{H}(\mathbf{x})\delta\mathbf{x} = -\mathbf{F}(\mathbf{x}) 
\Leftrightarrow \left[\mathbf{J}_{\mathbf{R}}^{T}(\mathbf{x})\mathbf{J}_{\mathbf{R}}(\mathbf{x})\right]\delta\mathbf{x} = -\mathbf{J}_{\mathbf{R}}^{T}(\mathbf{x})\mathbf{R}(\mathbf{x}) \tag{6.12}$$

The converged solution vector  $\mathbf{x}$  is reached if the norm of the residual function  $\phi(\mathbf{x})$  is less than the given tolerance  $\varepsilon$  of about  $10^{-6}$ , cf. Eq. (6.8).

Using the computing scheme, the NL equation system (6.1) is iteratively solved with the Gauss-Newton algorithm for  $\mathbf{S}(\mathbf{x}) \approx \mathbf{0}$ , as shown in Fig. 6.2. However, the convergence of solutions for weakly NL equation systems using the GN algorithm strongly depends on the guessed values of the initial unknown vector  $\mathbf{x}_0$ . Thus, the initial values should be appropriately selected for the nonlinear equation system. Note that the Gauss-Newton algorithm (GN) is not suitable for strongly NL equation systems; however, the Levenberg-Marquardt algorithm (LM) is suitable for it.

<span id="page-104-0"></span>**Fig. 6.2** Gauss-Newton algorithm for a *weakly NL* equation system

![](_page_104_Figure_3.jpeg)

# **6.3** NL Equation Systems with Levenberg-Marquardt Algorithm

As early discussed, the Levenberg-Marquardt algorithm for  $S(x) \neq 0$  is used to solve a *strongly NL* equation system f(x) = 0. In this case, the modified residual vector is calculated as

$$\tilde{\mathbf{R}}(\mathbf{x}^{(k)}) = \mathbf{R}(\mathbf{x}^{(k)}) + \sum_{j=1}^{n} \frac{\partial \mathbf{R}(\mathbf{x}^{(k)})}{\partial x_{j}^{(k)}} \underbrace{\left(x_{j}^{(k+1)} - x_{j}^{(k)}\right)}_{\delta x_{j}^{(k)}} + \dots$$

$$= \mathbf{R}(\mathbf{x}^{(k)}) + \mathbf{J}_{\mathbf{R}}(\mathbf{x}^{(k)}) \delta \mathbf{x}^{(k)} + \dots$$
(6.13)

The modified functional vector  $\tilde{\mathbf{F}}(\mathbf{x})$  of the equation system could be expressed as the gradient of a modified residual function  $\tilde{\phi}(\mathbf{x})$ , cf. Eq. (6.8):

$$\tilde{\mathbf{F}}(\mathbf{x}) \equiv \nabla \tilde{\phi}(\mathbf{x}) = \mathbf{f}(\mathbf{x}) - \tilde{\mathbf{R}}(\mathbf{x}); 
\tilde{\mathbf{F}}(\mathbf{x}) = \mathbf{J}_{\tilde{\mathbf{R}}}^{T}(\mathbf{x})\tilde{\mathbf{R}}(\mathbf{x})$$
(6.14)

The converged solution vector  $\mathbf{x}$  is reached if the norm of the residual function  $\tilde{\phi}(\mathbf{x})$  is less than the given tolerance  $\varepsilon$  of about  $10^{-6}$ .

$$\tilde{\phi}(\mathbf{x}) = \frac{1}{2} \|\tilde{\mathbf{R}}(\mathbf{x})\|^2 = \frac{1}{2} \sum_{i=1}^{n} \tilde{\varepsilon}_i^2(\mathbf{x}) \le \varepsilon$$
(6.15)

<span id="page-105-0"></span>Analogously, the modified Hessian matrix is defined as the Jacobian of  $\tilde{\mathbf{F}}(\mathbf{x})$  that results from Eq. (6.14) as

$$\tilde{\mathbf{H}}(\mathbf{x}) \equiv \mathbf{J}_{\tilde{\mathbf{F}}}(\mathbf{x}) = \mathbf{J}_{\nabla \tilde{\boldsymbol{\phi}}}(\mathbf{x}) = \mathbf{J} \left[ \mathbf{J}_{\tilde{\mathbf{R}}}^{T}(\mathbf{x}) \tilde{\mathbf{R}}(\mathbf{x}) \right]$$
(6.16)

Using Taylor's series, the modified Hessian matrix is written for  $S(x) \neq 0$  in

$$\tilde{\mathbf{H}}(\mathbf{x}) = \mathbf{J}_{\tilde{\mathbf{R}}}^{T}(\mathbf{x})\mathbf{J}_{\tilde{\mathbf{R}}}(\mathbf{x}) + \underbrace{\sum_{i=1}^{n} \frac{\partial^{2} \tilde{\varepsilon}_{i}(\mathbf{x})}{\partial x_{j} \partial x_{k}} \tilde{\varepsilon}_{i}(\mathbf{x})}_{\mathbf{S}(\mathbf{x}) \neq \mathbf{0}} + \dots, \forall j, k = 1, \dots, n$$

$$= \mathbf{J}_{\tilde{\mathbf{R}}}^{T}(\mathbf{x})\mathbf{J}_{\tilde{\mathbf{R}}}(\mathbf{x}) + \mathbf{S}(\mathbf{x}) + \dots$$
(6.17)

According to Eq. (6.12), one obtains using Eqs. (6.14 and 6.17) the following equation:

$$\tilde{\mathbf{J}}_{\tilde{\mathbf{F}}}(\mathbf{x})\delta\mathbf{x} \equiv \tilde{\mathbf{H}}(\mathbf{x})\delta\mathbf{x} = -\tilde{\mathbf{F}}(\mathbf{x}) 
\Leftrightarrow \left[\tilde{\mathbf{J}}_{\tilde{\mathbf{R}}}^{T}(\mathbf{x})\tilde{\mathbf{J}}_{\tilde{\mathbf{R}}}(\mathbf{x}) + \mathbf{S}(\mathbf{x})\right]\delta\mathbf{x} = -\tilde{\mathbf{J}}_{\tilde{\mathbf{R}}}^{T}(\mathbf{x})\tilde{\mathbf{R}}(\mathbf{x})$$
(6.18)

The solution vector  $\mathbf{x}$  is converged if the norm of the residual function  $\tilde{\phi}(\mathbf{x})$  is less than the given tolerance  $\varepsilon$  of about  $10^{-6}$ , cf. Eq. (6.15).

Using the computing scheme, the *NL* equation system (6.1) is iteratively solved with the Levenberg-Marquardt algorithm, as shown in Fig. 6.3.

![](_page_105_Figure_10.jpeg)

Fig. 6.3 Levenberg-Marquardt algorithm for a strongly NL equation system

<span id="page-106-0"></span>The Levenberg-Marquardt algorithm (LM) is very suitable for strongly NL equation systems. However, the convergence of solutions strongly depends on the guessed values of the initial vector x0. Therefore, the initial values should be appropriately selected for the dealing nonlinear equation system.

### 6.4 Solving NL Equation Systems with MATLAB

Both Gauss-Newton (GN) and Levenberg-Marquardt (LM) algorithms are adapted to fsolve in MATLAB for solving NL equation systems with n unknowns. The GN algorithm is suitable for weakly NL or linear equation systems. For strongly NL equation systems, the LM algorithm is recommended with appropriate initial values for the unknowns.

The MATLAB code using fsolve is written for solving nonlinear equation systems fX(X) = 0 with n unknowns of the unknown vector X(i).

```
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%%%
% Solving a NL equation system of n unknowns X(i)
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%%%
Function NLES_GN_LM % NonLinear Equation Systems with GN and LM
%
%Input for iteration
iter1 = 1000;
iter2 = 1000;
epsX = 1E-6;
%
% Input for initial values of unknowns X(i),i=1,…,n:
X0_1 = 0.1;
X0_2 = 0.1;
…
X0_n = 0.1;
%
% Initial values for n unknowns X(i),i=1,…,n:
X0(1) = X0_1;
X0(2) = X0_2;
…
X0(n) = X0_n;
%
% @Eq_System of n unknowns X(i)
% The Gauss-Newton algorithm for weakly NLES
% Options = optimoptions ('fsolve', 'Display', 'Iter-detailed',…
% 'TolFun', epsX, 'TolX', epsX, 'MaxFunctionEvaluations',…
```

```
% iter1,'MaxIterations', iter2, 'Algorithm',…
% 'trust-region-dogleg', 'StepTolerance', 1e-12);
%
% The Levenberg-Marquardt algorithm for strongly NLES
Options = optimoptions ('fsolve', 'Display', 'Iter-detailed',…
      'TolFun', epsX, 'TolX', epsX, 'MaxFunctionEvaluations',…
      iter1,'MaxIterations', iter2, 'Algorithm',…
      'levenberg-marquardt','StepTolerance', 1e-12);
%
%Solving the @Eq_System(X)
[X, fval, exitflag, output] = fsolve(@Eq_System,X0,options);
fval;
exitflag;
output;
%
               Function [fX] = Eq_System(X)
% Unknowns X(i),i=1,…,n
X1 = X(1);
X2 = X(2);
…
Xn = X(n);
%
% fX = 0 for n nonlinear equations
fX(1) = f1(X1,X2,…,Xn);
fX(2) = f2(X1,X2,…,Xn);
…
fX(n) = fn(X1,X2,…,Xn);
return
end
```

### References

- 1. Nguyen-Schäfer, H., Schmidt, J.P.: Tensor Analysis and Elementary Differential Geometry for Physicists and Engineers, 2nd edn. Springer, Berlin, Heidelberg (2017)
- 2. Quarteroni, A., Saleri, F., Gervasio, P.: Scientific Computing with MATLAB and Octave, 4th edn. Springer, Berlin, Heidelberg (2014)
- 3. Nguyen-Schäfer, H.: Computational Design of Rolling Bearings. Springer International Publishing, Switzerland (2016)
- 4. Antia, H.M.: Numerical Methods for Scientists and Engineers, 2nd edn. Birkhäuser, Basel-Boston-Berlin (2002)

### <span id="page-108-0"></span>Appendix A

### **Calculating the System Lifetime**

In practice, a mechanical system consists of n independent components, e.g. bearings or gears. The system will fail if any component in it does not function. Therefore, the system lifetime depends on the individual lifetime of each component.

The percentile lifetime  $L_p$  is written using two-parameter Weibull distribution as [1]

$$L_p = \eta \left[ -\ln\left(1 - \frac{p}{100}\right) \right]^{\frac{1}{\beta}} = \eta \left(\ln\frac{1}{S}\right)^{\frac{1}{\beta}}$$

$$\therefore \ln S(t) = -\left(\frac{L_p}{\eta}\right)^{\beta} \Rightarrow S(t) = \exp\left[-\left(\frac{L_p}{\eta}\right)^{\beta}\right]$$

For p = 10 (i.e. 10% failure probability), the lifetime  $L_{10}$  results from the scale parameter  $\eta$  and the shape parameter  $\beta$  as [1]

$$L_{10} = \eta \left( \ln \frac{1}{0.9} \right)^{\frac{1}{\beta}} \Rightarrow \frac{1}{\eta^{\beta}} = \frac{-\ln 0.9}{L_{10}^{\beta}}$$

The survival probability function  $S_{sys}(t)$  of the system is calculated from the individual survival probability functions  $S_i(t)$  of the n independent components as

$$S_{\text{sys}}(t) = \prod_{i=1}^{n} S_i(t) = S_1(t) \times S_2(t) \times \cdots \times S_n(t)$$

Let the shape parameters of the components  $\beta_i$  be equal to shape parameter of the system  $\beta$  (called Weibull's slope). Using two-parameter Weibull distribution, one obtains the survival probability function

$$S_{sys}(t) = \exp\left[-\left(\frac{t}{\eta_{sys}}\right)^{\beta}\right] = \prod_{i=1}^{n} S_{i}(t)$$
$$= \prod_{i=1}^{n} \exp\left[-\left(\frac{t}{\eta_{i}}\right)^{\beta_{i}}\right] = \exp\left[-\sum_{i=1}^{n} \left(\frac{t}{\eta_{i}}\right)^{\beta}\right]$$

Therefore.

$$\left(\frac{t}{\eta_{sys}}\right)^{\beta} = \sum_{i=1}^{n} \left(\frac{t}{\eta_{i}}\right)^{\beta} \Leftrightarrow \frac{1}{\eta_{sys}^{\beta}} = \sum_{i=1}^{n} \frac{1}{\eta_{i}^{\beta}}$$

Using the relation between the lifetime  $L_{10}$  and the scale parameter  $\eta$ , the fatigue lifetime of the system consisting of n independent components results as

$$\begin{split} \frac{1}{\eta_{sys}^{\beta}} &= \sum_{i=1}^{n} \frac{1}{\eta_{i}^{\beta}} \\ &\Rightarrow \frac{-\ln 0.9}{L_{10,sys}^{\beta}} = -\sum_{i=1}^{n} \frac{\ln 0.9}{L_{10,i}^{\beta}} = -\ln 0.9 \times \sum_{i=1}^{n} \frac{1}{L_{10,i}^{\beta}} \end{split}$$

Having dropped the term of  $-\ln$  (0.9) in both sides of the above equation, one obtains the system lifetime  $L_{10,sys}$ 

$$\frac{1}{L_{10,sys}^{\beta}} = \sum_{i=1}^{n} \frac{1}{L_{10,i}^{\beta}} \Rightarrow L_{10,sys} = \left(\sum_{i=1}^{n} L_{10,i}^{-\beta}\right)^{-1/\beta}$$
(A.1)

Some different Weibull's slopes > 0 are usually applied [2, 3]

- $\beta = 10/9$  to ball bearings;
- $-\beta = 9/8$  to roller bearings;
- $-\beta = 2.3$  to greases;
- $-\beta = 1.125$  to gearboxes;
- $-\beta = 2.5$  to gears.

In this case, the system lifetime  $L_{10,sys}$  is calculated using the two-parameter Weibull distribution as

$$\frac{1}{L_{10,sys}^{\beta}} = \sum_{i=1}^{n} \frac{1}{L_{10,i}^{\beta}} \Leftrightarrow \sum_{i=1}^{n} \left(\frac{L_{10,sys}}{L_{10,i}}\right)^{\beta} = 1$$

where

*n* is the components of the system;

 $\beta$  is the Weibull slope;

 $L_{10i}$  is the individual lifetime of the component i.

The fraction at 10% failure probability of the component i is defined as

$$f_{10,i} \equiv \left(\frac{L_{10,sys}}{L_{10,i}}\right)^{\beta}$$
 for  $i = 1,...,n$ .

Generally, the fatigue lifetime of a system that consists of two or more different component groups, e.g. n bearings and m gears in a gearbox is computed with different Weibull slopes of  $\beta_b$  and  $\beta_g$  as

$$\frac{1}{(Lh_{10})^{\beta}} = \sum_{i=1}^{n} \frac{1}{(Lh_{10,i})^{\beta_{b}}} + \sum_{j=1}^{m} \frac{1}{(Lh_{10,j})^{\beta_{g}}}$$

$$\Rightarrow Lh_{10} = \left(\sum_{i=1}^{n} \frac{1}{(Lh_{10,i})^{\beta_{b}}} + \sum_{j=1}^{m} \frac{1}{(Lh_{10,j})^{\beta_{g}}}\right)^{-1/\beta} \tag{A.2}$$

where  $\beta$  is the combined Weibull slope of the gearbox that is determined using least squares method in the Weibull plot.

### References

- Nguyen-Schäfer, H.: Computational Design of Rolling Bearings. Springer International Publishing, Basel-Boston-Berlin (2016)
- Lugt, P.M.: Grease Lubrication in Rolling Bearings. Tribology Series. Wiley, The Netherlands (2013)
- Zaretsky, E.V.: Design of Oil-Lubricated Machine Components for Life and Reliability. Gear Technology (2007)

### <span id="page-111-0"></span>Appendix B

### **Linear Regression Analysis**

Using least squares method (LSM), the regression line of n measured testing samples is determined so that the sum of residuals squared of the regression model is minimized [1, 2]. Note that the residual  $\varepsilon_i$  is the discrepancy between the ordinates of the point  $P_i$  and its corresponding point on the fitted regression line, s. Fig. B1.

The equation of the fitted regression line is written in the coordinate system x–y as

$$y = a + bx \tag{B.1}$$

where b is the slope of the regression line.

Each point of the *n* testing samples is expressed as  $P_i(x, y)$  for i = 1, 2, ..., n in the diagram x–y. The ordinates  $y_i$  of the testing samples can be written in their relating abscissas  $x_i$  and residual  $\varepsilon_i$  as

$$y_i = y + \varepsilon_i = (a + bx_i) + \varepsilon_i$$
  

$$\Rightarrow \varepsilon_i = y_i - a - bx_i$$
(B.2)

The main task of the least squares method is to minimize the sum of residuals squared  $S_R$  of the linear regression model in Eq. (B.2).

$$S_R(a,b) \equiv \sum_{i=1}^n \varepsilon_i^2 = \sum_{i=1}^n (y_i - a - bx_i)^2 = \min.$$
 (B.3)

To find the minimum of  $S_R$  (a,b), the partial derivatives with respect to the parameters a and b must vanish together:

<span id="page-112-0"></span>**Fig. B1** Regression line y(x) of n testing samples

![](_page_112_Figure_3.jpeg)

$$\frac{\partial S_R(a,b)}{\partial a} = -2\sum_{i=1}^n (y_i - a - bx_i) = 0;$$

$$\frac{\partial S_R(a,b)}{\partial b} = -2\sum_{i=1}^n [(y_i - \overline{y}) - b(x_i - \overline{x})] \cdot (x_i - \overline{x}) = 0$$
(B.4)

where  $\overline{x}$  and  $\overline{y}$  are defined as the arithmetic mean values of x and y, respectively. Having solved the above equations, the estimated parameters  $a^*$  and  $b^*$  of the regression line result as

$$b^* = \frac{\sum_{i=1}^{n} (x_i - \bar{x}) \cdot (y_i - \bar{y})}{\sum_{i=1}^{n} (x_i - \bar{x})^2} = \frac{\sum_{i=1}^{n} x_i y_i - \frac{1}{n} \sum_{i=1}^{n} x_i \cdot \sum_{j=1}^{n} y_j}{\sum_{i=1}^{n} x_i^2 - \frac{1}{n} (\sum_{i=1}^{n} x_i)^2}$$

$$= \frac{n \cdot (\bar{x}\bar{y} - (\bar{x} \cdot \bar{y}))}{n \cdot (\bar{x}^2 - \bar{x}^2)} = \frac{n \cdot \sigma_{xy}}{n \cdot \sigma_x^2} = \frac{n \cdot Cov(x, y)}{n \cdot Var(x)} = \frac{Cov(x, y)}{Var(x)}$$
(B.5)

and

$$a^* = \overline{y} - b^* \overline{x} = \frac{1}{n} \left( \sum_{j=1}^n y_j - b^* \sum_{i=1}^n x_i \right)$$
 (B.6)

Therefore, the estimated regression line is written as

$$y = a^* + b^*x \tag{B.7}$$

<span id="page-113-0"></span>Substituting Eq. (B.6) into Eq. (B.7), one obtains

$$y - \overline{y} = b^*(x - \overline{x}) \equiv r_{xy} \frac{\sigma_y}{\sigma_x} (x - \overline{x});$$
with  $b^* \equiv r_{xy} \frac{\sigma_y}{\sigma_x}$ 
(B.8)

Therefore,

$$\hat{y} \equiv \frac{(y - \overline{y})}{\sigma_y} = r_{xy} \frac{(x - \overline{x})}{\sigma_x} \equiv r_{xy} \hat{x}$$
 (B.9)

where  $r_{xy}$  is defined as the slope of the regression line of the standardized coordinates  $\hat{x}$  and  $\hat{y}$  of x and y through the origin  $(\bar{x}, \bar{y})$ .

In fact, the slope  $r_{xy}$  is the sample correlation coefficient between the coordinates x and y of the measured data and results from Eqs. (B.5–B.8) as

$$r_{xy} = b^* \frac{\sigma_x}{\sigma_y} = \frac{Cov(x, y)}{Var(x)} \cdot \frac{\sigma_x}{\sigma_y} = \frac{\sigma_{xy}}{\sigma_x^2} \cdot \frac{\sigma_x}{\sigma_y} = \frac{\sigma_{xy}}{\sigma_x \sigma_y}$$

$$= \frac{S_{xy}}{\sqrt{S_{xx}S_{yy}}}$$
(B.10)

where the sums of squares of x and y are defined as

$$S_{xx} \equiv \sum_{i=1}^{n} x_i^2 - \frac{1}{n} \left( \sum_{i=1}^{n} x_i \right)^2 = n\sigma_x^2 \equiv nVar(x);$$

$$S_{yy} \equiv \sum_{j=1}^{n} y_j^2 - \frac{1}{n} \left( \sum_{j=1}^{n} y_j \right)^2 = n\sigma_y^2 \equiv nVar(y);$$

$$S_{xy} \equiv \sum_{i=1}^{n} x_i y_i - \frac{1}{n} \sum_{i=1}^{n} x_i \sum_{j=1}^{n} y_j = n\sigma_{xy} \equiv nCov(x, y)$$

The population correlation coefficient  $\rho$  is calculated as

$$\rho = \frac{\sum_{i=1}^{n} (x_i - \overline{x}) \cdot (y_i - \overline{y})}{\sqrt{\sum_{i=1}^{n} (x_i - \overline{x})^2} \cdot \sqrt{\sum_{i=1}^{n} (y_i - \overline{y})^2}} = \frac{n\sigma_{xy}}{\sqrt{n}\sigma_x \cdot \sqrt{n}\sigma_y}$$

$$= \frac{\sigma_{xy}}{\sigma_x \sigma_y} = r_{xy}$$
(B.11)

Equation (B.11) shows that the population correlation coefficient  $\rho$  has the same value of the sample correlation coefficient  $r_{xy}$  of x and y. Note that the nearer the

<span id="page-114-0"></span>absolute value of the population correlation coefficient to one ( $\rho \approx 1$ ), the better the linear fitting becomes.

The coefficient of determination R is defined as the population correlation coefficient squared.

$$R \equiv \rho^2 = \left(\frac{\sigma_{xy}}{\sigma_x \sigma_y}\right)^2 \tag{B.12}$$

The variance of the estimated parameter  $b^*$  is calculated as

$$Var(b^*) = \frac{\sum_{j=1}^{n} \varepsilon_j^{*2}}{(n-2) \sum_{i=1}^{n} (x_i - \overline{x})^2}$$
(B.13)

The variance of the estimated parameter  $a^*$  results as

$$Var(a^*) = Var(b^*) \cdot \left(\frac{1}{n} \sum_{i=1}^{n} x_i^2\right)$$

$$= \frac{\sum_{j=1}^{n} \varepsilon_j^{*2} \cdot \sum_{i=1}^{n} x_i^2}{n(n-2) \sum_{i=1}^{n} (x_i - \overline{x})^2}$$
(B.14)

### References

- 1. McCool, J.I.: Using the Weibull Distribution. Wiley, Hoboken (2012)
- Nguyen-Schäfer, H.: Computational Design of Rolling Bearings. Springer International Publishing, Basel-Boston-Berlin (2016)

### <span id="page-115-0"></span>Appendix C

### Cluster-Weighting Modelling (CWM)

To generate the accelerated load spectrum discussed in Chap. 5, a cluster-weighting model is used in the machine learning for clustering the data points in k clusters. The data points are collected online by applying artificial intelligence (AI) and machine learning (ML) to different operational conditions. The CWM is a computing algorithm based on nonlinear predictions of dependent output values from the independent input variables.

The joint probability density function of a measured data point (y, x) is calculated as

$$p(y,x) = \sum_{j=1}^{k} w_j \cdot p_j(y,x)$$
 (C.1)

where

wj is the weight of the function pj(y, x) for cluster j; pj(y, x) is the joint probability density function for cluster j.

Obviously, the sum of the weights of all k clusters must be equal one as

$$\sum_{j=1}^{k} w_j = 1 (C.2)$$

The probability density function for cluster j is decomposed into

$$p_j(y,x) = p_j(y|x) \cdot p_j(x)$$
 (C.3)

Firstly, the conditional probability density function pjðy xj Þ for cluster j predicts the output value y from input variable x in cluster j. It can be computed using linear or nonlinear regression models from the measured data in cluster j [1], cf. App. B.

<sup>©</sup> Springer Nature Switzerland AG 2019

<sup>111</sup>

Secondly, the density function  $p_j(x)$  of variable x in cluster j results from its measured data using the fitting method [1, 2]. Moreover, the Gaussian probability density function of the normal distribution could be applied to the density function  $p_j(x)$  as follows:

$$p_j(x) \equiv g(x) = \frac{1}{\sigma\sqrt{2\pi}} \exp\left[-\frac{1}{2} \left(\frac{x - \langle x \rangle}{\sigma}\right)^2\right]$$
 (C.4)

where

- $\sigma$  is the standard deviation;
- $\langle x \rangle$  is the expectation value in cluster j, cf. [3].

The mean value  $\overline{x}$  of the variables x for cluster j becomes the expectation value when the number N of the data points is very large according to the strong law of statistics [1, 3].

$$\lim_{N \to \infty} \overline{x} \equiv \lim_{N \to \infty} \frac{1}{N} \sum_{i=1}^{N} x_i = \langle x \rangle$$
 (C.5)

### References

- Nguyen-Schäfer, H.: Computational Design of Rolling Bearings. Springer International Publishing, Basel-Boston-Berlin (2016)
- 2. McCool, J.I.: Using the Weibull Distribution. Wiley, Hoboken (2012)
- Nguyen-Schäfer, H., Schmidt, J.P.: Tensor Analysis and Elementary Differential Geometry for Physicists and Engineers, 2nd edn. Springer, Berlin, Heidelberg (2017)

### <span id="page-117-0"></span>Index

| A                                        | Equivalent load, 56               |
|------------------------------------------|-----------------------------------|
| Accelerated load spectrum, 87            | Extended bearing lifetime, 25, 57 |
| Additional torque, 56                    | Extended lifetime, 27, 59         |
| Artificial intelligence, 88              |                                   |
| Autonomy driving, 88                     | F                                 |
| Axial bearing stiffness, 28, 60          | Failure probability, 27, 59, 103  |
| Axial endplay, 79                        | Frictional power, 56              |
| Axial force, 73                          |                                   |
|                                          | Н                                 |
| В                                        | Helical angle, 73                 |
| Barus coefficient, 21                    | Helical gear, 73                  |
| Bending stiffness, 29, 61                | Hertzian contact width, 17, 53    |
|                                          | Hertzian pressure, 18, 53         |
| C                                        | Hessian matrix, 100               |
| Centrifugal force, 16                    |                                   |
| Cluster mean, 92                         | I                                 |
| Cluster-Weighting Modelling (CWM), 111   | Influence factor, 81              |
| Coefficient of determination, 110        |                                   |
| Contact stiffness coefficient, 7, 47, 49 | J                                 |
| Contour profile, 7, 47                   | Jacobian, 96                      |
| Curb, 3                                  |                                   |
| Cylinder roller bearings, 41             | K                                 |
| -                                        | Kerb, 3                           |
| D                                        | Kerb load, 31, 63                 |
| Damage number, 87, 88                    | k-means algorithm, 92             |
| Dynamic load rating, 26, 58              |                                   |
|                                          | L                                 |
| E                                        | Least squares method, 97          |
| Effective elasticity module, 17, 53      | Load parameter, 21                |
| Endplay, 79                              | Load torque, 55                   |

114 Index

| M                                          | Reusner's factor,<br>26, 58            |
|--------------------------------------------|----------------------------------------|
| Machine learning, 88, 92, 111              | Roelands index, 21                     |
| Material parameter, 21                     |                                        |
| Minimum load, 14                           | S                                      |
| Modified<br>reference lifetime, 26, 58     | Sample correlation coefficient,<br>109 |
|                                            | Scale parameter, 103                   |
| N                                          | Shape parameter, 103                   |
| Normal pressure angle, 75                  | Speed parameter, 20                    |
|                                            | Survival probability, 103              |
| O                                          |                                        |
| O arrangement, 4                           | T                                      |
| Oil-film<br>thickness, 18                  | Tangential force, 73                   |
| Operating axial endplay, 81                | Tapered roller bearings, 1             |
| Operating pressure angle, 74               | Tilting angle, 45                      |
|                                            | Time to Market (TTM), 91               |
| P                                          | Transforming factor, 81                |
| Park-locking wheel, 42                     |                                        |
| Pitch radius, 43                           | V                                      |
| Population correlation coefficient,<br>109 | Viscous torque, 56                     |
| Preload, 15                                |                                        |
| Pulse Width Modulation (PWM), 37, 69       | W                                      |
|                                            | Weibull's slope,<br>27, 59, 103        |
| R                                          | Wöhler curve,<br>93                    |
| Radial bearing stiffness, 28, 59           |                                        |
| Radial force, 74, 75                       | X                                      |
| Reusner's correction factor,<br>7, 47      | X arrangement, 3                       |