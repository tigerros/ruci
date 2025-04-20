window.BENCHMARK_DATA = {
  "lastUpdate": 1745110398493,
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
          "id": "67263d1383056eedb4146848cc1567f38467a2b5",
          "message": "Squashed commit of the following:\n\ncommit 1f2b90ae831dfb10c93c7016e876d65551e7e214\nAuthor: tigerros <aurel.leonard.danel@gmail.com>\nDate:   Wed Apr 16 19:57:57 2025 +0200\n\n    Update README.md\n\ncommit acb634dc1770fac3cfd0c9498ca7ecb202bbc0a6\nAuthor: tigerros <aurel.leonard.danel@gmail.com>\nDate:   Wed Apr 16 18:27:48 2025 +0200\n\n    Update sort-manifests.yml\n\ncommit fe632a0c639556f462cb47f6d43e1242c564b775\nAuthor: tigerros <aurel.leonard.danel@gmail.com>\nDate:   Wed Apr 16 18:14:40 2025 +0200\n\n    fix workflows\n\ncommit a47e696710740b04b3aebaff332b3f447afeb905\nAuthor: tigerros <aurel.leonard.danel@gmail.com>\nDate:   Wed Apr 16 18:09:52 2025 +0200\n\n    improve github workflows, refactor benchmarks\n\ncommit 163abc7b0842af0a8261bf7bfac4ecee92a57fbe\nAuthor: tigerros <aurel.leonard.danel@gmail.com>\nDate:   Wed Apr 16 03:32:31 2025 +0200\n\n    Update bench.yml\n\ncommit 5c077d8e5d7cd0886e8b01e2b474ca95af0571f4\nAuthor: tigerros <aurel.leonard.danel@gmail.com>\nDate:   Wed Apr 16 03:28:56 2025 +0200\n\n    update readme, workflow\n\ncommit cae155d5fb0968a12b56bede1577b64a6678f88f\nAuthor: tigerros <aurel.leonard.danel@gmail.com>\nDate:   Wed Apr 16 02:16:37 2025 +0200\n\n    Update bench.yml\n\ncommit 4a8cd63141881a6e9cf45a74ea49d867a8bf7b72\nAuthor: tigerros <aurel.leonard.danel@gmail.com>\nDate:   Wed Apr 16 02:11:03 2025 +0200\n\n    fmt\n\ncommit 80b7f2575d470ce174a76804b99c49857bf8ad1f\nAuthor: tigerros <aurel.leonard.danel@gmail.com>\nDate:   Wed Apr 16 02:08:56 2025 +0200\n\n    fix\n\ncommit d0d492eccd078b6eee972fbd8f8db500f9cf6cc2\nAuthor: tigerros <aurel.leonard.danel@gmail.com>\nDate:   Wed Apr 16 02:04:38 2025 +0200\n\n    benchmark workflow\n\ncommit f510ec1188a38f46d41230e389e7e4ccb3905fec\nAuthor: tigerros <aurel.leonard.danel@gmail.com>\nDate:   Wed Apr 16 01:32:03 2025 +0200\n\n    use libtest instead of criterion\n\ncommit 726b87a897d04a66ca815829ef5f0b27a3a78a2d\nAuthor: tigerros <aurel.leonard.danel@gmail.com>\nDate:   Tue Apr 15 22:10:47 2025 +0200\n\n    add Id::updated\n\ncommit af182a46ecaf2567e2f48f252fd90d89dea9e9d8\nAuthor: tigerros <aurel.leonard.danel@gmail.com>\nDate:   Tue Apr 15 20:44:44 2025 +0200\n\n    polishing\n\ncommit 1484f6411fe1318066fa50f8fee7b0642c469d91\nAuthor: tigerros <aurel.leonard.danel@gmail.com>\nDate:   Tue Apr 15 20:14:32 2025 +0200\n\n    benchjes\n\ncommit 20472f3bb831e27e510b085e228ec846f13894ce\nAuthor: tigerros <aurel.leonard.danel@gmail.com>\nDate:   Tue Apr 15 18:46:47 2025 +0200\n\n    finish?\n\ncommit 9d714378c02840116c3dc13c79d5e08ea7d5dc5f\nAuthor: tigerros <aurel.leonard.danel@gmail.com>\nDate:   Tue Apr 15 02:54:06 2025 +0200\n\n    almost\n\ncommit 6ff781bdfcb2ef4ec670f836f5530073f09d8a5a\nAuthor: tigerros <aurel.leonard.danel@gmail.com>\nDate:   Sat Apr 12 20:29:02 2025 +0200\n\n    wip\n\ncommit 6f5f607c4876c9777b44cfa248a0a85207f938e5\nAuthor: tigerros <aurel.leonard.danel@gmail.com>\nDate:   Sat Apr 12 19:47:32 2025 +0200\n\n    Update from_str_parts.rs\n\ncommit 9f4c2d4e66ebab489edef2295301c999f42713ba\nAuthor: tigerros <aurel.leonard.danel@gmail.com>\nDate:   Tue Apr 8 18:49:09 2025 +0200\n\n    cow rework wip",
          "timestamp": "2025-04-16T20:05:17+02:00",
          "tree_id": "2388a665d435a80e558b81e749d8ce614d9a6c7d",
          "url": "https://github.com/tigerros/ruci/commit/67263d1383056eedb4146848cc1567f38467a2b5"
        },
        "date": 1744826796495,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 984.69,
            "range": "± 20.96",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3047.97,
            "range": "± 76.05",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27603.28,
            "range": "± 688.82",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.71,
            "range": "± 0.28",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 612.38,
            "range": "± 9.89",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2336.09,
            "range": "± 30.80",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 196.41,
            "range": "± 1.74",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 210.28,
            "range": "± 2.56",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 390.99,
            "range": "± 6.84",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 473.72,
            "range": "± 8.15",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 99.28,
            "range": "± 1.45",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 156.06,
            "range": "± 2.35",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 264.25,
            "range": "± 3.47",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 277.44,
            "range": "± 3.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.42,
            "range": "± 3.76",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.92,
            "range": "± 0.57",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.67,
            "range": "± 0.45",
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
          "id": "78e86f50afaf30159f35d7e75e87be2f80ec6aea",
          "message": "fix readme",
          "timestamp": "2025-04-16T20:52:57+02:00",
          "tree_id": "da5de49256a365fb857f5b65f63332d312c0b7df",
          "url": "https://github.com/tigerros/ruci/commit/78e86f50afaf30159f35d7e75e87be2f80ec6aea"
        },
        "date": 1744829643875,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 983.84,
            "range": "± 14.58",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3036.86,
            "range": "± 68.08",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27522.55,
            "range": "± 377.66",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.72,
            "range": "± 0.57",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 603.72,
            "range": "± 17.12",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2341.64,
            "range": "± 71.57",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 197.85,
            "range": "± 67.35",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 208.42,
            "range": "± 127.15",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 390.97,
            "range": "± 7.75",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 469.29,
            "range": "± 8.75",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 98.16,
            "range": "± 0.97",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 156.38,
            "range": "± 2.80",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 263.54,
            "range": "± 31.11",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 269.24,
            "range": "± 3.53",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.3,
            "range": "± 0.17",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 43.06,
            "range": "± 0.58",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.74,
            "range": "± 16.08",
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
          "id": "00e55a275f0f8c367e9788f7e351982bd1b8867a",
          "message": "Update README.md",
          "timestamp": "2025-04-16T20:53:28+02:00",
          "tree_id": "834a8abfd5ce209e183e1318563da2f0588f302f",
          "url": "https://github.com/tigerros/ruci/commit/00e55a275f0f8c367e9788f7e351982bd1b8867a"
        },
        "date": 1744829669128,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 976.98,
            "range": "± 40.47",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3030.17,
            "range": "± 43.81",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27766.51,
            "range": "± 16408.71",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.68,
            "range": "± 0.51",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 603.59,
            "range": "± 11.81",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2317.38,
            "range": "± 32.13",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 196.51,
            "range": "± 1.40",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 211.42,
            "range": "± 8.22",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 396.81,
            "range": "± 36.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 470.76,
            "range": "± 7.32",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 98.27,
            "range": "± 3.01",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 153.41,
            "range": "± 2.73",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 260.73,
            "range": "± 48.82",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 271.84,
            "range": "± 60.91",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.46,
            "range": "± 0.27",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.58,
            "range": "± 0.45",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.64,
            "range": "± 0.30",
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
          "id": "0f6d0bf77a35cd5bb43bd635497601b29931b7eb",
          "message": "re-export things from {engine,gui}, finish traits",
          "timestamp": "2025-04-18T19:37:18+02:00",
          "tree_id": "c286a34ad034eeb6d7c5dd8ca4679ea25ba14826",
          "url": "https://github.com/tigerros/ruci/commit/0f6d0bf77a35cd5bb43bd635497601b29931b7eb"
        },
        "date": 1744997907853,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1010.99,
            "range": "± 16.34",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3052.46,
            "range": "± 32.39",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27884.42,
            "range": "± 414.65",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.68,
            "range": "± 0.09",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 613.38,
            "range": "± 23.80",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2322.87,
            "range": "± 15.07",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 201.76,
            "range": "± 2.77",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 213.62,
            "range": "± 4.56",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 403.22,
            "range": "± 7.57",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 475.38,
            "range": "± 58.01",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 97.03,
            "range": "± 0.66",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 150.85,
            "range": "± 1.99",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 260.28,
            "range": "± 18.46",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 272.49,
            "range": "± 3.56",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.65,
            "range": "± 0.34",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.56,
            "range": "± 0.94",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.16,
            "range": "± 0.79",
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
          "id": "996e76bf6d0ec06dd9271addb7d19b600820ee11",
          "message": "fmt sort",
          "timestamp": "2025-04-19T02:29:38+02:00",
          "tree_id": "59e943229bc6a966550ce342a7d714e52d8edb0d",
          "url": "https://github.com/tigerros/ruci/commit/996e76bf6d0ec06dd9271addb7d19b600820ee11"
        },
        "date": 1745022648749,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1021.38,
            "range": "± 21.37",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3116.75,
            "range": "± 46.21",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27905.81,
            "range": "± 413.64",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.68,
            "range": "± 0.21",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 647.32,
            "range": "± 21.35",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2319.63,
            "range": "± 36.63",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 196.24,
            "range": "± 3.27",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 208.01,
            "range": "± 3.98",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 395.03,
            "range": "± 5.69",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 480.45,
            "range": "± 14.80",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 97.04,
            "range": "± 1.82",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 148.25,
            "range": "± 64.45",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 262.62,
            "range": "± 3.11",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 291.79,
            "range": "± 4.89",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.44,
            "range": "± 0.29",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.34,
            "range": "± 0.52",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.09,
            "range": "± 0.83",
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
          "id": "7bb2bc92f24474a93e670b3a0a585169f5a531d8",
          "message": "fix lifetime of Engine::read messages\n\nit returned 'static which sounds good, except it isn't because it's forced and so there's issues with variance. consent is important!",
          "timestamp": "2025-04-19T22:06:28+02:00",
          "tree_id": "34c996f12656e6b0536e904befebec6f0e451019",
          "url": "https://github.com/tigerros/ruci/commit/7bb2bc92f24474a93e670b3a0a585169f5a531d8"
        },
        "date": 1745093256724,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 991.66,
            "range": "± 16.51",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 2913.64,
            "range": "± 42.38",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28042.45,
            "range": "± 356.75",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.71,
            "range": "± 0.23",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 617.11,
            "range": "± 15.42",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2348.97,
            "range": "± 24.22",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 203.44,
            "range": "± 19.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 210.19,
            "range": "± 3.40",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 410.01,
            "range": "± 6.02",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 501.49,
            "range": "± 10.52",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 99.15,
            "range": "± 2.07",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 152.44,
            "range": "± 1.26",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 268.82,
            "range": "± 10.48",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 271.17,
            "range": "± 139.51",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.25,
            "range": "± 0.24",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.78,
            "range": "± 1.04",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.19,
            "range": "± 1.03",
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
          "id": "6ccebf73e3749ded0c07d36c2e9c0e9576343fa5",
          "message": "fmt",
          "timestamp": "2025-04-19T22:06:37+02:00",
          "tree_id": "168baabc2fc0f77b07bccb01199d03c3026cf733",
          "url": "https://github.com/tigerros/ruci/commit/6ccebf73e3749ded0c07d36c2e9c0e9576343fa5"
        },
        "date": 1745093259987,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 990.93,
            "range": "± 343.65",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 2921.79,
            "range": "± 35.10",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27989.33,
            "range": "± 250.03",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.68,
            "range": "± 0.09",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 592.08,
            "range": "± 11.83",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2357.99,
            "range": "± 78.89",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 195.81,
            "range": "± 1.81",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 208.47,
            "range": "± 2.23",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 411.91,
            "range": "± 22.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 495.97,
            "range": "± 11.84",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 99.72,
            "range": "± 6.63",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 152.31,
            "range": "± 2.24",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 268.09,
            "range": "± 4.25",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 270.9,
            "range": "± 3.85",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.43,
            "range": "± 0.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.96,
            "range": "± 0.79",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 43.31,
            "range": "± 1.74",
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
          "id": "5a40344a55aad1ca472bfa1b3bbf252cec29858a",
          "message": "oops\n\ntest running for 4 hours 😅",
          "timestamp": "2025-04-20T02:52:11+02:00",
          "tree_id": "2c148bc368b5fff2749b242573db516959669808",
          "url": "https://github.com/tigerros/ruci/commit/5a40344a55aad1ca472bfa1b3bbf252cec29858a"
        },
        "date": 1745110396256,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1001.29,
            "range": "± 30.68",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 2986.74,
            "range": "± 23.39",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27843.31,
            "range": "± 193.16",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.69,
            "range": "± 0.40",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 635.81,
            "range": "± 13.58",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2371.67,
            "range": "± 22.73",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 196.36,
            "range": "± 3.22",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 207.35,
            "range": "± 1.19",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 401.34,
            "range": "± 6.93",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 486.84,
            "range": "± 7.19",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 98.24,
            "range": "± 1.40",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 150.18,
            "range": "± 3.27",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 262.9,
            "range": "± 4.93",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 267.37,
            "range": "± 3.53",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.29,
            "range": "± 0.21",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.92,
            "range": "± 0.56",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 43.33,
            "range": "± 0.73",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}