![](_page_0_Picture_1.jpeg)

# A New Running Torque Formula for Tapered Roller Bearings Under Axial Load

S. Aihara

Research Center, Nippon Seiko K.K., Fujisawa, Kanagawa, Japan

Conventional formula for calculating the running torque of tapered roller bearings often showed discrepancy from actual running torque, particularly under axial load. Therefore, an equation was formulated based on the knowledge of EHL rolling resistance and EHL oil film thickness. Careful examination of actual bearing running torque suggested the load dependency of EHL rolling resistance which previous theory did not include. Such load effect was confirmed by means of two disc machine and the equation was partly corrected. A new running torque formula of a tapered roller bearing under axial load was proposed and good agreement with actual bearing torque was confirmed.

#### 1 Introduction

Tapered roller bearings are widely used in automobiles and many industrial machines. Following the worldwide efforts to save energy, recent research on tapered roller bearings has been concentrated on the reduction of frictional torque. Furthermore, rolling bearings are being used under increasingly severe conditions, particularly, higher loads and speeds.

The frictional torque of rolling bearings is usually calculated with a two term equation that includes the torque due to applied load and torque due to viscous drag. This equation [1] is:

$$M = f_1 F_\beta d_m + f_0 (\nu_0 n)^{2/3} d_m^3 \times 10^{-8} \quad (\nu_0 n > 2000)$$

