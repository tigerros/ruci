window.BENCHMARK_DATA = {
  "lastUpdate": 1761840779628,
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
          "id": "a8a68211713fc2f00097793d46fcc257c2ac26f2",
          "message": "fix info string parsing, unify sync and async Engine",
          "timestamp": "2025-04-21T23:56:51+02:00",
          "tree_id": "4c2f38da404f3a38c1229e528915bd8eb9184063",
          "url": "https://github.com/tigerros/ruci/commit/a8a68211713fc2f00097793d46fcc257c2ac26f2"
        },
        "date": 1745272683469,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1005.19,
            "range": "± 15.71",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3292.05,
            "range": "± 38.39",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27908.73,
            "range": "± 2722.21",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.01,
            "range": "± 0.14",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 645.11,
            "range": "± 13.73",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2355.35,
            "range": "± 43.44",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 193.36,
            "range": "± 6.40",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 207.9,
            "range": "± 1.74",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 403.05,
            "range": "± 7.80",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 491.48,
            "range": "± 9.05",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 104.5,
            "range": "± 1.79",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 165.01,
            "range": "± 3.26",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 260.49,
            "range": "± 2.86",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 270.38,
            "range": "± 26.96",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.43,
            "range": "± 0.53",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.79,
            "range": "± 0.95",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.16,
            "range": "± 0.53",
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
          "id": "f5031c5615fd34edfb2328d81ab6bb4ab72aa41b",
          "message": "fmt+sort",
          "timestamp": "2025-04-22T00:12:50+02:00",
          "tree_id": "c57dfe91341b56c213d496f0c6f379738f095e74",
          "url": "https://github.com/tigerros/ruci/commit/f5031c5615fd34edfb2328d81ab6bb4ab72aa41b"
        },
        "date": 1745273642174,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1005.93,
            "range": "± 14.15",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3240.86,
            "range": "± 67.84",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27839.55,
            "range": "± 431.32",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.01,
            "range": "± 1.81",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 668.16,
            "range": "± 15.92",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2348.98,
            "range": "± 32.19",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 193.34,
            "range": "± 2.92",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 211.87,
            "range": "± 10.43",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 412.96,
            "range": "± 7.24",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 506.1,
            "range": "± 8.95",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 102.85,
            "range": "± 2.01",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 166.14,
            "range": "± 2.04",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 254.5,
            "range": "± 4.04",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 270,
            "range": "± 6.06",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.43,
            "range": "± 0.30",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.8,
            "range": "± 0.54",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.17,
            "range": "± 1.17",
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
          "id": "cec35daaadf053a6acb0ac1eeefa0be37a8bbdef",
          "message": "rewording",
          "timestamp": "2025-04-22T00:24:49+02:00",
          "tree_id": "ad49c2d5608bf143a265319eeb303f7a8b8f0a8e",
          "url": "https://github.com/tigerros/ruci/commit/cec35daaadf053a6acb0ac1eeefa0be37a8bbdef"
        },
        "date": 1745274359666,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1002.17,
            "range": "± 14.09",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3276.92,
            "range": "± 874.20",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27786.73,
            "range": "± 726.86",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.01,
            "range": "± 0.63",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 634.08,
            "range": "± 8.88",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2320.46,
            "range": "± 38.76",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 194.37,
            "range": "± 25.44",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 208.4,
            "range": "± 4.07",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 410.63,
            "range": "± 7.28",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 488.86,
            "range": "± 11.28",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 102.32,
            "range": "± 2.82",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 163.69,
            "range": "± 2.86",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 253.09,
            "range": "± 2.71",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 266.91,
            "range": "± 5.32",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.17,
            "range": "± 0.43",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.78,
            "range": "± 1.38",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 43.82,
            "range": "± 0.62",
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
          "id": "6b26494f7f294f5859f7b3097d4fd690a7b32e84",
          "message": "rewording",
          "timestamp": "2025-04-22T00:25:10+02:00",
          "tree_id": "8463d2e610b3b2d1bdf467fbbf5f84dfa8efb770",
          "url": "https://github.com/tigerros/ruci/commit/6b26494f7f294f5859f7b3097d4fd690a7b32e84"
        },
        "date": 1745274385190,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 999.68,
            "range": "± 23.25",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3330.07,
            "range": "± 59.68",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28023.67,
            "range": "± 210.86",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23,
            "range": "± 0.18",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 672.44,
            "range": "± 18.65",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2367.52,
            "range": "± 41.05",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 199.6,
            "range": "± 2.57",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 213.89,
            "range": "± 4.87",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 402.83,
            "range": "± 7.66",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 493.35,
            "range": "± 12.09",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 103.04,
            "range": "± 2.32",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 165.26,
            "range": "± 2.05",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 260.95,
            "range": "± 4.53",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 271.74,
            "range": "± 6.46",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.42,
            "range": "± 0.13",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.78,
            "range": "± 0.23",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 43.29,
            "range": "± 0.56",
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
          "id": "384e5f19beae20535a6943f83db88deddd4d7b85",
          "message": "readme, extra test for strict Engine",
          "timestamp": "2025-04-22T01:41:54+02:00",
          "tree_id": "51b7793ecbe688578e5ea8502b52f16be6e4ce44",
          "url": "https://github.com/tigerros/ruci/commit/384e5f19beae20535a6943f83db88deddd4d7b85"
        },
        "date": 1745278992601,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1015.52,
            "range": "± 74.48",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3291.14,
            "range": "± 683.73",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28116.65,
            "range": "± 559.37",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23,
            "range": "± 4.63",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 672.62,
            "range": "± 24.72",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2362.34,
            "range": "± 54.57",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 193.86,
            "range": "± 2.81",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 209.98,
            "range": "± 5.10",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 398.03,
            "range": "± 7.90",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 481.18,
            "range": "± 15.16",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 102.01,
            "range": "± 1.68",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 162.61,
            "range": "± 1.35",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 255.89,
            "range": "± 6.42",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 268.97,
            "range": "± 3.07",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.47,
            "range": "± 0.19",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.89,
            "range": "± 0.55",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.36,
            "range": "± 0.35",
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
          "id": "82a962eb719cb97d4d021d5aee07f7414b101a53",
          "message": "remove `Engine::from_path`, rename `ConnectionError` to `FromProcessError`\n\n`from_path` didn't give the user the ability to control the process. couldn't even wait on it. very bad",
          "timestamp": "2025-04-22T22:04:44+02:00",
          "tree_id": "fa942fcbb18c384e398069a829eae4640732c118",
          "url": "https://github.com/tigerros/ruci/commit/82a962eb719cb97d4d021d5aee07f7414b101a53"
        },
        "date": 1745352350042,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 956.66,
            "range": "± 229.98",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3070.97,
            "range": "± 35.22",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27900.21,
            "range": "± 376.17",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.64,
            "range": "± 0.76",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 615.72,
            "range": "± 9.75",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2388.15,
            "range": "± 39.40",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 199.76,
            "range": "± 2.00",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 207.05,
            "range": "± 3.79",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 405.57,
            "range": "± 65.32",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 488.63,
            "range": "± 17.08",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 96.93,
            "range": "± 5.09",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 159.28,
            "range": "± 2.31",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 261.46,
            "range": "± 6.02",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 280.8,
            "range": "± 5.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.59,
            "range": "± 0.41",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.38,
            "range": "± 3.54",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.32,
            "range": "± 0.74",
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
          "id": "d05cd0238cee7a62a5da3d3582d13b578ca99c99",
          "message": "Update README.md",
          "timestamp": "2025-04-22T22:10:28+02:00",
          "tree_id": "ef2eef315c96ed9b844e26ada8fc85b28dd3ee10",
          "url": "https://github.com/tigerros/ruci/commit/d05cd0238cee7a62a5da3d3582d13b578ca99c99"
        },
        "date": 1745352697723,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 970.2,
            "range": "± 95.59",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3098.98,
            "range": "± 34.25",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27908.05,
            "range": "± 375.42",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.63,
            "range": "± 0.27",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 622.27,
            "range": "± 9.31",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2430.85,
            "range": "± 870.15",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 200.87,
            "range": "± 2.18",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 208.99,
            "range": "± 3.45",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 395.54,
            "range": "± 7.31",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 501.91,
            "range": "± 8.87",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 96.92,
            "range": "± 1.23",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 159.91,
            "range": "± 2.91",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 260.88,
            "range": "± 2.76",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 279.91,
            "range": "± 6.16",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.47,
            "range": "± 0.34",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.56,
            "range": "± 0.54",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.09,
            "range": "± 0.39",
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
          "id": "4a4f2988e7b2e8fcc6448bf52c0bab37e5a1b3f5",
          "message": "improve error messages + bump coverage\n\nmake the `_lifetimes` compile time check a test, and test errors. kind pointless cause the logic is so simple, but eh. it did make me scrutinize the messages a bit more. now they're shorter, clearer",
          "timestamp": "2025-04-23T22:18:29+02:00",
          "tree_id": "c6a39fcab1164c76092ae9de11eec8605b131ed8",
          "url": "https://github.com/tigerros/ruci/commit/4a4f2988e7b2e8fcc6448bf52c0bab37e5a1b3f5"
        },
        "date": 1745439573617,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 951.41,
            "range": "± 25.86",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3078.42,
            "range": "± 59.56",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27869.3,
            "range": "± 367.10",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.62,
            "range": "± 0.55",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 615.21,
            "range": "± 23.21",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2361.2,
            "range": "± 25.10",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 194.98,
            "range": "± 1.76",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 210.95,
            "range": "± 9.07",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 404.84,
            "range": "± 24.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 487.16,
            "range": "± 8.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 97.62,
            "range": "± 0.95",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 158.15,
            "range": "± 4.15",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 257.79,
            "range": "± 2.31",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 278.99,
            "range": "± 15.96",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.29,
            "range": "± 0.36",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.39,
            "range": "± 0.48",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.11,
            "range": "± 0.42",
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
          "id": "5c9e7caf57a75c6b4c440cd16328962125f41d8c",
          "message": "Update errors.rs",
          "timestamp": "2025-04-23T22:22:24+02:00",
          "tree_id": "1393c6dde2ac4ec0eb152b52979fa05180e42afd",
          "url": "https://github.com/tigerros/ruci/commit/5c9e7caf57a75c6b4c440cd16328962125f41d8c"
        },
        "date": 1745439809464,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 960.62,
            "range": "± 18.14",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3060.48,
            "range": "± 69.84",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28995.9,
            "range": "± 13618.12",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 40.7,
            "range": "± 0.17",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 607.95,
            "range": "± 17.57",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2401.14,
            "range": "± 64.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 198.46,
            "range": "± 3.37",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 214.87,
            "range": "± 4.78",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 409.54,
            "range": "± 8.83",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 509.74,
            "range": "± 6.92",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 96.95,
            "range": "± 1.07",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 157.95,
            "range": "± 1.86",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 261.77,
            "range": "± 3.16",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 282.15,
            "range": "± 2.51",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.2,
            "range": "± 0.28",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.95,
            "range": "± 0.42",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.94,
            "range": "± 1.18",
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
          "id": "8427caf2f1a2a901d607020d7b20aaa006d2a9ac",
          "message": "fix clippy",
          "timestamp": "2025-04-23T22:28:20+02:00",
          "tree_id": "38bd763d32e8547b59ca2af0f0bf2d55a1facc67",
          "url": "https://github.com/tigerros/ruci/commit/8427caf2f1a2a901d607020d7b20aaa006d2a9ac"
        },
        "date": 1745440167767,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 950.94,
            "range": "± 7.99",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3108.97,
            "range": "± 1867.56",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28526.34,
            "range": "± 10211.42",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.62,
            "range": "± 0.15",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 610.64,
            "range": "± 17.53",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2401.17,
            "range": "± 71.49",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 231.18,
            "range": "± 7.10",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 252.52,
            "range": "± 2.06",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 450.24,
            "range": "± 9.27",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 503.07,
            "range": "± 8.91",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 134.8,
            "range": "± 1.51",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 194.33,
            "range": "± 3.89",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 282.51,
            "range": "± 3.16",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 296.27,
            "range": "± 400.05",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.39,
            "range": "± 0.42",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.51,
            "range": "± 23.63",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.19,
            "range": "± 0.54",
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
          "id": "047b6f11783ce50642c22e9acf19751c3274d3dc",
          "message": "doc `FromProcessError`",
          "timestamp": "2025-04-24T00:07:54+02:00",
          "tree_id": "1fe120e9dbb68c2756b019a3b49b3d3b8d24e192",
          "url": "https://github.com/tigerros/ruci/commit/047b6f11783ce50642c22e9acf19751c3274d3dc"
        },
        "date": 1745446139455,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 946.52,
            "range": "± 12.59",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3085.85,
            "range": "± 35.23",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28084.96,
            "range": "± 322.13",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.63,
            "range": "± 0.20",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 594.18,
            "range": "± 33.94",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2342.81,
            "range": "± 52.71",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 193.92,
            "range": "± 2.44",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 211.86,
            "range": "± 31.16",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 395.74,
            "range": "± 9.43",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 487.07,
            "range": "± 18.10",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 96.96,
            "range": "± 0.77",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 156.49,
            "range": "± 2.09",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 256.52,
            "range": "± 3.19",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 281.74,
            "range": "± 3.81",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.27,
            "range": "± 0.35",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.22,
            "range": "± 0.23",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.9,
            "range": "± 0.22",
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
          "id": "69dd06ded70989e10e5a9aa9f8ef94e9cab62718",
          "message": "renamed `Engine` fields + additional bounds on `traits::Message`\n\nengine fields weren't very clear before. You would probably think `Engine.in` is the input to the engine, but nope. I just called it that because  we read from the engine, I guess that's `in`. But then it should be called a `Gui`.\n\nadditional trait bounds make it more convenient to use, really no downside because it's sealed",
          "timestamp": "2025-04-24T00:35:55+02:00",
          "tree_id": "c1e06a31e7fab0d2e8a3b414789f8bcf2f519f26",
          "url": "https://github.com/tigerros/ruci/commit/69dd06ded70989e10e5a9aa9f8ef94e9cab62718"
        },
        "date": 1745447819632,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1010.63,
            "range": "± 297.07",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3053.62,
            "range": "± 46.71",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28042.34,
            "range": "± 304.00",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.63,
            "range": "± 0.15",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 606.62,
            "range": "± 17.68",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2385.79,
            "range": "± 28.79",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 193.81,
            "range": "± 4.80",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 210.31,
            "range": "± 10.34",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 397.4,
            "range": "± 36.46",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 490.84,
            "range": "± 10.92",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 97.54,
            "range": "± 0.93",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 159.89,
            "range": "± 2.76",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 260.6,
            "range": "± 3.96",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 281.14,
            "range": "± 3.38",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.97,
            "range": "± 0.51",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.23,
            "range": "± 1.03",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 43.89,
            "range": "± 1.00",
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
          "id": "0c2a84aebef0adacd3f42cbd835772dcfffe3cfb",
          "message": "update workspace version to 1",
          "timestamp": "2025-04-25T01:21:02+02:00",
          "tree_id": "80c0ab394f09e184497d9691a2e2c5b2b2cd9761",
          "url": "https://github.com/tigerros/ruci/commit/0c2a84aebef0adacd3f42cbd835772dcfffe3cfb"
        },
        "date": 1745536926665,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 966.35,
            "range": "± 27.95",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3064.48,
            "range": "± 85.19",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27730.9,
            "range": "± 410.71",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.62,
            "range": "± 0.24",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 634.81,
            "range": "± 11.07",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2393.37,
            "range": "± 42.43",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 194.07,
            "range": "± 2.45",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 205.65,
            "range": "± 2.20",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 394.49,
            "range": "± 9.73",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 484.27,
            "range": "± 9.58",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 99.09,
            "range": "± 1.75",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 166.92,
            "range": "± 3.72",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 261.64,
            "range": "± 2.74",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 272.8,
            "range": "± 4.23",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.42,
            "range": "± 0.17",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.74,
            "range": "± 0.37",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.18,
            "range": "± 5.18",
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
          "id": "7d4d8230c74a5893b9cccd82001d9db972963f6e",
          "message": "1.0.1 hotfix\n\nadd `_async` suffix to async version of `Engine::from_process`. technically a breaking change but I'm not changing to `2.0.0` for the 1 minute it was out",
          "timestamp": "2025-04-25T01:34:39+02:00",
          "tree_id": "7d65f2188aaa721581cf64f26a48d405ffa4dbd4",
          "url": "https://github.com/tigerros/ruci/commit/7d4d8230c74a5893b9cccd82001d9db972963f6e"
        },
        "date": 1745537742582,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 949.84,
            "range": "± 30.41",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3080.5,
            "range": "± 33.88",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27986.73,
            "range": "± 381.81",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.64,
            "range": "± 0.14",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 638.69,
            "range": "± 14.84",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2356.4,
            "range": "± 24.40",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 196.6,
            "range": "± 4.42",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 208.52,
            "range": "± 5.55",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 400.68,
            "range": "± 4.91",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 485,
            "range": "± 10.97",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 100.45,
            "range": "± 1.15",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 166.65,
            "range": "± 3.38",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 269.22,
            "range": "± 70.24",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 269.61,
            "range": "± 4.03",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.42,
            "range": "± 0.14",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.78,
            "range": "± 0.40",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.19,
            "range": "± 0.54",
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
          "id": "943242e42c8ef986782b8bdfaa620428024b467b",
          "message": "1.0.1 hotfix\n\nadd `_async` suffix to async version of `Engine::from_process`. technically a breaking change but I'm not changing to `2.0.0` for the 1 minute it was out",
          "timestamp": "2025-04-25T01:37:03+02:00",
          "tree_id": "36199202f18f20f12a240fe3d4d7207b4c9a8102",
          "url": "https://github.com/tigerros/ruci/commit/943242e42c8ef986782b8bdfaa620428024b467b"
        },
        "date": 1745537889103,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 957.3,
            "range": "± 11.07",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3106.33,
            "range": "± 48.11",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28027.8,
            "range": "± 609.85",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.63,
            "range": "± 0.14",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 624.48,
            "range": "± 8.01",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2395.56,
            "range": "± 33.17",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 196.87,
            "range": "± 2.23",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 208.5,
            "range": "± 3.18",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 397.88,
            "range": "± 2.93",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 479.91,
            "range": "± 8.54",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 97,
            "range": "± 0.72",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 153.79,
            "range": "± 2.11",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 260.97,
            "range": "± 2.33",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 265.89,
            "range": "± 3.37",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.43,
            "range": "± 0.10",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.24,
            "range": "± 0.45",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.94,
            "range": "± 0.21",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f8ec6f9bbfa3d7f93d02505f25b04c14766ae7e2",
          "message": "scan vulnerabilities with OSV\n\nusing osv.dev instead of rustsec.org because osv.dev actually has rustsec.org as a source",
          "timestamp": "2025-04-26T22:27:54+02:00",
          "tree_id": "6c038c9e59855b775d2444c5886ff36d1fa838c6",
          "url": "https://github.com/tigerros/ruci/commit/f8ec6f9bbfa3d7f93d02505f25b04c14766ae7e2"
        },
        "date": 1745699341853,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 967.23,
            "range": "± 24.25",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3267.24,
            "range": "± 38.93",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27771.41,
            "range": "± 508.61",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.39,
            "range": "± 0.26",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 658.02,
            "range": "± 26.01",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2398.86,
            "range": "± 43.49",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 194.16,
            "range": "± 2.71",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 214.12,
            "range": "± 86.83",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 398.21,
            "range": "± 5.20",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 495.71,
            "range": "± 11.81",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 99.35,
            "range": "± 0.48",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 153.63,
            "range": "± 5.19",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 259.51,
            "range": "± 3.36",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 278.79,
            "range": "± 19.08",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.2,
            "range": "± 0.45",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.32,
            "range": "± 0.84",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 43,
            "range": "± 0.71",
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
          "id": "9286e4949bd9e6c7e26feb85c6b3cde1d64ad3ce",
          "message": "fix osv scanner",
          "timestamp": "2025-04-26T23:20:19+02:00",
          "tree_id": "e80e33f5f3b2ad5d6fd19730fad904cfe61d7ec2",
          "url": "https://github.com/tigerros/ruci/commit/9286e4949bd9e6c7e26feb85c6b3cde1d64ad3ce"
        },
        "date": 1745702479405,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 974.76,
            "range": "± 12.50",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3333.4,
            "range": "± 1118.08",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28193.65,
            "range": "± 383.05",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 27.53,
            "range": "± 12.34",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 673.57,
            "range": "± 15.06",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2347.68,
            "range": "± 26.78",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 193.53,
            "range": "± 1.08",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 205.39,
            "range": "± 13.92",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 409.61,
            "range": "± 7.53",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 497.23,
            "range": "± 5.63",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 97.4,
            "range": "± 1.08",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 151.84,
            "range": "± 16.59",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 258.95,
            "range": "± 3.48",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 287.12,
            "range": "± 5.49",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 24.12,
            "range": "± 0.34",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.39,
            "range": "± 0.69",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.47,
            "range": "± 0.41",
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
          "id": "6e44a657f6d2db0f31d483a9e04ccd846476046b",
          "message": "fix osv scanner",
          "timestamp": "2025-04-26T23:24:14+02:00",
          "tree_id": "5b5a725e54584975eb719d7c8edcf4eb83b98326",
          "url": "https://github.com/tigerros/ruci/commit/6e44a657f6d2db0f31d483a9e04ccd846476046b"
        },
        "date": 1745702716584,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 992.51,
            "range": "± 16.89",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3300.63,
            "range": "± 31.86",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27802.34,
            "range": "± 1755.17",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.4,
            "range": "± 0.13",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 643.84,
            "range": "± 7.63",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2334.79,
            "range": "± 37.20",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 196.29,
            "range": "± 1.89",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 208.75,
            "range": "± 2.04",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 423.19,
            "range": "± 6.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 512.91,
            "range": "± 111.35",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 99.45,
            "range": "± 0.97",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 152.66,
            "range": "± 4.73",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 269.56,
            "range": "± 4.08",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 296.35,
            "range": "± 6.34",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.25,
            "range": "± 0.31",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 44.22,
            "range": "± 0.51",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.12,
            "range": "± 0.68",
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
          "id": "759b98bfa2205528c272065b67ff5465a930800e",
          "message": "fix osv scanner",
          "timestamp": "2025-04-26T23:44:40+02:00",
          "tree_id": "0a4113f00fb05de5996fc462bb22299dc4078a46",
          "url": "https://github.com/tigerros/ruci/commit/759b98bfa2205528c272065b67ff5465a930800e"
        },
        "date": 1745703941229,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 964.79,
            "range": "± 24.39",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3282.41,
            "range": "± 94.63",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27893.6,
            "range": "± 569.24",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.38,
            "range": "± 9.87",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 687.29,
            "range": "± 19.58",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2397.24,
            "range": "± 43.71",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 196.36,
            "range": "± 1.16",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 207.57,
            "range": "± 2.91",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 415.31,
            "range": "± 10.03",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 485.93,
            "range": "± 12.64",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 96.74,
            "range": "± 1.17",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 148.65,
            "range": "± 1.96",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 261.23,
            "range": "± 41.95",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 284.34,
            "range": "± 4.22",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.28,
            "range": "± 0.41",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.29,
            "range": "± 1.15",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 43.02,
            "range": "± 41.74",
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
          "id": "c54da6de64c69d7e1b394cfa360d996984b11d98",
          "message": "Merge branch 'master' of https://github.com/tigerros/ruci",
          "timestamp": "2025-04-26T23:52:37+02:00",
          "tree_id": "b7ff37e1148267edcbfc43df1bacb267d6e4d4b6",
          "url": "https://github.com/tigerros/ruci/commit/c54da6de64c69d7e1b394cfa360d996984b11d98"
        },
        "date": 1745704482454,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1018.19,
            "range": "± 31.46",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3420.27,
            "range": "± 88.61",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 29774.97,
            "range": "± 4804.72",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 24.11,
            "range": "± 1.40",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 696.72,
            "range": "± 25.34",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2481.04,
            "range": "± 95.71",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 212,
            "range": "± 13.28",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 222.79,
            "range": "± 17.84",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 434.75,
            "range": "± 21.98",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 513.61,
            "range": "± 35.27",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 104.11,
            "range": "± 9.85",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 163.45,
            "range": "± 11.40",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 273.94,
            "range": "± 14.61",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 289.28,
            "range": "± 10.64",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 23.13,
            "range": "± 0.60",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 44.94,
            "range": "± 3.18",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 46.4,
            "range": "± 2.11",
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
          "id": "e4e8b72f4f7f35d33ad11787d1706728989355f1",
          "message": "fix osv scanner",
          "timestamp": "2025-04-27T00:00:14+02:00",
          "tree_id": "fffdc79a88054ea5293c8601a521ef96d005820e",
          "url": "https://github.com/tigerros/ruci/commit/e4e8b72f4f7f35d33ad11787d1706728989355f1"
        },
        "date": 1745704880366,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 970.52,
            "range": "± 29.65",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3283.03,
            "range": "± 88.97",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27577.84,
            "range": "± 283.72",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.4,
            "range": "± 0.12",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 694.14,
            "range": "± 13.38",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2465.5,
            "range": "± 70.25",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 196.68,
            "range": "± 2.36",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 209.16,
            "range": "± 1.30",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 405.08,
            "range": "± 6.05",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 489.9,
            "range": "± 7.42",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 97.4,
            "range": "± 1.36",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 151.4,
            "range": "± 4.11",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 261.07,
            "range": "± 4.31",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 275.14,
            "range": "± 2.32",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.27,
            "range": "± 0.23",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.25,
            "range": "± 0.44",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.13,
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
          "id": "7582707abe0c0e093afbc0357876fe787631d74d",
          "message": "fix osv scanner",
          "timestamp": "2025-04-27T00:12:42+02:00",
          "tree_id": "d39b0054f5cae43adeb46c6a3da3783c4e78632c",
          "url": "https://github.com/tigerros/ruci/commit/7582707abe0c0e093afbc0357876fe787631d74d"
        },
        "date": 1745705634948,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 965.88,
            "range": "± 12.06",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3262.69,
            "range": "± 62.89",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28049.04,
            "range": "± 307.75",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.39,
            "range": "± 0.25",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 647.51,
            "range": "± 11.69",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2424.14,
            "range": "± 32.75",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 194.44,
            "range": "± 1.43",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 210.75,
            "range": "± 2.79",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 398.01,
            "range": "± 5.81",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 485.59,
            "range": "± 7.04",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 96.54,
            "range": "± 0.73",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 152.09,
            "range": "± 5.00",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 255.19,
            "range": "± 4.54",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 279.85,
            "range": "± 4.16",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.7,
            "range": "± 0.28",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.3,
            "range": "± 0.49",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 43.35,
            "range": "± 0.39",
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
          "id": "79ca9919082c94edba63b337b2a32344ed89faba",
          "message": "fix osv scanner",
          "timestamp": "2025-04-27T00:14:33+02:00",
          "tree_id": "0647813468c8569d6ec97f084126a0d210bd835d",
          "url": "https://github.com/tigerros/ruci/commit/79ca9919082c94edba63b337b2a32344ed89faba"
        },
        "date": 1745705734173,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 981.48,
            "range": "± 18.30",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3283.72,
            "range": "± 74.78",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28017.01,
            "range": "± 402.75",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.39,
            "range": "± 0.21",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 680.08,
            "range": "± 17.70",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2412.62,
            "range": "± 20.71",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 194.08,
            "range": "± 3.90",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 205.21,
            "range": "± 4.63",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 401.59,
            "range": "± 4.24",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 480.9,
            "range": "± 13.43",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 108.01,
            "range": "± 96.67",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 150.57,
            "range": "± 3.67",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 258.82,
            "range": "± 2.88",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 280.64,
            "range": "± 2.96",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.45,
            "range": "± 0.49",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.25,
            "range": "± 0.34",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.93,
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
          "id": "0092541d3eeea548c451cb544aad8f08e4e013b4",
          "message": "fix osv scanner",
          "timestamp": "2025-04-27T00:17:24+02:00",
          "tree_id": "007ebaddb365a1d7e68ffc1aa77968b23ab5b6f2",
          "url": "https://github.com/tigerros/ruci/commit/0092541d3eeea548c451cb544aad8f08e4e013b4"
        },
        "date": 1745705908212,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 965.63,
            "range": "± 13.24",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3274.59,
            "range": "± 38.02",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27789.88,
            "range": "± 358.57",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.39,
            "range": "± 0.21",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 682.92,
            "range": "± 13.08",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2343.61,
            "range": "± 29.40",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 195.08,
            "range": "± 8.81",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 208.02,
            "range": "± 2.14",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 403.01,
            "range": "± 7.37",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 490.16,
            "range": "± 8.08",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 96.67,
            "range": "± 0.71",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 149.82,
            "range": "± 2.74",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 255.07,
            "range": "± 3.21",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 282.15,
            "range": "± 4.17",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.19,
            "range": "± 0.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.15,
            "range": "± 0.96",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.03,
            "range": "± 0.34",
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
          "id": "77d60ad55a497691f2b2af5d8f1fcf6e2a20b5d5",
          "message": "fix osv scanner",
          "timestamp": "2025-04-27T00:26:00+02:00",
          "tree_id": "6dcc4fd3430ba8cc4a70d46f7e73d159d46582b1",
          "url": "https://github.com/tigerros/ruci/commit/77d60ad55a497691f2b2af5d8f1fcf6e2a20b5d5"
        },
        "date": 1745706432299,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1000.71,
            "range": "± 47.67",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3324.86,
            "range": "± 158.66",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27869.29,
            "range": "± 10312.30",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.38,
            "range": "± 0.26",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 647.1,
            "range": "± 14.36",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2389.5,
            "range": "± 51.49",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 193.91,
            "range": "± 2.33",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 210.5,
            "range": "± 5.42",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 405.78,
            "range": "± 5.59",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 489.58,
            "range": "± 9.55",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 100.52,
            "range": "± 0.99",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 150.03,
            "range": "± 8.08",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 263.74,
            "range": "± 4.59",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 283.83,
            "range": "± 2.96",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.27,
            "range": "± 0.94",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.27,
            "range": "± 0.75",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.11,
            "range": "± 0.34",
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
          "id": "a84aeb66df0203d81592898661660dd071d67188",
          "message": "ignore `paste` vulnerability\n\nreason why is in the osv-scanner.toml file",
          "timestamp": "2025-04-27T00:42:34+02:00",
          "tree_id": "eb574e14a5f45adb43133880b9dcde96ebbcbe65",
          "url": "https://github.com/tigerros/ruci/commit/a84aeb66df0203d81592898661660dd071d67188"
        },
        "date": 1745707413690,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 965.72,
            "range": "± 11.84",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3329.38,
            "range": "± 56.95",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27885.66,
            "range": "± 322.75",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.38,
            "range": "± 0.12",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 658.9,
            "range": "± 14.32",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2362.62,
            "range": "± 60.52",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 193.51,
            "range": "± 1.27",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 205.04,
            "range": "± 1.30",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 406.04,
            "range": "± 7.10",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 483.22,
            "range": "± 13.63",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 96.44,
            "range": "± 0.75",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 152.32,
            "range": "± 4.86",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 265.28,
            "range": "± 4.77",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 280.79,
            "range": "± 3.83",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.28,
            "range": "± 0.73",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.35,
            "range": "± 1.03",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.99,
            "range": "± 0.42",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ef6cf1931ca8f24010d3b3f5b368ebc81efb142d",
          "message": "Create dependabot.yml",
          "timestamp": "2025-04-27T00:51:56+02:00",
          "tree_id": "2efacc7566756755f4b3c2144ef3c9fe57ae7720",
          "url": "https://github.com/tigerros/ruci/commit/ef6cf1931ca8f24010d3b3f5b368ebc81efb142d"
        },
        "date": 1745707979838,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 972.97,
            "range": "± 21.06",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3252.72,
            "range": "± 82.47",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27824.05,
            "range": "± 542.04",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.42,
            "range": "± 5.55",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 661.89,
            "range": "± 10.18",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2408.63,
            "range": "± 43.60",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 195.36,
            "range": "± 20.28",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 207.91,
            "range": "± 3.35",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 423.05,
            "range": "± 13.65",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 497.02,
            "range": "± 12.49",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 97.99,
            "range": "± 1.34",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 156.66,
            "range": "± 5.14",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 263.57,
            "range": "± 3.43",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 283.61,
            "range": "± 6.72",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.55,
            "range": "± 0.28",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.89,
            "range": "± 0.42",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.16,
            "range": "± 0.42",
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
          "id": "bede632f434af5692d15d5b1d30a8578ad35deef",
          "message": "`Gui` struct, tcpstream example",
          "timestamp": "2025-04-28T03:06:30+02:00",
          "tree_id": "893324128840dc7f540dd02e1dfb218bfd98262d",
          "url": "https://github.com/tigerros/ruci/commit/bede632f434af5692d15d5b1d30a8578ad35deef"
        },
        "date": 1745802460403,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1012.45,
            "range": "± 42.02",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3272.57,
            "range": "± 42.83",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28121.28,
            "range": "± 323.38",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.11,
            "range": "± 0.16",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 653.76,
            "range": "± 10.07",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2367.27,
            "range": "± 233.20",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 195.49,
            "range": "± 12.27",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 209,
            "range": "± 3.84",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 414.76,
            "range": "± 16.06",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 500.41,
            "range": "± 10.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 99.17,
            "range": "± 0.81",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 160.32,
            "range": "± 3.84",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 269.13,
            "range": "± 8.74",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 277.2,
            "range": "± 4.06",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.31,
            "range": "± 0.41",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 43.17,
            "range": "± 0.74",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 43.27,
            "range": "± 0.19",
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
          "id": "208989ffc2b2ae24b0b455b0b1c6e4e8ae1fe3b1",
          "message": "`Gui` tests, polishing",
          "timestamp": "2025-04-29T22:24:51+02:00",
          "tree_id": "738101271fbbd307ec129236d2476c55347f7bd3",
          "url": "https://github.com/tigerros/ruci/commit/208989ffc2b2ae24b0b455b0b1c6e4e8ae1fe3b1"
        },
        "date": 1745958363313,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 952.16,
            "range": "± 12.98",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3305.47,
            "range": "± 58.79",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27768.05,
            "range": "± 497.59",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.69,
            "range": "± 0.09",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 580.86,
            "range": "± 26.21",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2384.34,
            "range": "± 80.73",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 194.91,
            "range": "± 2.10",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 230.71,
            "range": "± 2.28",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 457.48,
            "range": "± 5.71",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 495.78,
            "range": "± 13.69",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 108.82,
            "range": "± 1.80",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 127.68,
            "range": "± 1.42",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 231.46,
            "range": "± 3.56",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 242.11,
            "range": "± 4.00",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.38,
            "range": "± 0.25",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.81,
            "range": "± 0.48",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.14,
            "range": "± 0.23",
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
          "id": "ef4741318c0ef5aa5ade208edabd3c8d6bbef505",
          "message": "add semver check for breaking changes",
          "timestamp": "2025-04-29T23:03:12+02:00",
          "tree_id": "6f5371e950dd949ca7e292615f6d6ef6c0a3460b",
          "url": "https://github.com/tigerros/ruci/commit/ef4741318c0ef5aa5ade208edabd3c8d6bbef505"
        },
        "date": 1745960666793,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 945.25,
            "range": "± 23.69",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3035.49,
            "range": "± 38.41",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27668.05,
            "range": "± 497.25",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.72,
            "range": "± 0.41",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 587.91,
            "range": "± 22.92",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2364.82,
            "range": "± 56.65",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 196.11,
            "range": "± 2.97",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 224.17,
            "range": "± 4.29",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 398.54,
            "range": "± 5.50",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 484.03,
            "range": "± 6.86",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 126.39,
            "range": "± 2.29",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 148.34,
            "range": "± 5.00",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 259.57,
            "range": "± 4.45",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 243.86,
            "range": "± 51.64",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.87,
            "range": "± 0.82",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.72,
            "range": "± 0.95",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.22,
            "range": "± 0.62",
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
          "id": "151d91dd840b50ea9b50802265b61672a6ff93b6",
          "message": "this commit is a test for the semver check workflow",
          "timestamp": "2025-04-29T23:03:53+02:00",
          "tree_id": "9f6596fc80b0b18d6c830bfdf055338a37d063d9",
          "url": "https://github.com/tigerros/ruci/commit/151d91dd840b50ea9b50802265b61672a6ff93b6"
        },
        "date": 1745960702731,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 985.65,
            "range": "± 91.80",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3127.82,
            "range": "± 70.24",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27670.93,
            "range": "± 303.03",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.7,
            "range": "± 11.38",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 586.35,
            "range": "± 32.00",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2414.87,
            "range": "± 43.93",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 195.55,
            "range": "± 5.17",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 210.2,
            "range": "± 3.49",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 397,
            "range": "± 6.41",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 486.34,
            "range": "± 17.51",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 134.29,
            "range": "± 1.22",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 155.24,
            "range": "± 1.35",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 279.89,
            "range": "± 3.14",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 274.5,
            "range": "± 3.16",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.42,
            "range": "± 1.30",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.84,
            "range": "± 0.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.1,
            "range": "± 0.33",
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
          "id": "985294773f11cd323b434a6375cd7d1fac4332bc",
          "message": "fix wrong feature cfg",
          "timestamp": "2025-04-29T23:05:07+02:00",
          "tree_id": "de0b71ac9aa01a26a0852634fb35ee7958ca6f0f",
          "url": "https://github.com/tigerros/ruci/commit/985294773f11cd323b434a6375cd7d1fac4332bc"
        },
        "date": 1745960778936,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 963.48,
            "range": "± 17.71",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3143.75,
            "range": "± 968.75",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27754.81,
            "range": "± 440.20",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.77,
            "range": "± 22.12",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 575.54,
            "range": "± 17.40",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2385.84,
            "range": "± 36.50",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 197.44,
            "range": "± 4.62",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 220.21,
            "range": "± 5.14",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 405.1,
            "range": "± 7.13",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 511.5,
            "range": "± 11.67",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 132.22,
            "range": "± 1.75",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 131.69,
            "range": "± 1.87",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 245.07,
            "range": "± 2.94",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 250.47,
            "range": "± 8.29",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.74,
            "range": "± 0.24",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 42.36,
            "range": "± 0.32",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.81,
            "range": "± 1.22",
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
          "id": "a2844993626f65a07069ba6c84fcdceea7718c0f",
          "message": "fmt, clippy",
          "timestamp": "2025-04-29T23:07:30+02:00",
          "tree_id": "c1da687e47948209cfcc96ed68f32bebc9ecf541",
          "url": "https://github.com/tigerros/ruci/commit/a2844993626f65a07069ba6c84fcdceea7718c0f"
        },
        "date": 1745960921217,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 975.26,
            "range": "± 15.04",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 2960.21,
            "range": "± 36.61",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27570.47,
            "range": "± 17229.25",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.71,
            "range": "± 4.34",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 595.31,
            "range": "± 14.90",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2337.6,
            "range": "± 236.24",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 196.68,
            "range": "± 3.17",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 208.89,
            "range": "± 2.52",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 395.93,
            "range": "± 6.38",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 498.12,
            "range": "± 12.98",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 131.75,
            "range": "± 1.17",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 155.98,
            "range": "± 2.10",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 277.27,
            "range": "± 6.44",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 276.9,
            "range": "± 3.67",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.44,
            "range": "± 0.62",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.25,
            "range": "± 3.56",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.92,
            "range": "± 0.33",
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
          "id": "9b639d9139f5a233a2f673d29250d794ae17c88f",
          "message": "clippy, a little more coverage",
          "timestamp": "2025-04-29T23:13:16+02:00",
          "tree_id": "e955c1274e47dd86e6d07268e2ef896545e453cf",
          "url": "https://github.com/tigerros/ruci/commit/9b639d9139f5a233a2f673d29250d794ae17c88f"
        },
        "date": 1745961277744,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 964.11,
            "range": "± 20.77",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 2955.9,
            "range": "± 30.50",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27647.85,
            "range": "± 4429.31",
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
            "value": 597.45,
            "range": "± 10.24",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2370.24,
            "range": "± 46.32",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 193.36,
            "range": "± 24.99",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 210.79,
            "range": "± 2.43",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 393.4,
            "range": "± 6.13",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 475.11,
            "range": "± 10.86",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 136.22,
            "range": "± 2.55",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 158.92,
            "range": "± 1.52",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 277.97,
            "range": "± 3.96",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 277.18,
            "range": "± 3.51",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.43,
            "range": "± 0.13",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.76,
            "range": "± 0.55",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.12,
            "range": "± 1.26",
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
          "id": "eab42a98498e64b37fd58eefd40adb7803dc847d",
          "message": "improve tests\n\nnow when testing a message the `crate::Message`, `gui/engine::Message` is also tested",
          "timestamp": "2025-04-30T02:18:17+02:00",
          "tree_id": "76c48073dea0fcbe654f7e296f8d5cbadce243c6",
          "url": "https://github.com/tigerros/ruci/commit/eab42a98498e64b37fd58eefd40adb7803dc847d"
        },
        "date": 1745972376252,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 982.6,
            "range": "± 8.26",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 2975.51,
            "range": "± 86.94",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27502.44,
            "range": "± 279.29",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.69,
            "range": "± 0.25",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 584.68,
            "range": "± 17.19",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2405.35,
            "range": "± 48.09",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 199.41,
            "range": "± 4.41",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 210.97,
            "range": "± 5.27",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 415.44,
            "range": "± 11.08",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 485.31,
            "range": "± 7.51",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 129.07,
            "range": "± 1.26",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 152.82,
            "range": "± 2.08",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 273.08,
            "range": "± 2.91",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 273.81,
            "range": "± 34.89",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.53,
            "range": "± 0.27",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.77,
            "range": "± 0.35",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.56,
            "range": "± 0.43",
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
          "id": "050d49d02b6e3254aab932a6fb9fe6c0b3424810",
          "message": "fix import",
          "timestamp": "2025-04-30T02:22:57+02:00",
          "tree_id": "f0a68a54760bc5a3ee23d11e9f1ccc1dd5129dbc",
          "url": "https://github.com/tigerros/ruci/commit/050d49d02b6e3254aab932a6fb9fe6c0b3424810"
        },
        "date": 1745972646389,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1028.42,
            "range": "± 294.42",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 2992.03,
            "range": "± 41.83",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28031.22,
            "range": "± 328.28",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.7,
            "range": "± 0.17",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 603.42,
            "range": "± 39.12",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2353.97,
            "range": "± 57.12",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 206.26,
            "range": "± 2.42",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 247.33,
            "range": "± 4.12",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 401.38,
            "range": "± 4.74",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 500.4,
            "range": "± 14.65",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 110.53,
            "range": "± 1.38",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 131.16,
            "range": "± 1.84",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 233.79,
            "range": "± 25.79",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 242.18,
            "range": "± 4.33",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.75,
            "range": "± 0.49",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 42.32,
            "range": "± 0.38",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.78,
            "range": "± 1.50",
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
          "id": "c9383671efd9a4997aa5313a27c485aa19bda853",
          "message": "1.1.0\n\n- add `Gui` struct for communicating with a GUI\n- recommend `Engine.strict` to be `true`\n- even more flexible I/O, `Engine::send` doesnt require `Read`, `Engine::read` does not require `Write`\n- docs improvements\n- readme improvements",
          "timestamp": "2025-05-01T01:41:06+02:00",
          "tree_id": "747ce378b8659e95d823e17a0ebc4772e79b6ab8",
          "url": "https://github.com/tigerros/ruci/commit/c9383671efd9a4997aa5313a27c485aa19bda853"
        },
        "date": 1746056536771,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 946.2,
            "range": "± 15.52",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3082.17,
            "range": "± 87.49",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27668.33,
            "range": "± 278.95",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.72,
            "range": "± 21.53",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 618.87,
            "range": "± 15.01",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2374.12,
            "range": "± 70.31",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 198.76,
            "range": "± 103.41",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 208.87,
            "range": "± 2.67",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 397.34,
            "range": "± 6.85",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 492.66,
            "range": "± 13.46",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 144.41,
            "range": "± 1.79",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 169.97,
            "range": "± 1.87",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 277.81,
            "range": "± 26.20",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 282.62,
            "range": "± 4.65",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.75,
            "range": "± 0.24",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 42.41,
            "range": "± 1.07",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.78,
            "range": "± 0.28",
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
          "id": "4eed1ecdddbe9722200add50d967755cef722fb7",
          "message": "Merge branch 'master' of https://github.com/tigerros/ruci",
          "timestamp": "2025-05-01T02:42:41+02:00",
          "tree_id": "c6e33fe0efeee16149d3802faecadc12b27a1eca",
          "url": "https://github.com/tigerros/ruci/commit/4eed1ecdddbe9722200add50d967755cef722fb7"
        },
        "date": 1746060233902,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 951.67,
            "range": "± 14.60",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3159.42,
            "range": "± 739.77",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27383.27,
            "range": "± 319.06",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.71,
            "range": "± 0.14",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 628.37,
            "range": "± 8.84",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2383.14,
            "range": "± 32.53",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 196.85,
            "range": "± 4.51",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 231.62,
            "range": "± 9.66",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 401.38,
            "range": "± 4.57",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 506.94,
            "range": "± 7.30",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 109.83,
            "range": "± 7.54",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 136.94,
            "range": "± 2.10",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 229.22,
            "range": "± 4.03",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 253.06,
            "range": "± 3.86",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.41,
            "range": "± 0.09",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.74,
            "range": "± 0.44",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.12,
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
          "id": "bc709163b5159fd0c9f4b1640b5c7cdee443ce4d",
          "message": "2.0.0\n\nignore 1.1.0 - made it into 2.0.0\n\nbreaking:\n- lifted `MessageParseError.expected`. Previously it was in all variants, now its a field and the variants are an enum with no fields.\n- `ReadError` now has a `got` field to tell you what caused an error\n- even more flexible I/O, `Engine::send` doesnt require `Read`, `Engine::read` does not require `Write`\n\nother:\n- add `Gui` struct for communicating with a GUI\n- recommend `Engine.strict` to be `true`\n- more tests => fix `movestogo` being serialized as `moves_to_go`\n- docs improvements\n- readme improvements",
          "timestamp": "2025-05-01T20:49:27+02:00",
          "tree_id": "80371a0c3bfc91cf5d4c0dff91f5bd7ca972fb91",
          "url": "https://github.com/tigerros/ruci/commit/bc709163b5159fd0c9f4b1640b5c7cdee443ce4d"
        },
        "date": 1746125444664,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 961.04,
            "range": "± 9.63",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 2997.72,
            "range": "± 47.43",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28890.36,
            "range": "± 478.18",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.4,
            "range": "± 0.22",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 609.85,
            "range": "± 13.38",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2438.77,
            "range": "± 31.01",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 197.14,
            "range": "± 4.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 234.27,
            "range": "± 4.12",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 398.72,
            "range": "± 7.25",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 470.56,
            "range": "± 10.43",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 113.47,
            "range": "± 1.82",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 130,
            "range": "± 2.02",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 235.18,
            "range": "± 4.58",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 230.2,
            "range": "± 2.62",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.75,
            "range": "± 1.77",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.74,
            "range": "± 0.19",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 43.92,
            "range": "± 0.17",
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
          "id": "6475dae20574151cd2bab00aac9211daac621c06",
          "message": "adjust workflows running on branches",
          "timestamp": "2025-05-01T20:56:26+02:00",
          "tree_id": "de3cee00e4fc8a7bdfe4a35ceed533b02831a220",
          "url": "https://github.com/tigerros/ruci/commit/6475dae20574151cd2bab00aac9211daac621c06"
        },
        "date": 1746125859582,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1161.83,
            "range": "± 32.45",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 2976.61,
            "range": "± 57.55",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28970.68,
            "range": "± 356.10",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.42,
            "range": "± 0.96",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 618.83,
            "range": "± 14.98",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2434.25,
            "range": "± 13.64",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 195.64,
            "range": "± 2.82",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 246.3,
            "range": "± 3.58",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 443.31,
            "range": "± 18.44",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 462.3,
            "range": "± 9.12",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 106.45,
            "range": "± 1.34",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 126.75,
            "range": "± 1.24",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 231.98,
            "range": "± 3.77",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 232.71,
            "range": "± 3.48",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.5,
            "range": "± 3.62",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.88,
            "range": "± 6.37",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.94,
            "range": "± 0.16",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "729ecbcef22ad702ab6288c4fd465e92ac975bad",
          "message": "Merge pull request #7 from tigerros/dependabot/github_actions/google/osv-scanner-action-2.0.2\n\nBump google/osv-scanner-action from 2.0.1 to 2.0.2",
          "timestamp": "2025-05-05T07:12:11+02:00",
          "tree_id": "3a649285c04a394f3a61454ee8790b2a25b5e565",
          "url": "https://github.com/tigerros/ruci/commit/729ecbcef22ad702ab6288c4fd465e92ac975bad"
        },
        "date": 1746422003982,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 978.99,
            "range": "± 21.60",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3092.94,
            "range": "± 32.63",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28104.63,
            "range": "± 291.56",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.01,
            "range": "± 0.20",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 590.64,
            "range": "± 10.22",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2345.18,
            "range": "± 42.30",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 200.73,
            "range": "± 1.72",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 211.9,
            "range": "± 1.84",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 399.77,
            "range": "± 7.09",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 480.16,
            "range": "± 7.05",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 139.56,
            "range": "± 2.70",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 137.95,
            "range": "± 3.51",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 246.97,
            "range": "± 3.99",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 255.91,
            "range": "± 3.96",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.47,
            "range": "± 0.49",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 42.66,
            "range": "± 0.21",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.48,
            "range": "± 0.38",
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
          "id": "ac8664fbf50f05f56db227179e9e20e0b867d5ad",
          "message": "clippy workflow on thumbv6m-none-eabi",
          "timestamp": "2025-05-05T15:00:21+02:00",
          "tree_id": "ab5c30abd93f6123625f2304b89251659f1c0157",
          "url": "https://github.com/tigerros/ruci/commit/ac8664fbf50f05f56db227179e9e20e0b867d5ad"
        },
        "date": 1746450087906,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 986.1,
            "range": "± 21.52",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 2974.17,
            "range": "± 28.06",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28336.15,
            "range": "± 733.48",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.02,
            "range": "± 0.40",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 599.88,
            "range": "± 18.01",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2300.87,
            "range": "± 77.24",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 202.53,
            "range": "± 3.56",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 234.04,
            "range": "± 2.19",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 400.47,
            "range": "± 9.04",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 478.38,
            "range": "± 8.73",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 110.07,
            "range": "± 1.45",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 133.3,
            "range": "± 2.56",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 230.17,
            "range": "± 6.24",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 238.32,
            "range": "± 2.74",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.49,
            "range": "± 0.29",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 42.62,
            "range": "± 0.77",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.49,
            "range": "± 0.74",
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
          "id": "260ff9ad03073773e463bdf28c18dd73a7841723",
          "message": "Update clippy.yml",
          "timestamp": "2025-05-05T15:24:18+02:00",
          "tree_id": "7564d86cc49f5c57c3cd04e10e4562cc8e459295",
          "url": "https://github.com/tigerros/ruci/commit/260ff9ad03073773e463bdf28c18dd73a7841723"
        },
        "date": 1746451516901,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 976.9,
            "range": "± 7.72",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 2949.85,
            "range": "± 155.49",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28156.25,
            "range": "± 736.47",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23,
            "range": "± 0.23",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 619.21,
            "range": "± 20.77",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2400.46,
            "range": "± 29.17",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 203.25,
            "range": "± 5.37",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 236.74,
            "range": "± 3.49",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 448.42,
            "range": "± 8.13",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 480.95,
            "range": "± 5.84",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 98.44,
            "range": "± 1.11",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 134.29,
            "range": "± 5.18",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 223.31,
            "range": "± 1.09",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 236.19,
            "range": "± 2.84",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.82,
            "range": "± 0.45",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 42.63,
            "range": "± 0.48",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.46,
            "range": "± 0.57",
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
          "id": "33de56554c5f8663697f49637405f621c16fb232",
          "message": "add purely check workflow, doc check workflow",
          "timestamp": "2025-05-07T01:06:15+02:00",
          "tree_id": "aedb290572d2729206d2ace63427a35524d24c58",
          "url": "https://github.com/tigerros/ruci/commit/33de56554c5f8663697f49637405f621c16fb232"
        },
        "date": 1746572842042,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 983.95,
            "range": "± 111.79",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 2990.04,
            "range": "± 34.39",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27912.42,
            "range": "± 427.72",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.38,
            "range": "± 0.11",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 588.92,
            "range": "± 20.88",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2398.03,
            "range": "± 40.07",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 195.43,
            "range": "± 3.08",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 224.73,
            "range": "± 3.46",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 384.81,
            "range": "± 7.90",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 467.72,
            "range": "± 6.72",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 131.48,
            "range": "± 1.61",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 139.3,
            "range": "± 1.67",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 237.76,
            "range": "± 1.98",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 247.59,
            "range": "± 2.18",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 21.88,
            "range": "± 0.33",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.2,
            "range": "± 0.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.7,
            "range": "± 0.41",
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
          "id": "8dc5baf31dac5367ed5d29a47923bec455c62611",
          "message": "fix doc link",
          "timestamp": "2025-05-07T01:16:43+02:00",
          "tree_id": "4e1721310982edcea2dbf2b972340d94d668376c",
          "url": "https://github.com/tigerros/ruci/commit/8dc5baf31dac5367ed5d29a47923bec455c62611"
        },
        "date": 1746573465790,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 968.38,
            "range": "± 14.35",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 2942.04,
            "range": "± 31.54",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27832.53,
            "range": "± 326.07",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.39,
            "range": "± 0.50",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 616.29,
            "range": "± 26.91",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2387.19,
            "range": "± 57.51",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 198.77,
            "range": "± 1.85",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 235.97,
            "range": "± 3.51",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 412.63,
            "range": "± 5.04",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 483.74,
            "range": "± 9.25",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 111.87,
            "range": "± 0.81",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 136.86,
            "range": "± 3.86",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 229.95,
            "range": "± 2.30",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 241.72,
            "range": "± 2.55",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.17,
            "range": "± 0.62",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.54,
            "range": "± 0.44",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 43.25,
            "range": "± 1.11",
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
          "id": "dd690886866a895b983f2e155b11627b577c0b14",
          "message": "Update check.yml",
          "timestamp": "2025-05-07T01:19:03+02:00",
          "tree_id": "a38860988bdbc7998184189cc1cdcc1b1bdc78e5",
          "url": "https://github.com/tigerros/ruci/commit/dd690886866a895b983f2e155b11627b577c0b14"
        },
        "date": 1746573606730,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 979.56,
            "range": "± 14.77",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 2973.07,
            "range": "± 113.29",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27957.28,
            "range": "± 706.37",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.4,
            "range": "± 0.20",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 625.26,
            "range": "± 10.40",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2412.03,
            "range": "± 33.83",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 196.73,
            "range": "± 1.95",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 222.02,
            "range": "± 2.49",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 387.33,
            "range": "± 5.72",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 473.72,
            "range": "± 107.86",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 135.96,
            "range": "± 4.64",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 137.35,
            "range": "± 2.03",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 220.23,
            "range": "± 1.81",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 231.25,
            "range": "± 2.30",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 21.9,
            "range": "± 0.45",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.19,
            "range": "± 0.63",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.64,
            "range": "± 0.64",
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
          "id": "b8f65906e0cfdee950560cb0d4915f6612073140",
          "message": "Update check.yml",
          "timestamp": "2025-05-07T01:29:23+02:00",
          "tree_id": "293e4a145458bc3f212db37b35d5d1bff77c57f3",
          "url": "https://github.com/tigerros/ruci/commit/b8f65906e0cfdee950560cb0d4915f6612073140"
        },
        "date": 1746574236059,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 978.66,
            "range": "± 15.11",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3034.45,
            "range": "± 148.30",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27929.66,
            "range": "± 383.25",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.4,
            "range": "± 0.23",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 601.03,
            "range": "± 19.94",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2370.46,
            "range": "± 27.54",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 200.36,
            "range": "± 2.57",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 214.1,
            "range": "± 3.12",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 389.31,
            "range": "± 6.33",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 479.19,
            "range": "± 8.45",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 137.5,
            "range": "± 1.37",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 158.39,
            "range": "± 2.56",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 256.54,
            "range": "± 5.04",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 271.33,
            "range": "± 2.92",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.55,
            "range": "± 0.43",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 42.43,
            "range": "± 0.80",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 43.24,
            "range": "± 0.73",
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
          "id": "9c17c548562d281df019c906250c8b60fbbfd34c",
          "message": "update deps to min possible ver using `-Z direct-minimal-versions`\n\nalso make benchmark alert threshold lower",
          "timestamp": "2025-05-07T16:50:41+02:00",
          "tree_id": "cc55d9f660b95455a40d86ddf53f111f84b6c58a",
          "url": "https://github.com/tigerros/ruci/commit/9c17c548562d281df019c906250c8b60fbbfd34c"
        },
        "date": 1746629516117,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1027.62,
            "range": "± 11.74",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3248.64,
            "range": "± 46.51",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28440.06,
            "range": "± 340.98",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.77,
            "range": "± 0.38",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 643.08,
            "range": "± 17.41",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2428.77,
            "range": "± 39.16",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 204.39,
            "range": "± 21.91",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 209.38,
            "range": "± 3.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 382.33,
            "range": "± 4.05",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 461.08,
            "range": "± 8.90",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 139.35,
            "range": "± 3.36",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 163.28,
            "range": "± 2.64",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 256.03,
            "range": "± 3.73",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 267.44,
            "range": "± 4.28",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.32,
            "range": "± 0.40",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.24,
            "range": "± 0.31",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.84,
            "range": "± 0.50",
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
          "id": "35c3022a85432802ddfbe4d2eb219c42fd57b30e",
          "message": "sort manifests, check stable",
          "timestamp": "2025-05-07T18:28:27+02:00",
          "tree_id": "ebcbe998e5105ff9ae4bd590318cec230a47a2a5",
          "url": "https://github.com/tigerros/ruci/commit/35c3022a85432802ddfbe4d2eb219c42fd57b30e"
        },
        "date": 1746635376581,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1018.23,
            "range": "± 14.73",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3235.54,
            "range": "± 55.33",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28421.66,
            "range": "± 551.23",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.7,
            "range": "± 1.11",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 624.35,
            "range": "± 27.70",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2451.54,
            "range": "± 35.18",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 202.49,
            "range": "± 1.82",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 208.05,
            "range": "± 1.92",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 400.22,
            "range": "± 7.77",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 459.58,
            "range": "± 9.31",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 132.77,
            "range": "± 3.78",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 137.8,
            "range": "± 3.54",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 224,
            "range": "± 2.62",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 246.82,
            "range": "± 3.06",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.33,
            "range": "± 0.38",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.18,
            "range": "± 3.59",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.75,
            "range": "± 0.29",
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
          "id": "774b5991c4a02f8496923ff2b9b8c77e31025501",
          "message": "better examples, fix workflow",
          "timestamp": "2025-05-07T20:54:55+02:00",
          "tree_id": "7ea431207a094001b401bb928f8e870d4fbf8f72",
          "url": "https://github.com/tigerros/ruci/commit/774b5991c4a02f8496923ff2b9b8c77e31025501"
        },
        "date": 1746644162227,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1003.64,
            "range": "± 15.34",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3257.52,
            "range": "± 45.43",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28500.33,
            "range": "± 486.46",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.71,
            "range": "± 0.13",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 639.68,
            "range": "± 20.91",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2444.17,
            "range": "± 76.10",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 199.2,
            "range": "± 1.82",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 205.54,
            "range": "± 2.79",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 395.3,
            "range": "± 7.30",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 465.95,
            "range": "± 11.75",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 130.62,
            "range": "± 2.15",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 157.85,
            "range": "± 18.84",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 247.33,
            "range": "± 2.14",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 262.52,
            "range": "± 3.54",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.38,
            "range": "± 0.56",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.15,
            "range": "± 0.34",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 43.45,
            "range": "± 0.76",
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
          "id": "45ad9a8966212734dc9a2dd1194bb4ef2c4244a3",
          "message": "update to 2024 edition",
          "timestamp": "2025-05-07T22:11:54+02:00",
          "tree_id": "705e738a990803b58ba8ddd8d007fcc5387b7260",
          "url": "https://github.com/tigerros/ruci/commit/45ad9a8966212734dc9a2dd1194bb4ef2c4244a3"
        },
        "date": 1746648782094,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1012.33,
            "range": "± 11.73",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3278.01,
            "range": "± 69.55",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28378.16,
            "range": "± 472.85",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.39,
            "range": "± 0.28",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 631.11,
            "range": "± 25.54",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2469.68,
            "range": "± 15.92",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 202.17,
            "range": "± 1.70",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 208.11,
            "range": "± 4.23",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 393.94,
            "range": "± 138.28",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 470.6,
            "range": "± 7.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 137.04,
            "range": "± 11.29",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 163.64,
            "range": "± 2.82",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 252.47,
            "range": "± 4.87",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 265,
            "range": "± 4.03",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.46,
            "range": "± 0.59",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.27,
            "range": "± 0.81",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 43.45,
            "range": "± 17.05",
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
          "id": "5c460e7ab078479ad8e3fa2c656d2fa963d7c11e",
          "message": "fmt",
          "timestamp": "2025-05-07T22:15:34+02:00",
          "tree_id": "c134520724971c24474b5897a4e2c62a475b170a",
          "url": "https://github.com/tigerros/ruci/commit/5c460e7ab078479ad8e3fa2c656d2fa963d7c11e"
        },
        "date": 1746649015847,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1019.03,
            "range": "± 20.17",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3225.36,
            "range": "± 52.64",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28571.47,
            "range": "± 254.00",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.38,
            "range": "± 0.12",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 617.8,
            "range": "± 49.92",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2446.74,
            "range": "± 29.15",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 201.57,
            "range": "± 2.00",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 210.66,
            "range": "± 4.80",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 385.58,
            "range": "± 5.73",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 469.87,
            "range": "± 6.65",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 132.31,
            "range": "± 4.49",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 159.34,
            "range": "± 4.98",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 250.27,
            "range": "± 2.28",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 261.09,
            "range": "± 3.85",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.4,
            "range": "± 0.31",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.21,
            "range": "± 0.74",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.79,
            "range": "± 0.73",
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
          "id": "0efe608baae43e094869efc7a65ca5ead1dfc71a",
          "message": "2.1.0\n\nupdate rust-version and edition to match the code. i dont consider it a breaking change because it wouldnt compile on those earlier ones anyway",
          "timestamp": "2025-05-07T22:24:40+02:00",
          "tree_id": "f9c3baf3b273292b876912b796b6f37418b7579f",
          "url": "https://github.com/tigerros/ruci/commit/0efe608baae43e094869efc7a65ca5ead1dfc71a"
        },
        "date": 1746649546864,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1004.39,
            "range": "± 14.75",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3267.68,
            "range": "± 45.96",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28436.53,
            "range": "± 248.43",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.4,
            "range": "± 5.45",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 643.08,
            "range": "± 59.33",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2419.65,
            "range": "± 26.01",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 208.56,
            "range": "± 2.79",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 252.35,
            "range": "± 3.72",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 386.6,
            "range": "± 6.10",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 468.76,
            "range": "± 11.47",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 129.19,
            "range": "± 2.09",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 148.89,
            "range": "± 2.36",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 225.44,
            "range": "± 2.90",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 242.01,
            "range": "± 2.33",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.38,
            "range": "± 0.46",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.17,
            "range": "± 0.72",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 43.48,
            "range": "± 32.62",
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
          "id": "a640ceb4e6cf1c243c86947106c3a86e62c93a9c",
          "message": "make fmt check workflow use nightly fmt",
          "timestamp": "2025-05-08T01:13:29+02:00",
          "tree_id": "df3e790a069114e9c6157e41e3e36af4d15571f5",
          "url": "https://github.com/tigerros/ruci/commit/a640ceb4e6cf1c243c86947106c3a86e62c93a9c"
        },
        "date": 1746659673644,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1017.22,
            "range": "± 35.64",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3247.78,
            "range": "± 63.15",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28550.14,
            "range": "± 1163.70",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.4,
            "range": "± 1.70",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 637.47,
            "range": "± 19.23",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2406.51,
            "range": "± 40.24",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 192.93,
            "range": "± 4.24",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 205.75,
            "range": "± 3.06",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 391.82,
            "range": "± 6.08",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 471.03,
            "range": "± 7.52",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 143.23,
            "range": "± 2.37",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 158.86,
            "range": "± 2.65",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 256.27,
            "range": "± 4.15",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 269.36,
            "range": "± 2.40",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.26,
            "range": "± 4.06",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.17,
            "range": "± 0.31",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.77,
            "range": "± 0.33",
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
          "id": "3d32f8e42412443ed4522de490819ace18ca5b40",
          "message": "make fmt check workflow use nightly fmt",
          "timestamp": "2025-05-08T01:44:18+02:00",
          "tree_id": "247d6af05538dba4dbf4357385939e60af924fa0",
          "url": "https://github.com/tigerros/ruci/commit/3d32f8e42412443ed4522de490819ace18ca5b40"
        },
        "date": 1746661525327,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1000.91,
            "range": "± 356.50",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3317.1,
            "range": "± 35.31",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28429.89,
            "range": "± 380.83",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 22.39,
            "range": "± 0.16",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 616.58,
            "range": "± 28.15",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2423.54,
            "range": "± 51.12",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 195.89,
            "range": "± 2.12",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 209.21,
            "range": "± 2.86",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 393.96,
            "range": "± 6.93",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 470.49,
            "range": "± 8.15",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 144.88,
            "range": "± 3.49",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 161.6,
            "range": "± 2.29",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 260.04,
            "range": "± 2.11",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 271.79,
            "range": "± 3.89",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.47,
            "range": "± 0.35",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.25,
            "range": "± 0.52",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 43.45,
            "range": "± 0.72",
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
          "id": "61a4d328c99784d7492c108d2ce4023f3dd1ea2a",
          "message": "add `-Z direct-minimal-versions` to check workflow",
          "timestamp": "2025-05-11T17:38:46+02:00",
          "tree_id": "f0c39c12e87e6ba0134153d882aea54fbd792ab8",
          "url": "https://github.com/tigerros/ruci/commit/61a4d328c99784d7492c108d2ce4023f3dd1ea2a"
        },
        "date": 1746977992522,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 968.61,
            "range": "± 11.41",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3030.34,
            "range": "± 58.18",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28355.78,
            "range": "± 467.26",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 24.25,
            "range": "± 0.19",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 621.95,
            "range": "± 18.06",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2395.7,
            "range": "± 31.92",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 194.16,
            "range": "± 3.54",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 210.89,
            "range": "± 1.74",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 398.24,
            "range": "± 12.14",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 468.69,
            "range": "± 9.98",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 129.4,
            "range": "± 2.26",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 152.7,
            "range": "± 2.82",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 258.07,
            "range": "± 5.16",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 257.17,
            "range": "± 4.13",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 21.9,
            "range": "± 0.34",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.19,
            "range": "± 0.60",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 42.7,
            "range": "± 0.78",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "committer": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "distinct": true,
          "id": "a91657e0aac70eae37ce34d3e3a2ce29ddedbbcb",
          "message": "remove empty default feature",
          "timestamp": "2025-06-07T20:35:03+02:00",
          "tree_id": "d3c19efe919e5c251b0880dcee1e79755f87f92f",
          "url": "https://github.com/tigerros/ruci/commit/a91657e0aac70eae37ce34d3e3a2ce29ddedbbcb"
        },
        "date": 1749321355214,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 976.82,
            "range": "± 26.69",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3178.78,
            "range": "± 44.45",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27657.88,
            "range": "± 498.96",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.32,
            "range": "± 5.72",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 615.84,
            "range": "± 13.34",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2360.92,
            "range": "± 38.46",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 195.53,
            "range": "± 3.66",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 240.51,
            "range": "± 3.32",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 443.3,
            "range": "± 6.12",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 470.79,
            "range": "± 7.02",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 100.41,
            "range": "± 1.13",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 127.8,
            "range": "± 1.54",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 223.27,
            "range": "± 3.66",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 230.14,
            "range": "± 2.42",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.73,
            "range": "± 0.28",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.84,
            "range": "± 0.44",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.23,
            "range": "± 0.34",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "committer": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "distinct": true,
          "id": "2503155cb38becacefffaba817768f3a7ccbf525",
          "message": "add doctests to coverage",
          "timestamp": "2025-06-07T22:53:03+02:00",
          "tree_id": "ca68e9c4567823d95e59017e0d5190822b1d6d0e",
          "url": "https://github.com/tigerros/ruci/commit/2503155cb38becacefffaba817768f3a7ccbf525"
        },
        "date": 1749329643072,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 969.72,
            "range": "± 21.20",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3154.92,
            "range": "± 35.32",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27906.28,
            "range": "± 580.53",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.33,
            "range": "± 0.22",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 611.07,
            "range": "± 13.41",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2349.83,
            "range": "± 40.89",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 195.73,
            "range": "± 5.57",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 241.1,
            "range": "± 1.59",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 438.8,
            "range": "± 7.29",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 461.03,
            "range": "± 112.88",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 100.73,
            "range": "± 0.93",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 126.79,
            "range": "± 1.47",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 213.29,
            "range": "± 2.49",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 223.92,
            "range": "± 1.88",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.85,
            "range": "± 0.92",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.56,
            "range": "± 0.19",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 43.32,
            "range": "± 0.45",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "committer": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "distinct": true,
          "id": "00d58b82aff848a39413212fb7914e2e726c8095",
          "message": "add safety comment to readme",
          "timestamp": "2025-06-07T23:41:59+02:00",
          "tree_id": "ad599d6870f84dae8dd7e87235d6378410731c69",
          "url": "https://github.com/tigerros/ruci/commit/00d58b82aff848a39413212fb7914e2e726c8095"
        },
        "date": 1749332570414,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 985.11,
            "range": "± 18.91",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3134.69,
            "range": "± 1969.27",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27965.43,
            "range": "± 2774.97",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.31,
            "range": "± 0.40",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 621.1,
            "range": "± 15.72",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2366.03,
            "range": "± 45.97",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 195.93,
            "range": "± 2.67",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 243.71,
            "range": "± 2.04",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 443.24,
            "range": "± 6.58",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 468.39,
            "range": "± 6.14",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 110.5,
            "range": "± 1.68",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 128.67,
            "range": "± 7.84",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 227.63,
            "range": "± 2.45",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 224.13,
            "range": "± 4.17",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 26.38,
            "range": "± 0.90",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 42.17,
            "range": "± 0.92",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.22,
            "range": "± 0.87",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "committer": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "distinct": true,
          "id": "91816e8148aeef2e1db3f2ba836e0fa16db935c6",
          "message": "update to shakmaty 0.28.0, remove BestMove::take_normal\n\ntake_normal no longer necessary since UciMove is now Copy",
          "timestamp": "2025-06-16T19:31:02+02:00",
          "tree_id": "e7aafcfcf72ea47c619437585c109d47ebb2e9f5",
          "url": "https://github.com/tigerros/ruci/commit/91816e8148aeef2e1db3f2ba836e0fa16db935c6"
        },
        "date": 1750095129656,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 963.19,
            "range": "± 18.39",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3021.47,
            "range": "± 57.63",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28517.61,
            "range": "± 477.48",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.32,
            "range": "± 12.84",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 611.36,
            "range": "± 18.84",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2385.92,
            "range": "± 37.23",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 195.86,
            "range": "± 2.53",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 249.29,
            "range": "± 4.87",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 453,
            "range": "± 13.92",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 481.12,
            "range": "± 7.53",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 105.68,
            "range": "± 1.58",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 129.86,
            "range": "± 1.79",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 226.24,
            "range": "± 2.03",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 227.24,
            "range": "± 2.59",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 22.41,
            "range": "± 0.17",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 42.3,
            "range": "± 0.80",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 43.23,
            "range": "± 0.83",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "committer": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "distinct": true,
          "id": "64063d20d364a911c4c85934fa5f6124a1def313",
          "message": "sort manifest, fix clippy workflow",
          "timestamp": "2025-06-25T17:59:37+02:00",
          "tree_id": "6d39f8a16650ae4ac29a96adbda6f370431038d9",
          "url": "https://github.com/tigerros/ruci/commit/64063d20d364a911c4c85934fa5f6124a1def313"
        },
        "date": 1750867253554,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 984.64,
            "range": "± 13.81",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3167.72,
            "range": "± 66.34",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27963.89,
            "range": "± 535.56",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.33,
            "range": "± 0.50",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 592.77,
            "range": "± 9.46",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2403.19,
            "range": "± 55.06",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 208.13,
            "range": "± 3.76",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 258.81,
            "range": "± 6.81",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 468.03,
            "range": "± 6.58",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 504.93,
            "range": "± 5.56",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 109.1,
            "range": "± 2.64",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 137.08,
            "range": "± 1.43",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 239.07,
            "range": "± 2.79",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 243.34,
            "range": "± 1.79",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 24.02,
            "range": "± 0.09",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 44.19,
            "range": "± 0.22",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 46.86,
            "range": "± 0.70",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "committer": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "distinct": true,
          "id": "8c0b9962185e86bfe317f5f29c52e63394fc10ac",
          "message": "run cargo doc on all features",
          "timestamp": "2025-06-25T18:04:50+02:00",
          "tree_id": "1805d9076368afa6faaf506fb4ae6cb0c045a3a3",
          "url": "https://github.com/tigerros/ruci/commit/8c0b9962185e86bfe317f5f29c52e63394fc10ac"
        },
        "date": 1750867569221,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 976.85,
            "range": "± 19.28",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3144.49,
            "range": "± 27.47",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28024.54,
            "range": "± 188.20",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.31,
            "range": "± 0.16",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 625.21,
            "range": "± 26.78",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2340.12,
            "range": "± 49.35",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 197.79,
            "range": "± 5.27",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 251.32,
            "range": "± 4.66",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 448.24,
            "range": "± 7.22",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 499.78,
            "range": "± 5.80",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 113.72,
            "range": "± 1.68",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 139.6,
            "range": "± 140.13",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 229.9,
            "range": "± 2.63",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 254.06,
            "range": "± 5.45",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 23.99,
            "range": "± 0.14",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 45.21,
            "range": "± 0.35",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 46.24,
            "range": "± 0.30",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "committer": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "distinct": true,
          "id": "111309b562df7970e7302ca33dea40c134ed38e8",
          "message": "switch to latest tarpaulin instead of nightly",
          "timestamp": "2025-06-25T18:41:19+02:00",
          "tree_id": "e46931c762e53b16ad10bfabbc07441becd843a5",
          "url": "https://github.com/tigerros/ruci/commit/111309b562df7970e7302ca33dea40c134ed38e8"
        },
        "date": 1750869761587,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1010.09,
            "range": "± 24.72",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3018.67,
            "range": "± 32.30",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28301.65,
            "range": "± 263.74",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.33,
            "range": "± 0.22",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 626.88,
            "range": "± 12.99",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2436.26,
            "range": "± 44.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 199.71,
            "range": "± 2.95",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 254.1,
            "range": "± 2.15",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 448.62,
            "range": "± 5.68",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 508.08,
            "range": "± 5.46",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 113.94,
            "range": "± 2.38",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 138.95,
            "range": "± 9.33",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 228.43,
            "range": "± 3.41",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 250.32,
            "range": "± 7.02",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 23.76,
            "range": "± 0.14",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 43.92,
            "range": "± 0.23",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 45.15,
            "range": "± 0.43",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "committer": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "distinct": true,
          "id": "ac1e80bfeeb14c11b61533053881793b2149b84b",
          "message": "Update Cargo.toml",
          "timestamp": "2025-06-26T19:46:22+02:00",
          "tree_id": "5e1810c2c9d6a43fed003f2dfb2a94adedfc8a4d",
          "url": "https://github.com/tigerros/ruci/commit/ac1e80bfeeb14c11b61533053881793b2149b84b"
        },
        "date": 1750960058192,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 990.82,
            "range": "± 22.89",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3172.04,
            "range": "± 174.84",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28032.04,
            "range": "± 432.75",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.31,
            "range": "± 0.14",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 642.32,
            "range": "± 31.22",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2376.02,
            "range": "± 37.85",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 206.06,
            "range": "± 4.20",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 260.58,
            "range": "± 1.80",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 449.67,
            "range": "± 7.72",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 486.25,
            "range": "± 4.89",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 109.35,
            "range": "± 1.37",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 137.75,
            "range": "± 2.68",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 227.94,
            "range": "± 1.74",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 248.76,
            "range": "± 2.56",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 23.34,
            "range": "± 0.16",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 42.96,
            "range": "± 0.26",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 45.44,
            "range": "± 0.46",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "committer": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "distinct": true,
          "id": "b9608d4633a50d58b2088bc38322c4515ad85d49",
          "message": "add `into_owned` to turn lifetimes into `'static`",
          "timestamp": "2025-07-01T19:56:13+02:00",
          "tree_id": "2a502a63a58d9b941f4546436c99111cdda8d366",
          "url": "https://github.com/tigerros/ruci/commit/b9608d4633a50d58b2088bc38322c4515ad85d49"
        },
        "date": 1751392647702,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 966.37,
            "range": "± 225.79",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 2876.87,
            "range": "± 190.36",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 26517.57,
            "range": "± 2170.08",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.34,
            "range": "± 2.03",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 587.11,
            "range": "± 59.31",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2287.75,
            "range": "± 178.02",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 181.82,
            "range": "± 9.18",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 237.44,
            "range": "± 19.07",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 422.15,
            "range": "± 32.14",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 464.96,
            "range": "± 35.13",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 105.55,
            "range": "± 9.09",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 129.68,
            "range": "± 11.98",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 216.97,
            "range": "± 17.77",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 228.56,
            "range": "± 16.18",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 23.98,
            "range": "± 1.19",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 41.31,
            "range": "± 2.87",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 45.89,
            "range": "± 3.69",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "committer": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "distinct": true,
          "id": "0f31052cc50f9acc26732d16772c69b1f58040f4",
          "message": "clariofy how to run examples",
          "timestamp": "2025-07-02T16:52:30+02:00",
          "tree_id": "354c6ae88e5e708255b4f5dcd13858f93876cc10",
          "url": "https://github.com/tigerros/ruci/commit/0f31052cc50f9acc26732d16772c69b1f58040f4"
        },
        "date": 1751468024577,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1126.11,
            "range": "± 16.26",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 2995.16,
            "range": "± 52.38",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28134.11,
            "range": "± 388.24",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.31,
            "range": "± 0.23",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 603.88,
            "range": "± 10.95",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2460.02,
            "range": "± 34.54",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 196.58,
            "range": "± 3.20",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 239.69,
            "range": "± 4.69",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 454.67,
            "range": "± 10.85",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 475.96,
            "range": "± 15.36",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 107.25,
            "range": "± 2.00",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 131.13,
            "range": "± 1.86",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 225.15,
            "range": "± 51.00",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 233.64,
            "range": "± 2.85",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 23.38,
            "range": "± 0.18",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 42.64,
            "range": "± 0.18",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 46.04,
            "range": "± 0.27",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "committer": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "distinct": true,
          "id": "76c1a586cae590807fd8d70b4953a15ef480989a",
          "message": "move features up in readme",
          "timestamp": "2025-07-02T16:54:11+02:00",
          "tree_id": "e58caea4a5264d366139f635498296ef86ae1420",
          "url": "https://github.com/tigerros/ruci/commit/76c1a586cae590807fd8d70b4953a15ef480989a"
        },
        "date": 1751468123602,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1011.1,
            "range": "± 22.27",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3006.45,
            "range": "± 32.60",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28080.03,
            "range": "± 463.63",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.32,
            "range": "± 0.29",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 611.16,
            "range": "± 10.92",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2414.08,
            "range": "± 25.99",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 199.56,
            "range": "± 5.21",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 244.35,
            "range": "± 4.17",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 463.68,
            "range": "± 7.06",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 477.37,
            "range": "± 10.91",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 110.88,
            "range": "± 1.56",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 134.99,
            "range": "± 0.93",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 236.93,
            "range": "± 4.71",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 248.79,
            "range": "± 7.31",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 23.43,
            "range": "± 0.32",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 44.51,
            "range": "± 0.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 46.66,
            "range": "± 0.25",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "committer": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "distinct": true,
          "id": "8e689f8fbf5f980c9557e9fded18df7d0be106d6",
          "message": "Revert \"clariofy how to run examples\"\n\nThis reverts commit 0f31052cc50f9acc26732d16772c69b1f58040f4.",
          "timestamp": "2025-07-02T16:54:41+02:00",
          "tree_id": "842b560b69ebb51b84495f0a25f63c9d1e36e6ad",
          "url": "https://github.com/tigerros/ruci/commit/8e689f8fbf5f980c9557e9fded18df7d0be106d6"
        },
        "date": 1751468151030,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1018.15,
            "range": "± 26.59",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3136.94,
            "range": "± 75.97",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28099.44,
            "range": "± 477.65",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.35,
            "range": "± 3.87",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 626.01,
            "range": "± 11.56",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2417.09,
            "range": "± 37.89",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 196.15,
            "range": "± 45.03",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 243.27,
            "range": "± 20.26",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 453.54,
            "range": "± 36.34",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 472.64,
            "range": "± 10.44",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 113.48,
            "range": "± 76.13",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 138.42,
            "range": "± 1.93",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 225.79,
            "range": "± 3.15",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 240.91,
            "range": "± 3.49",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 23.67,
            "range": "± 0.10",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 44.47,
            "range": "± 0.96",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 46.63,
            "range": "± 0.23",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "github.flashbulb541@passmail.com",
            "name": "tigerros",
            "username": "tigerros"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "465d8ca90ab41e0e5431d975a7192daf1bb70ba4",
          "message": "Merge pull request #11 from tigerros/dependabot/github_actions/google/osv-scanner-action-2.1.0\n\nBump google/osv-scanner-action from 2.0.3 to 2.1.0",
          "timestamp": "2025-07-22T00:39:22+02:00",
          "tree_id": "38ec83a788d17233a2d41e50555d6521295a3f6e",
          "url": "https://github.com/tigerros/ruci/commit/465d8ca90ab41e0e5431d975a7192daf1bb70ba4"
        },
        "date": 1753137622603,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 960.95,
            "range": "± 19.42",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3032.35,
            "range": "± 45.47",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27859.78,
            "range": "± 590.06",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.32,
            "range": "± 0.25",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 596.12,
            "range": "± 12.49",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2378.65,
            "range": "± 63.47",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 213.93,
            "range": "± 4.22",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 253.31,
            "range": "± 3.72",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 478.99,
            "range": "± 10.92",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 487.91,
            "range": "± 5.39",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 126.95,
            "range": "± 2.21",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 154.65,
            "range": "± 4.28",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 241.88,
            "range": "± 6.82",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 243.28,
            "range": "± 6.01",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 24.27,
            "range": "± 0.28",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 45.12,
            "range": "± 0.34",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 47.56,
            "range": "± 0.28",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "committer": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "distinct": true,
          "id": "8d74b3432722f2e56c5adb477b227874623a652b",
          "message": "updates",
          "timestamp": "2025-08-30T17:26:52+02:00",
          "tree_id": "4b45a18f3edb6b7a30bcac459be1595dc242ca9e",
          "url": "https://github.com/tigerros/ruci/commit/8d74b3432722f2e56c5adb477b227874623a652b"
        },
        "date": 1756567683454,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 965.74,
            "range": "± 14.99",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3314,
            "range": "± 22.70",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27812.92,
            "range": "± 364.88",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.94,
            "range": "± 0.33",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 652.7,
            "range": "± 17.30",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2388.9,
            "range": "± 31.06",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 204.25,
            "range": "± 1.65",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 253.09,
            "range": "± 3.42",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 465.68,
            "range": "± 5.15",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 504.85,
            "range": "± 12.21",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 107.46,
            "range": "± 88.24",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 132.84,
            "range": "± 2.63",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 244.15,
            "range": "± 2.04",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 246.03,
            "range": "± 3.90",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 23.66,
            "range": "± 0.16",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 46.05,
            "range": "± 0.42",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 47.28,
            "range": "± 0.33",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "github.flashbulb541@passmail.com",
            "name": "tigerros",
            "username": "tigerros"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2271c644eb7bfe570816e8629e8bf097736d78c8",
          "message": "Merge pull request #14 from tigerros/dependabot/github_actions/actions/checkout-5\n\nBump actions/checkout from 4 to 5",
          "timestamp": "2025-08-30T17:28:07+02:00",
          "tree_id": "028a073ae19e3df3cc9c361fc84cc5ff8e55a76e",
          "url": "https://github.com/tigerros/ruci/commit/2271c644eb7bfe570816e8629e8bf097736d78c8"
        },
        "date": 1756567756170,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 961.17,
            "range": "± 19.52",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3390.82,
            "range": "± 42.40",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28118.92,
            "range": "± 696.93",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.93,
            "range": "± 1.19",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 638.63,
            "range": "± 18.11",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2405.96,
            "range": "± 56.57",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 206.45,
            "range": "± 1.52",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 255.7,
            "range": "± 3.16",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 466.49,
            "range": "± 4.74",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 510.17,
            "range": "± 423.10",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 102.75,
            "range": "± 1.40",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 133.8,
            "range": "± 2.21",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 237.41,
            "range": "± 3.64",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 233.07,
            "range": "± 3.06",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 23.06,
            "range": "± 0.32",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 44.46,
            "range": "± 0.28",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 46.68,
            "range": "± 0.34",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "committer": {
            "email": "76173380+tigerros@users.noreply.github.com",
            "name": "Leonard",
            "username": "tigerros"
          },
          "distinct": true,
          "id": "f5d92f327cc94bd256eef1561c9200088fa724ae",
          "message": "Update osv-scanner.yml",
          "timestamp": "2025-08-30T17:48:30+02:00",
          "tree_id": "a22d6256599379862e386a01e6e3cb52c660e394",
          "url": "https://github.com/tigerros/ruci/commit/f5d92f327cc94bd256eef1561c9200088fa724ae"
        },
        "date": 1756568978423,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 981.1,
            "range": "± 636.38",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3331.76,
            "range": "± 57.47",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28056.73,
            "range": "± 13560.21",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.94,
            "range": "± 0.22",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 652.35,
            "range": "± 11.39",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2383.47,
            "range": "± 35.53",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 205.16,
            "range": "± 2.29",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 259.97,
            "range": "± 5.19",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 469.71,
            "range": "± 8.03",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 496.38,
            "range": "± 6.44",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 103.02,
            "range": "± 3.04",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 137.15,
            "range": "± 1.44",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 235.23,
            "range": "± 4.83",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 237.53,
            "range": "± 1.84",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 23.64,
            "range": "± 0.66",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 45.11,
            "range": "± 0.47",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 47.28,
            "range": "± 0.25",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "github.flashbulb541@passmail.com",
            "name": "tigerros",
            "username": "tigerros"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e6fa02daac73bf1748957b67a7b7aef247c04435",
          "message": "Merge pull request #15 from tigerros/dependabot/github_actions/google/osv-scanner-action-2.2.3\n\nBump google/osv-scanner-action from 2.2.2 to 2.2.3",
          "timestamp": "2025-10-13T12:10:12+02:00",
          "tree_id": "299f2e18a17ac6820126a5fefa81af8f7a6678e0",
          "url": "https://github.com/tigerros/ruci/commit/e6fa02daac73bf1748957b67a7b7aef247c04435"
        },
        "date": 1760350287189,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 997.1,
            "range": "± 9.9",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3752.92,
            "range": "± 49.72",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 28010.04,
            "range": "± 278.27",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.02,
            "range": "± 0.22",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 729.56,
            "range": "± 20.17",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2289.17,
            "range": "± 372.12",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 199.73,
            "range": "± 1.66",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 260.07,
            "range": "± 4.49",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 458.83,
            "range": "± 4.94",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 484.63,
            "range": "± 149.77",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 115.46,
            "range": "± 1.88",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 141.81,
            "range": "± 1.75",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 242.99,
            "range": "± 1.41",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 237.04,
            "range": "± 3.06",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 23.94,
            "range": "± 0.05",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 44.23,
            "range": "± 0.32",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.8,
            "range": "± 0.21",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "github.flashbulb541@passmail.com",
            "name": "tigerros",
            "username": "tigerros"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ae0eaa6ca39197e79b04211db5d640486acb3fb5",
          "message": "Merge pull request #16 from tigerros/dependabot/github_actions/actions/upload-artifact-5\n\nBump actions/upload-artifact from 4 to 5",
          "timestamp": "2025-10-30T17:11:50+01:00",
          "tree_id": "a8f0e8d54eb81f25ac8544bc2b5396f93986ef5b",
          "url": "https://github.com/tigerros/ruci/commit/ae0eaa6ca39197e79b04211db5d640486acb3fb5"
        },
        "date": 1761840776554,
        "tool": "cargo",
        "benches": [
          {
            "name": "from_str::info::ruci",
            "value": 1034.68,
            "range": "± 20.18",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::shakmaty_uci",
            "value": 3338.88,
            "range": "± 33.44",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::info::vampirc",
            "value": 27324.15,
            "range": "± 203.25",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::ruci",
            "value": 23.02,
            "range": "± 0.41",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::shakmaty_uci",
            "value": 672.7,
            "range": "± 16.99",
            "unit": "ns/iter"
          },
          {
            "name": "from_str::uci_ok::vampirc",
            "value": 2258.61,
            "range": "± 28.94",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_borrowed",
            "value": 198.3,
            "range": "± 2.9",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::ruci_owned",
            "value": 276.2,
            "range": "± 2.17",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::shakmaty_uci",
            "value": 468.54,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::go::vampirc",
            "value": 477.23,
            "range": "± 5.28",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_borrowed",
            "value": 109.65,
            "range": "± 1.63",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::ruci_owned",
            "value": 133.79,
            "range": "± 65.05",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::shakmaty_uci",
            "value": 229.81,
            "range": "± 45.64",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::register::vampirc",
            "value": 226.5,
            "range": "± 2.4",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::ruci",
            "value": 23.65,
            "range": "± 1.26",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::shakmaty_uci",
            "value": 42.97,
            "range": "± 0.38",
            "unit": "ns/iter"
          },
          {
            "name": "to_str::uci_ok::vampirc",
            "value": 44.49,
            "range": "± 0.8",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}