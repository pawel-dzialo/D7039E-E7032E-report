.ALIASES
R_R1            R1(1=N00249 2=N00264 ) CN @EEG_AMPLIFIER.IdealOPamp(sch_1):INS127@ANALOG.R.Normal(chips)
R_R2            R2(1=N00242 2=N00279 ) CN @EEG_AMPLIFIER.IdealOPamp(sch_1):INS143@ANALOG.R.Normal(chips)
R_R3            R3(1=N00223 2=N00216 ) CN @EEG_AMPLIFIER.IdealOPamp(sch_1):INS159@ANALOG.R.Normal(chips)
R_R4            R4(1=N00216 2=N00249 ) CN @EEG_AMPLIFIER.IdealOPamp(sch_1):INS184@ANALOG.R.Normal(chips)
R_R5            R5(1=N00223 2=N00242 ) CN @EEG_AMPLIFIER.IdealOPamp(sch_1):INS200@ANALOG.R.Normal(chips)
R_R6            R6(1=0 2=N00279 ) CN @EEG_AMPLIFIER.IdealOPamp(sch_1):INS312@ANALOG.R.Normal(chips)
R_R7            R7(1=N00264 2=N00367 ) CN @EEG_AMPLIFIER.IdealOPamp(sch_1):INS338@ANALOG.R.Normal(chips)
V_V3            V3(+=N00497 -=0 ) CN @EEG_AMPLIFIER.IdealOPamp(sch_1):INS539@SOURCE.VSIN.Normal(chips)
V_V4            V4(+=VCC -=0 ) CN @EEG_AMPLIFIER.IdealOPamp(sch_1):INS2504@SOURCE.VDC.Normal(chips)
V_V5            V5(+=-VCC -=0 ) CN @EEG_AMPLIFIER.IdealOPamp(sch_1):INS3195@SOURCE.VDC.Normal(chips)
X_U10           U10(+=N00497 -=N00216 V+=VCC V-=-VCC OUT=N00249 ) CN
+@EEG_AMPLIFIER.IdealOPamp(sch_1):INS3467@TEX_INST.TLC2201/102/TI.Normal(chips)
X_U11           U11(+=0 -=N00223 V+=VCC V-=-VCC OUT=N00242 ) CN
+@EEG_AMPLIFIER.IdealOPamp(sch_1):INS3551@TEX_INST.TLC2201/102/TI.Normal(chips)
X_U12           U12(+=N00279 -=N00264 V+=VCC V-=-VCC OUT=N00367 ) CN
+@EEG_AMPLIFIER.IdealOPamp(sch_1):INS3678@TEX_INST.TLC2201/102/TI.Normal(chips)
_    _(-VCC=-VCC)
_    _(VCC=VCC)
.ENDALIASES