(kgf.mm) (1

where,  $f_1$  = factor depending on bearing design and load;  $F_{\beta}$  = term depending on the magnitude and direction of load;  $d_m$  = bearing pitch diameter (cm);  $f_0$  = factor depending on lubrication;  $\nu_0$  = kinematic viscosity (mm<sup>2</sup>/s); n = rotational speed (rpm).

Equation (1) is based on both theoretical and experimental analysis and applies to tapered roller bearings as well as other types. However, when the axial load is high, including purely axial loads, the measured torque of tapered roller bearings varies greatly from that calculated using equation (1). Figure 1 shows a rather extreme example. In this case, the measured torques are much higher than those calculated with the maximum difference approaching almost 300 percent. But the calculated torque can sometimes be higher than the actual torque depending on the combination of load, speed, and lubricant viscosity.

Secondly, equation (1) was derived before the EHL (elasto-hydrodynamic lubrication) theory was developed. The main application of EHL theory to rolling bearings is determining

Contributed by the Tribology Division of The American Society of Mechanical Engineers and presented at the ASME/ASLE Joint Tribology Conference, Pittsburgh, Pa., October 20-22, 1986. Manuscript received by the Tribology Division. Paper No. 86-Trib-57.

the oil film thickness and its effect on the fatigue life or deterioration of bearings. Since the study of rolling friction is limited, the running torque of tapered roller bearings has been investigated concentrating on EHL viscous rolling resistance.

# 2 Analysis of Running Torque of Tapered Roller Bearings

- 2.1 Running Torque Equation. The friction in tapered roller bearings is composed of the following four parts:
- (1) rolling friction between rollers and inner and outer raceways (elastic hysterisis and EHL rolling resistance)
  - (2) sliding friction between roller ends and inner ring rib
  - (3) sliding friction between rollers and cage pockets
  - (4) drag due to viscosity of lubricant

When the rotational speed is limited, parts (3) and (4) are

![](_page_0_Figure_23.jpeg)

Fig. 1 Comparison between measured torque and torque calculated using Palmgren's equation

![](_page_1_Picture_1.jpeg)

Fig. 2 Forces acting on tapered roller

![](_page_1_Picture_3.jpeg)

Fig. 3 Simplified model for analysis

small compared with parts (1) and (2). Therefore parts (1) and (2) only are included in this analysis.

When a pure axial load is applied to a tapered rolling bearing, three separate loads act on each roller as shown in Fig. 2. The actual loads are distributed loads, however, they are shown as concentrated loads in Fig. 2. For normal radial tapered roller bearing, the outer raceway load  $(Q_e)$  is almost the same as the inner raceway load  $(Q_i)$ , so each load may be expressed as follows:

$$Q_{\rho} = Q_{i} = F_{\alpha}/z \sin \alpha \tag{2}$$

$$Q_f = F_a \sin 2\beta / z \sin \alpha \tag{3}$$

where  $Q_f = \text{load}$  at roller end and rib contact;  $F_a = \text{applied}$  axial load; z = number of rollers;  $\alpha = \text{outer raceway half}$  angle;  $2\beta = \text{roller head angle}$ ; e = height of roller end and rib contact.

Such loads, or forces, act when a bearing is stationary as well as when it is rotating. In the case of rotation, frictional forces and moments are applied on rollers. Since tapered roller bearings are designed so the rollers roll along the raceway without differential sliding or spinning, frictional forces occur that are perpendicular to the plane of Fig. 2.

The frictional forces acting on a roller are as shown in Fig. 3, which is a view in the axial direction. Here, as a simplified model, a tapered roller bearing is approximated by a cylindrical roller bearing with inner-race rib for convenience. The roller diameter  $(D_a)$  was defined as the mean diameter of the roller and the inner and outer raceway radii  $(R_i$  and  $R_0)$  were defined as those at the center of the rollers. Though not precise,  $R_i + D_a$  equals  $R_0$  approximately for radial tapered roller bearings.

All the frictional force are shown in Fig. 3, where,

 $M_i$ ,  $M_0$  = rolling resistance in raceway- roller contact inlet  $F_{si}$ ,  $F_{s0}$  = sliding friction in raceway- roller contact area  $F_{sf}$  = sliding friction in roller end- rib contact area

i, 0, and f correspond to inner, outer, and flange (rib), respectively. When a pure axial load is considered, every roller is subjected to forces and moments shown in Fig. 3. Though the rolling resistance (or friction) is represented by tangential forces by various researchers, it should be represented by moments that impede a roller from rolling as shown later.

From the balance of forces and moments on a roller, the following equations are obtained,

Forces: 
$$F_{s0} - F_{si} = F_{sf} \tag{4}$$

Moments: 
$$M_i + M_0 = \frac{D_a}{2} F_{s0} + \frac{D_a}{2} F_{si} + \left(\frac{D_a}{2} - e\right) F_{sf}$$
 (5)

By combining equations (4) and (5),  $F_{s0}$  and  $F_{si}$  can be eliminated leaving  $M_i$ ,  $M_0$ , and  $F_{sf}$ . The running torque of a tapered roller bearing is defined as the moment acting on the outer ring (M); therefore, considering the number of rollers,

$$M = z(R_0 F_{s0} - M_0) \tag{6}$$

As mentioned above,  $F_{s0}$  can be eliminated from equation (6), and M becomes,

$$M = z \cdot \frac{1}{D_a} (R_0 M_i + R_i M_0) + z \frac{R_0}{D_a} e \cdot F_{sf}$$
 (7)

$$= M_R + M_S \tag{8}$$

Equation (8) can be interpreted in two ways. Firstly, the torque of a tapered roller bearing consists of rolling resistance and sliding friction, or secondly, it consists of friction at the raceways and that at the rib. This means that each component can be evaluated separately.

 $M_i$  and  $M_0$  in equation (7) consists of the EHL viscous rolling resistance and elastic hysterisis; however, with oil lubrication the effect of elastic hysterisis is much smaller than that of viscous resistance. The EHL viscous resistance can be calculated as shown below. On the other hand,  $F_{sf}$  consists of both Coulomb friction and viscous traction (drag), but Coulomb friction is the more important one.  $F_{sf}$  can be calculated as well. M is, therefore, possible to calculate using known parameters.

Before going further, rolling resistance should be discussed. In rolling bearings, the rollers are supposed to simply "roll" along the raceways, but in reality, some slippage exists in the roller-raceway contact areas which causes sliding friction or traction as shown in Fig. 3. This traction is the driving force that causes the rollers to rotate and is essential. However, this sliding friction is not independent, but is determined by the moments  $M_i$ ,  $M_0$  and the sliding friction between the roller ends and rib  $F_{sf}$ . This means that the driving force is a result of rolling resistance. When the rolling resistance is low, only a small driving force is required and for high rolling resistance, a large driving force is necessary and sometimes results in much sliding, particularly when the load is light.

It should be remembered that we cannot set any slip ratio for rolling bearings beforehand, but that the slip ratio is determined by the operating conditions including lubrication. This is often misunderstood by engineers and researchers when the effect of sliding friction is considered.

2.2 Sliding Friction in Roller End and Rib Contact Area,  $M_s$ .  $F_{sf}$  in the second term of equation (7) is the force caused by the sliding between the roller end and rib. Introducing a coefficient of friction  $\mu$ ,  $F_{sf}$  is expressed by  $\mu Q_f$ . Here,  $Q_f$  is the force against the rib as shown in Fig. 2. For a purely axial load  $(F_a)$ ,  $Q_f$  is given by equation (3).  $M_s$  can be rewritten as follows:

$$M_{s} = z \left(\frac{R_{0}}{D_{a}}\right) e \cdot F_{sf}$$

$$= z \left(\frac{R_{0}}{D_{a}}\right) e \cdot \mu \cdot Q_{f}$$

$$= z \left(\frac{R_{0}}{D_{a}}\right) e \cdot \mu - \frac{\sin 2\beta}{z \sin \alpha} F_{a}$$

$$= \frac{2R_{0} \cdot \sin\beta}{D_{a} \cdot \sin\alpha} e \cdot \mu \cdot \cos\beta \cdot F_{a}$$
(9)

In Fig. 2,

![](_page_2_Figure_1.jpeg)

Fig. 4 Composition of torque

$$\overline{AB} = \frac{D_a}{2} \frac{1}{\sin\beta} = \frac{R_0}{\sin\alpha}$$

$$\frac{2R_0 \cdot \sin\beta}{D_a \cdot \sin\alpha} = 1$$
(10)

Consequently,

$$M_s = e \cdot \mu \cdot \cos \beta \cdot F_a \tag{11}$$

Usually  $\beta$  is small so  $\cos \beta = 1$ ,

$$M_s = e \mu F_a \tag{12}$$

Equation (11) is the same as one used for the starting torque of tapered roller bearings where  $\mu \approx 0.2$ ; however,  $\mu$  in equation (11) is not constant but changes with the operating conditions.

 $M_s$  was measured using special loose rib experimentally by Korren [2] and Dalmaz, et al. [3]. Both reported that the sliding friction on the rib is the highest when a bearing first starts to rotate and drops rapidly as the rotational speed increases owing to formation of the fluid film. The mechanism of friction change may be explained with a Stribeck chart. The composition of torque is shown in Fig. 4. The sliding friction  $(M_s)$  is related to the degree of film formation or portion of load supported by asperity contacts.

Figure 5 shows the relationship between the mean asperity load and oil film/surface roughness ratio (oil film parameter) based on the analysis by Patir and Cheng [4] using a partial EHL model for low contact pressure less than 0.4 GPa and with relatively longitudinal roughness. This curve is approximated by the equation,

$$F_c/W_{\text{total}} = \exp(-1.8 \ \Lambda^{1.2})$$
 (13)

where,  $F_c$  = mean asperity load;  $W_{\text{total}}$  = total load;  $\Lambda$  = oil film parameter (central film thickness/composite roughness).

The asperity load  $(F_c)$  is the cause of dry friction (Coulomb friction) and the rest of the load that is supported by a fluid film is the cause of fluid-film friction. As mentioned previously, the coefficient of Coulomb friction is around 0.2 and that for fluid-film friction is less than 0.01; therefore, only Coulomb friction is considered.

Figure 5 shows that almost all the load is supported by the fluid film for  $\Lambda$  over 2; therefore, Coulomb friction becomes important for only thin films. In Fig. 5, the amount of asperity load is related to the oil film parameter, but when friction is concerned, the left side of equation (13) is equivalent to the ratio of friction coefficients; hence, equation (13) becomes

$$\mu/\mu_0 = \exp(-1.8\Lambda^{1.2}) \tag{14}$$

and  $M_s$  is

$$M_s = e \cdot \mu_0 \cdot F_a \exp(-1.8\Lambda_r^{1.2}) \tag{15}$$

where  $\mu_0 = 0.2$ . The subscript r on  $\Lambda$  indicates rib contact.

2.3 Oil Film Thickness in Roller End and Rib Contact Area. The shape of the Hertzian contact area between the

![](_page_2_Figure_20.jpeg)

Fig. 5 Ratio of asperity load to total load

roller end and rib is elliptical with the major axis in the direction of rotation at the center of contact, that is, the ellipticity ratio k < 1. This is sometimes called "needle contact." Hamrock and Dowson [5] derived both central and minimum film thickness equations based on a curve fit to different solutions in which ellipticity ratio spanned the range 1 < k < 8. But their equation can be extrapolated for a needle contact.

Gledhill, et al. [6] measured the minimum oil film thickness in needle contacts by means of an interferometric method. They found that measured minimum oil film thickness corresponded well with theoretical predictions. They could not identify the central film thickness since the area corresponding to minimum film thickness occupied a large part of the total area.

Koye and Winer [7] measured the minimum film thickness including the condition of needle contact by means of an optical interferometry technique, too, Their comprehensive experimental study revealed that the Hamrock and Dowson's equation could be used for fractional ellipticity with a reasonable degree of confidence.

Gadallah and Dalmaz [8] measured film thickness with a simulated roller end and rib contact by means of an optical method. Their measurements were smaller than the theoretical values due to frequent starvation. On the other hand, Swingler [9] showed that the lubrication in a roller end and rib contact varied from EHL to isoviscous-elastic or isoviscous-rigid depending on the load and speed, and that oil films can be much thicker than those predicted by EHL film thickness equations.

Gledhill et al. also showed that Hamrock and Dowson's central film is much thicker than the minimum film thickness for needle contacts. Considering these facts, the following film thickness was introduced to calculate  $\Lambda_r$  instead of central film thickness equation by Hamrock and Dowson. The constant multiplier (1.25) is based on the ratio of central/minimum film thickness for moderate EHL conditions.

$$h_0 = 1.25 \times h_{\min H - D} \tag{16}$$

Here,  $h_{\min H-D}$  is Hamrock and Dowson's minimum film

$$h_{\min H-D} = 3.63 \ U^{0.68} G^{0.49} W^{-0.073} (1 - e^{-0.68k}) R_x$$
 (17)

where,  $U = \eta_0 u/E' R_x$ ;  $G = \alpha_0 E'$ ;  $W = F_r/E' R_x^2$ ; k = 1.03  $(R_y/R_x)^{0.64}$ ;  $R_x =$  equivalent radius in the rolling direction;  $\eta_0 =$  lubricant viscosity at ambient pressure; u = rolling speed (average speed);  $R_y =$  equivalent radius perpendicular to rolling direction;  $\alpha_0 =$  pressure coefficient of viscosity; E' = reduced elastic modulus.

**2.4 Rolling Resistance in Roller-Raceway Contact Area**  $(M_i, M_0)$ . The lubrication in roller and raceway contact is usually elasto-hydrodynamic, and most of the rolling friction in such contact areas in tapered roller bearings is EHL rolling resistance due to the viscous lubricant in the inlet.

As for the rolling resistance with rigid contact, the following equation was obtained [10]

$$t_{r,\text{rigid}} = 2.08(\eta_0 u \ w)^{1/2}$$
 (18)

where t, is the tangential force per unit width and w is the load per unit width.

Crook [11] calculated the EHL rolling resistance with a Newtonian fluid and Goksem and Hargreaves [12] rewrote his results in the form of the following equation for isothermal rolling resistance.

$$t_{r,\text{EHL}} = t_{r0} = \frac{4.318}{\alpha_0} (G\ U)^{0.658} W^{0.0126} R_e$$
 (19)

From equation (19), the rolling resistance is known to be mostly independent of applied load.

Goksem and Hargreaves also obtained rolling resistance considering inlet viscous shear heating.

$$t_{r,i-s} = t_r = \frac{R_e W}{\alpha_0} \exp\left(\left(\frac{52}{79} + \frac{1}{94} L^{20/63}\right) \ln D\right)$$

$$-\left(\frac{114}{701} + \ln\left(1 + \frac{86}{209}L^{70/99}\right)\right)\right) \tag{20}$$

