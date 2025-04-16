window.BENCHMARK_DATA = {
  "lastUpdate": 1744819856589,
  "repoUrl": "https://github.com/tigerros/ruci",
  "entries": {
    "Bench": [
      {
        "commit": {
          "author": {
            "email": "aurel.leonard.danel@gmail.com",
            "name": "tigerros",
            "username": "tigerros"
          },
          "committer": {
            "email": "aurel.leonard.danel@gmail.com",
            "name": "tigerros",
            "username": "tigerros"
          },
          "distinct": true,
          "id": "163abc7b0842af0a8261bf7bfac4ecee92a57fbe",
          "message": "Update bench.yml",
          "timestamp": "2025-04-16T03:32:31+02:00",
          "tree_id": "d98ff543f2ac6f95e021798a8deccb2923609a0f",
          "url": "https://github.com/tigerros/ruci/commit/163abc7b0842af0a8261bf7bfac4ecee92a57fbe"
        },
        "date": 1744767209978,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::ruci_info",
            "value": 1008.97,
            "range": "± 16.23",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::ruci_uci_ok",
            "value": 23.31,
            "range": "± 0.24",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::shakmaty_uci_info",
            "value": 2988.23,
            "range": "± 52.28",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::shakmaty_uci_uci_ok",
            "value": 606.63,
            "range": "± 8.91",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::vampirc_info",
            "value": 28393.34,
            "range": "± 618.22",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::vampirc_uci_ok",
            "value": 2313.1,
            "range": "± 48.49",
            "unit": "ns/iter"
          },
          {
            "name": "go_to_str::ruci_borrowed",
            "value": 202.14,
            "range": "± 2.90",
            "unit": "ns/iter"
          },
          {
            "name": "go_to_str::ruci_owned",
            "value": 214.69,
            "range": "± 3.25",
            "unit": "ns/iter"
          },
          {
            "name": "go_to_str::shakmaty_uci",
            "value": 408.25,
            "range": "± 8.60",
            "unit": "ns/iter"
          },
          {
            "name": "go_to_str::vampirc",
            "value": 482.58,
            "range": "± 7.16",
            "unit": "ns/iter"
          },
          {
            "name": "register_to_str::ruci_borrowed",
            "value": 100.73,
            "range": "± 2.14",
            "unit": "ns/iter"
          },
          {
            "name": "register_to_str::ruci_owned",
            "value": 153.9,
            "range": "± 7.75",
            "unit": "ns/iter"
          },
          {
            "name": "register_to_str::shakmaty_uci",
            "value": 254.75,
            "range": "± 2.50",
            "unit": "ns/iter"
          },
          {
            "name": "register_to_str::vampirc",
            "value": 282.1,
            "range": "± 86.36",
            "unit": "ns/iter"
          },
          {
            "name": "uci_ok_to_str::ruci",
            "value": 22.29,
            "range": "± 0.66",
            "unit": "ns/iter"
          },
          {
            "name": "uci_ok_to_str::shakmaty_uci",
            "value": 41.03,
            "range": "± 0.92",
            "unit": "ns/iter"
          },
          {
            "name": "uci_ok_to_str::vampirc",
            "value": 43.56,
            "range": "± 0.66",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "aurel.leonard.danel@gmail.com",
            "name": "tigerros",
            "username": "tigerros"
          },
          "committer": {
            "email": "aurel.leonard.danel@gmail.com",
            "name": "tigerros",
            "username": "tigerros"
          },
          "distinct": true,
          "id": "a47e696710740b04b3aebaff332b3f447afeb905",
          "message": "improve github workflows, refactor benchmarks",
          "timestamp": "2025-04-16T18:09:52+02:00",
          "tree_id": "d36c4317cdd13163f3dd1272dbf0e56c40229110",
          "url": "https://github.com/tigerros/ruci/commit/a47e696710740b04b3aebaff332b3f447afeb905"
        },
        "date": 1744819854119,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 986.12,
            "range": "± 9.83",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3000.42,
            "range": "± 34.95",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27591.53,
            "range": "± 981.51",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.69,
            "range": "± 0.11",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 609.78,
            "range": "± 15.78",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2364.83,
            "range": "± 28.18",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 197.13,
            "range": "± 1.45",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 211.19,
            "range": "± 2.27",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 390.04,
            "range": "± 4.61",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 468.34,
            "range": "± 9.67",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 97.74,
            "range": "± 1.49",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 152.54,
            "range": "± 2.76",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 265.32,
            "range": "± 1.73",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 274.34,
            "range": "± 3.58",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.3,
            "range": "± 0.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.44,
            "range": "± 0.43",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.35,
            "range": "± 0.43",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}