window.BENCHMARK_DATA = {
  "lastUpdate": 1744762664982,
  "repoUrl": "https://github.com/tigerros/ruci",
  "entries": {
    "Benchmark": [
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
          "id": "cae155d5fb0968a12b56bede1577b64a6678f88f",
          "message": "Update bench.yml",
          "timestamp": "2025-04-16T02:16:37+02:00",
          "tree_id": "f3fb26cbc72d9c0b0189e2f61223675c95c617a2",
          "url": "https://github.com/tigerros/ruci/commit/cae155d5fb0968a12b56bede1577b64a6678f88f"
        },
        "date": 1744762662578,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::ruci_info",
            "value": 994.72,
            "range": "± 11.45",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::ruci_uci_ok",
            "value": 61.86,
            "range": "± 0.47",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::shakmaty_uci_info",
            "value": 3252.06,
            "range": "± 35.19",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::shakmaty_uci_uci_ok",
            "value": 633.7,
            "range": "± 17.55",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::vampirc_info",
            "value": 28421.52,
            "range": "± 580.70",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::vampirc_uci_ok",
            "value": 2387.12,
            "range": "± 52.52",
            "unit": "ns/iter"
          },
          {
            "name": "go_to_str::ruci_slice",
            "value": 200.11,
            "range": "± 2.91",
            "unit": "ns/iter"
          },
          {
            "name": "go_to_str::ruci_vec",
            "value": 212.45,
            "range": "± 36.56",
            "unit": "ns/iter"
          },
          {
            "name": "go_to_str::shakmaty_uci",
            "value": 418.97,
            "range": "± 94.37",
            "unit": "ns/iter"
          },
          {
            "name": "go_to_str::vampirc",
            "value": 485.55,
            "range": "± 14.76",
            "unit": "ns/iter"
          },
          {
            "name": "register_to_str::ruci_borrowed",
            "value": 100.48,
            "range": "± 1.11",
            "unit": "ns/iter"
          },
          {
            "name": "register_to_str::ruci_owned",
            "value": 153.87,
            "range": "± 2.34",
            "unit": "ns/iter"
          },
          {
            "name": "register_to_str::shakmaty_uci",
            "value": 259.07,
            "range": "± 3.52",
            "unit": "ns/iter"
          },
          {
            "name": "register_to_str::vampirc",
            "value": 280.13,
            "range": "± 2.23",
            "unit": "ns/iter"
          },
          {
            "name": "uci_ok_to_str::ruci",
            "value": 22.44,
            "range": "± 0.47",
            "unit": "ns/iter"
          },
          {
            "name": "uci_ok_to_str::shakmaty_uci",
            "value": 41.01,
            "range": "± 0.39",
            "unit": "ns/iter"
          },
          {
            "name": "uci_ok_to_str::vampirc",
            "value": 43.55,
            "range": "± 0.44",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}