where L is the thermal loading parameter and D is the dynamic loading parameter;  $L=\eta_0\beta u^2/k'$ ;  $D=(9\pi/2)^{1/2}\times GU/W^{1.5}$ ;  $\beta=$  temperature coefficient of viscosity; k'= thermal conductivity of lubricant;  $R_e=$  equivalent radius.

They considered the effect of inlet shear heating for a wide range of the thermal loading parameter ( $L=0\sim5000$ ). However, for most rolling bearings, L varies only between zero and several tens. In this range, equations (19) and (20) were compared for different conditions and their ratios were plotted against L. An approximation is possible according to Murch and Wilson [13], and we modified equation (20) as follows:

$$t_r = \left(\frac{1}{1 + 0.29 I^{0.78}}\right) t_{r0} \tag{21}$$

Equation (21) proved to be a reasonable approximation for rolling resistance under EHL conditions by means of a two disk machine [14].

Previous studies have dealt with only tangential force; however, as Johnson [15] indicated, rolling resistance is a moment caused by hydrodynamic action in the contact zone. The pressure distribution shifts forward into the inlet area ahead of the nominal contact zone. This forward displacement of pressure produces a moment which opposes the motion of the rollers, this is the cause of viscous rolling resistance. The moment is the tangential force multiplied by the roller radius  $(t_r \times R)$  for two parallel rollers with same radius against each other. The rollers with different diameters, the rolling resistance is given by  $t_r \times 2R_e$  with the equivalent radius.

