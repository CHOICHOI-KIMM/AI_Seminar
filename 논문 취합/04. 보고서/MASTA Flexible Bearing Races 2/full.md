# MASTA Flexible Bearing Races

# MASTA Flexible Bearing Races

# SMT

Within many transmission analysis models and the calculations done by many bearing manufacturers it is assumed that rolling bearing races are rigid. All deflection is considered to be Hertzian at the contact and the bearing races are not allowed to go out of round  
This assumption is valid in many cases however there are a significant number of cases where the distortion of bearing races can have a significant impact on bearing load distribution and life.  
One well known application in which this effect is important is bearings which support planet gears.

![](images/4e76fbe2b9c8589344078dd34bd1261b69ff1c07d2809fbf55849d35af2f05da.jpg)

MASTA contains functionality for the inclusion of the flexibility of bearings and analyses its effect on the bearing load distributions and lives.  
Two methods are implemented:

A method based on a well known method by Harris, T. A. Harris, "Rolling Bearing Analysis (fourth edition)", Wiley and Sons (2001). This is an analytical method specific to bearings mounted within planet gears in planetary gear sets.  
An FE based approach which includes bearing races as part of an Imported FE component. This case is very general and not restricted to planet bearings.

This document discusses the analysis of a planetary bearing with flexible outer race and compares results from the two different analysis methods.

![](images/f310509f747caca21285ef6d641c355f63b89ea763d8ad09e3fa9d426307ce14.jpg)

![](images/f75a450f36c9d886912dedde6c7c94e40b788ac2bf8c0dc66caa6e44fdfdd1f3.jpg)

# The Harris solution

In Harris' book 'Rolling Bearing Analysis' a scheme for considering race deflection for bearings mounted within planet gears is outlined.  
> Harris considers the effect on the outer race of the tangential gear force, separating gear force, and moment on the gear teeth.

![](images/c3dbccc64c79700f41b903d14e941421942324d3d978d293baa87fbbf94d6777.jpg)

The Harris solution is based on an influence coefficient matrix for a thin circular ring calculated via classical elasticity methods by Lutz.

![](images/cf6ed3791de93267f50054b2c63d5a437923e448e73e89fc1b916a3f13171d94.jpg)

$$
\delta_ {j} = \frac {R ^ {3}}{\pi E I} q _ {i} \sum_ {m - 2} ^ {m - \infty} \frac {\cos (m (\theta_ {i} - \theta_ {j}))}{(m ^ {2} - 1) ^ {2}}
$$

Deflection at angular position  $\Theta_{j}$  due to normal load  $q$  at  $\Theta_{i}$

In the original Harris solution the assumption is made that the loads are symmetrically located about a diameter, and thus only considers half the bearing elements.  
In general however the load on the planetary bearing will not be symmetric along a diameter which cuts through an element and so MASTA does not make this assumption.

The Harris solution in the analysis for planetary gear bearings is set by selecting "Include Gear Blank Elastic Distortion?", either in:

Load Cases and Duty Cycle Mode.  
System Deflection in the Properties Window with a Load Case or Duty Cycle selected.

![](images/e31723a32225425fd68f9aebede9ed34f2521d3091efc2983342f7d4600ab77e.jpg)

![](images/85ba08fa18d215ec699f8ddb38247a1f2406c52c7b85937156be66164632c1bf.jpg)

# Imported FE Component Solution

Using MASTA's Imported FE Component functionality an FE model of the inner/outer shaft is imported into MASTA with nodes at locations of contact between bearing elements and the ring.  
This FE Model provides MASTA with a stiffness matrix which relates force and deflections for a general housing/race geometry.  
The Imported FE component is connected to the MASTA bearing model and the FE stiffness is used within the bearing models in MASTA's System Deflection calculation to calculate its effect on the bearing load distribution at each iteration of the system model  
See further MASTA documentation on Imported FE Components for further information on setting up Imported FE Components for MASTA

