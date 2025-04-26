window.BENCHMARK_DATA = {
  "lastUpdate": 1745707415741,
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
      }
    ]
  }
}