$$M_{i,0} = t_r \times 2 \ R_{e,i,0} \tag{22}$$

Here,  $t_r$  is calculated using equation (21) and  $R_e$  is calculated for the inner and outer raceways separately (equation (22), Appendix 1).

Now the running torque can be calculated since the necessary parameters are known.

# 3 Modification of Theoretical Running Torque Equation

3.1 Running Torque Measurement. The measurement of running torque of a single tapered roller bearing is not easy, so a specially designed test rig was used. The primary parts of the rig are shown in Fig. 6. Each test bearing was mounted on the end of a vertical shaft and axial load was applied using a hydrostatic thrust bearing. Hence the bearing housing was free to rotate. Oil was supplied by a circulating-oil system and the

![](_page_3_Picture_20.jpeg)

Fig. 6 Running torque test rig

bearing, as well as the oil, was warmed to a fixed temperature before each torque measurement.

As the inner ring rotated, the bearing's frictional torque tended to turn the bearing housing (outer ring). To measure the running torque, the force needed to restrain the housing was measured and converted to torque. The torque was recorded while slowly and continuously increasing speed to 3300 rpm. The bearings were rather freely chosen from among those with bore diameters over 17 mm and outer ring diameters under 240 mm using different housings and shaftend fittings.

The load varied from 1 kN to 1 4 kN. The measured torques were compared with the theoretical ones, equation (7) in Section 2 with substitutions from equations (22), (21), (19), (17), (16), and (15). One typical example is shown in Fig. 7 for bearing 32307C (35 mm $^{\phi}$  × 80 mm $^{\phi}$  × 32.75 mm). To obtain the oil film parameter ( $\Lambda_r$ ) at the roller end and rib contact, the initial composite roughness was used.

