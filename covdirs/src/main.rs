use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct CovDir {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    children: Vec<CovDir>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    coverage: Vec<i32>,
    coveragePercent: f64,
    linesCovered: u32,
    linesMissed: u32,
    linesTotal: u32,
    name: String,
}

fn main() {
    let results = r#"{
        "children": {
            "/": {
                "children": {
                    "foo": {
                        "children": {
                            "d.cpp": {
                                "coverage": [
                                    10,
                                    0
                                ],
                                "coveragePercent": 50.0,
                                "linesCovered": 1,
                                "linesMissed": 1,
                                "linesTotal": 2,
                                "name": "d.cpp"
                            }
                        },
                        "coveragePercent": 50.0,
                        "linesCovered": 1,
                        "linesMissed": 1,
                        "linesTotal": 2,
                        "name": "foo"
                    }
                },
                "coveragePercent": 50.0,
                "linesCovered": 1,
                "linesMissed": 1,
                "linesTotal": 2,
                "name": "/"
            },
            "foo": {
                "children": {
                    "bar": {
                        "children": {
                            "a.cpp": {
                                "coverage": [
                                    10,
                                    11
                                ],
                                "coveragePercent": 100.0,
                                "linesCovered": 2,
                                "linesMissed": 0,
                                "linesTotal": 2,
                                "name": "a.cpp"
                            },
                            "b.cpp": {
                                "coverage": [
                                    0,
                                    10,
                                    -1,
                                    0
                                ],
                                "coveragePercent": 33.33,
                                "linesCovered": 1,
                                "linesMissed": 2,
                                "linesTotal": 3,
                                "name": "b.cpp"
                            }
                        },
                        "coveragePercent": 60.0,
                        "linesCovered": 3,
                        "linesMissed": 2,
                        "linesTotal": 5,
                        "name": "bar"
                    },
                    "c.cpp": {
                        "coverage": [
                            10,
                            -1,
                            -1,
                            1
                        ],
                        "coveragePercent": 100.0,
                        "linesCovered": 2,
                        "linesMissed": 0,
                        "linesTotal": 2,
                        "name": "c.cpp"
                    }
                },
                "coveragePercent": 71.43,
                "linesCovered": 5,
                "linesMissed": 2,
                "linesTotal": 7,
                "name": "foo"
            }
        },
        "coveragePercent": 66.67,
        "linesCovered": 6,
        "linesMissed": 3,
        "linesTotal": 9,
        "name": ""
    }"#;
    let c: CovDir = serde_json::from_str(results).unwrap();
    println!("{:#?}", c);
}