$\gg$  With either the Harris solution option selected or the FE model included the MASTA System Deflection Analysis is run as usual.  
> Note: Load Cases and Duty Cycles can be easily set up with and without these effects on for easy comparison. The effect on the bearing load distribution and therefore lives will be seen.  
$\gg$  With the bearing selected additional results within the Bearing Results Tab give Race Deflections and Race Separations at the elements (where race separations include the relative centre line displacements of inner and outer races)

![](images/d442cefb1d971596987515d72ee9d9827ee3256b055351a0cae7304f3c09dd10.jpg)

# Comparison of Methods

> Results using both the Harris solution and the Imported FE solution can be compared within MASTA.  
An article by Mignot et. al. [3] L. Mignot, L. Bonnard, V. Abousleiman, "Analysis of Load Distribution in Planet Gear Bearings", Gear Technology Magazine (September 2011) considers outer race deflection for a cylindrical roller and a taper roller bearing planetary bearing.  
The article contained results both for FE models and for their analysis using the Harris model for race deflection:

The element loads after the race has distorted.  
The 'ring radial displacement' (i.e. the encroachment of the races on the elements).

These results were read manually off enlarged versions of the figures in the article.

# The Mignot (cylindrical) bearing

$\succ$  MASTA models were created with the same parameters as those of the article for direct comparison.  
Measurements are given in the article however the not all details are provided, e.g.

Roller profiling details  
Bearing material  
Gear properties

Given bearing properties  

<table><tr><td>Number of elements</td><td>12</td></tr><tr><td>Roller diameter (mm)</td><td>12.5</td></tr><tr><td>Roller length (mm)</td><td>40</td></tr><tr><td>Radial internal clearance (mm)</td><td>0</td></tr><tr><td>Outer race second moment of area (mm4)</td><td>3,081</td></tr><tr><td>Outer race mean radius (mm)</td><td>70.5</td></tr></table>

Given gear properties  

<table><tr><td>Pitch radius (mm)</td><td>79.5</td></tr><tr><td>Tangential force (N)</td><td>27,096</td></tr><tr><td>Separating force (N)</td><td>9,862</td></tr><tr><td>Moment (N.mm)</td><td>249,372</td></tr></table>

# Comparison of results

A MASTA model was built according to the dimensions provided in the article.  
Within the MASTA model an Imported FE Component was created and set up to represent the planet gear blank (see next couple of slides).  
System Deflection analyses were run for a Load Case corresponding to the loading specified in the article.  
Results were compared for analyses including the Imported FE component with analyses where the analytical method is used.

# MASTA FE Model Set Up

For the results of the FE based method, a design for the outer race (ring) as specified by the Mignot article was made in CAD and imported into ANSYS Workbench.  
The inner surface of the race was split into 24 equally sized patches – one patch per element with a patch between to allow for spacing between each element.

![](images/52dab805dbd1ae5dcf5e223e054352ec2be72328abd89b2b83718d1ca13ef2f1.jpg)

# MASTA FE Model Set Up

Additionally, two nodes were created at the pitch radius of the gear to allow the influence of the gear forces.  
> Stiffness reduction was performed in ANSYS and the stiffness and node positions imported into MASTA.

![](images/34cea48594795f4674d7b28cd8b2a5f2ab0e3ff9705184888a05cac5fe925b38.jpg)

![](images/24e10ec7833989301689023717fefd7ef7f6d35707eb1784615119c3f78cec0d.jpg)

The nodes for the bearing elements were connected to the planet bearing in the MASTA model and the gear mesh nodes connected to the sun and annulus gears.

# Comparison of results

The figure below shows a comparison of individual rolling bearing element loads for the two MASTA methods together with the results presented for the "Harris Modified" model in the Mignot et. al. article

![](images/671c1dfc1f909b3d4b6ffdeb8c7390b7dd5c8d2010c3530b39c4941130831093.jpg)

# Comparison of results

# SMT