Figure 7 shows that the starting torque and running torque at low speeds both correlate with theory. However, as the speed increases, in this case at speeds over about 300 rpm, the discrepancy between the measured torque and theoretical torque is apparent. The main difference is the load dependency which is large for the measurements but small theoretically. Comparing Fig. 7 with the torque composition in Fig. 4, it is clear that the prediction of  $M_S$  was good and that the cause of the discrepancy lies in  $M_R$ , rolling resistance. Therefore  $M_R$  will be more carefully examined.

3.2 Modification of Rolling Resistance Equation. In Section 2, EHL rolling resistance was said to be the major part of total rolling resistance and elastic hysterisis was negligible. However, the load dependency of rolling resistance in Fig. 7 may be attributed to elastic hysterisis. The measurement of dry rolling resistance with the two-disk machine revealed that the increase with load is too great to attribute to elastic hysterisis.

Therefore, EHL rolling resistance should be carefully reviewed. First, the effect of speed was checked as shown in Fig. 8. The solid lines show the running torque for several bearings and the slope of the broken line corresponds to the isothermal EHL rolling resistance of equation (19). As the speed increases, the measured torque tends to cease increasing because of the effect of inlet shear heating. The speed at which the inlet shear heating appears depends on the bearing size. From Fig. 8, it is apparent that the slope of the measured torques is similar to the theoretical slope in the range where the lines are nearly straight.

