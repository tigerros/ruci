window.BENCHMARK_DATA = {
  "lastUpdate": 1744826345869,
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
          "id": "1f2b90ae831dfb10c93c7016e876d65551e7e214",
          "message": "Update README.md",
          "timestamp": "2025-04-16T19:57:57+02:00",
          "tree_id": "2388a665d435a80e558b81e749d8ce614d9a6c7d",
          "url": "https://github.com/tigerros/ruci/commit/1f2b90ae831dfb10c93c7016e876d65551e7e214"
        },
        "date": 1744826343632,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 988.89,
            "range": "± 74.13",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3022.16,
            "range": "± 33.66",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27519.08,
            "range": "± 138.61",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.69,
            "range": "± 0.21",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 610.09,
            "range": "± 6.42",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2303.43,
            "range": "± 54.85",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 201.5,
            "range": "± 2.24",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 213.96,
            "range": "± 3.50",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 396.59,
            "range": "± 53.48",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 480.58,
            "range": "± 8.13",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 97.58,
            "range": "± 1.08",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 155,
            "range": "± 2.63",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 257.02,
            "range": "± 5.18",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 281.92,
            "range": "± 4.13",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.34,
            "range": "± 0.51",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.46,
            "range": "± 0.90",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.7,
            "range": "± 5.24",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}