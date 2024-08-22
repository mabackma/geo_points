use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug,Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PolygonGeometry {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "pointProperty")]
    pub point_property: PointProperty,
    #[serde(rename = "polygonProperty")]
    pub polygon_property: PolygonProperty,
}

#[derive(Serialize, Deserialize, Debug,Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PointProperty {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Point")]
    pub point: Point,
}

#[derive(Serialize, Deserialize, Debug,Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    #[serde(rename = "@srsName", default)]
    pub srs_name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "coordinates")]
    pub coordinates: String,
}

#[derive(Serialize, Deserialize, Debug,Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PolygonProperty {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Polygon")]
    pub polygon: Polygon,
}

#[derive(Serialize, Deserialize, Debug,Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Polygon {
    #[serde(rename = "@srsName", default)]
    pub srs_name: String,
    #[serde(rename = "$text", default)]
    pub text: Option<String>,
    #[serde(rename = "interior", default)]
    pub interior: Vec<Interior>,
    #[serde(rename = "exterior")]
    pub exterior: Exterior,
}

#[derive(Serialize, Deserialize, Debug,Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Interior {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "LinearRing")]
    pub linear_ring: LinearRing,
}


#[derive(Serialize, Deserialize, Debug,Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Exterior {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "LinearRing")]
    pub linear_ring: LinearRing,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LinearRing {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "coordinates")]
    pub coordinates: String,
}

/* "polygonProperty": {
                          "Polygon": {
                            "exterior": {
                              "LinearRing": {
                                "coordinates": "428126.3826,7371086.9101 428130.3826,7371109.9101 428135.3826,7371134.9101 428143.3826,7371154.9101 428149.2554,7371170.7445 428210.9676,7371146.3844 428233.2396,7371127.8244 428244.3756,7371106.4803 428243.3793,7371094.816 428282.6128,7371095.0936 428302.4305,7371095.3106 428302.7691,7371095.0167 428313.1802,7371095.4283 428323.8257,7371095.5449 428336.0029,7371104.4979 428371.3456,7371099.8571 428384.3268,7371099.006 428386.684,7371030.027 428389.1444,7371012.6331 428381.0053,7370982.4565 428378.0703,7370973.5234 428365.0159,7370965.4239 428352.9519,7370958.9278 428341.2243,7370950.2382 428327.2753,7370940.8663 428313.7347,7370929.472 428300.227,7370913.3179 428287.5247,7370892.8691 428274.9835,7370881.4286 428263.7271,7370876.1894 428248.6887,7370870.3741 428231.4687,7370860.6526 428222.3012,7370851.8108 428215.1656,7370838.1166 428208.6996,7370822.6383 428204.7194,7370801.285 428201.3175,7370776.1484 428197.8928,7370750.512 428189.8579,7370728.0942 428180.5292,7370710.2441 428175.3582,7370695.7077 428172.4552,7370689.8071 428171.5272,7370682.383 428172.9192,7370676.351 428178.0232,7370668.463 428181.3213,7370661.9528 428181.5383,7370655.6023 428178.9512,7370650.8309 428174.9494,7370648.3942 428168.1004,7370646.4572 428162.4793,7370649.4721 428151.3068,7370662.511 428141.5321,7370678.7408 428141.5244,7370688.1516 428141.4464,7370700.3318 428147.685,7370714.7693 428157.8633,7370723.8148 428163.6456,7370729.8081 428170.9512,7370741.7414 428175.0544,7370754.8246 428173.9574,7370774.6598 428169.9066,7370790.1239 428163.47,7370802.6931 428160.6799,7370818.3492 428156.6049,7370827.8039 428146.7375,7370836.525 428131.9985,7370848.2262 428122.1096,7370861.957 428122.6005,7370872.703 428126.5552,7370882.5374 428133.3954,7370895.2435 428144.2117,7370912.7744 428154.8568,7370926.5566 428164.0242,7370935.3985 428173.907,7370943.4559 428179.4522,7370955.2202 428186.8035,7370968.1532 428193.2137,7370987.8914 428191.6259,7370996.9806 428186.677,7371009.2305 428178.7299,7371021.6192 428167.8187,7371034.8964 428153.4337,7371054.3448 428150.6778,7371070.7506 428151.9493,7371076.7908 428126.3826,7371086.9101"
                              }
                            },
                            "interior": {
                              "LinearRing": {
                                "coordinates": "428240.2155,7370915.0933 428246.9845,7370915.2809 428262.3413,7370922.5842 428277.0627,7370932.4213 428292.3067,7370942.735 428300.6675,7370950.362 428304.1211,7370954.71 428305.8144,7370958.8891 428306.5082,7370963.1144 428303.0457,7370969.5355 428296.7664,7370974.5844 428287.3288,7370976.2733 428269.7347,7370969.324 428253.003,7370959.3296 428242.2565,7370948.8077 428237.428,7370941.7685 428234.6315,7370930.8558 428234.2088,7370920.6305 428235.8436,7370918.0504 428240.2155,7370915.0933"
                              }
                            }
                          }
                        } 
                        
                        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolygonGeometry {
    pub point_property: PointProperty,
    pub polygon_property: PolygonProperty,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointProperty {
    #[serde(rename = "Point")]
    pub point: Point,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    pub coordinates: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolygonProperty {
    #[serde(rename = "Polygon")]
    pub polygon: Polygon,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Polygon {
    pub exterior: Exterior,
    pub interior: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exterior {
    #[serde(rename = "LinearRing")]
    pub linear_ring: LinearRing,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinearRing {
    pub coordinates: String,
}
                        
                        */