The relationship between torque and axial load, which is proportional to the roller load, at those speeds where the measured speed influence corresponds to the theoretical slope in Fig. 8, is shown in Fig. 9. The slope of the broken line is also based on the theoretical resistance of equation (19). The slopes of the measured torques are nearly the same regardless

![](_page_4_Figure_1.jpeg)

Fig. 7 Comparison between measured torque and theoretical torque based on EHL

![](_page_4_Figure_3.jpeg)

Fig. 8 Relationship between speed and running torque

of the bearing size and speed. The relationship is approximated by

$$M_R \propto F_a^{0.3} \tag{23}$$

For a given load, small bearings are subjected to greater contact pressures than large bearings. Therefore,  $M_R$  is plotted against the EHL nondimensional load parameter, W, using the effective roller length, I, and average roller radius,  $D_a/2$ , which is almost equivalent to the reduced radius at the inner and outer raceway and roller contact areas. W is given by

$$W = \frac{2 F_a}{D_a \cdot l \cdot z \cdot \sin \alpha \cdot E'}$$
 (24)

The measured torques were compared with the theoretical torques and the results shown in Fig. 10. For a wide range of bearing sizes, each point falls within a narrow range and can be represented by a single straight line without significant error. Designating the vertical axis as f(W), Fig. 10 can be expressed by

$$f(W) = \left(\frac{2 F_a}{D_a \cdot l \cdot z \cdot \sin \alpha \cdot E'}\right)^{0.3} \times 20.4 \tag{25}$$

3.3 EHL Rolling Resistance Tests. The results of the previous section indicates that the EHL rolling resistance may depend on load. Therefore, the EHL rolling resistance was measured at various loads by means of a two-disk machine.

![](_page_4_Figure_12.jpeg)

Fig. 9 Relationship between axial load and running torque

![](_page_4_Figure_14.jpeg)

Fig. 10 Correction coefficient

The experimental method was reported in detail elsewhere [15]. The disks were supported by externally pressurized air bearings with porous bushing since the rolling resistance is very small. Both disks were 100 mm in diameter and the diametrical difference was kept within 1  $\mu$ m.

Rolling resistances were measured at eight loads, three rolling speeds and with two lubricants, namely

Load 0.13, 0.19, 0.24, 0.3, 0.45, 0.6, 0.75, 0.9 MN/m
max. pressure 0.45, 0.56, 0.62, 0.69, 0.85, 0.98, 1.1, 1.2 GPa
rolling speed viscosity 2, 4, 6 m/s
13, 70 mm²/s

The results are shown in Fig. 11. Though the slopes are slightly different, they are approximately 0.3. Eguchi and Yamamoto [16] also measured EHL rolling resistance with a two disk machine using a different method and obtained a similar load-rolling resistance relationship expressed by  $t_r \propto w^{0.2}$ .

The experimental exponent of 0.2-0.3 corresponds to the one for the correction coefficient f(W) in equation (25). This means that the EHL rolling resistance actually depends upon load. The reason why the conventional theories lack such an

![](_page_5_Figure_1.jpeg)

Fig. 11 Measurement of rolling resistance using two disk machine

![](_page_5_Figure_3.jpeg)

Fig. 12 Comparison between measured torque and revised equation (26)

effect may be attributed to the exclusion of the contact area from the calculations.

# 4 Revised Equation for Tapered Roller Bearing Running Torque

Based on the analyses and experiments reported here, the following equation is being proposed as a new one for tapered roller bearing running torque.

$$M = \frac{z}{D_a} (R_0 M_i + R_i M_0) + e \cdot \mu_0 \cos \beta \cdot F_a \exp(-1.8 \Lambda_r^{1.2})$$
 (26)

where

$$M_{i,0} = \left[ \left( \frac{1.76 \times 10^2}{1 + 0.29 \ L^{0.78}} \right) \frac{1}{\alpha_0} (G \ U)^{0.658} W^{0.31} \cdot R_e^2 \cdot 1 \right]_{i,0}$$
 (27)

Equation (26) was compared with the experimental results of Figs. 1 and 7 and the correlation is excellent as shown by Fig. 12. The agreement between the measured torque and equation (26) was adequately confirmed with data from nearly two hundred bearings tested at standard operating conditions. Equation (26) was also compared with data for various oil viscosities and the agreement was much better than the conventional equation as shown by Fig. 13.

Witte [17] developed an operating torque formula based on the torque measurement of tapered roller bearings and found load dependency similar to equation (27) by the author. However, it was made clear for the first time that the load dependency occurred at the roller-raceway contacts from the analysis and measurement described here.

For a pair of tapered roller bearings with a preload and externally applied loads, equation (26) was also quite accurate as shown by Fig. 14. The torque increased only a little even when a radial load was also applied. Though equation (26) was derived for a purely axial load, it applies also to combined

![](_page_5_Figure_14.jpeg)

Fig. 13 Relationship between viscosity and running torque

![](_page_5_Figure_16.jpeg)

Fig. 14 Running torque of duplex bearing

loads if the load distribution is known. The running torque was calculated assuming a load distribution produced by a combined load as shown by Fig. 14, and it was almost the same as that for a purely axial load. From a subsequent analysis, equation (26) was found to be valid for combined loads as long as every roller is loaded.

#### 5 Conclusion

A revised equation for calculating the running torque of tapered roller bearings was formulated based on EHL rolling resistance and EHL oil film thickness. The influence of external loads on torque was determined experimentally and good correlation with test data was confirmed. This is an improvement over other equations which predicted almost no dependence on an applied load for EHL rolling resistance. It was also made clear that the load dependency of running torque initiates at the roller-raceway contacts experimentally by means of a two disk machine supported by hydrostatic air bearings. Good agreement was confirmed between the proposed running torque equation and experimental torque data

for nearly two hundred tapered roller bearings from 17 mm to 120 mm in bore size.

This revised equation was instrumental in the development of new low-torque tapered roller bearings.

EHL rolling resistance should be examined carefully to explain the load dependency found by the author and with other researcher's experiments.

## Acknowledgments

I want to give credit to Professor H S. Cheng of Northwestern University, Evanston, whose computer program was used to derive Fig. 5.

### References

- Harris, T. A., Rolling Bearing Analysis, Wiley, 1966.
   Korren, K., "Gleitreibung und Grenzbelastung an den Bordflachen von Drehzahl, Belastung, Schmierstoff und Gestaltung nach Versuch und Berechnung," VDI Zeit., Reihe 1 Nr. 11, 1967.
- 3 Dalmaz, G., Tessier, J. F., and Dudgrangne, G., "Friction Improvement in Cycloydal Motion Contacts: Rib-Roller End Contact in Tapered Roller Bears," Proc. 7th Leeds-Lyon Symp. on Tribology, 1980, pp. 175-185.
  4 Patir, N., and Cheng, H. S., "An Average Flow Model for Determining
- Effects of Three-Dimensional Roughness on Partial Hydrodynamic Lubrication," ASME JOURNAL OF LUBRICATION TECHNOLOGY, Vol. 100, 1978, pp. 12-17.
- 5 Hamrock, B. J., and Dowson, D., "Isothermal Elastohydrodynamic Lubrication of Point Contacts. Part III, Fully Flooded Results," ASME Jour-NAL OF LUBRICATION TECHNOLOGY, Vol. 99, 1977, pp. 264-276.
- 6 Gledhill, R. H., Jackson, A., and Cameron, A., "An Interferometric Study of the EHL of Elliptical Contacts Aligned in the Direction of Rolling," Proc. 5th Leeds-Lyon Symp. on Tribology, 1978, pp. 106-120.

  7 Koye, K. A., and Winer, W. O., "An Experimental Evaluation of the
- Hamrock and Dowson Minimum Film Thickness Equation for Fully Flooded EHD Point Contacts," ASME JOURNAL OF LUBRICATION TECHNOLOGY, Vol. 103, No. 2, 1981, pp. 284-294.

  8 Gadallah, N., and Dalmaz, G., "Hydrodynamic Lubrication of the Rib-
- Roller End Contact of a Tapered Roller Bearing," ASME JOURNAL OF TRIBOLOGY, Vol. 106, 1984, pp. 265-274.
- 9 Swingler, C. L., "Regimes of Fluid Film Lubrication at the Rib-Roller Contact in a Tapered Roller Bearing," AGARD-CP-323, 1982.

  10 Archard, J. F., and Baglin, K. P., "Nondimensional Presentation of Fric-
- tional Tractions in Elastohydrodynamic Lubrication-Part I: Fully Flooded Conditions," ASME JOURNAL OF LUBRICATION TECHNOLOGY, Vol. 97, 1975, pp. 398-411.
- 11 Crook, A. W., "The Lubrication of Rollers. IV. Measurements of Friction and Effective Viscosity," Phil. Trans. A, 1056, Vol. 255, 1963, pp. 281-317.
- 12 Goksem, P. G., and Hargreaves, R. A., "The Effect of Viscous Shear Heating on Both Film Thickness and Rolling Traction in an EHL Line Contact. Part 1: Fully Flooded Conditions," ASME JOURNAL OF LUBRICATION TECHNOLOGY, Vol. 100, 1978, pp. 346-358.
- 13 Murch, L. E., and Wilson, W. R. D., "A Thermal Elastohydrodynamic Inlet Zone Analysis," ASME JOURNAL OF LUBRICATION TECHNOLOGY, Vol. 97, 1975, pp. 212-216.
- 14 Johnson, K. L., "Regimes of Elastohydrodynamic Lubrication," Journal of Mechanical Engineering Science, Vol. 12, No. 1, 1970, pp. 9-16.
- 15 Aihara, S., and Sawamoto, T., "An Experimental Study of Rolling Friction with References to Surface Roughness at Elasto-hydrodynamic Contacts," Proc. 11th Leeds-Lyon Symp. on Tribology, 1984, pp. 302-308.
- 16 Eguchi, M., and Yamamoto, T., "A Study of Pure Rolling Friction Between Two Rollers," JSLE Preprint, 1981-5, pp. 137-140. (in Japanese)
- 17 Witte, D. C., "Operating Torque of Tapered Roller Bearings," ASLE Trans., Vol. 16, No. 1, 1973, pp. 61-67.

## APPENDIX 1

## Derivation of Rolling Resistance (Equation (22))

Rolling resistance is a moment which acts in the direction to prevent the roller from rolling. However, theoretical EHL rolling friction,  $t_r$ , is given as a tangential force as shown in Fig. A-1. Therefore, rolling resistance should be expressed with  $t_r$ .

1) In the Case of Same Roller Radius. According to the balance of moments in Fig. A-2, rolling resistance,  $M_R$ , becomes as follows.

$$M_R - t_r \times R = 0 \tag{A1}$$

$$M_R = t_r \times R \tag{A2}$$

![](_page_6_Picture_30.jpeg)

Rolling resistance and tangential force

![](_page_6_Figure_32.jpeg)

Fig. A-2 Rollers with same radius

![](_page_6_Picture_34.jpeg)

Fig. A-3 Rollers with different radius

Rolling resistance is a multiplication of tangential force and roller radius.

2) In the Case of Rollers With Different Radius. Roller radii are  $R_1$  and  $R_2$ , respectively. If equation (A2) is used to calculate rolling resistances, the rolling resistance acting on the upper roller will differ from that acting on the lower roller. Therefore, to make each rolling resistance be equal,  $\Delta t_r$ should be added as shown in Fig. A-3.

$$M_R = (t_r + \Delta t_r)R_1$$
  
=  $(t_r - \Delta t_r)R_2$  (A3)

and

$$t_r(R_1 - R_2) = \Delta t_r(R_1 + R_2)$$
 (A4)

Hence,

$$M_R = R_1 \cdot t_r \left( 1 + \frac{R_1 - R_2}{R_1 + R_2} \right) = \frac{2R_1 \cdot R_2}{R_1 + R_2} t_r$$
 (A5)

Now, equivalent radius  $R_{\rho}$  is defined a

$$\frac{1}{R_e} = \frac{1}{R_1} + \frac{1}{R_2} = \frac{R_1 + R_2}{R_1 \cdot R_2}$$
 (A6)

Thus, equation (A5) becomes

$$M_R = 2 R_e t_r \tag{A7}$$

This is same expression as equation (22).

For the same roller radii, equivalent radius is half of roller radius, Therefore, equation (A2) is equivalent to equation (A7).