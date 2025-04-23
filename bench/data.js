window.BENCHMARK_DATA = {
  "lastUpdate": 1745439575852,
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
      }
    ]
  }
}