The calculated element loads for the MASTA analytical model are in good agreement with the results of the Mignot Modified Harris model.  
Further both analytical models are in good agreement with those of the MASTA analysis using FE.  
As element normal loads are in good agreement further analysis rating results in MASTA such as predicted lives and safety factors for the analytical and Imported FE methods are also in good agreement for this case.  
As the FE results for the element loads in the FE analysis done by Mignot et al. are in good agreement with their analytical models the same is also true as compared to all results in MASTA.

# Comparison of results

The figure below shows a comparison of the calculated radial displacements of the bearing races at the element locations.

![](images/006d036e393d46ed83b9698548cceceadb5315cd966ada2edac79c4ec85f2377.jpg)

# Comparison of results

# SMT

The calculated radial displacements of the bearing races at the element locations are in good agreement with the results of the Mignot modified Harris model.  
The displacements of the analytical model are in good agreement with the FE model where the displacements are positive.  
The displacements do not agree with the MASTA FE solution so well where displacements are negative.  
Negative displacements indicate separation and as can be seen on the chart of element loads these are the locations where the elements are not loaded.  
The same conclusion was reached by Mignot who explained the differences as being a consequence of the effects of tension and shear which are not taken into account in the analytical model.  
As the differences are only where elements are not loaded they do not impact on bearing lives.

# Comparison of results

The figure below shows a comparison of the element loads for the MASTA Analytical Model and for the same model without the inclusion of race distortion.

![](images/fd7516f483247235f94b092d9c7eebfc25888e973cb13a068504dc8816fddff1.jpg)

# Comparison of results

The Inclusion of the race distortion effect in this case significantly effects the load distribution between elements.  
The race distortion flattens the race in a certain direction leading to a spread of the load between the elements loading more elements.

![](images/33d9db399d76b31336b5c6bf526595317f5d357df90479d0dabc0f6b1fac87c9.jpg)  
MASTA Analytical Model

![](images/111b968ea3fbfd85c17a4d0063d4a438edeebc8edb027d6c496c661309789210.jpg)  
Gear Contacts are at Elements 1 & 7

![](images/5fd9ba21862851277e5e615f8904a75ea92e4ae5b46641260b07aa1b0c9d9dd6.jpg)  
No Race Distortion

# Conclusions

MASTA contains two methods for the inclusion of bearing race flexibility.

An analytical method for planet bearings is based on a well known method by Harris.  
This method is trivial to include in the analysis.  
An alternative method using Imported FE components is less trivial to set up but much more general in approach and can model bearing races of any shape such as those integrated into a housing.

A comparison of the two methods with each other, in the case where they are both applicable, and with results presented in the literature show good agreement.  
For certain cases where the support of the bearing races is relatively thin the effect of race deflection on bearing loading and lifetime can be significant.

# Thank you for your attention

![](images/aba3d1706c12489c2b942f693e88cf665a3d7c497e5eded3f273ee404955239d.jpg)

SMT LLC - North America

![](images/17d78fc5e6aef491a1de523e85afaa83f5e201781fe5505cbdf76c2de96724ff.jpg)

SMT UK - Nottingham HQ & Testing Facility

![](images/db72c5cfc194ddf60bf99b79281a853586c8d5a1c912b448dfcf3829f5a101ae.jpg)

SMT Portugal

SMT China - Beijing

![](images/bef0c1975a1a7e392943711b50c343533a0c78765b73d251cceecc00136778ba.jpg)

SMT China - Shanghai

SMT Japan

![](images/ccb055ecb235133c37a257355d7f1e8f13ac39be721232e579cfb9fb5f44eeb5.jpg)

# SMT

CHARTWELL HOUSE, 67-69 HOUNDS GATE

NOTTINGHAM, UK

NG1 6BB

tel. +44 (0) 115 941 9839 | fax. +44 (0) 115 958 1583

www.smartmt.com

Follow us on

![](images/8d83485b27616a192fcac5c63bc9d278b29c29e14bc79e9677ea5307a7345483.jpg)

![](images/22d093ed9e3b79d55f259fb3263c45bc64a298a3404c704b27ad271ea39123e6.jpg)

![](images/bee17d526b4e0d13b07c9cb8cb597b27b47e6aa4d9eaaf2afc721cda700a9502.jpg)