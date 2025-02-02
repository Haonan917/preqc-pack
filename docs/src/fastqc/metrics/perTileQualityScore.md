#  [Per Tile Sequence Quality](https://www.bioinformatics.babraham.ac.uk/projects/fastqc/Help/3%20Analysis%20Modules/12%20Per%20Tile%20Sequence%20Quality.html)

## Summary

This module will only appear in your analysis results if you're using an Illumina library which retains its original sequence identifiers. Encoded in these is the flowcell tile from which each read came. The module allows you to look at the quality scores from each tile across all of your bases to see if there was a loss in quality associated with only one part of the flowcell.

+ **X-labels**：It divides all positions into several groups  which contain a certain digit of position or the range of positions. 

+ **Tiles**：A set of all tiles id found in file.

+ **Means**：  The set of  average quality for each tile.

  

## Example

```
"per_tile_quality_score": {
            
            "x_labels": [
                "1",
                "2",
                "3",
                "4",
                "5",
                "6",
                "7",
                "8",
                ...
            ],
            "tiles": [
                1101,
                1102,
                1103,
                1104,
                1105,
                1106,
                1107,
                1108,
                1109,
                1110,
                1111,
                1112
            ],
            "means": [
                [
                    -0.006346050119049096,
                    -0.0654172611747228,
                    0.04078626086620574,
                    0.018562823162376674,
                    -0.07341049875898875,
                    -0.016429787792191064,
                    -0.01372768334803709,
                    -0.03346466757579947,
                    ...
                ],
                [
                    -0.0936709871750665,
                    -0.0331261565078762,
                    0.06435934332282045,
                    -0.037159459006495865,
                    0.03486157659394706,
                    0.006024915022308619,
                    0.01689568685248588,
                    -0.008014505719849296,
                    ...
                ],
                [
                    0.19160803277671334,
                    0.0720129699881511,
                    0.046999435285798086,
                    -0.016470494564551075,
                    0.08760990913324918,
                    -0.056012653372455645,
                    0.05859393421414438,
                    -0.0757104323515776,
                    ...
                ],
                [
                    -0.08292170351253958,
                    -0.04457089187183527,
                    -0.02132341694053963,
                    0.09498880685247002,
                    -0.02909020660642625,
                    -0.08919508767951356,
                    0.06544754755240234,
                    0.04402972234209557,
                    ...
                ],
                [
                    -0.05259925769379237,
                    0.17734065966617152,
                    0.05323547448775656,
                    -0.031241010441334538,
                    0.06060992018126399,
                    0.057363400444444324,
                    0.049313153043023306,
                    0.036693847332685436,
                    ...
                ],
                [
                    0.04623782481657912,
                    -0.1742257660234543,
                    0.008555611227521354,
                    0.0780235468237791,
                    0.05726359559632499,
                    0.07284318681385571,
                    -0.4294163047979822,
                    0.03223193133478475,
                    ...
                ],
                [
                    0.006707578514678403,
                    0.1224868275087374,
                    0.0819144995306047,
                    -0.0927164958223301,
                    0.07133213032459196,
                    -0.021621178716586087,
                    -0.02652805075825171,
                    0.024320665561241128,
                    ...
                ],
                [
                    -0.1892194282145212,
                    0.05937301539450601,
                    -0.009251856831049565,
                    0.00966011187865945,
                    -0.0506608728653859,
                    -0.000005505544429240672,
                    0.0768259129498503,
                    -0.021634040275564814,
                    ...
                ],
                [
                    -0.01482087894996198,
                    -0.04388643789315694,
                    -0.15656598135250732,
                    -0.02290621225324685,
                    -0.02460270912033735,
                    0.05281604056343525,
                    -0.027855395398596272,
                    -0.02307402952937565,
                    ...
                ],
                [
                    0.0997225814284306,
                    0.023947398136456854,
                    0.00972046412779548,
                    0.025144240173403887,
                    -0.015702242983785197,
                    -0.03782832466826136,
                    0.21077909870614775,
                    0.006800753397236292,
                    ...
                ],
                [
                    0.033955607721260606,
                    -0.09951870036752553,
                    -0.08152383052087941,
                    -0.07023362168592229,
                    -0.06128076790366066,
                    -0.03250179949124998,
                    0.07098260870256468,
                    0.001405350739482003,
                    ...
                ],
                [
                    0.06134668040730418,
                    0.005584343144498405,
                    -0.03690600320344117,
                    0.0443477648832129,
                    -0.05692983359072201,
                    0.06454679442065725,
                    -0.05131050771772294,
                    0.016415404744613227,
                    ...
                ]
            ]
        }
```

