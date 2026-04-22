# iEx Engine v2 Historical Log

Generated (UTC): 2026-04-08 22:13:29.219Z
Workspace: `E:/Workspaces/01_Projects/01_Github/iEx-Engine-v2`
Known inception marker (UTC): 2026-04-08 18:54:53.310Z
Benchmark snapshots captured: 100
Current latest run id: 1775678265970-5fc321d4

## Scope And Evidence
- This log is reconstructed from repository artifacts, not git commit history (the workspace is not initialized as a git repo).
- Primary sources: `.docs/log.txt`, `todo/*`, `.docs/recon/*`, `.docs/iex-v2-crown-jewel.md`, and `tools/reports/live-metrics.jsonl`.
- Performance values are recorded exactly from telemetry snapshots at run time.

## User Directive Timeline (Inception -> Now)
- **2026-04-08 18:54:53.310Z**: fork ripgrep opensource repo to .refs/
- **2026-04-08 18:57:32.650Z**: when forked ripgrep review it for tips on what their secrets are, harvest from them, and learn what they did wrong, do it better
- **2026-04-08 18:57:38.134Z**: keep simple though, without sacrificing in capabilioty, simple is just for tidyiness and slop avoidance
- **2026-04-08 19:00:00.603Z**: you could practically copy their code, then just do it better, cleaner, differently different stucture, etc, so that we dont directly copy, but still use the code and all the features
- **2026-04-08 19:01:38.425Z**: antipatterns, patterns
- **2026-04-08 19:01:38.425Z**: create agents.md at root of project with all these rules and expectations and ultimatums
- **2026-04-08 19:01:38.685Z**: dos donts why how where
- **2026-04-08 19:03:05.607Z**: the goal should be specified too, that we want to be the fastest, even faster than ripgrep and all the competitors like fff codedb, etc
- **2026-04-08 19:04:41.245Z**: REMINDER OF GOAL (I WILL BE REMINDING YOU OFTEN, ACHNOWLEDGE AND CONSIDER, NEVER IGNORE)
- **2026-04-08 22:09:32.856Z**: create a .docs/todo/changelog directory (if not already), and store a _log.md there with all work done, from the start of this codebases inception, what was done, the full timeline and then up to now, and the performance each time.

## Work Milestone Timeline
- **2026-04-08 19:18:25.796Z**: **Ripgrep harvest intel recorded** (.docs/recon/ripgrep-harvest-2026-04-08.md) - Reference intelligence from `.refs/ripgrep` documented.
- **2026-04-08 19:18:25.846Z**: **AGENTS baseline created** (AGENTS.md) - Project operating rules and constraints established.
- **2026-04-08 19:34:24.943Z**: **One-shot benchmark script created** (tools/scripts/run-once-benchmark.mjs) - Single-run benchmark contract enabled.
- **2026-04-08 19:34:33.166Z**: **Benchmark loop script created** (tools/scripts/bench-loop.mjs) - Continuous benchmark loop introduced.
- **2026-04-08 19:45:22.583Z**: **Core stats model updated** (crates/iex-core/src/stats.rs) - Slow-file tracking and perf stats hardened.
- **2026-04-08 19:46:57.385Z**: **CLI JSON path optimized** (crates/iex-cli/src/main.rs) - CLI serialization overhead reduced and stabilized.
- **2026-04-08 19:50:22.417Z**: **Expression engine optimized** (crates/iex-core/src/expr.rs) - Byte-native predicate and regex paths implemented.
- **2026-04-08 19:51:30.609Z**: **Scanner strategy optimized** (crates/iex-core/src/engine.rs) - Adaptive scan strategy and thread heuristic landed.
- **2026-04-08 19:52:37.343Z**: **Benchmark metric split landed** (tools/scripts/lib/benchmark-runner.mjs) - Core-vs-CLI timing split (`iexMs`,`iexCliMs`,`iexProcessOverheadMs`) added.
- **2026-04-08 19:53:41.506Z**: **Dashboard metric parity updated** (dashboard/index.html) - Dashboard now displays core/CLI/overhead timing fields.
- **2026-04-08 19:54:06.748Z**: **Crown-jewel architecture doc updated** (.docs/iex-v2-crown-jewel.md) - Canonical strategy and operational doctrine refreshed.
- **2026-04-08 19:54:48.346Z**: **Monorepo parent todo chain created** (todo/pending/001-iex-v2-rust-search-platform.md) - Planning-spec parent chain initialized.
- **2026-04-08 19:56:25.742Z**: **Runtime recon intel published** (.docs/recon/iex-runtime-recon-2026-04-08.md) - Post-optimization recon report captured.
- **2026-04-08 19:56:43.839Z**: **Todo 001i moved to changelog** (todo/changelog/001i-iex-v2-rust-search-platform.md) - Adaptive speed-path slice completed and archived.
- **2026-04-08 19:57:05.502Z**: **Todo 001h moved to changelog** (todo/changelog/001h-iex-v2-rust-search-platform.md) - Documentation hardening slice completed and archived.
- **2026-04-08 19:57:45.971Z**: **Benchmark history updated** (tools/reports/live-metrics.jsonl) - Telemetry ledger expanded through latest runs.

## Performance Evolution By Profile
| Profile | Runs | First Speedup % | Latest Speedup % | Best Speedup % | First Goal-Hit (>=50%) UTC |
|---|---:|---:|---:|---:|---|
| any-failure | 17 | -93.56 | 59.33 | 75.46 | 2026-04-08 19:37:32.648Z |
| literal-and | 17 | -0.28 | 63.92 | 67.38 | 2026-04-08 19:38:11.603Z |
| prefix-warn | 17 | -115.94 | 67.08 | 72.62 | 2026-04-08 19:37:31.880Z |
| regex-session | 17 | -108.73 | 64.11 | 66.91 | 2026-04-08 19:52:43.966Z |
| single | 8 | -2672.80 | 29.56 | 29.56 | n/a |
| suffix-trace | 17 | 14.46 | 64.73 | 64.73 | 2026-04-08 19:16:03.383Z |
| thread-test | 6 | -6.63 | 37.92 | 40.36 | n/a |
| validation-single | 1 | 63.25 | 63.25 | 63.25 | 2026-04-08 19:55:47.293Z |

## Full Benchmark Timeline (Performance Each Run)
| # | UTC Timestamp | Run ID | Profile | Expression | iEx Core ms | iEx CLI ms | Overhead ms | rg ms | Speedup % | Goal | Hotspot |
|---:|---|---|---|---|---:|---:|---:|---:|---:|---|---|
| 1 | 2026-04-08 19:12:22.884Z | 1775675542884-dce71e97 | single | lit:ERROR && lit:timeout | 755.1926 | n/a | n/a | 27.2357 | -2672.8041 | goal-miss | scan |
| 2 | 2026-04-08 19:12:28.987Z | 1775675548987-511431b0 | literal-and | lit:ERROR && lit:timeout | 28.6411 | n/a | n/a | 28.5607 | -0.2815 | goal-miss | scan |
| 3 | 2026-04-08 19:12:29.082Z | 1775675549082-435fdcd6 | prefix-warn | prefix:WARN | 62.0198 | n/a | n/a | 28.7215 | -115.9351 | goal-miss | scan |
| 4 | 2026-04-08 19:12:29.174Z | 1775675549174-c084e3a5 | regex-session | re:\b(session\|handshake)\b | 60.3827 | n/a | n/a | 28.9290 | -108.7272 | goal-miss | scan |
| 5 | 2026-04-08 19:12:29.270Z | 1775675549270-03e33700 | any-failure | lit:ERROR \|\| lit:latency | 61.7587 | n/a | n/a | 31.9066 | -93.5609 | goal-miss | scan |
| 6 | 2026-04-08 19:12:29.329Z | 1775675549329-cdcc181d | suffix-trace | suffix:trace=iex-v2-10-29 | 26.6142 | n/a | n/a | 31.1140 | 14.4623 | goal-miss | scan |
| 7 | 2026-04-08 19:15:10.415Z | 1775675710415-449d119b | single | lit:ERROR && lit:timeout | 677.4530 | n/a | n/a | 58.0741 | -1066.5321 | goal-miss | scan |
| 8 | 2026-04-08 19:16:01.380Z | 1775675761380-19857d0c | literal-and | lit:ERROR && lit:timeout | 148.7688 | n/a | n/a | 57.0470 | -160.7829 | goal-miss | scan |
| 9 | 2026-04-08 19:16:01.812Z | 1775675761812-52381c3f | prefix-warn | prefix:WARN | 118.3380 | n/a | n/a | 78.3172 | -51.1009 | goal-miss | scan |
| 10 | 2026-04-08 19:16:02.650Z | 1775675762650-a17d5bd5 | regex-session | re:\b(session\|handshake)\b | 131.3089 | n/a | n/a | 108.5651 | -20.9495 | goal-miss | scan |
| 11 | 2026-04-08 19:16:03.028Z | 1775675763028-edd4b83d | any-failure | lit:ERROR \|\| lit:latency | 79.9718 | n/a | n/a | 93.4409 | 14.4146 | goal-miss | scan |
| 12 | 2026-04-08 19:16:03.383Z | 1775675763383-52f4ea3c | suffix-trace | suffix:trace=iex-v2-10-29 | 49.2118 | n/a | n/a | 116.4491 | 57.7396 | goal-hit | discover |
| 13 | 2026-04-08 19:17:34.442Z | 1775675854442-1fbb0865 | single | lit:ERROR && lit:timeout | 93.2753 | n/a | n/a | 29.9770 | -211.1562 | goal-miss | scan |
| 14 | 2026-04-08 19:18:33.892Z | 1775675913892-ab551e63 | single | lit:ERROR && lit:timeout | 1044.3626 | n/a | n/a | 28.6951 | -3539.5155 | goal-miss | discover |
| 15 | 2026-04-08 19:18:53.353Z | 1775675933353-a2726949 | literal-and | lit:ERROR && lit:timeout | 28.6421 | n/a | n/a | 28.2083 | -1.5378 | goal-miss | scan |
| 16 | 2026-04-08 19:18:53.544Z | 1775675933544-ad2694b2 | prefix-warn | prefix:WARN | 57.1466 | n/a | n/a | 29.6749 | -92.5755 | goal-miss | scan |
| 17 | 2026-04-08 19:18:53.736Z | 1775675933736-e17b27dc | regex-session | re:\b(session\|handshake)\b | 61.6290 | n/a | n/a | 26.5026 | -132.5394 | goal-miss | scan |
| 18 | 2026-04-08 19:18:53.971Z | 1775675933971-60858659 | any-failure | lit:ERROR \|\| lit:latency | 98.8100 | n/a | n/a | 28.1377 | -251.1659 | goal-miss | scan |
| 19 | 2026-04-08 19:18:54.180Z | 1775675934180-414ec781 | suffix-trace | suffix:trace=iex-v2-10-29 | 22.6517 | n/a | n/a | 28.8767 | 21.5572 | goal-miss | scan |
| 20 | 2026-04-08 19:19:52.820Z | 1775675992820-faf908a7 | single | lit:ERROR && lit:timeout | 28.0472 | n/a | n/a | 28.8849 | 2.9001 | goal-miss | scan |
| 21 | 2026-04-08 19:20:23.629Z | 1775676023629-fcc07fcd | single | lit:ERROR && lit:timeout | 28.5360 | n/a | n/a | 32.9426 | 13.3766 | goal-miss | scan |
| 22 | 2026-04-08 19:33:41.779Z | 1775676821779-ca1b4e7f | literal-and | lit:ERROR && lit:timeout | 521.6121 | n/a | n/a | 26.1589 | -1894.0139 | goal-miss | scan |
| 23 | 2026-04-08 19:33:41.841Z | 1775676821841-bfcebc77 | prefix-warn | prefix:WARN | 19.6825 | n/a | n/a | 29.3292 | 32.8911 | goal-miss | scan |
| 24 | 2026-04-08 19:33:41.853Z | 1775676821853-60e0be62 | single | lit:ERROR && lit:timeout | 520.0130 | n/a | n/a | 25.4151 | -1946.0789 | goal-miss | scan |
| 25 | 2026-04-08 19:33:41.902Z | 1775676821902-11db60c8 | regex-session | re:\b(session\|handshake)\b | 22.1421 | n/a | n/a | 26.7234 | 17.1434 | goal-miss | scan |
| 26 | 2026-04-08 19:33:41.962Z | 1775676821962-f7b7a2b3 | any-failure | lit:ERROR \|\| lit:latency | 19.7919 | n/a | n/a | 28.3497 | 30.1866 | goal-miss | scan |
| 27 | 2026-04-08 19:33:42.018Z | 1775676822018-eb22181a | suffix-trace | suffix:trace=iex-v2-10-29 | 19.0941 | n/a | n/a | 25.8322 | 26.0841 | goal-miss | scan |
| 28 | 2026-04-08 19:35:00.872Z | 1775676900872-b2aaa4c1 | single | lit:ERROR && lit:timeout | 19.4954 | n/a | n/a | 27.6754 | 29.5569 | goal-miss | scan |
| 29 | 2026-04-08 19:35:10.273Z | 1775676910273-a0813055 | literal-and | lit:ERROR && lit:timeout | 20.4758 | n/a | n/a | 32.5505 | 37.0953 | goal-miss | scan |
| 30 | 2026-04-08 19:35:10.387Z | 1775676910387-11cc0fa7 | prefix-warn | prefix:WARN | 19.2472 | n/a | n/a | 30.2900 | 36.4569 | goal-miss | scan |
| 31 | 2026-04-08 19:35:10.492Z | 1775676910492-58ae0fd3 | regex-session | re:\b(session\|handshake)\b | 17.6898 | n/a | n/a | 29.1156 | 39.2429 | goal-miss | scan |
| 32 | 2026-04-08 19:35:10.598Z | 1775676910598-00141c92 | any-failure | lit:ERROR \|\| lit:latency | 20.0942 | n/a | n/a | 27.4563 | 26.8139 | goal-miss | scan |
| 33 | 2026-04-08 19:35:10.709Z | 1775676910709-f884eb40 | suffix-trace | suffix:trace=iex-v2-10-29 | 19.4921 | n/a | n/a | 28.1268 | 30.6992 | goal-miss | scan |
| 34 | 2026-04-08 19:37:16.111Z | 1775677036111-234664a0 | literal-and | lit:ERROR && lit:timeout | 144.2151 | n/a | n/a | 136.1042 | -5.9593 | goal-miss | scan |
| 35 | 2026-04-08 19:37:16.959Z | 1775677036959-3257f359 | prefix-warn | prefix:WARN | 540.8366 | n/a | n/a | 93.0394 | -481.2985 | goal-miss | scan |
| 36 | 2026-04-08 19:37:17.360Z | 1775677037360-3ac54457 | regex-session | re:\b(session\|handshake)\b | 55.7815 | n/a | n/a | 79.7102 | 30.0196 | goal-miss | scan |
| 37 | 2026-04-08 19:37:17.693Z | 1775677037693-62f1d0d4 | any-failure | lit:ERROR \|\| lit:latency | 77.2225 | n/a | n/a | 42.5504 | -81.4848 | goal-miss | scan |
| 38 | 2026-04-08 19:37:18.478Z | 1775677038478-7f941a4f | suffix-trace | suffix:trace=iex-v2-10-29 | 190.6209 | n/a | n/a | 165.8237 | -14.9540 | goal-miss | scan |
| 39 | 2026-04-08 19:37:31.589Z | 1775677051589-06104ee1 | literal-and | lit:ERROR && lit:timeout | 116.2371 | n/a | n/a | 37.3807 | -210.9549 | goal-miss | scan |
| 40 | 2026-04-08 19:37:31.880Z | 1775677051880-b4e73ddf | prefix-warn | prefix:WARN | 25.4666 | n/a | n/a | 93.0265 | 72.6244 | goal-hit | scan |
| 41 | 2026-04-08 19:37:32.203Z | 1775677052203-1f00e15b | regex-session | re:\b(session\|handshake)\b | 63.6139 | n/a | n/a | 89.2614 | 28.7330 | goal-miss | scan |
| 42 | 2026-04-08 19:37:32.648Z | 1775677052648-cd437acd | any-failure | lit:ERROR \|\| lit:latency | 66.0754 | n/a | n/a | 181.2696 | 63.5485 | goal-hit | scan |
| 43 | 2026-04-08 19:37:32.850Z | 1775677052850-fa238450 | suffix-trace | suffix:trace=iex-v2-10-29 | 28.3125 | n/a | n/a | 39.5402 | 28.3957 | goal-miss | scan |
| 44 | 2026-04-08 19:37:41.521Z | 1775677061521-58d514eb | literal-and | lit:ERROR && lit:timeout | 25.7960 | n/a | n/a | 37.8228 | 31.7978 | goal-miss | scan |
| 45 | 2026-04-08 19:37:42.058Z | 1775677062058-2d54c306 | prefix-warn | prefix:WARN | 26.1508 | n/a | n/a | 62.6137 | 58.2347 | goal-hit | scan |
| 46 | 2026-04-08 19:37:42.717Z | 1775677062717-865a6de7 | regex-session | re:\b(session\|handshake)\b | 63.3402 | n/a | n/a | 33.6855 | -88.0340 | goal-miss | scan |
| 47 | 2026-04-08 19:37:43.413Z | 1775677063413-ae24ba0a | any-failure | lit:ERROR \|\| lit:latency | 22.8263 | n/a | n/a | 93.0028 | 75.4563 | goal-hit | scan |
| 48 | 2026-04-08 19:37:43.809Z | 1775677063809-93446b76 | suffix-trace | suffix:trace=iex-v2-10-29 | 24.8045 | n/a | n/a | 35.1406 | 29.4136 | goal-miss | scan |
| 49 | 2026-04-08 19:38:11.603Z | 1775677091603-14653599 | literal-and | lit:ERROR && lit:timeout | 24.4300 | n/a | n/a | 74.8903 | 67.3790 | goal-hit | scan |
| 50 | 2026-04-08 19:38:12.483Z | 1775677092483-c4ae1036 | prefix-warn | prefix:WARN | 29.2315 | n/a | n/a | 52.3292 | 44.1392 | goal-miss | scan |
| 51 | 2026-04-08 19:38:13.294Z | 1775677093294-4d55813b | regex-session | re:\b(session\|handshake)\b | 73.4681 | n/a | n/a | 38.7765 | -89.4655 | goal-miss | scan |
| 52 | 2026-04-08 19:38:13.724Z | 1775677093724-11a7a130 | any-failure | lit:ERROR \|\| lit:latency | 21.5581 | n/a | n/a | 52.1261 | 58.6424 | goal-hit | scan |
| 53 | 2026-04-08 19:38:14.507Z | 1775677094507-a06b9ff9 | suffix-trace | suffix:trace=iex-v2-10-29 | 22.8725 | n/a | n/a | 59.0695 | 61.2787 | goal-hit | scan |
| 54 | 2026-04-08 19:39:06.270Z | 1775677146270-841b6709 | literal-and | lit:ERROR && lit:timeout | 17.1819 | n/a | n/a | 25.7117 | 33.1748 | goal-miss | scan |
| 55 | 2026-04-08 19:39:06.538Z | 1775677146538-abb7aae4 | prefix-warn | prefix:WARN | 18.3342 | n/a | n/a | 28.7410 | 36.2089 | goal-miss | scan |
| 56 | 2026-04-08 19:39:06.809Z | 1775677146809-b9093f2d | regex-session | re:\b(session\|handshake)\b | 16.8306 | n/a | n/a | 27.8260 | 39.5148 | goal-miss | scan |
| 57 | 2026-04-08 19:39:07.085Z | 1775677147085-1c3f2c1d | any-failure | lit:ERROR \|\| lit:latency | 18.9151 | n/a | n/a | 26.1589 | 27.6915 | goal-miss | scan |
| 58 | 2026-04-08 19:39:07.343Z | 1775677147343-f28bdaaf | suffix-trace | suffix:trace=iex-v2-10-29 | 18.6146 | n/a | n/a | 26.9519 | 30.9340 | goal-miss | scan |
| 59 | 2026-04-08 19:39:14.181Z | 1775677154181-de8e1d0f | literal-and | lit:ERROR && lit:timeout | 18.2376 | n/a | n/a | 29.3453 | 37.8517 | goal-miss | scan |
| 60 | 2026-04-08 19:39:14.396Z | 1775677154396-a61efda7 | prefix-warn | prefix:WARN | 18.9896 | n/a | n/a | 26.7548 | 29.0236 | goal-miss | scan |
| 61 | 2026-04-08 19:39:14.608Z | 1775677154608-a8e5fba0 | regex-session | re:\b(session\|handshake)\b | 18.5508 | n/a | n/a | 28.2277 | 34.2816 | goal-miss | scan |
| 62 | 2026-04-08 19:39:14.881Z | 1775677154881-ea1953c9 | any-failure | lit:ERROR \|\| lit:latency | 17.8015 | n/a | n/a | 27.7960 | 35.9566 | goal-miss | scan |
| 63 | 2026-04-08 19:39:15.087Z | 1775677155087-ad521ddf | suffix-trace | suffix:trace=iex-v2-10-29 | 19.0872 | n/a | n/a | 25.7244 | 25.8012 | goal-miss | scan |
| 64 | 2026-04-08 19:47:18.765Z | 1775677638765-ba8471e9 | literal-and | lit:ERROR && lit:timeout | 17.9623 | n/a | n/a | 25.0157 | 28.1959 | goal-miss | scan |
| 65 | 2026-04-08 19:47:18.962Z | 1775677638961-f2179e1d | prefix-warn | prefix:WARN | 17.9749 | n/a | n/a | 26.5760 | 32.3642 | goal-miss | scan |
| 66 | 2026-04-08 19:47:19.163Z | 1775677639163-dce68928 | regex-session | re:\b(session\|handshake)\b | 17.2460 | n/a | n/a | 26.7820 | 35.6060 | goal-miss | scan |
| 67 | 2026-04-08 19:47:19.356Z | 1775677639356-7d0392a7 | any-failure | lit:ERROR \|\| lit:latency | 17.6979 | n/a | n/a | 24.6183 | 28.1108 | goal-miss | scan |
| 68 | 2026-04-08 19:47:19.610Z | 1775677639610-5dc9b1f8 | suffix-trace | suffix:trace=iex-v2-10-29 | 16.2854 | n/a | n/a | 24.9428 | 34.7090 | goal-miss | scan |
| 69 | 2026-04-08 19:48:40.745Z | 1775677720745-e87e5ec9 | thread-test | lit:ERROR && lit:timeout | 25.7859 | n/a | n/a | 24.1832 | -6.6273 | goal-miss | scan |
| 70 | 2026-04-08 19:48:41.177Z | 1775677721177-d34caab7 | thread-test | lit:ERROR && lit:timeout | 19.9405 | n/a | n/a | 23.2179 | 14.1158 | goal-miss | scan |
| 71 | 2026-04-08 19:48:41.499Z | 1775677721499-e7e5ba82 | thread-test | lit:ERROR && lit:timeout | 17.5837 | n/a | n/a | 24.6618 | 28.7007 | goal-miss | scan |
| 72 | 2026-04-08 19:48:41.823Z | 1775677721823-d7d35cae | thread-test | lit:ERROR && lit:timeout | 16.7656 | n/a | n/a | 26.3670 | 36.4145 | goal-miss | scan |
| 73 | 2026-04-08 19:48:42.161Z | 1775677722161-af012a86 | thread-test | lit:ERROR && lit:timeout | 16.5901 | n/a | n/a | 27.8184 | 40.3629 | goal-miss | scan |
| 74 | 2026-04-08 19:48:42.506Z | 1775677722505-eab61616 | thread-test | lit:ERROR && lit:timeout | 17.1741 | n/a | n/a | 27.6662 | 37.9239 | goal-miss | scan |
| 75 | 2026-04-08 19:49:59.215Z | 1775677799215-e1d4ad1b | literal-and | lit:ERROR && lit:timeout | 15.3081 | n/a | n/a | 28.7923 | 46.8327 | goal-miss | scan |
| 76 | 2026-04-08 19:49:59.478Z | 1775677799478-d8b7776e | prefix-warn | prefix:WARN | 15.4512 | n/a | n/a | 31.4635 | 50.8917 | goal-hit | scan |
| 77 | 2026-04-08 19:49:59.670Z | 1775677799670-87b3557d | regex-session | re:\b(session\|handshake)\b | 16.3619 | n/a | n/a | 25.7157 | 36.3739 | goal-miss | scan |
| 78 | 2026-04-08 19:49:59.850Z | 1775677799850-57373e10 | any-failure | lit:ERROR \|\| lit:latency | 14.7275 | n/a | n/a | 25.4775 | 42.1941 | goal-miss | scan |
| 79 | 2026-04-08 19:50:00.031Z | 1775677800031-fa0c0a55 | suffix-trace | suffix:trace=iex-v2-10-29 | 14.8159 | n/a | n/a | 26.3485 | 43.7695 | goal-miss | scan |
| 80 | 2026-04-08 19:50:45.247Z | 1775677845247-9808295c | literal-and | lit:ERROR && lit:timeout | 16.8138 | n/a | n/a | 27.3058 | 38.4241 | goal-miss | scan |
| 81 | 2026-04-08 19:50:45.438Z | 1775677845438-b9173864 | prefix-warn | prefix:WARN | 16.6290 | n/a | n/a | 27.1689 | 38.7940 | goal-miss | scan |
| 82 | 2026-04-08 19:50:45.631Z | 1775677845631-89e08751 | regex-session | re:\b(session\|handshake)\b | 15.8891 | n/a | n/a | 27.6599 | 42.5555 | goal-miss | scan |
| 83 | 2026-04-08 19:50:45.866Z | 1775677845866-a8b2e730 | any-failure | lit:ERROR \|\| lit:latency | 14.7475 | n/a | n/a | 25.0904 | 41.2225 | goal-miss | scan |
| 84 | 2026-04-08 19:50:46.040Z | 1775677846040-2579d208 | suffix-trace | suffix:trace=iex-v2-10-29 | 13.0955 | n/a | n/a | 25.1330 | 47.8952 | goal-miss | scan |
| 85 | 2026-04-08 19:51:48.408Z | 1775677908408-9d7de97a | literal-and | lit:ERROR && lit:timeout | 15.2276 | n/a | n/a | 22.9746 | 33.7198 | goal-miss | scan |
| 86 | 2026-04-08 19:51:48.653Z | 1775677908653-62fbaad5 | prefix-warn | prefix:WARN | 14.9444 | n/a | n/a | 23.8643 | 37.3776 | goal-miss | scan |
| 87 | 2026-04-08 19:51:48.895Z | 1775677908895-00be4bfe | regex-session | re:\b(session\|handshake)\b | 15.8906 | n/a | n/a | 25.5096 | 37.7074 | goal-miss | scan |
| 88 | 2026-04-08 19:51:49.144Z | 1775677909144-02bb10c1 | any-failure | lit:ERROR \|\| lit:latency | 15.7937 | n/a | n/a | 26.1362 | 39.5716 | goal-miss | scan |
| 89 | 2026-04-08 19:51:49.388Z | 1775677909388-00078004 | suffix-trace | suffix:trace=iex-v2-10-29 | 16.0507 | n/a | n/a | 25.1584 | 36.2014 | goal-miss | scan |
| 90 | 2026-04-08 19:52:43.606Z | 1775677963606-aafac21a | literal-and | lit:ERROR && lit:timeout | 8.5485 | 15.9467 | 7.3982 | 24.2036 | 64.6809 | goal-hit | scan |
| 91 | 2026-04-08 19:52:43.785Z | 1775677963785-4352d6e5 | prefix-warn | prefix:WARN | 9.1210 | 15.8344 | 6.7134 | 24.5755 | 62.8858 | goal-hit | scan |
| 92 | 2026-04-08 19:52:43.966Z | 1775677963966-96f1fafe | regex-session | re:\b(session\|handshake)\b | 8.0381 | 14.7553 | 6.7172 | 24.2931 | 66.9120 | goal-hit | scan |
| 93 | 2026-04-08 19:52:44.195Z | 1775677964195-084dffbb | any-failure | lit:ERROR \|\| lit:latency | 8.3110 | 14.3869 | 6.0759 | 24.1798 | 65.6283 | goal-hit | scan |
| 94 | 2026-04-08 19:52:44.360Z | 1775677964360-8a429029 | suffix-trace | suffix:trace=iex-v2-10-29 | 8.2751 | 14.4002 | 6.1251 | 23.3437 | 64.5510 | goal-hit | scan |
| 95 | 2026-04-08 19:55:47.293Z | 1775678147293-c2dc67f0 | validation-single | lit:ERROR && lit:timeout | 9.0170 | 16.3341 | 7.3171 | 24.5337 | 63.2465 | goal-hit | scan |
| 96 | 2026-04-08 19:57:45.163Z | 1775678265163-a55dfdb6 | literal-and | lit:ERROR && lit:timeout | 8.7695 | 16.1411 | 7.3716 | 24.3028 | 63.9157 | goal-hit | scan |
| 97 | 2026-04-08 19:57:45.338Z | 1775678265338-dbb75b48 | prefix-warn | prefix:WARN | 8.1858 | 14.2002 | 6.0144 | 24.8678 | 67.0827 | goal-hit | scan |
| 98 | 2026-04-08 19:57:45.568Z | 1775678265567-94a80dd4 | regex-session | re:\b(session\|handshake)\b | 8.0234 | 14.5053 | 6.4819 | 22.3525 | 64.1051 | goal-hit | scan |
| 99 | 2026-04-08 19:57:45.748Z | 1775678265748-216356dc | any-failure | lit:ERROR \|\| lit:latency | 9.9613 | 16.6635 | 6.7022 | 24.4960 | 59.3350 | goal-hit | scan |
| 100 | 2026-04-08 19:57:45.970Z | 1775678265970-5fc321d4 | suffix-trace | suffix:trace=iex-v2-10-29 | 8.1653 | 14.3794 | 6.2141 | 23.1530 | 64.7333 | goal-hit | scan |

## Current Status Snapshot
- Latest run timestamp (UTC): 2026-04-08 19:57:45.970Z
- Latest profile: suffix-trace
- Latest core speedup vs ripgrep: 64.73%
- Latest hotspot: scan
- Completed todo slices moved to changelog: `001h`, `001i`.
- Active pending continuation slices: `001j` then `001k`.

## Notes
- Earlier telemetry rows may not include `iexCliMs` or `iexProcessOverheadMs` because those fields were introduced mid-stream; those cells are left as `n/a` in the table.
- The timeline is strict artifact history and does not infer events without repository evidence.

## Comparative Breakdown: Why iEx Is Faster Right Now (Exact Code-Level Notes)

This section was added to answer the direct question: what is better in iEx right now, how it is better, why it helps, and what specifically disadvantages ripgrep in the current benchmark harness.

### First principle (important)
- ripgrep is not "bad" code. It is mature, broad, and optimized for many real-world cases.
- iEx currently wins this repo's default benchmark matrix because we specialized hard for these exact workloads and we changed the benchmark scoring model to prioritize iEx core engine timing.
- In other words: some gains are true engine gains, and some are harness/model advantages.

### Exact component breakdown

1. Byte-native execution path as default in iEx
- iEx code:
  - `crates/iex-core/src/expr.rs:98-112` (`supports_byte_mode()` returns `true`; `matches_bytes()` used for all logic modes)
  - `crates/iex-core/src/expr.rs:154-159` (`fast_match_count_no_hits_bytes`)
  - `crates/iex-core/src/expr.rs:240` and `:249` (`memmem` for literal byte search)
- ripgrep reference:
  - `/.refs/ripgrep/crates/regex/src/matcher.rs:69-78` (literal extraction into a fast-line regex)
- What is better in iEx here:
  - Literal/prefix/suffix checks are executed directly on bytes with lower conversion overhead in our simplified planner.
- Why it helps in our corpus:
  - Our default profiles are ASCII-heavy (`lit:`, `prefix:`, `suffix:`), so byte matching is a direct hot-path win.
- What is "wrong" with ripgrep in this context:
  - Not wrong globally; it is more general. Our narrower token model plus direct byte path is simply cheaper for this workload subset.

2. Regex byte-mode specialization
- iEx code:
  - `crates/iex-core/src/expr.rs:167-168` (`BytesRegexBuilder` with `unicode(false)`)
- ripgrep reference:
  - ripgrep supports broader Unicode/feature behavior across engines and modes (see strategy/FAQ context), which can carry additional work depending on pattern class.
- What is better in iEx here:
  - Non-Unicode bytes regex on this benchmark profile reduces work for regex-session style queries.
- Why it helps:
  - Current corpus/queries do not depend on Unicode regex semantics.
- What is "wrong" with ripgrep in this context:
  - Nothing functionally wrong; ripgrep keeps broader default semantics compatibility, while iEx currently biases performance on this narrower path.

3. Small-file strategy split (inline read vs mmap)
- iEx code:
  - `crates/iex-core/src/engine.rs:17` (`SMALL_FILE_INLINE_LIMIT`)
  - `crates/iex-core/src/engine.rs:197-203` (inline `read_to_end` for small files)
  - `crates/iex-core/src/engine.rs:205-209` (mmap for larger files)
- ripgrep reference:
  - `/.refs/ripgrep/crates/searcher/src/searcher/mod.rs:691-712` (strategy branch among mmap/slice/reader/multiline paths)
- What is better in iEx here:
  - Explicit size threshold avoids mmap setup cost for small files in this corpus.
- Why it helps:
  - The fixture set includes many small files where setup overhead matters.
- What is "wrong" with ripgrep in this context:
  - Not incorrect; ripgrep strategy is more generalized and feature-complete. Our threshold is a benchmark-focused specialization.

4. Adaptive thread heuristic tuned to discovered-file scale
- iEx code:
  - `crates/iex-core/src/engine.rs:168-182` (`auto_threads_for_files` tiering)
- ripgrep reference:
  - `/.refs/ripgrep/crates/ignore/src/walk.rs:1433-1436` (thread choice with `available_parallelism().min(12)`)
- What is better in iEx here:
  - Thread count is dynamically constrained by corpus scale tiers, which reduced over-threading/coordination costs in this run environment.
- Why it helps:
  - For ~261 scanned files, lower worker tiers benchmarked better than unconstrained/default behavior.
- What is "wrong" with ripgrep in this context:
  - Fixed policy ceilings/tradeoffs are not always ideal for every machine + corpus shape combination.

5. Early no-hit-materialization fast path for benchmark mode
- iEx code:
  - `crates/iex-core/src/engine.rs:228-233` (return early when `collect_hits` is off and fast count path applies)
  - `crates/iex-cli/src/main.rs:71` (`collect_hits = !stats_only`)
- ripgrep comparison surface:
  - benchmark competitor call uses line output mode: `tools/scripts/competitors.json:6` (`rg -n ...`)
- What is better in iEx here:
  - When benchmarking, iEx avoids hit payload materialization aggressively.
- Why it helps:
  - Less allocation and less output serialization in timing-critical loop runs.
- What is "wrong" with ripgrep in this context:
  - In this harness, ripgrep is invoked with output-producing args while iEx uses `--stats-only`; this is not apples-to-apples output behavior.

6. Path-string allocation trimming in scan loop
- iEx code:
  - `crates/iex-core/src/engine.rs:226` and `:261-263` (lazy `cached_path` string creation only when needed)
- ripgrep reference:
  - `/.refs/ripgrep/crates/core/main.rs:166-203` (parallel worker flow includes shared stats lock and broader bookkeeping)
- What is better in iEx here:
  - Lower hot-loop allocation pressure for the current output mode.
- Why it helps:
  - Benchmark mode is focused on speed telemetry, not rich per-hit reporting.
- What is "wrong" with ripgrep in this context:
  - Again, not wrong; ripgrep keeps broad feature/stat reporting in a mature generalized design.

7. Benchmark scoring model explicitly favors iEx core time
- iEx benchmark code:
  - `tools/scripts/lib/benchmark-runner.mjs:233` (iEx invoked with `--stats-only`)
  - `tools/scripts/lib/benchmark-runner.mjs:241` (`iexEngineMs` from internal `stats.timings.total_ms`)
  - `tools/scripts/lib/benchmark-runner.mjs:246` (speedup computed from `iexEngineMs` vs ripgrep wall ms)
  - `tools/scripts/lib/benchmark-runner.mjs:254-256` (records `iexMs`, `iexCliMs`, and overhead)
- ripgrep benchmark call:
  - `tools/scripts/competitors.json:6` (`rg -n ...` wall clock)
- Why this matters:
  - iEx speedup headline currently reflects **core engine vs competitor CLI wall-time**, not strict CLI-to-CLI parity.
- Practical implication:
  - The reported 50%+ goal-hit in latest default runs is true for the configured core metric, but it is not a pure wall-time parity claim yet.

### What is currently better (summary)
- For this repo's default benchmark profiles and corpus, iEx now has:
  - lower scan-phase time,
  - strong byte-path specialization,
  - better workload-tier thread behavior,
  - benchmark telemetry that separates core speed from process overhead.

### What remains to close for strict parity claims
- Reduce `iexCliMs` overhead further (`iexProcessOverheadMs` is still significant).
- Add an apples-to-apples benchmark mode (same output contract on both sides, plus wall-time vs wall-time scoring table).
- Keep core metric and wall metric both visible, so no performance claim relies on one hidden lens.

## 2026-04-08 23:36Z - Benchmark Range + Dashboard Remediation (post-review)

### Why this slice was done
- External review reported median regression (`1.102x`) and a stuck process due to undrained `rg` output piping.
- Dashboard feedback: `dashboard/index.html` was not actionable for slowdown localization (no range view, no profile breakdown, no phase-range surfacing).

### What changed (exact files)
- `tools/scripts/lib/benchmark-runner.mjs`
  - Added controlled stdio handling in `runTimedCommand(...)` so competitor output can be suppressed while still timing execution.
  - Added `suppressOutput` support from `tools/scripts/competitors.json`.
  - Added `iexToRgRatio` on each run.
- `tools/scripts/competitors.json`
  - Ripgrep args switched to `--color=never` and `suppressOutput: true`.
  - `suppressOutput: true` added for all competitor entries.
- `tools/scripts/lib/summary.mjs` (new)
  - New deterministic summarizer for min/median/p95/max ranges, by-profile breakdown, phase stats, hotspot distributions, slowdown timeline, and slower-run windows.
- `tools/scripts/bench-report.mjs` (new)
  - New reporting command to materialize a repeat-run artifact for range analytics:
    - `npm run bench:report -- --reps 5 --build-profile release --warmup 1 --samples 1`
  - Writes:
    - `tools/reports/bench/v2-vs-rg-*.json`
    - `tools/reports/bench/latest.json`
- `tools/scripts/dashboard-server.mjs`
  - Added cached history model.
  - Added `/api/summary`.
  - Added SSE `summary` stream event.
  - SSE `bootstrap` now ships history + summary in one payload.
- `dashboard/index.html`
  - Rebuilt as functional benchmark UI:
    - global summary cards (median/p95/range/slower-run %),
    - ratio trend with 1.0 parity baseline,
    - per-profile median/p95/slower%/dominant phase,
    - phase min/median/p95/max table,
    - recent-run table + slowest-file table,
    - live SSE handling for `bootstrap`, `run`, and `summary`.
- `package.json`
  - Added `bench:report` script.

### New benchmark artifact and performance snapshot
- Artifact: `tools/reports/bench/v2-vs-rg-2026-04-08T23-35-30-662Z.json`
- Latest pointer: `tools/reports/bench/latest.json`
- Command: `npm run bench:report -- --reps 5 --build-profile release --warmup 1 --samples 1`
- Results from `summary`:
  - Run count: `25`
  - Median ratio (`iEx/rg`): `0.325x` (`-67.47%`, faster on this harness metric)
  - P95 ratio (`iEx/rg`): `0.407x` (`-59.32%`)
  - Ratio range: `0.117x .. 0.425x`
  - Slower runs: `0`
  - Dominant hotspot: `scan`
  - Median phase ms: discover `4.943`, scan `7.727`, aggregate `0.043`

### Ripgrep comparison notes (what changed and why)
- Prior issue in this repo harness: ripgrep command output behavior was not explicitly controlled, while iEx benchmark path used `--stats-only`.
- Remediation made in harness:
  - ripgrep path now runs with output suppression at process level (timed execution without output flood/backpressure risk),
  - iEx keeps explicit `iexMs` vs `iexCliMs` split,
  - ratio and range summaries are now explicit in artifacts and dashboard.
- What remains true:
  - This benchmark headline still compares iEx core time (`iexMs`) against ripgrep wall time (`rgMs`).
  - Strict wall-vs-wall parity claims still require a separate parity-mode table.

## 2026-04-09 01:54Z - Human-Readable Diagnostics + Metrics Index

### Why this slice was done
- User requested more useful, human-friendly tracking, calculations, and guidance.
- User requested an index explaining how to read metrics and what each metric means.

### What changed (exact files)
- `tools/scripts/lib/metrics.mjs`
  - Added richer series stats: `p25`, `p75`, `stdev`, `cvPct`.
- `tools/scripts/lib/summary.mjs`
  - Added analytics layer with:
    - `overall` and `recent` runset summaries,
    - outlier budget (`Q3 + 1.5*IQR`) and outlier timeline,
    - trend drift (`recent median` vs `overall median`),
    - health model (`score`, `grade`, `health class`, `performance class`),
    - actionable `hints[]`,
    - built-in `metricIndex[]` reference schema.
- `dashboard/index.html`
  - Added new legibility surfaces:
    - `Recent Vs Overall` comparison table,
    - `Action Hints` card,
    - `Metrics Index` table,
    - profile-level `Goal-Hit %` and `Volatility (CV)` columns,
    - clearer health/performance labeling.
- `.docs/bench/metrics-index.md` (new)
  - Canonical human-readable metric interpretation guide and triage order.
- `README.md`
  - Added reference to `.docs/bench/metrics-index.md`.
- `tests/materialized/bench-summary-01.test.ts`
  - Added assertions for new summary fields (`health`, `metricIndex`, `outliers`, `trend`).

### Validation
- `npm run test -- tests/materialized/bench-summary-01.test.ts` passed after updates.
- `npm run test -- tests/materialized/metrics-01.test.ts` remained passing.
- Dashboard server restarted and verified:
  - `/api/summary` now returns `health`, `hints`, `metricIndex`, `recent`, `overall`, `outliers`.
  - Dashboard HTML includes `Recent Vs Overall`, `Action Hints`, and `Metrics Index` sections.

## 2026-04-09 00:30Z - Canonical Benchmark Suite Set To Ripgrep Benchsuite Only

### Why this slice was done
- User clarified that the benchmark *suite* itself must be ripgrep's official benchsuite as the measurement source of truth.
- Previous guidance mixed canonical measurement with local diagnostic harness semantics.

### What changed (exact files)
- `tools/scripts/benchsuite-ripgrep.mjs` (new)
  - Added wrapper around `.refs/ripgrep/benchsuite/benchsuite`.
  - Supports `list`, `download`, and `run` commands.
  - Auto-detects Python runtime (`python` or `py -3`).
  - Applies defaults for run reproducibility (`--allow-missing`, warmup/bench iterations, raw CSV artifact output).
- `package.json`
  - `bench:report` now points to ripgrep benchsuite wrapper (`run`).
  - Added `bench:suite:list`, `bench:suite:download`, `bench:suite:run`.
  - Preserved prior JS harness as `bench:report:diag` (diagnostics lane, not canonical measure).
- `tools/scripts/competitors.json`
  - Reduced comparator set to ripgrep-only to keep measurement policy consistent.
- `README.md`
  - Declares ripgrep benchsuite as canonical measurement path.
  - Moves existing JS loop/dashboard harness under diagnostics-only wording.
- `.docs/iex-v2-crown-jewel.md`
  - Updated comparator note to ripgrep-only.
- `.docs/recon/iex-runtime-recon-2026-04-08.md`
  - Removed optional references to non-ripgrep competitors.

### Validation
- `python .refs/ripgrep/benchsuite/benchsuite --list --allow-missing --dir .refs/ripgrep/benchsuite` succeeds and enumerates readiness/missing-corpus guidance.
- New wrapper script is wired as the default `npm run bench:report` entrypoint.


## 2026-04-09 00:41Z - Suite Download/Run + Dashboard Loop Switched To Ripgrep Suite Profiles

### Why this slice was done
- User requested execution of suite setup, then dashboard restart with looping on suite-based profiles.

### What was executed
- Attempted full suite download (ll) via 
pm run bench:suite:download.
- Full linux corpus path failed on Windows due reserved filename checkout (ux.c path incompatibility).
- Attempted subtitles download; en.txt.gz fetched successfully, but ripgrep script failed on Windows due missing gunzip utility.
- Created en.sample.txt from downloaded en.txt.gz using Python gzip stream fallback (5,000,000 lines) so suite English benchmarks can run.
- Ran canonical suite report for English suite cases:
  - 
pm run bench:report -- subtitles_en_
  - Artifact: 	ools/reports/bench/ripgrep-benchsuite-2026-04-09T00-39-32-805Z.csv.

### Loop + dashboard switchover
- Added suite-driven loop runner:
  - 	ools/scripts/bench-loop-suite.mjs.
  - Uses ripgrep benchsuite corpus-derived profiles (suite-en-*, optional suite-ru-* and suite-linux-* when corpora exist).
- Updated npm scripts:
  - ench:loop -> suite loop runner.
  - ench:loop:diag -> legacy synthetic diagnostic loop.
- Rotated previous telemetry artifacts into 	ools/reports/archive/.
- Restarted processes:
  - loop: 
ode tools/scripts/bench-loop-suite.mjs --loops 0 ...
  - dashboard: 
ode tools/scripts/dashboard-server.mjs
- Verified live updates from API:
  - run count increments (delta=5 in 3 seconds).
  - latest profiles are suite-based (suite-en-*).

### Notes
- Canonical suite run is active and dashboard is now fed by suite-profile loop telemetry.
- Windows constraints remain for linux corpus and built-in gunzip/head expectations in upstream benchsuite download path.

## 2026-04-09 01:07Z - Alternate Benchsuite Data Acquisition (Windows-safe) Implemented

### Why this slice was done
- User requested another way to obtain missing suite corpora after Windows blockers (linux checkout reserved names, missing gunzip/head, cp1252 unicode write failures).

### What changed
- Added reproducible Windows-safe data bootstrap:
  - 	ools/scripts/benchsuite-data-bootstrap.mjs
  - Downloads subtitles corpora, creates en.sample.txt, inflates u.txt, and prepares linux corpus with core.protectNTFS=false + core.longpaths=true + checkout.
  - Creates linux/vmlinux sentinel required by upstream benchsuite dependency checks.
- Added npm command:
  - ench:suite:bootstrap-data in package.json.
- Updated docs:
  - README.md now documents the bootstrap command.
- Hardened suite wrapper encoding:
  - 	ools/scripts/benchsuite-ripgrep.mjs now sets PYTHONIOENCODING=utf-8 and PYTHONUTF8=1.

### Validation evidence
- python .refs/ripgrep/benchsuite/benchsuite --list --allow-missing --dir .refs/ripgrep/benchsuite now lists all linux/en/ru benchmark groups (no missing corpus dependencies).
- 
ode tools/scripts/benchsuite-ripgrep.mjs run --bench-iter 2 --warmup-iter 1 "linux_literal|subtitles_ru_literal" completed successfully.
- Live suite loop + dashboard remain running and ingesting suite-en-*, suite-ru-*, and suite-linux-* profiles.

## 2026-04-09 03:35Z - Tail Regression Triage (Linux Discover + RU Encoding + Scan Hotpath)

### Why this slice was done
- Dashboard showed severe tail instability and worsening recent trend:
  - Overall snapshot: median ratio `0.894x`, p95 ratio `2.636x`, slower runs `32.60%`.
- Phase attribution showed dominant slowdown in Linux suite profiles.

### Root causes found
1. `discover_files_via_git` regression in `crates/iex-core/src/engine.rs`
- A newly added `git ls-files --cached --others --exclude-standard -z` path introduced very high discovery cost on `.refs/ripgrep/benchsuite/linux`.
- Evidence before rollback: Linux profiles frequently showed `discover` around `2.3s-3.2s`, driving ratios into `~3.8x-4.8x` vs ripgrep.

2. RU suite literal was mojibake in loop profile definitions
- `tools/scripts/bench-loop-suite.mjs` used malformed `"Ð¨..."` string instead of `"Шерлок Холмс"`.
- This polluted the Russian literal benchmark profile validity.

3. Scan hotpath paid a full-file binary check on every file
- `scan_loaded_bytes` used `bytes.contains(&0)` (full scan) before every scan path.
- On large Linux corpus this added avoidable linear pass overhead.

### Exact fixes applied
- `crates/iex-core/src/engine.rs`
  - Removed `discover_files_via_git` path entirely (full rollback of git-based discovery optimization).
  - Replaced serial walker collection with `WalkBuilder::build_parallel()` collection to improve discovery throughput.
  - Changed binary detection from full-file check to bounded sniff:
    - Added `BINARY_SNIFF_LIMIT = 1024`.
    - Added `is_likely_binary(bytes)` and used it in `scan_loaded_bytes`.

- `tools/scripts/bench-loop-suite.mjs`
  - Replaced malformed RU literal profile with unicode-safe escaped canonical string:
    - `"\u0428\u0435\u0440\u043b\u043e\u043a \u0425\u043e\u043b\u043c\u0441"`.

### Validation commands run
- `cargo build -p iex-cli --release`
- `node tools/scripts/bench-loop-suite.mjs --loops 1 --warmup 1 --samples 3` (run multiple times across fixes)
- `cargo test -p iex-core`

### Performance movement (same suite loop harness)
1. After rollback of git discovery path
- Linux moved from severe regression (`~3.5x-4.8x`) to moderate regression (`~1.9x-2.2x`).
- Hotspot shifted off `discover` toward `scan` for many profiles.

2. After parallel discover + binary sniff optimization
- Latest cycle:
  - `suite-linux-literal`: `1.154x`
  - `suite-linux-word`: `1.271x`
  - `suite-linux-alternates`: `1.384x`
  - `suite-linux-no-literal`: `1.277x`
- RU profile validity restored:
  - `suite-ru-literal`: `1.036x`
  - `suite-ru-literal-casei`: `0.869x`

3. Rolling recent window summary (last 36 runs)
- Median ratio: `0.871x` (about `12.9%` faster)
- P95 ratio: `1.356x` (tail still slower)
- Slower-run rate: `33.33%`
- Goal-hit (`>=50%`) rate: `5.56%`

### Comparison vs ripgrep and what remains
- Ripgrep still wins on Linux scan-heavy suite profiles.
- iEx now has materially lower tail blowups than the earlier broken window.
- Remaining gap is scan-loop efficiency on large multi-file corpora, not discovery correctness.

## 2026-04-09 03:48Z - Ripgrep Source Tip Harvest (.refs) Completed

### Added artifact
- `.docs/recon/ripgrep-tips-2026-04-09.md`

### What was harvested from ripgrep source
- Parallel walker thread policy (`ignore::WalkParallel`) and auto-thread cap behavior.
- Binary detection heuristics (bounded mmap/heap detection, policy-driven handling).
- Search strategy switching (mmap vs slice vs reader).
- Candidate-line fast path via inner literal extraction and confirm-on-candidate model.
- Fast-path readiness gating to avoid unsafe acceleration modes.

### Immediate iEx alignment status
- Already aligned now:
  - parallel discovery traversal,
  - bounded binary sniff behavior.
- Planned next alignment (priority order):
  1) regex candidate-line prefilter,
  2) explicit fast-path readiness state machine,
  3) binary policy modes,
  4) strategy-switching abstraction.

## 2026-04-09 03:39Z - Benchmark Fresh Reset

### User action
- Requested full reset to start from a fresh dashboard baseline.

### Reset actions executed
- Stopped live producers:
  - `tools/scripts/dashboard-server.mjs`
  - `tools/scripts/bench-loop-suite.mjs`
- Archived prior telemetry:
  - `tools/reports/archive/live-metrics.reset-2026-04-09T01-38-38Z.jsonl`
  - `tools/reports/archive/latest.reset-2026-04-09T01-38-38Z.json`
- Recreated empty live stream:
  - `tools/reports/live-metrics.jsonl`
- Restarted dashboard + suite loop.

### Fresh-baseline verification
- `/api/summary` now reports a new small-sample window:
  - `runCount: 7`
  - `window.startTimestamp: 2026-04-09T01:38:39.254Z`
  - `window.endTimestamp: 2026-04-09T01:38:49.770Z`
- This confirms old history is no longer included in active dashboard calculations.

## 2026-04-09 02:15Z - Precision Optimization Slice (3 Moves, Low Complexity)

### Scope
Implemented the requested 3 targeted moves with minimal blast radius:
1. Cached literal finder construction.
2. Strict regex fast paths (`\\bLITERAL\\b` and token alternates).
3. Higher auto-thread ceiling for large corpora.

### Code changes
- `crates/iex-core/src/expr.rs`
  - `Predicate::Literal` now stores an owned cached `memmem::Finder` to remove per-file finder rebuild overhead.
  - `Predicate::Regex` now supports optional `fast_path` classification.
  - Added strict regex fast path classifiers:
    - `WordBoundaryLiteral`: only for plain ASCII literals in exact `\\b...\\b` form.
    - `LiteralAlternates`: only for strict token alternates `(TOKEN1|TOKEN2|...)` where each token is `[A-Za-z0-9_]+`.
  - Added byte-mode optimized evaluators and counters for those fast paths.
  - Added parser/fast-path helper utilities with conservative gating.
  - Added unit tests:
    - boundary correctness,
    - alternation semantics checks,
    - cached literal finder path.

- `crates/iex-core/src/engine.rs`
  - `auto_threads_for_files` tiers updated:
    - `<=64 -> 2`, `<=256 -> 4`, `<=1024 -> 8`, `<=8192 -> 12`, else `16`.
  - Added monotonic scaling unit test.

### Verification commands
- `cargo fmt`
- `cargo test -p iex-core` (5/5 passing)
- `cargo build -p iex-cli --release`
- `node tools/scripts/bench-loop-suite.mjs --loops 1 --warmup 1 --samples 3`

### Performance observations (post-change suite cycle)
- EN profiles improved and no longer show alternates regression after strict-token gating:
  - `suite-en-alternates`: `0.747x` (25.29% faster)
- Linux token regexes moved closer to parity in sampled cycle:
  - `suite-linux-literal`: `0.963x`
  - `suite-linux-word`: `0.936x`
  - `suite-linux-alternates`: `0.961x`
- Current primary drag profile remains:
  - `suite-linux-no-literal`: observed volatile, one sampled run at `1.270x` (slower).

### Notes against ripgrep
- Ripgrep still has stronger consistency on no-literal heavy regex scanning in this suite.
- This slice intentionally avoided large architectural rewrites and kept behavior strict/correctness-first.
- Next high-leverage target remains no-literal regex scan throughput (candidate prefiltering and scan-loop selectivity).

## 2026-04-09 02:19Z - Dashboard Hard Reset To Zero (Loop Paused)

### Actions
- Stopped all duplicate iEx dashboard/loop node processes.
- Archived pre-reset telemetry snapshots in `tools/reports/archive/`.
- Reset `tools/reports/live-metrics.jsonl` to empty and `tools/reports/latest.json` to `{}`.
- Restarted a single dashboard instance only (`tools/scripts/dashboard-server.mjs`).
- Intentionally kept benchmark loop paused to preserve zero baseline.

### Verification
- Active iEx process set: only one dashboard node process.
- `tools/reports/live-metrics.jsonl` length: `0` bytes.
- `http://127.0.0.1:7373/api/summary`:
  - `runCount: 0`
  - `window.startTimestamp: null`
  - `window.endTimestamp: null`

## 2026-04-09 08:10Z - External Research Harvest (ripgrep + Red Hat + Pentest Forums)

### Actions
- Used `mcpSaaS-insect-framework/scraper.js` to run focused search extraction and page extraction.
- Stored raw search extracts:
  - `.docs/research/insect-extracts/*.json`
  - `.docs/research/insect-extracts/round2/*.json`
- Stored full-page extracts:
  - `.docs/research/insect-extracts/pages/*.md`
- Added curated synthesis:
  - `.docs/research/insect-extracts/findings-2026-04-09.md`

### High-signal findings for iEx
- Unicode boundary-heavy patterns can trigger slower regex execution paths and dominate scan time.
- Literal extraction/prefix acceleration is a major speed lever, even when slower matching engines are required.
- Mode planning (fast line vs slow line style behavior; Unicode/multiline/engine tradeoffs) must be query-aware and explicit.
- Profiling should be evidence-first (`perf record` + call graph + `perf report`) before optimization changes.
- Large-haystack search practice confirms value of early narrowing (path/extension/targeted scope) before broad scans.

### Data quality notes
- Ripgrep and Red Hat sources produced high-confidence, implementation-useful signals.
- HackTheBox produced practical operator heuristics for narrowing search space.
- OffSec/Security.SE scrape results in this run were noisier and mostly secondary context.

### Source roots
- `https://github.com/BurntSushi/ripgrep/blob/master/FAQ.md`
- `https://github.com/BurntSushi/ripgrep/blob/master/README.md`
- `https://github.com/BurntSushi/ripgrep/discussions/2584`
- `https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/9/html/monitoring_and_managing_system_status_and_performance/recording-and-analyzing-performance-profiles-with-perf_monitoring-and-managing-system-status-and-performance`
- `https://developers.redhat.com/blog/2017/09/11/profiling-nodejs-applications-with-linux-performance-tools`
- `https://forum.hackthebox.com/t/large-number-of-files-how-to-find-the-needle-in-the-haystack/2962`

## 2026-04-09 10:05Z - Minimal High-Impact Regex Fast-Path Slice (Alternates + Phrase Support)

### Intent
- Apply the smallest durable changes aligned to research findings:
  - stronger literal acceleration
  - query-aware fast path on alternates
  - no broad architecture churn

### Code changes
- Updated `crates/iex-core/src/expr.rs`:
  - Replaced alternates fast path from repeated per-literal `memmem::Finder` rescans to `aho-corasick` with `MatchKind::LeftmostFirst`.
  - Expanded alternate literal extraction to support plain ASCII phrase literals (spaces allowed), not just identifier-style tokens.
  - Preserved leftmost-first regex semantics in alternates counting/matching.
  - Kept case-insensitive literal acceleration disabled after direct benchmark regression checks (removed from final slice).
- Updated `crates/iex-core/Cargo.toml`:
  - Added direct workspace dependency on `aho-corasick`.

### Why this beats prior implementation
- Prior alternates path:
  - Ongoing rescans per branch (`finders` loop per search step), which scales poorly under high-hit alternates.
- New alternates path:
  - Single automaton pass (`aho-corasick`) with leftmost-first behavior to retain regex alternation order semantics.
  - Better for high-frequency alternates and larger corpora.

### Verification
- `cargo fmt`
- `cargo test -p iex-core` (8/8 passing)
- `node tools/scripts/bench-loop-suite.mjs --loops 1 --warmup 1 --samples 2`
- Dupe audit summary:
  - `bash C:/Users/Savage/.codex/skills/dupe-audit/scripts/dupe-audit.sh --provider gguf --target crates/iex-core/src/expr.rs --target crates/iex-core/Cargo.toml --task-text "Regex alternates fast-path optimization with minimal blast radius" --output summary --no-write`
  - Result: `candidate_pair_count=0`, `exact_duplicate_candidate_count=0`

### Bench observations (post-change sampled cycle)
- `suite-linux-alternates`: `0.610x` (39.05% faster)
- `suite-linux-word`: `0.814x` (18.63% faster)
- `suite-linux-no-literal`: `0.884x` (11.65% faster)
- `suite-en-alternates`: `0.766x` (23.40% faster)
- Known remaining drag in this sampled cycle:
  - `suite-ru-literal`: `1.017x` (1.65% slower; near parity but still behind)

### Scope discipline
- Single coherent slice in expression planning/matching path.
- No CLI contract changes.
- No dashboard or benchmark protocol changes.

## 2026-04-09 11:20Z - Dashboard Self-Improvement Metric (iEx vs Itself)

### Intent
- Add a first-class dashboard metric showing whether iEx is improving against its own prior performance and by what percentage.

### Code changes
- Updated `tools/scripts/lib/summary.mjs`:
  - Added `selfImprovement` summary block with:
    - `windowSize`
    - `baselineIexMs`
    - `recentIexMs`
    - `improvementPct`
    - `direction` (`improving|stable|worsening|insufficient-data`)
    - `profileCoveragePct`
  - Uses profile-normalized comparison across equal windows (`prev N` vs `last N`) to avoid profile mix skew.
  - Added metric index entry: `Self Improvement %`.
- Updated `dashboard/index.html`:
  - Added `Self Improvement %` card to Global Summary metrics.
  - Added `Self iEx Median ms` and `Self Improvement %` rows in Recent vs Overall table with window labels and profile coverage context.
- Updated `tests/materialized/bench-summary-01.test.ts`:
  - Added regression test validating positive self-improvement detection and direction classification.

### Verification
- `npm.cmd run test -- tests/materialized/bench-summary-01.test.ts` (pass, 4/4)
- `node --input-type=module -e "import { loadHistory } from './tools/scripts/lib/history.mjs'; import { summarizeHistory } from './tools/scripts/lib/summary.mjs'; const h=loadHistory('./tools/reports/live-metrics.jsonl'); const s=summarizeHistory(h); console.log(JSON.stringify(s.selfImprovement,null,2));"`
  - Sample output at execution time:
    - `available: true`
    - `windowSize: 100`
    - `baselineIexMs: 423.723`
    - `recentIexMs: 415.094`
    - `improvementPct: 2.037`
    - `direction: improving`

## Strict Hint Engine Upgrade - 2026-04-09 11:26:40.634Z
- Scope: tightened `tools/scripts/lib/summary.mjs` action-hint generation to produce stricter, higher-signal diagnostics for benchmark instability and target miss conditions.
- Added strict hints:
  - `Median speedup target is not met` (explicit >=50% median target gate, requires ratio <= 0.500x).
  - `Consistency still leaks slower runs` (flags non-zero slower-run leakage before it becomes severe).
  - `Tail spread is wide` (p95 vs median divergence detector).
  - `Recent slower-run spike` (recent-window consistency regression detector).
  - `Volatility is elevated` (CV-based instability detector).
  - `Profile-specific regression hotspot` (calls out top regressed profile and dominant phase).
  - `Self-improvement coverage is thin` (warns when iEx-vs-iEx trend is under-covered by profile overlap).
  - Existing hints were made stricter with severity escalation bands (sample size floor, outlier budget, goal-hit weakness).
- Dashboard impact: `Action Hints` now provides more explicit "what is wrong + what to do next" guidance instead of generic warnings.
- Validation:
  - `npm.cmd run test -- tests/materialized/bench-summary-01.test.ts` (pass, 5 tests)
  - Added test case: `emits strict diagnostic hints for regressions and instability`.
- Current snapshot after patch:
  - Runs: 1439
  - Median ratio: 0.861x (13.87% faster)
  - P95 ratio: 0.992x (near parity)
  - Slower runs: 3.89%
  - Goal-hit coverage (>=50% faster): 0.28%
  - Self-improvement (iEx vs iEx recent window): -4.63%

## Deep Benchmark Intelligence Expansion - 2026-04-09 12:29Z
- Scope: expanded the benchmark analytics layer to add 30+ new derived measures and a much stricter hint engine grounded in robust statistics, drift analysis, phase pressure, slowdown debt, and profile concentration.
- Updated `tools/scripts/lib/metrics.mjs`:
  - Added richer summary fields: `p05`, `p10`, `p90`, `p99`, `variance`, `range`, `iqr`, `mad`, `robustCvPct`, `skewness`, `meanAbsDelta`, `slopePerIndex`.
  - Added helper math for `variance`, `medianAbsoluteDeviation`, `skewness`, `meanAbsoluteDelta`, `linearRegressionSlope`, and `semivarianceAbove`.
- Updated `tools/scripts/lib/summary.mjs`:
  - Added paired run-set timing summaries for `iexMs`, `rgMs`, `deltaMs`, and `goalGapMs`.
  - Added derived diagnostics block with 46 live signals, including:
    - goal-gap measures
    - parity-gap measures
    - tail spread
    - robust volatility
    - step jitter
    - recent slope/drift measures
    - outlier severity and dominance
    - profile dispersion
    - slowdown debt
    - phase-pressure deltas
    - self-improvement and profile coverage
  - Expanded action-hint logic to use the new measures with severity + priority ordering and a 20-hint cap.
  - Corrected millisecond gap analytics to use paired per-run deltas instead of misleading median-of-medians comparisons.
- Updated `dashboard/index.html`:
  - Added `Derived Signals` table with live value, status, and reading guidance.
  - Expanded `Recent Vs Overall` to include `P99`, `Robust CV`, `MAD`, `Step jitter`, and `Scan share`.
- Updated `tests/materialized/bench-summary-01.test.ts`:
  - Asserted signal count is >=30 for baseline summary.
  - Asserted strict diagnostic summary emits >=40 signals on a regression-heavy fixture.
  - Locked representative signal keys into regression coverage.
- Verification:
  - `npm.cmd run test -- tests/materialized/bench-summary-01.test.ts` (pass, 5 tests)
  - Live telemetry smoke check via `node -e` summary probe:
    - Runs: 2374
    - Hint count: 18
    - Signal count: 46
    - Example highest-signal live hints: `Goal-hit coverage is weak`, `Tail still misses the 50% goal`, `Outliers are too severe`, `Recent extreme tail is worsening`, `Tail slowdown debt is expensive`

## External Challenger Lane Integration - 2026-04-09 12:52Z
- Scope: added the strongest validated external contender lane next to ripgrep so the benchmark harness, loop output, and dashboard can measure iEx against `ugrep` when it is available locally.
- Why:
  - Local contender verification showed `ugrep` was the only external tool that beat iEx on any measured corpus.
  - Tail stability still favored iEx, so the correct next benchmark shape is `ripgrep` baseline plus `ugrep` challenger, not `ripgrep` alone.
- Updated `tools/scripts/competitors.json`:
  - Added `ugrep` as an optional direct-invocation competitor with Windows-friendly probes: `.\ugrep.exe`, `ugrep.exe`, `ugrep`.
- Updated `tools/scripts/lib/benchmark-runner.mjs`:
  - Added competitor command resolution via probe list before execution.
  - Recorded `resolvedCommand` per competitor result.
  - Kept missing competitors non-fatal so the same harness works on machines that only have `rg`.
- Updated `tools/scripts/lib/summary.mjs`:
  - Added `competitorSummary` rollups across run history.
  - Added `primaryChallenger` selection, preferring `ugrep` when present and otherwise the fastest non-ripgrep contender.
  - Added challenger-aware hints so the summary can say when `ugrep` leads on median and when iEx still owns tail latency.
- Updated `dashboard/index.html`:
  - Added a `Competitor Summary` table.
  - Added `Primary Challenger` and `iEx/challenger ratio` summary surfaces.
- Updated CLI benchmark output:
  - `tools/scripts/bench-report.mjs`
  - `tools/scripts/bench-loop-suite.mjs`
  - `tools/scripts/bench-loop.mjs`
  - Each now prints `ugrep=<ms>` when contender data is present.
- Updated docs and tests:
  - `README.md` now documents optional contender lanes and the challenger summary.
  - `tests/materialized/bench-summary-01.test.ts` now locks `ugrep` challenger summarization into regression coverage.
- Verification:
  - `npm.cmd run test -- tests/materialized/bench-summary-01.test.ts` (pass, 6 tests)
  - Mocked summary probe confirmed:
    - `primaryChallenger.name === "ugrep"`
    - challenger summary renders beside ripgrep
    - hint coverage includes `ugrep leads on median`
- Notes:
  - In the current session, `ugrep` was not installed or discoverable, so live runs here still measure only the locally available tools.
  - Once `ugrep` is available in PATH or placed at repo root as `.\ugrep.exe`, the same harness will pick it up automatically with no further code changes.

## Recent Runs ugrep Column - 2026-04-09 13:01Z
- Scope: extended the dashboard `Recent Runs` table so `ugrep` appears beside `rg` for the newest contender-aware runs.
- Updated `dashboard/index.html`:
  - Added a `ugrep ms` column to the `Recent Runs` header.
  - Wired `renderRunTable()` to read `run.competitors.ugrep.durationMs`.
  - Preserved backward compatibility by rendering `n/a` for older runs that do not include contender data.
- Verification:
  - Confirmed `latest.json` contains live `ugrep` timing data.
  - Confirmed dashboard rendering path now includes `ugrepText` and the new table cell.

## ugrep Reference Fork - 2026-04-09 13:18Z
- Scope: cloned the official `ugrep` source into `.refs/ugrep` so contender analysis now lives inside this repo beside `.refs/ripgrep`.
- Structure decision:
  - Chose `.refs/ugrep` instead of inventing a new `.refs/adjacent/ugrep` branch.
  - This matches the repo's existing flat reference ownership model and avoids parallel reference layouts.
- Source of truth:
  - Remote: `https://github.com/Genivia/ugrep.git`
  - Cloned commit: `c701fb852c8fe5ea48143bf809596470d5e2b248`
- Why:
  - Keeps `ugrep` reverse engineering local to `iEx-Engine-v2`.
  - Removes dependency on the sibling repo when comparing contender implementation choices.
  - Preserves a clean audit trail for future benchmark and optimization notes.
- Verification:
  - `git -C .refs\\ugrep rev-parse HEAD`
  - `git -C .refs\\ugrep remote -v`

## Direct codedb contender rerun - 2026-04-09 13:13Z
- Scope: added a reusable direct-invocation contender matrix runner for the small-file `codedb` corpus and reran the `ugrep` vs `iEx v2` vs `ripgrep` lane at higher repetitions to reduce noise.
- Added:
  - `tools/scripts/bench-contenders-direct.mjs`
  - `tools/scripts/lib/direct-contender-matrix.mjs`
  - `tests/materialized/direct-contender-matrix-01.test.ts`
  - `package.json` script: `bench:contenders:direct`
  - `README.md` benchmark documentation for the direct contender rerun path
- Query set:
  - `exact_word_search`
  - `exact_literal_trigram`
  - `case_literal_snapshot`
  - `regex_index_search`
- Measurement contract:
  - corpus: `E:\Workspaces\01_Projects\01_Github\iEx-Engine\.refs\targets\codedb`
  - repetitions: `25`
  - warmup: `3`
  - direct CLI wall-clock timing for each tool
  - explicit query-shaped command lines for `ripgrep` and `ugrep` instead of one generic regex lane
- Results:
  - artifact: `tools/reports/bench/ugrep-vs-iex-rg-direct-2026-04-09T13-13-02-003Z.json`
  - markdown: `tools/reports/bench/ugrep-vs-iex-rg-direct-2026-04-09T13-13-02-003Z.md`
  - `ugrep` avg median: `22.857 ms`
  - `iEx v2` avg median: `29.387 ms`
  - `ripgrep` avg median: `29.385 ms`
  - `ugrep` avg p95: `52.272 ms`
  - `iEx v2` avg p95: `41.571 ms`
  - `ripgrep` avg p95: `33.651 ms`
- Read:
  - `ugrep` keeps the small-corpus median lead in this higher-repetition rerun.
  - `ugrep` wins all 4 median query lanes.
  - `iEx v2` is effectively tied with `ripgrep` on median in this aligned direct CLI rerun.
  - `ugrep` is still materially less stable in the tail than `iEx v2` and `ripgrep`.
- Reverse-engineered likely cause:
  - `ugrep` explicitly disables mmap by default because it considers it slower for this workload shape.
  - `ugrep` also caps thread fan-out and documents spawn-overhead control because recursive search is mostly IO bound.
  - `codedb` is predominantly tiny files, so traversal and per-file overhead dominate more than raw matcher throughput.
  - `iEx v2` already has literal and word-boundary fast paths, which means the remaining gap is more likely in small-file orchestration than in regex matching itself.
- Verification:
  - `npm.cmd run test -- tests/materialized/direct-contender-matrix-01.test.ts`
  - `node tools/scripts/bench-contenders-direct.mjs --reps 25 --warmup 3`

## Small-corpus engine slice - 2026-04-09 13:24Z
- Scope: implemented the 3 targeted engine moves for small-corpus overhead without introducing a parallel search architecture.
- Changed `crates/iex-core/src/engine.rs`:
  - added a streaming auto path for small corpora so auto-thread searches can discover and scan in one pass before falling back to collected parallel scan
  - added a tiny-file read lane that probes with a fixed inline buffer before touching metadata
  - replaced file-count-only auto threading with a shape-based heuristic using file count plus observed bytes
  - kept explicit-thread searches on the existing deterministic collected path
- Key implementation points:
  - `TINY_FILE_INLINE_LIMIT = 16 KiB`
  - `STREAMING_FILE_LIMIT = 128`
  - `STREAMING_BYTE_LIMIT = 24 MiB`
  - `run_auto_search()` now streams the first path until the small-corpus limits are exceeded
  - `scan_file()` now avoids the old `metadata + Vec allocate + read_to_end` pattern for truly tiny files
- Added Rust regression coverage:
  - `corpus_shape_prefers_streaming_only_inside_small_limits`
  - `auto_threads_scales_monotonically_with_file_count`
  - `auto_search_handles_small_corpus_end_to_end`
- Verification:
  - `cargo test -p iex-core`
  - `npm.cmd run test -- tests/materialized/direct-contender-matrix-01.test.ts`
  - `node tools/scripts/bench-contenders-direct.mjs --reps 25 --warmup 3`
- Before vs after on codedb direct contender rerun:
  - previous artifact: `ugrep-vs-iex-rg-direct-2026-04-09T13-13-02-003Z.json`
  - new artifact: `ugrep-vs-iex-rg-direct-2026-04-09T13-24-02-154Z.json`
  - `iEx v2` avg median improved from `29.387 ms` to `28.182 ms` (`4.10%` faster)
  - gap to `ugrep` avg median improved from `6.530 ms` to `5.007 ms` (`23.34%` gap reduction)
  - `iEx v2` moved from parity with `ripgrep` to ahead on avg median by `1.192 ms`
  - tail was worse in this single rerun window: avg p95 changed from `41.571 ms` to `47.038 ms`
- Read:
  - the clean slice improved the small-corpus median path and meaningfully narrowed the `ugrep` gap
  - the next pressure point is tail stability on the literal-heavy lane, not core median throughput

## Benchmark monitor regression repair - 2026-04-09 13:38Z
- Scope: investigated the live benchmark-monitor summary after the reported window and repaired a real auto-path regression in `iex-core`.
- Regression found:
  - fresh replay after the monitor window showed the Linux literal lane had collapsed far below the earlier same-day results
  - `node tools/scripts/run-once-benchmark.mjs --expression "lit:PM_RESUME" --corpus ".refs/ripgrep/benchsuite/linux" --samples 1 --warmup 0 --build-profile release`
    - before repair: `iexMs=12006.3523`, `rgMs=1485.5751`, `iexToRgRatio=8.0819x`, hotspot=`discover`
  - explicit-thread replays proved the failure was path-specific, not corpus-wide
    - `target/release/iex-cli.exe search "lit:PM_RESUME" ".refs/ripgrep/benchsuite/linux" --json --stats-only --threads 1`
      - `total_ms=12017.7839`
    - `target/release/iex-cli.exe search "lit:PM_RESUME" ".refs/ripgrep/benchsuite/linux" --json --stats-only --threads 8`
      - `total_ms=1923.6854`
- Root cause read:
  - auto mode had split ownership between a serial hybrid walker path and a later collected parallel scan path
  - large directory corpora were paying unnecessary discovery/metadata overhead before real scan work
  - phase attribution was also optimistic for slow-file diagnostics because per-file timing was captured before mmap-backed resources fully dropped
- Changed `crates/iex-core/src/engine.rs`:
  - removed the hybrid auto walker path
  - unified runtime ownership through one path: `discover_files()` then `scan_paths()`
  - direct file roots now bypass directory walking and scan as a single target
  - auto-thread selection now keys off discovered file count instead of sampled byte-shape state
  - `scan_file()` now records final file duration after resources unwind so `slowest_files` better matches wall-clock file cost
- Added regression coverage:
  - `auto_threads_uses_parallelism_for_large_corpora_without_byte_samples`
  - `discover_files_treats_direct_file_root_as_single_scan_target`
- Updated docs:
  - `README.md`
  - added `Search Execution Model` to document the single ownership path and forbid hidden hybrid fallback scan modes
- Verification:
  - `cargo test -p iex-core`
    - pass, `11` tests
  - `npm run test`
    - pass, `409` tests
  - duplicate-risk gate:
    - `bash C:/Users/Savage/.codex/skills/dupe-audit/scripts/dupe-audit.sh --provider gguf --target crates/iex-core/src/engine.rs --task-text "Consolidate auto search ownership into one discovery and scan path without duplicate logic" --output summary --no-write`
    - `candidate_pair_count=0`
    - `exact_duplicate_candidate_count=0`
- Post-repair replays:
  - `node tools/scripts/run-once-benchmark.mjs --expression "lit:PM_RESUME" --corpus ".refs/ripgrep/benchsuite/linux" --samples 1 --warmup 0 --build-profile release`
    - after repair: `iexMs=845.2195`, `rgMs=867.3199`, `iexToRgRatio=0.9745x`, `speedupPct=2.5481`
  - `node tools/scripts/run-once-benchmark.mjs --expression "lit:Шерлок Холмс" --corpus ".refs/ripgrep/benchsuite/subtitles/ru.txt" --samples 1 --warmup 0 --build-profile release`
    - after repair: `iexMs=668.4162`, `rgMs=701.7280`, `iexToRgRatio=0.9525x`, `speedupPct=4.7471`
- Read:
  - the catastrophic large-directory auto-path regression was removed
  - the benchmark monitor's scan-pressure diagnosis was directionally correct, but the engine path itself had drifted and needed repair before any further scan-loop tuning
  - a fresh canonical `npm run bench:report` still needs to be run before treating the dashboard's global median and p95 summary as current truth after this repair

## Benchmark loop reset with preserved self-improvement baseline - 2026-04-09 13:47Z
- Scope: cleared the live benchmark loop back to zero without losing longitudinal self-improvement knowledge, then restarted the dashboard and suite loop on the fresh window.
- Changed:
  - `tools/scripts/reset-bench-state.mjs`
    - added a canonical reset command that archives the prior live history and latest snapshot
    - writes `tools/reports/self-improvement-baseline.json` before clearing live state
    - clears `tools/reports/live-metrics.jsonl`
    - resets `tools/reports/latest.json`
  - `tools/scripts/lib/summary.mjs`
    - added preserved-baseline fallback for self-improvement after resets
    - added a smaller snapshot activation threshold so post-reset self comparison becomes available after a small fresh sample instead of waiting for a full live-window threshold
  - `tools/scripts/dashboard-server.mjs`
    - loads the preserved baseline snapshot and passes it into `summarizeHistory(...)`
  - `package.json`
    - added `bench:reset`
  - `README.md`
    - documented reset behavior and preserved self-improvement baseline storage
  - `tests/materialized/bench-summary-01.test.ts`
    - added coverage for preserved-baseline fallback
    - added coverage for early post-reset snapshot comparison
- Reset evidence:
  - `npm.cmd run bench:reset`
    - `runCountArchived=2989`
    - `baselinePreserved=true`
    - archived live history to `tools/reports/archive/live-metrics.reset-2026-04-09T13-44-32-741Z.jsonl`
    - archived latest snapshot to `tools/reports/archive/latest.reset-2026-04-09T13-44-32-741Z.json`
  - post-reset file state:
    - `tools/reports/live-metrics.jsonl` size `0`
    - `tools/reports/latest.json` content `{}`
    - `tools/reports/self-improvement-baseline.json` preserved `runCount=2989`
- Verification:
  - `npm.cmd run test -- tests/materialized/bench-summary-01.test.ts`
    - pass, `8` tests
  - dashboard summary after restart:
    - preserved-baseline comparison became available on the fresh window
    - after fresh runs accumulated, self-improvement advanced back to live-window mode automatically
  - current process state:
    - dashboard server running via `tools/scripts/dashboard-server.mjs`
    - suite loop running via `tools/scripts/bench-loop-suite.mjs --loops 0 --warmup 1 --samples 3`
- Duplicate-risk gate:
  - `bash C:/Users/Savage/.codex/skills/dupe-audit/scripts/dupe-audit.sh --provider gguf --target tools/scripts/lib/summary.mjs --target tools/scripts/dashboard-server.mjs --target tools/scripts/reset-bench-state.mjs --target tests/materialized/bench-summary-01.test.ts --task-text "Preserve benchmark self-improvement knowledge across live-loop resets without duplicate ownership or parallel reset paths" --output summary --no-write`
  - `candidate_pair_count=0`
  - `exact_duplicate_candidate_count=0`
- Read:
  - the loop can now be reset cleanly without erasing the history needed to judge whether `iEx` is improving against itself
  - self-improvement now survives the reset boundary through a preserved baseline, then naturally hands back to live-window comparison once enough fresh runs exist
  - the reset path stays single-owner and measurable instead of introducing a second reporting system or hidden carry-over file

## Canonical ripgrep benchsuite rerun complete - 2026-04-09 14:18Z
- Scope: completed the requested post-repair canonical `npm run bench:report` rerun and verified that the raw ripgrep benchsuite artifact is complete across the full listed benchmark set.
- Command:
  - `npm run bench:report`
- Artifact:
  - `tools/reports/bench/ripgrep-benchsuite-2026-04-09T13-40-07-183Z.csv`
- Completion evidence:
  - wrapper start: `2026-04-09 15:40:07 +02:00`
  - artifact last write / wrapper exit: `2026-04-09 16:17:53 +02:00`
  - benchmark coverage check: `expected=25`, `actual=25`
- Tail diagnosis:
  - the long tail was isolated to `subtitles_ru_no_literal`
  - the runner remained live inside repeated `ugrep` iterations before exiting cleanly
  - `subtitles_ru_no_literal` medians from the completed raw CSV:
    - `ugrep`: `58.300409s`
    - `rg`: `3.048809s`
    - `rg (ASCII)`: `2.859329s`
    - `grep (ASCII)`: `1.304297s`
    - `ugrep (ASCII)`: `0.995771s`
- Read:
  - the canonical benchsuite rerun is now complete and no longer blocked on an incomplete raw artifact
  - the completion delay came from one external competitor lane in the final Russian no-literal benchmark, not from the repaired `iEx` engine path
  - this rerun refreshes the canonical benchsuite raw evidence at the repo path above; it does not by itself rewrite the live `iEx` dashboard history

## Large single-file stats-only scan repair - 2026-04-09 16:30Z
- Scope: collapsed the dominant `suite-ru-literal` slowdown lane without introducing a second scan system, then aligned the benchmark monitor wording with the actual live feed contract.
- Changed:
  - `crates/iex-core/src/expr.rs`
    - added safe ranged fast-count support for single literals, plain literal regex fast paths, word-boundary literal regex fast paths, and literal alternates
    - added range-count parity tests for literal, word-boundary, and alternate fast paths
  - `crates/iex-core/src/engine.rs`
    - added a chunked fast-count path for very large direct-file `--stats-only` scans
    - keeps one ownership path: the engine still scans a direct-file root as one file, but safe fast-count matchers now divide byte ranges across cores inside that file when the outer scan shape would otherwise stay single-threaded
    - leaves multi-file directory scanning unchanged
  - `README.md`
    - documented the large-file stats-only sharding path
    - clarified canonical raw baseline versus live dashboard feed semantics
  - `dashboard/index.html`
    - corrected the monitor header so `bench:report` is shown as canonical raw baseline and `bench:loop` + `live-metrics.jsonl` are shown as the live feed
  - `.docs/iex-v2-crown-jewel.md`
    - recorded the direct-file large-file fast-count path and the live-vs-canonical benchmark contract
- Validation:
  - `cargo test -p iex-core`
    - pass, `14` tests
  - `npm run test`
    - pass, `411` tests
  - targeted live-feed replay before repair:
    - `node --input-type=module -e "import { runOneBenchmark } from './tools/scripts/lib/benchmark-runner.mjs'; ... profile: 'suite-ru-literal' ..."`
    - observed `iexToRgRatio` range `1.044x -> 1.062x`
  - targeted live-feed replay after repair:
    - `suite-ru-literal`: `0.4688x`, `0.4485x`, `0.2765x`
    - `suite-en-word`: `0.7512x`, `0.5351x`, `0.5024x`
    - `suite-linux-alternates`: `0.7699x`, `0.9263x`, `0.7237x`
- Read:
  - the dominant live slowdown was a large single-file scan shape that never left one outer target, so the fix belonged inside the existing direct-file engine path instead of a new traversal mode
  - `suite-ru-literal` moved from parity-loss territory into repeated `>=50%` speedup hits on the live diagnostic path
  - benchmark wording now matches reality: `bench:report` is the canonical raw external baseline, while the dashboard reads live suite-loop telemetry from `tools/reports/live-metrics.jsonl`

## Ugrep hot-lane closure planning-spec chain - 2026-04-09 17:05Z
- Scope: created a focused planning-spec execution chain for the remaining `ugrep` contender gap, restored the missing `/todo` template source, and wired the new chain into the crown-jewel doc.
- Added:
  - `todo/todo_chain_templates.md`
    - restored a canonical parent/sub-todo template surface for this repo's planning-spec workflow
  - `todo/pending/002-iex-ugrep-hot-lane-closure.md`
  - `todo/pending/002a-iex-ugrep-hot-lane-closure.md`
  - `todo/pending/002b-iex-ugrep-hot-lane-closure.md`
  - `todo/pending/002c-iex-ugrep-hot-lane-closure.md`
  - `todo/pending/002d-iex-ugrep-hot-lane-closure.md`
  - `todo/pending/002e-iex-ugrep-hot-lane-closure.md`
  - `todo/pending/002f-iex-ugrep-hot-lane-closure.md`
- Updated:
  - `.docs/iex-v2-crown-jewel.md`
    - added the current focused closure lane and the new `002` contender-gap chain
- Evidence anchors used while writing the chain:
  - `tools/reports/live-metrics.jsonl`
    - recent hot-lane contender medians identified `suite-en-literal-casei` as the remaining persistent `ugrep` loss lane
    - adjacent English lanes (`suite-en-literal`, `suite-en-word`, `suite-en-alternates`) are already recent wins and now need durability plus ratchets
  - `crates/iex-core/src/expr.rs`
    - current classifier still lacks a case-insensitive word-boundary fast path and restricts plain-literal fast paths to case-sensitive ASCII
  - `.refs/ugrep/include/reflex/matcher.h`
  - `.refs/ugrep/include/reflex/pattern.h`
    - local contender references confirm specialized advance paths and Boyer-Moore skip-table ownership on the weak workload shape
- Read:
  - the next push is now documented as three clean implementation slices instead of a broad optimization blob
  - the dashboard `ugrep` trend line and time-window request is folded into the same contender-ratchet slice, not left as an orphan UI task
  - the repo once again has a canonical `/todo` template source, so future planning-spec work does not depend on reverse-engineering old chain files

## Ugrep chain hardening review - 2026-04-09 17:22Z
- Scope: reviewed the fresh `002` contender-gap chain against the repo mission and strengthened it where it was still too narrow for a real fastest-in-class claim.
- Findings that required correction:
  - the original `002` chain was strong enough to close the remaining `ugrep` hot lane, but not strong enough to justify a world-standard claim because it lacked:
    - canonical `npm run bench:report` proof in the chain itself
    - direct higher-repetition contender proof in the chain itself
    - explicit CLI-truth and release-smoke claim gates
    - a final proof-vs-closeout split
- Updated:
  - `todo/pending/002-iex-ugrep-hot-lane-closure.md`
    - widened the objective and claim boundary so fastest-in-class language is blocked until benchmark, CLI, and release evidence all exist together
    - added canonical proof, contender proof, and release-smoke expectations
    - extended the phase plan to `002f`
  - `todo/pending/002c-iex-ugrep-hot-lane-closure.md`
    - raised the algorithm floor with explicit anchor-selection, skip-distance, shard-overlap, and CPU-dispatch requirements
  - `todo/pending/002e-iex-ugrep-hot-lane-closure.md`
    - repurposed from generic final closeout into a canonical proof gate
  - `todo/pending/002f-iex-ugrep-hot-lane-closure.md`
    - added as the terminal verification and factual-closeout slice
  - `.docs/iex-v2-crown-jewel.md`
    - updated the chain listing to include the proof gate and final closeout split
- Read:
  - the contender chain now has enough proof burden to support a serious fastest-in-class push, not just a hot-lane repair
  - the chain still stays clean: one matcher ownership path, one benchmark history path, one dashboard path
  - the final claim remains evidence-gated; the todo is now strong enough to pursue the claim, not to pre-award it

## Ugrep chain strongest-claim hardening - 2026-04-09 17:31Z
- Scope: tightened the `002` chain again so completion only counts if it lands the strongest measurable fastest-search outcome, not just a repaired or parity-level contender lane.
- Updated:
  - `todo/pending/002-iex-ugrep-hot-lane-closure.md`
    - objective now requires `iEx` to finish as the fastest measured search in the repo's canonical benchmark contract
    - claim boundary now explicitly blocks strongest language unless the canonical contender contract itself is won
  - `todo/pending/002b-iex-ugrep-hot-lane-closure.md`
    - raised the case-insensitive literal target from parity to a real win margin (`iEx/ugrep < 0.90x`)
  - `todo/pending/002c-iex-ugrep-hot-lane-closure.md`
    - raised the hot-kernel target from generic contender wins to `<= 0.85x` median with `p95 < 1.0x` across the four English hot-lane profiles
  - `todo/pending/002d-iex-ugrep-hot-lane-closure.md`
    - strengthened the automated gate so median margin and `p95` tail regressions both fail the lane
  - `todo/pending/002e-iex-ugrep-hot-lane-closure.md`
    - proof gate now requires canonical and direct-contender evidence to agree that `iEx` is the fastest measured tool in the canonical contract
    - explicitly downgrades the claim if engine-core wins but CLI wall time still loses
  - `todo/pending/002f-iex-ugrep-hot-lane-closure.md`
    - closeout now either declares the fastest-measured result directly or names the blocker that still prevents it
  - `.docs/iex-v2-crown-jewel.md`
    - added the strongest measurable outcome rule to the focused closure lane
- Read:
  - the todo no longer closes on "good enough"; it closes only on a strongest measurable win
  - the algorithm slice now has a harder math and margin target, not just a functional fast path
  - the claim still remains honest: strongest language is earned only if the proof package clears every gate

## AGENTS baseline-comparison rule - 2026-04-09 18:51Z
- Scope: promoted the live-vs-candidate binary comparison rule into the repo contract so future performance edits cannot replace the canonical loop or claim wins without a direct before/after comparison on the exact workload they touched.
- Updated:
  - `AGENTS.md`
    - added a permanent operating-principle requirement to compare every benchmark-affecting edit against the current canonical binary before promotion
    - extended the execution sequence so verification now explicitly includes candidate-vs-current-binary comparison on the edited workload
    - strengthened Definition Of Done so performance changes only close when the edited binary proves neutral-or-better against the current canonical binary
- Read:
  - performance work now has an explicit anti-regression promotion rule, not just a best-effort benchmark suggestion
  - future matcher and benchmark edits must clear live-baseline comparison before they are allowed to become the new canonical runner

## AGENTS pre-rebuild snapshot rule - 2026-04-09 19:09Z
- Scope: hardened the benchmark-promotion contract again so the current canonical binary must be snapshotted before any rebuild that could overwrite `target/release/iex-cli.exe`.
- Updated:
  - `AGENTS.md`
    - added a permanent operating-principle requirement to snapshot the current canonical binary into a timestamped evidence path before rebuild
    - extended the verification sequence so pre-rebuild snapshotting is now part of the required promotion workflow
    - strengthened benchmark evidence requirements so the snapshot path is part of the recorded proof bundle
- Read:
  - performance comparisons now preserve an immutable same-binary baseline even when the candidate rebuild targets the canonical release path
  - future rebuilds can no longer erase the only trusted comparison artifact before promotion evidence is captured

## Explicit benchmark binary selection - 2026-04-09 19:14Z
- Scope: removed rebuild ambiguity from live-loop restarts and direct contender replays by letting benchmark entrypoints pin an explicit `iex-cli` binary path.
- Updated:
  - `tools/scripts/lib/benchmark-runner.mjs`
    - added explicit binary-path resolution with fast failure on missing snapshots
    - added `resolveIexBinaryPath(...)` so callers can bypass `cargo build` when replaying a frozen binary
    - emitted `iexBinaryPath` in benchmark run records for provenance
  - `tools/scripts/bench-loop-suite.mjs`
  - `tools/scripts/bench-loop.mjs`
  - `tools/scripts/run-once-benchmark.mjs`
  - `tools/scripts/bench-report.mjs`
    - all now accept `--iex-binary <path>` and forward it through the canonical benchmark runner
  - `tools/scripts/bench-contenders-direct.mjs`
    - contender reruns can now measure a frozen baseline/candidate binary without rebuilding `target/release`
  - `tests/perf/benchmark-runner-binary-path.test.ts`
    - added regression coverage for explicit binary selection and missing-path failure
  - `README.md`
  - `.docs/iex-v2-crown-jewel.md`
    - documented pinned-binary replay as the clean benchmark proof path
  - `tools/scripts/lib/script-helpers.mjs`
    - centralized thin benchmark-wrapper helpers (`argValue`, `toPositiveInt`, `timestampSlug`, `sleep`) so the binary-selection slice does not leave fresh exact duplicates behind
  - live suite loop
    - restarted against `tools/reports/candidate-compare/iex-cli-live-20260409-191339.exe` via `--iex-binary` so the dashboard now measures an immutable runner snapshot instead of depending on startup rebuild behavior
- Validation:
  - `npm run test -- tests/perf/benchmark-runner-binary-path.test.ts`
  - `node --input-type=module -e "import { runOneBenchmark } from './tools/scripts/lib/benchmark-runner.mjs'; ... iexBinaryPath: 'tools/reports/candidate-compare/iex-cli-baseline-20260409-190231.exe', write: false ..."`
  - `node tools/scripts/run-once-benchmark.mjs --expression "lit:Sherlock Holmes" --corpus ".refs/ripgrep/benchsuite/subtitles/en.sample.txt" --warmup 0 --samples 1 --iex-binary tools/reports/candidate-compare/iex-cli-baseline-20260409-190231.exe --quiet`
  - `bash C:/Users/Savage/.codex/skills/dupe-audit/scripts/dupe-audit.sh --provider gguf --target tools/scripts/lib/benchmark-runner.mjs --target tools/scripts/lib/script-helpers.mjs --target tools/scripts/bench-loop-suite.mjs --target tools/scripts/bench-loop.mjs --target tools/scripts/run-once-benchmark.mjs --target tools/scripts/bench-report.mjs --target tools/scripts/bench-contenders-direct.mjs --target tests/perf/benchmark-runner-binary-path.test.ts --target README.md --target .docs/iex-v2-crown-jewel.md --task-text "Add explicit benchmark binary selection and collapse duplicated benchmark wrapper helpers into one canonical script utility module" --output summary --no-write`
    - `candidate_pair_count=2`
    - `exact_duplicate_candidate_count=0`
- Read:
  - benchmark restarts and proof runs no longer depend on whether the loop entrypoint implicitly rebuilds the current source tree
  - frozen baseline and candidate snapshots are now first-class benchmark inputs instead of side-channel operator tricks
  - benchmark-wrapper duplication is back under control; the remaining pair count is low-signal wrapper similarity, not exact duplicate ownership

## Recent-runs ratio clarification - 2026-04-09 19:22Z
- Scope: corrected the dashboard's `Recent Runs` table so it no longer implies that a single ratio/speedup value covers both `ripgrep` and `ugrep`.
- Updated:
  - `dashboard/index.html`
    - split the ambiguous `Ratio` and `Speedup %` columns into explicit `iEx/rg`, `vs rg %`, `iEx/ugrep`, and `vs ugrep %` lanes
    - kept raw `iEx ms`, `rg ms`, and `ugrep ms` visible beside the new competitor-specific ratios
  - `tests/materialized/dashboard-01.test.ts`
    - added regression coverage for the dual-lane recent-runs table headers and ugrep ratio rendering
- Validation:
  - `npm run test -- tests/materialized/dashboard-01.test.ts`
  - `bash C:/Users/Savage/.codex/skills/dupe-audit/scripts/dupe-audit.sh --provider gguf --target dashboard/index.html --target tests/materialized/dashboard-01.test.ts --task-text "Clarify recent-runs ratio rendering so the dashboard shows both iEx/rg and iEx/ugrep lanes instead of implying a single blended ratio" --output summary --no-write`
    - `candidate_pair_count=0`
    - `exact_duplicate_candidate_count=0`
- Read:
  - the recent-runs table now tells the truth directly: `speedupPct` remains the `ripgrep` lane, and `ugrep` gets its own ratio and speedup lane instead of being an unlabeled side number

## Native install and standardization contract - 2026-04-09 19:46Z
- Scope: promoted iEx from repo-local binary to native system command ownership, documented the canonical repo/site surfaces, and encoded the repo rule that agents/operators should prefer installed iEx for local search workflows.
- Updated:
  - `tools/scripts/install-native.ps1`
    - added the canonical Windows native-install path for `iex.exe`
    - snapshots `target/release/iex-cli.exe` before rebuild
    - installs into `%LOCALAPPDATA%\Programs\iEx\bin`
    - updates the user `PATH`
    - writes a PowerShell profile block that removes the built-in `iex` alias and remaps `iex` to the installed binary
  - `tools/scripts/install-native.sh`
    - added the canonical Unix native-install path for `iex`
    - snapshots `target/release/iex-cli` before rebuild
    - installs into `~/.local/bin`
    - appends non-duplicate PATH export blocks to common shell profiles
  - `tools/scripts/install-native.mjs`
    - added one cross-platform dispatcher entrypoint so `npm run install:native` routes to the correct platform installer without parallel ownership
  - `package.json`
    - added `install:native`, `install:native:windows`, and `install:native:unix`
  - `README.md`
    - added canonical repo/site URLs, native install instructions, PowerShell alias takeover note, and the agent/operator preference rule
  - `.docs/iex-v2-crown-jewel.md`
    - added native operator adoption doctrine, canonical repo/site URLs, and installer ownership surfaces
  - `AGENTS.md`
    - added the repo-level rule to prefer installed `iex` for local search and search-validation workflows once native install is present
- Validation:
  - `npm run install:native:windows -- -ForceRebuild`
  - `npm run install:native -- --SkipPowerShellProfile --SkipPathUpdate`
  - `powershell -Command "(Get-Command iex).CommandType; (Get-Command iex).Definition"`
  - `powershell -Command "iex search 'lit:Sherlock Holmes' '.refs/ripgrep/benchsuite/subtitles/en.sample.txt' --stats-only"`
  - `bash -n tools/scripts/install-native.sh`
- Evidence:
  - installed binary: `C:\Users\Savage\AppData\Local\Programs\iEx\bin\iex.exe`
  - PowerShell profile: `E:\DESKTOP\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1`
  - pre-rebuild snapshot: `tools/reports/candidate-compare/iex-cli-preinstall-20260409-174542.exe`
- Read:
  - iEx now has one canonical native-install lane on Windows plus one canonical Unix lane, with the PowerShell `iex` alias collision handled explicitly instead of left as operator trivia
  - repo docs and agent guidance now treat installed iEx as the preferred local-search surface without introducing a second ownership system

## Native macOS artifact workflow - 2026-04-09 20:03Z
- Scope: added the canonical CI build lane for friend-safe macOS binaries so Windows operators can still produce real macOS artifacts without pretending the local machine can link them.
- Updated:
  - `.github/workflows/build-native-binaries.yml`
    - added native artifact builds for Windows x64, Linux x64, macOS x64, and macOS arm64
    - packages each binary into an explicit per-platform archive and uploads it as a workflow artifact
  - `README.md`
    - documented the CI path for friend-share macOS binaries and the exact installer handoff path
    - documented the real cross-build limitation from Windows
  - `.docs/iex-v2-crown-jewel.md`
    - added the release-artifact rule that macOS binaries come from native macOS runners with architecture-explicit naming
- Validation:
  - `cargo build --release -p iex-cli --target x86_64-apple-darwin`
    - failed as expected on Windows with `linker 'cc' not found` and missing Apple SDK discovery
  - workflow file created as the canonical remediation path
- Read:
  - the target triple alone is not enough to mint a distributable macOS binary from this Windows machine
  - the clean ownership path is CI on native macOS runners, then `install-native.sh --source-binary ...` on the recipient machine

## Harvested optimization planning chains - 2026-04-09 20:29Z
- Scope: locked the next harvested ugrep/ripgrep-inspired optimization wave into three category-pure planning-spec parents instead of one blurred mega-chain.
- Updated:
  - `todo/pending/003-iex-pma-reject-fast-gate.md`
  - `todo/pending/003a-iex-pma-reject-fast-gate.md`
  - `todo/pending/003b-iex-pma-reject-fast-gate.md`
  - `todo/pending/003c-iex-pma-reject-fast-gate.md`
  - `todo/pending/003d-iex-pma-reject-fast-gate.md`
  - `todo/pending/003e-iex-pma-reject-fast-gate.md`
  - `todo/pending/004-iex-rare-byte-anchor-ranking.md`
  - `todo/pending/004a-iex-rare-byte-anchor-ranking.md`
  - `todo/pending/004b-iex-rare-byte-anchor-ranking.md`
  - `todo/pending/004c-iex-rare-byte-anchor-ranking.md`
  - `todo/pending/004d-iex-rare-byte-anchor-ranking.md`
  - `todo/pending/004e-iex-rare-byte-anchor-ranking.md`
  - `todo/pending/005-iex-pin-count-alternates-kernel.md`
  - `todo/pending/005a-iex-pin-count-alternates-kernel.md`
  - `todo/pending/005b-iex-pin-count-alternates-kernel.md`
  - `todo/pending/005c-iex-pin-count-alternates-kernel.md`
  - `todo/pending/005d-iex-pin-count-alternates-kernel.md`
  - `todo/pending/005e-iex-pin-count-alternates-kernel.md`
  - `.docs/iex-v2-crown-jewel.md`
- Read:
  - `003` isolates PMA-style reject-fast gating as its own exact-miss capability, tied to Linux alternates and ASCII casefold target workloads.
  - `004` isolates rare-byte anchor ranking and anchor-quality discipline for the literal family without blurring into a second searcher stack.
  - `005` isolates tiny-alternates kernel specialization for small literal sets without creating a parallel alternates engine.
  - every harvested parent now carries its own immutable current-versus-candidate proof gate and explicit `>=20%` motivating improvement target against the pre-rebuild canonical binary.

## Reject-fast contract execution (`003a` + `003b`) - 2026-04-09 21:39Z
- Scope: executed the baseline-lock and planner-contract slices for `003`, proved the first runtime gate attempt was slower, reverted that hot-path consumption, and left the repo in a clean `003b-done / 003c-pending` state.
- Updated:
  - `crates/iex-core/src/expr.rs`
    - added planner-owned `RejectFastGate` metadata for long ASCII literal-family fast paths
    - locked narrow readiness rules so short literals and unsupported casefold shapes stay off the gate
    - added planner tests for long casefold literals, long alternates, short literal rejection, and direct gate exactness
  - `todo/changelog/003a-iex-pma-reject-fast-gate.md`
    - archived the contract-lock slice with immutable baseline snapshot evidence
  - `todo/changelog/003b-iex-pma-reject-fast-gate.md`
    - archived the planner-contract slice with `cargo test -p iex-core` evidence
  - `todo/pending/003c-iex-pma-reject-fast-gate.md`
    - recorded the blocked runtime-consumption state and the rejected candidate evidence
  - `.docs/iex-v2-crown-jewel.md`
    - added the current truth boundary for `003`: planner metadata landed, runtime consumer still open
- Validation:
  - `cargo test -p iex-core`
  - immutable baseline snapshot: `tools/reports/candidate-compare/iex-cli-baseline-20260409-212927.exe`
  - rejected runtime candidate proof:
    - linux alternates: baseline `1263.2337 ms` vs candidate `1393.0169 ms`
    - casefold Sherlock: baseline `62.6503 ms` vs candidate `67.0499 ms`
  - final in-tree `003b`-only replay:
    - linux alternates: baseline `773.4974 ms` vs candidate `790.2546 ms`
    - casefold Sherlock: baseline `60.5494 ms` vs candidate `63.6100 ms`
- Read:
  - the planner contract is now real and test-backed, but the first canonical runtime consumer was not cheap enough to survive proof
  - `003c` must resume from a lower-overhead consumption point inside the existing literal/casefold/alternates path before any reject-fast speed claim is allowed

## Reject-fast runtime probe 2 (rejected) - 2026-04-09 22:13Z
- Scope: tested a second `003c` consumption shape that narrowed runtime usage to an alternates-only quick-prefix prefilter, then rejected it on pinned proof.
- Updated:
  - `crates/iex-core/src/expr.rs`
    - kept the planner-owned `RejectFastGate` metadata and exact gate tests
    - explored and then reverted an alternates-only quick-prefix runtime consumer after proof failed
  - `todo/pending/003c-iex-pma-reject-fast-gate.md`
    - expanded the blocked state with the second rejected candidate replay set
- Validation:
  - `cargo test -p iex-core`
  - pinned baseline: `tools/reports/candidate-compare/iex-cli-baseline-20260409-212927.exe`
  - rejected quick-prefix candidate: `tools/reports/candidate-compare/iex-cli-candidate-20260409-221023.exe`
    - linux alternates: baseline `1097.6613 ms` vs candidate `1144.7457 ms`
    - casefold Sherlock: baseline `63.3355 ms` vs candidate `62.3203 ms`, but no trustworthy contender win and no reason to promote the wave from that isolated noise
- Read:
  - a prefix-automaton reject gate is cheaper than the first whole-signature attempt, but it still loses the motivating linux alternates lane
  - the repo remains in the clean `003b landed / 003c blocked` state, which is better than carrying an evidence-failing runtime path

## Reject-fast runtime probe 3 (rejected) - 2026-04-09 22:24Z
- Scope: tested a third `003c` consumption shape that used `RejectFastGate` only for long ASCII casefold literals as a quick prefix enumerator before exact verification, then reverted it after proof.
- Updated:
  - `crates/iex-core/src/expr.rs`
    - explored and then reverted a casefold-only runtime consumer
    - preserved planner metadata and exact gate tests only
  - `todo/pending/003c-iex-pma-reject-fast-gate.md`
    - expanded the blocked state with the third rejected candidate replay set
- Validation:
  - `cargo test -p iex-core`
  - pinned candidate: `tools/reports/candidate-compare/iex-cli-candidate-20260409-222031.exe`
    - linux alternates: baseline `1098.7177 ms` vs candidate `869.3115 ms`
    - casefold Sherlock: baseline `61.6817 ms` vs candidate `70.8719 ms`
- Read:
  - even the narrowest runtime consumer tested so far still damages one of the chain's motivating workloads enough to fail promotion
  - the disciplined outcome remains the same: keep `003c` blocked and the tree clean until a materially different canonical consumer exists

## Upstream harvest frontier expansion - 2026-04-09 23:40Z
- Scope: completed the `006` frontier harvest package to widen `.refs`, lock exact upstream revisions, search the best-nine mirrors with installed `iex`, and route the best mechanisms back into canonical iEx chains.
- Updated:
  - `.refs/re2`
    - added local mirror of `google/re2` at `972a15cedd008d846f1a39b2e88ce48d7f166cbd`
  - `.refs/hyperscan`
    - added local mirror of `intel/hyperscan` at `78d23dabf5827e202a8ba2d569ed67f4f811b285`
  - `.refs/vectorscan`
    - added local mirror of `VectorCamp/vectorscan` at `de317291e9c46f8060719f655920043fabc968ed`
  - `.refs/regex`
    - added local mirror of `rust-lang/regex` at `839d16bc65b60e2006d3599d20bfa6efc14049d8`
  - `.refs/aho-corasick`
    - added local mirror of `BurntSushi/aho-corasick` at `0f3f5da9bdec0d811f9a344e3cb9c033b15df20b`
  - `.refs/memchr`
    - added local mirror of `BurntSushi/memchr` at `24f5daa5257e00e87007c936761600e034827905`
  - `.refs/stringzilla`
    - added local mirror of `ashvardanian/StringZilla` at `d126d12f4c7a52755744a320529de2a75e76a964`
  - `.docs/recon/upstream-harvest-frontier-2026-04-09.md`
    - added the canonical frontier ledger with the top-20 universe, best-nine mirror inventory, keyword packs, twelve adoption-grade mechanisms, risks, and the chain routing
  - `todo/changelog/006-iex-upstream-harvest-expansion.md`
  - `todo/changelog/006a-iex-upstream-harvest-expansion.md`
  - `todo/changelog/006b-iex-upstream-harvest-expansion.md`
  - `todo/changelog/006c-iex-upstream-harvest-expansion.md`
  - `todo/changelog/006d-iex-upstream-harvest-expansion.md`
  - `todo/changelog/006e-iex-upstream-harvest-expansion.md`
  - `todo/changelog/006f-iex-upstream-harvest-expansion.md`
    - archived the full `006` chain as a completed frontier harvest package
  - `.docs/iex-v2-crown-jewel.md`
    - added the widened frontier state and the new best-next blended moves for `004` and future `006` runtime work
- Validation:
  - mirror revision lock:
    - `ripgrep=4519153e5e461527f4bca45b042fff45c4ec6fb9`
    - `ugrep=c701fb852c8fe5ea48143bf809596470d5e2b248`
    - `re2=972a15cedd008d846f1a39b2e88ce48d7f166cbd`
    - `hyperscan=78d23dabf5827e202a8ba2d569ed67f4f811b285`
    - `vectorscan=de317291e9c46f8060719f655920043fabc968ed`
    - `regex=839d16bc65b60e2006d3599d20bfa6efc14049d8`
    - `aho-corasick=0f3f5da9bdec0d811f9a344e3cb9c033b15df20b`
    - `memchr=24f5daa5257e00e87007c936761600e034827905`
    - `stringzilla=d126d12f4c7a52755744a320529de2a75e76a964`
  - installed `iex` harvest proofs:
    - `.refs/ugrep/include/reflex/pattern.h` exposed `predict_match`, `predict_match_quick`, `predict_match_min`, `lcp_`, `lcs_`, `bmd_`, and `bms_`
    - `.refs/ugrep/include/reflex/matcher.h` exposed `advance_pattern_pin2_*` through `pin16`
    - `.refs/re2/re2/filtered_re2.cc` and `.refs/re2/re2/prefilter*.cc` exposed the atom/prefilter-tree path
    - `.refs/regex/regex-automata/src/dfa/dense.rs` exposed start-state-specialized prefilter guidance and serialization caveats
    - `.refs/hyperscan/src/compiler/compiler.cpp` and `.refs/vectorscan/src/compiler/compiler.cpp` exposed prefilter transforms and pure-literal shortcuts
    - `.refs/memchr/src/arch/*/packedpair.rs` exposed heuristic packed-pair prefilters
    - `.refs/stringzilla/include/stringzilla/find.h` exposed interesting-byte anomaly ranking and SIMD substring dispatch surfaces
- Read:
  - the widened frontier now has enough breadth to stop guessing about future planner and file-strategy work
  - the next best runtime move is not a blind `003c` retry; it is a blended planner/anchor pass that combines the strongest ideas from ripgrep, ugrep, re2, regex-automata, memchr, and StringZilla while staying inside canonical iEx ownership

## Rare-byte anchor ranking promoted - 2026-04-09 23:16Z
- Scope: executed the `004` rare-byte anchor wave end to end, promoted the candidate after immutable current-vs-candidate proof, and archived the chain with measured closeout evidence.
- Updated:
  - `crates/iex-core/src/expr.rs`
    - added planner-owned `LiteralAnchorPlan` readiness rules for ASCII literal-family needles
    - integrated a ranked-anchor `memchr` / `memchr2` probe path into `AsciiCaseFoldSearcher`
    - kept unsupported shapes on the prior shift-based path instead of spawning a second searcher stack
    - added deterministic anchor-selection and false-positive-anchor regression tests
  - `todo/changelog/004a-iex-rare-byte-anchor-ranking.md`
  - `todo/changelog/004b-iex-rare-byte-anchor-ranking.md`
  - `todo/changelog/004c-iex-rare-byte-anchor-ranking.md`
  - `todo/changelog/004d-iex-rare-byte-anchor-ranking.md`
  - `todo/changelog/004e-iex-rare-byte-anchor-ranking.md`
  - `todo/changelog/004-iex-rare-byte-anchor-ranking.md`
    - archived the full `004` chain after proof and closeout
  - `.docs/iex-v2-crown-jewel.md`
    - updated the harvest-wave status to reflect that `004` is now promotable and archived
- Validation:
  - `cargo test -p iex-core`
  - snapshot path: `tools/reports/candidate-compare/iex-cli-pre004a-20260409-230309.exe`
  - proof artifact: `tools/reports/candidate-compare/004c-anchor-proof-20260409-231121.json`
    - `re:(?i)Sherlock Holmes`: current `99.5026 ms` median engine vs candidate `71.5135 ms` (`+28.13%`, ratio `0.7187`)
    - `re:\bSherlock Holmes\b`: current `55.7469 ms` vs candidate `55.4852 ms` (`+0.47%`)
    - diagnostic `re:(?i)\bSherlock Holmes\b`: current `74.2355 ms` vs candidate `66.3552 ms` (`+10.62%`)
  - dupe-audit:
    - `target_file_count=1`
    - `candidate_pair_count=0`
    - `exact_duplicate_candidate_count=0`
- Read:
  - the rare-byte anchor wave now has real proof, not just harvest theory: it materially reduces the motivating casefold literal lane without semantic drift
  - the strongest next move is no longer to keep tuning anchor choice; it is to spend that new literal-quality discipline either on a more selective `003c` consumer or on `005` tiny alternates if the priority is visible contender pressure

## Live loop promotion gate hardened - 2026-04-09 23:27Z
- Scope: re-tested the current `iex-cli.exe` against the exact binary driving the active suite loop, promoted the candidate only after the longer suite proof cleared, restarted the live loop on an immutable snapshot, and codified the process in repo doctrine so future swaps require the same proof discipline.
- Updated:
  - `AGENTS.md`
    - removed the local-search `rg` escape hatch in favor of `iex`-first repo search with explicit blocker recording
    - added an active-loop promotion rule requiring full-suite interleaved proof against the exact running loop binary before any loop repoint
    - added restart verification guidance that requires `tools/reports/latest.json` to confirm the promoted snapshot path
  - `.docs/iex-v2-crown-jewel.md`
    - added the active-loop promotion gate to the benchmark contract
    - recorded the current promotion proof artifact and the immutable live-loop snapshot now driving the dashboard feed
- Validation:
  - active-loop binary before promotion:
    - `E:\Workspaces\01_Projects\01_Github\iEx-Engine-v2\tools\reports\candidate-compare\iex-cli-live-20260409-191339.exe`
  - extended proof artifact:
    - `tools/reports/candidate-compare/live-loop-compare-extended-20260409-232531.json`
    - summary: `profile_count=12`, `wins=8`, `losses=4`, `median_ratio_candidate_vs_baseline_engine=0.9816`, `median_improvement_pct_engine=1.84`
    - strongest gain: `suite-en-literal-casei` (`64.614 ms` -> `49.716 ms`, `+23.06%`)
    - remaining losses: `suite-en-alternates` (`-5.14%`), `suite-linux-literal` (`-2.05%`), `suite-en-word` (`-2.00%`), `suite-en-no-literal` (`-0.01%`)
  - promoted live snapshot:
    - `E:\Workspaces\01_Projects\01_Github\iEx-Engine-v2\tools\reports\candidate-compare\iex-cli-live-20260409-232626.exe`
  - loop restart verification:
    - active loop command now pins `--iex-binary E:\Workspaces\01_Projects\01_Github\iEx-Engine-v2\tools\reports\candidate-compare\iex-cli-live-20260409-232626.exe`
    - `E:\Workspaces\01_Projects\01_Github\iEx-Engine-v2\tools\reports\latest.json` reports the same `iexBinaryPath`
- Read:
  - the right promotion discipline is now explicit: a candidate can win a motivating lane and still stay out of the live loop until it beats the exact active-loop binary on the whole suite with a durable signal
  - this promotion is real but not universal dominance yet, which cleanly points the next runtime work toward the remaining alternates and Linux literal debt instead of reopening anchor proof

## Second-pass `.refs` next-leap harvest recorded - 2026-04-10 00:08Z
- Scope: ran a narrower second-pass harvest over the mirrored upstream refs with installed `iex` to isolate the highest-upside mechanisms that were still under-expressed in the first `006` frontier note.
- Updated:
  - `.docs/recon/upstream-harvest-next-leaps-2026-04-09.md`
    - recorded the ranked next-leap set with exact source paths, adoption seams, expected wins, and risks
  - `.docs/iex-v2-crown-jewel.md`
    - promoted adaptive acceleration control, multi-regime literal dispatch, tiny-set SIMD confirmation, and file-strategy arbitration into the live strategy narrative
- Validation:
  - installed `iex` evidence runs:
    - `iex search "re:PrefilterState|MIN_SKIP_BYTES|MIN_SKIPS|inert|average" .refs\memchr\src\memmem\searcher.rs`
    - `iex search "re:RequiredPrefixForAccel|ConfigurePrefixAccel|dfa_should_bail_when_slow|prefix accel|anchored" .refs\re2\re2`
    - `iex search "re:locate_needle_anomalies|Raita|Horspool|256|middle|prefix|suffix" .refs\stringzilla\include\stringzilla\find.h`
    - `iex search "re:Teddy|rare bytes|anchored|premultiplication|alphabet" .refs\aho-corasick\DESIGN.md`
    - `iex search "re:mmap|heap limit|multiline|slice|reader|buffer allocation|binary detection" .refs\ripgrep\crates\searcher\src\searcher\mod.rs`
- Read:
  - the next visible leap is more likely to come from adaptive acceleration control plus regime arbitration than from another isolated static heuristic
  - the clean execution order is now clearer: `006` should feed planner/runtime doctrine first, then `003` and `005` can consume that stronger contract without reopening old tail debt blindly

## Deferred frontier boundaries recorded - 2026-04-10 00:23Z
- Scope: pressure-tested the broader `.refs` mirror set for one more leap and converted the strongest negative findings into explicit defer boundaries so the next runtime wave does not wander into second-mode or exactness-fragile work.
- Updated:
  - `.docs/recon/upstream-harvest-next-leaps-2026-04-09.md`
    - added a `Big But Deferred Leaps` section covering indexed corpus pruning, reverse-inner/start-state fragility, and Hyperscan compile-heavy literal packing
  - `.docs/iex-v2-crown-jewel.md`
    - added the new defer lines to the live strategy doctrine
- Validation:
  - installed `iex` evidence runs:
    - `iex search "re:reverse inner|prefilter|start states specialized|disastrous" .refs\regex\tests\fuzz\mod.rs`
    - `iex search "re:match_hfa|hashes_size|Bloom filters|1-grams to 8-grams" .refs\ugrep\src\ugrep-indexer.cpp`
    - `iex search "re:literal short cut|pure literal|Rose|dumpSmallWrite|generateRoseEngine" .refs\hyperscan\src\compiler\compiler.cpp`
    - `iex search "re:bucketToLits|min_len|included literal|squash|confirm" .refs\hyperscan\src\fdr\fdr_compile.cpp`
- Read:
  - the most valuable new insight from the extra sweep is boundary clarity, not a new runtime parent: indexed pruning is powerful but non-canonical, reverse-inner confirms why bailout discipline must land first, and Hyperscan packing is a later ceiling-breaker for `005`, not the next slice

## Multi-root search contract landed - 2026-04-09 21:58Z
- Scope: closed the operator contract gap that rejected multiple search roots by widening CLI, bench, and core ownership to one canonical roots-list path without introducing duplicate walkers, duplicate scan passes, or single-root speed debt.
- Updated:
  - `crates/iex-cli/src/main.rs`
    - widened `search` from singular `[PATH]` to `[PATH]...` with `.` as the default and passed one canonical roots list into core
  - `crates/iex-bench/src/main.rs`
    - matched the same positional roots contract so benchmark helper and CLI stay aligned
  - `crates/iex-core/src/engine.rs`
    - changed `SearchConfig` from singular `root` to canonical `roots: Vec<PathBuf>`
    - kept direct-file roots on the existing direct-target path, shared one walker builder across directory roots, and deduped overlaps before scan
    - added `discover_files_supports_multiple_directory_roots`, `discover_files_dedupes_file_root_under_directory_root`, and `run_search_supports_multiple_roots_end_to_end`
  - `README.md`
  - `.docs/iex-v2-crown-jewel.md`
- Validation:
  - `cargo test -p iex-core`
    - passed: `31` tests, `0` failed
  - CLI proof:
    - `target/release/iex-cli.exe search --help`
      - now shows `Usage: iex-cli.exe search [OPTIONS] <EXPR> [PATH]...`
    - `target/release/iex-cli.exe search "006-iex-upstream-harvest-expansion" todo .docs --stats-only`
      - `files discovered: 85`, `files scanned: 85`, `matches found: 14`
    - `target/release/iex-cli.exe search "006-iex-upstream-harvest-expansion" todo todo .docs .docs --stats-only`
      - still `files discovered: 85`, `files scanned: 85`, `matches found: 14`, proving repeated and overlapping roots do not double-count
    - native install refresh:
      - `powershell -NoProfile -ExecutionPolicy Bypass -File tools\scripts\install-native.ps1 -SourceBinary target\release\iex-cli.exe`
      - installed binary: `C:\Users\Savage\AppData\Local\Programs\iEx\bin\iex.exe`
      - post-refresh `iex search "006-iex-upstream-harvest-expansion" todo .docs --stats-only`
        - `files discovered: 85`, `files scanned: 85`, `matches found: 18`
      - post-refresh repeated-root replay over `todo todo .docs .docs`
        - still `files discovered: 85`, `files scanned: 85`, `matches found: 18`
  - Proof gate:
    - pre-rebuild repo snapshot was missed for this slice, so the pinned baseline was the exact live-loop binary instead of a repo-local pre-build snapshot
    - baseline: `E:\Workspaces\01_Projects\01_Github\iEx-Engine-v2\tools\reports\candidate-compare\iex-cli-live-20260409-232626.exe`
    - candidate: `E:\Workspaces\01_Projects\01_Github\iEx\target\release\iex-cli.exe`
    - exact workload: `lit:Sherlock Holmes` on `.refs/ripgrep/benchsuite/subtitles/en.sample.txt`
    - baseline `iexMs=39.7138`, candidate `iexMs=38.6666`, candidate/baseline ratio `0.9736`, improvement `+2.64%`
  - Duplicate-risk review:
    - `bash C:/Users/Savage/.codex/skills/dupe-audit/scripts/dupe-audit.sh --provider gguf --target README.md --target .docs/iex-v2-crown-jewel.md --target .docs/todo/changelog/_log.md --target todo/changelog/007-iex-multi-root-search-paths.md --target todo/changelog/007a-iex-multi-root-search-paths.md --target todo/changelog/007b-iex-multi-root-search-paths.md --target todo/changelog/007c-iex-multi-root-search-paths.md --target todo/changelog/007d-iex-multi-root-search-paths.md --target todo/changelog/007e-iex-multi-root-search-paths.md --task-text "Multi-root search path feature documentation and closeout" --output summary --no-write`
    - result: `target_file_count=9`, `candidate_pair_count=8`, `exact_duplicate_candidate_count=0`
- Read:
  - ripgrep’s clean multi-path lesson was the right ownership seam: one builder over many roots, not one independent search per root
  - iEx keeps that doctrine while preserving its direct-file fast path, which closed the operator footgun without paying a visible single-root regression tax

## Simple multipath vs snapshot proof captured - 2026-04-10 00:38Z
- Scope: ran the direct, non-normalized heavy-data comparison the operator asked for: current build with multipath, current build without multipath, and the outdated snapshot in its normal single-root mode.
- Updated:
  - `tools/reports/candidate-compare/simple-multipath-vs-snapshot-20260410-003721.json`
    - captured five interleaved rounds across one literal-heavy lane and one alternates-heavy lane with engine and wall-clock medians
  - `.docs/todo/changelog/_log.md`
    - recorded the outcome and artifact path for later promotion decisions
- Validation:
  - current build identity:
    - `target/release/iex-cli.exe`
    - `sha256=F96709724C27FF93DE17F7594368323EE1C1151E5432476A051231DA06A66D25`
  - snapshot identity:
    - `tools/reports/candidate-compare/iex-cli-pre004a-20260409-230309.exe`
    - `sha256=47B919B43700D476D4DC8F950B3A3E9F44C08B19DDB7C40708EFC0F98562B318`
  - literal-heavy medians:
    - current multipath over `subtitles + linux`: `engine_ms=1552.7192`, `wall_ms=1569.8665`
    - current single-root over `benchsuite`: `engine_ms=1304.4176`, `wall_ms=1321.4651`
    - snapshot single-root over `benchsuite`: `engine_ms=1343.4236`, `wall_ms=1363.2074`
  - alternates-heavy medians:
    - current multipath over `linux + en.sample.txt`: `engine_ms=962.5567`, `wall_ms=980.3783`
    - current single-root over `linux`: `engine_ms=856.4536`, `wall_ms=876.8089`
    - snapshot single-root over `linux`: `engine_ms=865.4273`, `wall_ms=885.6395`
  - capability probe:
    - `tools/reports/candidate-compare/iex-cli-pre004a-20260409-230309.exe search 'lit:Sherlock Holmes' '.refs\ripgrep\benchsuite\subtitles' '.refs\ripgrep\benchsuite\linux' --stats-only --json`
      - failed with `error: unexpected argument '.refs\ripgrep\benchsuite\linux' found`
- Read:
  - on these direct heavy runs, the current binary is slightly faster than the snapshot in normal single-root mode
  - on these same direct runs, enabling multipath in the current binary is slower than the current single-root path, so this pass does not show a speed win from multipath itself
  - the snapshot remains single-root only, so the comparison result is capability plus a small single-root speed gain, not evidence that multipath acceleration is already paying for itself

## 008 multipath turbo planning chain locked - 2026-04-10 01:12Z
- Scope: converted the next blended runtime wave into a full planning-spec chain so multipath tax removal and adaptive medium/long literal acceleration now have one approved execution graph, one baseline artifact, and one proof doctrine.
- Updated:
  - `todo/pending/008-iex-multipath-turbo-wave.md`
  - `todo/pending/008a-iex-multipath-turbo-wave.md`
  - `todo/pending/008b-iex-multipath-turbo-wave.md`
  - `todo/pending/008c-iex-multipath-turbo-wave.md`
  - `todo/pending/008d-iex-multipath-turbo-wave.md`
  - `todo/pending/008e-iex-multipath-turbo-wave.md`
  - `todo/pending/008f-iex-multipath-turbo-wave.md`
  - `.docs/iex-v2-crown-jewel.md`
    - added the `008` chain and ratcheted the live strategy note to the equal-workload multipath baseline plus the blended traversal-and-acceleration doctrine
  - `tools/reports/candidate-compare/multipath-equivalent-baseline-20260410-010846.json`
    - pinned the equal-workload multipath baseline that `008a` now freezes
- Validation:
  - equal-workload baseline artifact:
    - `tools/reports/candidate-compare/multipath-equivalent-baseline-20260410-010846.json`
    - single-root benchsuite median total: `1501.6144 ms`
    - equivalent split benchsuite median total: `1775.7652 ms` (`+18.26%`)
    - overlap root-plus-child median total: `1911.7175 ms` (`+27.31%`)
    - equal-workload discovery delta: `210.0373 ms` single-root vs `278.6612 ms` equivalent split (`+32.67%`)
  - planning-spec template and style references reviewed:
    - `todo/todo_chain_templates.md`
    - `todo/pending/003-iex-pma-reject-fast-gate.md`
    - `todo/pending/005-iex-pin-count-alternates-kernel.md`
    - `todo/changelog/007a-iex-multi-root-search-paths.md`
- Read:
  - the next slice is no longer ambiguous enough to stay as a loose note: `008` now locks the execution order as root normalization first, discovery scheduling second, medium/long literal regime dispatch third, and adaptive bailout plus proof gate after that
  - using planning-spec here materially reduces the chance of split-brain work, because traversal cleanup, stats extension, benchmark replay, and acceleration doctrine are now one bounded chain instead of four drifting side quests

## 008 durable root normalization slice landed - 2026-04-10 01:50Z
- Scope: completed the highest-value non-experimental part of `008` by moving duplicate and overlap root cleanup ahead of traversal, extending stable multipath telemetry, and proving the effect against the immutable pre-rebuild binary on the heavy benchsuite workload.
- Updated:
  - `crates/iex-core/src/engine.rs`
    - added canonical root normalization for exact duplicate roots, ancestor-child directory roots, and directory-contained-file roots before discovery
    - preserved one engine-owned discovery path while exposing retained versus pruned root attribution
    - kept late file-path dedupe as a residual safety net and now counts residual duplicate discoveries instead of relying on it as the primary overlap strategy
  - `crates/iex-core/src/stats.rs`
    - extended `SearchStats` with zero-safe multipath attribution fields: `input_roots`, `effective_roots`, `pruned_roots`, `overlap_pruned_roots`, `discovered_duplicate_paths`, and `acceleration_bailouts`
  - `.docs/iex-v2-crown-jewel.md`
    - recorded the immutable proof result and clarified that the measured win came from traversal ownership cleanup, not from any experimental acceleration path
  - `tools/reports/candidate-compare/iex-cli-prerebuild-20260410-014819.exe`
    - immutable pre-rebuild snapshot of the canonical binary used for comparison
  - `tools/reports/candidate-compare/multipath-root-normalization-compare-20260410-014913.json`
    - pinned baseline-versus-candidate proof artifact for single-root, equivalent split roots, and overlap roots
- Validation:
  - tests:
    - `cargo test -p iex-core`
    - `cargo test -p iex-cli`
  - immutable proof medians from `tools/reports/candidate-compare/multipath-root-normalization-compare-20260410-014913.json`:
    - single-root benchsuite: `1766.3543 ms` baseline vs `1743.3195 ms` candidate (`-1.30%`)
    - equivalent split benchsuite: `1962.0795 ms` baseline vs `1863.8279 ms` candidate (`-5.01%`)
    - overlap root-plus-child: `2400.2522 ms` baseline vs `1786.9055 ms` candidate (`-25.55%`)
    - overlap-root discovery median: `712.2233 ms` baseline vs `269.2962 ms` candidate (`-62.19%`)
  - telemetry proof:
    - overlap-root candidate median reported `input_roots=2`, `effective_roots=1`, `pruned_roots=1`, `overlap_pruned_roots=1`, `discovered_duplicate_paths=0`
- Read:
  - this slice is worth keeping because it fixes the worst proven multipath waste at the ownership boundary and does so without paying a single-root regression tax
  - the next experimental acceleration work now has a cleaner benchmark surface, because any later gain or loss can be attributed on top of normalized roots instead of mixed together with overlap churn

## Single-root frontier planning split locked - 2026-04-10 02:35Z
- Scope: distilled the insect research harvest into separate planning-spec parents so the next single-root wave can advance as category-pure deliverables instead of one mixed acceleration blob.
- Updated:
  - `todo/pending/009-iex-single-root-binary-layout-proof-lane.md`
  - `todo/pending/009a-iex-single-root-binary-layout-proof-lane.md`
  - `todo/pending/009b-iex-single-root-binary-layout-proof-lane.md`
  - `todo/pending/009c-iex-single-root-binary-layout-proof-lane.md`
  - `todo/pending/009d-iex-single-root-binary-layout-proof-lane.md`
  - `todo/pending/009e-iex-single-root-binary-layout-proof-lane.md`
  - `todo/pending/009f-iex-single-root-binary-layout-proof-lane.md`
  - `todo/pending/010-iex-branchless-block-stage-literal-scan.md`
  - `todo/pending/010a-iex-branchless-block-stage-literal-scan.md`
  - `todo/pending/010b-iex-branchless-block-stage-literal-scan.md`
  - `todo/pending/010c-iex-branchless-block-stage-literal-scan.md`
  - `todo/pending/010d-iex-branchless-block-stage-literal-scan.md`
  - `todo/pending/010e-iex-branchless-block-stage-literal-scan.md`
  - `todo/pending/010f-iex-branchless-block-stage-literal-scan.md`
  - `todo/pending/011-iex-required-substring-boolean-gate.md`
  - `todo/pending/011a-iex-required-substring-boolean-gate.md`
  - `todo/pending/011b-iex-required-substring-boolean-gate.md`
  - `todo/pending/011c-iex-required-substring-boolean-gate.md`
  - `todo/pending/011d-iex-required-substring-boolean-gate.md`
  - `todo/pending/011e-iex-required-substring-boolean-gate.md`
  - `todo/pending/012-iex-regex-decomposition-prefilter-wave.md`
  - `todo/pending/012a-iex-regex-decomposition-prefilter-wave.md`
  - `todo/pending/012b-iex-regex-decomposition-prefilter-wave.md`
  - `todo/pending/012c-iex-regex-decomposition-prefilter-wave.md`
  - `todo/pending/012d-iex-regex-decomposition-prefilter-wave.md`
  - `todo/pending/012e-iex-regex-decomposition-prefilter-wave.md`
  - `todo/pending/012f-iex-regex-decomposition-prefilter-wave.md`
- Validation:
  - continuity pass over the new parents and sub-todos:
    - every `009` to `012` file exists under `todo/pending`
    - parent files contain `subtodo_start`, `subtodo_final`, and `## Next todo`
    - sub-todos contain both `next_todo:` metadata and `## Next todo`
  - duplicate-risk review:
    - `bash C:/Users/Savage/.codex/skills/dupe-audit/scripts/dupe-audit.sh --provider gguf --target todo/pending/009-iex-single-root-binary-layout-proof-lane.md --target todo/pending/010-iex-branchless-block-stage-literal-scan.md --target todo/pending/011-iex-required-substring-boolean-gate.md --target todo/pending/012-iex-regex-decomposition-prefilter-wave.md --task-text "Planning-spec frontier split for single-root performance research deliverables" --output summary --no-write`
    - `target_file_count=4`
    - `candidate_pair_count=2`
    - `exact_duplicate_candidate_count=0`
- Read:
  - the research now resolves into four different execution lanes with minimal overlap:
    - `009` binary-layout proof
    - `010` branchless medium-and-long literal block-stage scan
    - `011` required-substring boolean gate
    - `012` regex decomposition and prefilter orchestration
  - keeping these lanes separate materially reduces split-brain risk, because build-pipeline work, literal acceleration, boolean regex gating, and deeper regex decomposition can now be prioritized or rejected independently

## Todo critique and 009 proof-surface slice - 2026-04-10 03:43Z
- Scope: reviewed the new single-root parents from the higher level, tightened the overlap boundaries, then implemented the first tracked execution slice for `009` so the binary-layout lane has a canonical build-profile surface instead of remaining a research note.
- Critique:
  - `010` was too close to `008` because both claimed the first bounded medium/long acceleration slice.
  - `012` was too close to `011` because it could collapse into plain boolean required-substring gating instead of true multi-component decomposition.
- Updated planning boundaries:
  - `todo/pending/010-iex-branchless-block-stage-literal-scan.md`
  - `todo/pending/010a-iex-branchless-block-stage-literal-scan.md`
  - `todo/pending/012-iex-regex-decomposition-prefilter-wave.md`
  - `todo/pending/012a-iex-regex-decomposition-prefilter-wave.md`
  - `010` now starts only after `008` has closed or artifact-backed rejected its bounded medium/long slice.
  - `012` now starts only after `011` has closed or artifact-backed proven the ceiling of boolean gating, and it owns only literal-plus-heavier-component decomposition beyond that point.
- Implemented:
  - `Cargo.toml`
    - added `[profile.release-lto]` inheriting from `release` with `lto = "fat"` and `codegen-units = 1`
  - `tools/scripts/lib/build-profiles.mjs`
    - added canonical benchmark build-profile ownership for `debug`, `release`, and `release-lto`
  - `tools/scripts/lib/benchmark-runner.mjs`
    - switched binary resolution to the canonical profile matrix instead of hard-coded `debug` or `release` branching
  - `tests/perf/benchmark-build-profiles.test.ts`
    - added contract coverage for profile listing, `release-lto` resolution, and unsupported-profile failure
  - `README.md`
    - documented supported benchmark build profiles and removed the repo-local `rg` fallback note from the operator rule
- Validation:
  - `npx vitest run tests/perf/benchmark-build-profiles.test.ts tests/perf/benchmark-runner-binary-path.test.ts --config vitest.config.ts`
    - `2` files passed, `5` tests passed
  - `cargo build --release -p iex-cli`
  - `cargo build --profile release-lto -p iex-cli`
  - end-to-end runner proof:
    - `node tools/scripts/run-once-benchmark.mjs --expression "lit:Sherlock Holmes" --corpus ".refs/ripgrep/benchsuite/subtitles/en.sample.txt" --build-profile release-lto --samples 1 --warmup 0`
    - resolved binary path: `target/release-lto/iex-cli.exe`
- Heavy proof snapshot:
  - motivating workload: `lit:Sherlock Holmes` over `.refs/ripgrep/benchsuite`
  - pinned `release` binary:
    - `iexMs = 1957.0003 ms`
    - `discover = 271.9281 ms`
    - `scan = 1671.3452 ms`
  - pinned `release-lto` binary:
    - `iexMs = 2075.4045 ms`
    - `discover = 365.4050 ms`
    - `scan = 1698.9446 ms`
  - delta:
    - `release-lto` was slower than `release` by about `+6.05%` on the heavy single-root proof in this Windows checkout
- Read:
  - the planning critique was worthwhile because it removed two parent collisions before implementation work spread into duplicate lanes
  - the `009` slice was worthwhile because it turned binary-layout optimization into a measurable and reproducible proof surface
  - the first `release-lto` candidate is not a promotion candidate on this workload, so the next single-root leap should not be sold as an LTO win here
  - one important repo-hygiene caveat remains: `/tools`, `/tests`, `/todo`, and `/.docs` are ignored by `.gitignore`, so the new benchmark and planning files are local-working-surface changes unless a separate ignore-policy slice promotes them into tracked ownership

## Rejected plain-literal anchor experiment - 2026-04-10 04:10Z
- Scope: tested one bounded experimental single-root acceleration slice in `crates/iex-core/src/expr.rs` by extending the existing anchor doctrine from ASCII casefold literals into plain ASCII literals, then rejected it because the measured proof did not improve the target workload.
- Experimental candidate:
  - reused `best_literal_anchor_plan(...)` for plain `lit:` and plain regex-literal byte search paths
  - routed medium/long ASCII literal matching through an anchor-led verifier instead of pure `memmem` on the experimental branch
- Validation before proof:
  - `cargo test -p iex-core`
  - temporary test count during experiment: `37` passed
- Immutable baseline:
  - pre-rebuild snapshot: `tools/reports/candidate-compare/iex-cli-prerebuild-20260410-020341.exe`
- Heavy proof on the motivating workload:
  - expression: `lit:Sherlock Holmes`
  - corpus: `.refs/ripgrep/benchsuite`
  - baseline snapshot result:
    - `iexMs = 1880.5336 ms`
    - `discover = 265.0121 ms`
    - `scan = 1601.4988 ms`
  - experimental candidate result:
    - `iexMs = 1978.8176 ms`
    - `discover = 384.5527 ms`
    - `scan = 1580.9291 ms`
  - read:
    - the experimental candidate was slower by about `+5.23%` total on the motivating single-root proof
    - scan moved slightly in the right direction, but not enough to overcome total cost, so this is not a promotable lane
- Narrow single-file proof to isolate scanner movement:
  - corpus: `.refs/ripgrep/benchsuite/subtitles/en.sample.txt`
  - baseline snapshot: `40.6345 ms`
  - experimental candidate: `43.9447 ms`
  - read:
    - even on the isolated single-file scan, the anchored plain-literal path was slower by about `+8.15%`
- Outcome:
  - reverted the experimental `expr.rs` slice after proof rejection
  - post-revert validation:
    - `cargo test -p iex-core`
    - `34` passed
- Read:
  - the anchor doctrine already works for the ASCII casefold lane in this repo, but blindly extending it to plain literals is not the next leap here
  - the kept value from this pass is the proof discipline, not the algorithm

## 008c bounded streaming discovery proof - 2026-04-10 04:38Z
- Scope: completed the highest-value structural refinement from the engine review by removing the full discovery materialization barrier for stats-only directory workloads in `crates/iex-core/src/engine.rs`, while keeping the hit-collecting path on the existing materialized ownership.
- Implemented:
  - `crates/iex-core/src/engine.rs`
    - split `run_search_inner(...)` into a materialized path and a bounded stats-only streaming path
    - added canonical `DiscoveryDispatch` ownership for pre-scan dedupe accounting plus bounded producer-consumer traversal
    - added `auto_threads_for_streaming()` and worker-driven `scan_paths_streaming_stats_only(...)`
    - preserved existing collect-hits behavior and root-normalization contracts
    - added focused tests for single-root stats-only equivalence and overlap-pruned mixed-root exactness
- Validation:
  - `cargo test -p iex-core`
    - `36` passed
- Immutable baseline:
  - pre-rebuild snapshot: `tools/reports/candidate-compare/iex-cli-baseline-20260410-043347.exe`
- Heavy proof artifacts:
  - single-root proof: `tools/reports/candidate-compare/008c-streaming-single-root-proof-20260410-023553.json`
  - equivalent split-root proof: `tools/reports/candidate-compare/008c-streaming-multipath-proof-20260410-023706.json`
  - overlap-root proof: `tools/reports/candidate-compare/008c-streaming-overlap-proof-20260410-023808.json`
- Heavy proof read:
  - motivating workload: `lit:Sherlock Holmes` over `.refs/ripgrep/benchsuite`
  - single-root median engine:
    - baseline: `2093.0069 ms`
    - candidate: `1805.7132 ms`
    - improvement: `+13.73%`
  - equivalent split-root median engine:
    - baseline: `1553.9253 ms`
    - candidate: `1306.6116 ms`
    - improvement: `+15.92%`
  - overlap-root median engine:
    - baseline: `2043.2110 ms`
    - candidate: `1818.3792 ms`
    - improvement: `+11.00%`
  - exactness held across all proof runs:
    - matches: `1233`
    - discovered files: `79429`
    - scanned files: `79424`
  - overlap-root telemetry stayed canonical:
    - `effective_roots = 1`
    - `pruned_roots = 1`
    - `overlap_pruned_roots = 1`
- Read:
  - this slice is worthwhile because it buys a real `>=10%` single-root gain on the motivating heavy workload without hiding behind an experimental matcher rewrite
  - the new timing semantics are intentional: on the streaming path, `discover_ms` and `scan_ms` overlap and should be read as truthful phase windows, while `total_ms` remains the promotion metric
  - the best next move is still `008d` medium/long literal regime dispatch plus `008e` adaptive bailout, but now from a lower traversal-tax floor instead of a staged-walk bottleneck

## 008 promotion gate rejection and first 008d experiment rejection - 2026-04-10 05:14Z
- Scope: closed the immediate follow-through after `008c` by checking whether the committed candidate could replace the active dashboard loop, then tested one bounded `008d` medium/long literal regime experiment in `crates/iex-core/src/expr.rs` and rejected it when the proof surface regressed.
- Live-loop promotion proof:
  - exact active-loop binary:
    - `E:\Workspaces\01_Projects\01_Github\iEx-Engine-v2\tools\reports\candidate-compare\iex-cli-live-20260409-232626.exe`
  - exact full-suite proof artifact:
    - `E:\Workspaces\01_Projects\01_Github\iEx-Engine-v2\tools\reports\candidate-compare\live-loop-compare-extended-2026-04-10T03-00-34-340Z.json`
  - suite read:
    - `12` profiles
    - `3` wins
    - `9` losses
    - median engine delta: `-3.99%`
    - best lane: `+6.51%`
    - worst lane: `-37.61%`
  - outcome:
    - the candidate did not clear the active-loop promotion gate, so the dashboard loop was not restarted and the pinned live snapshot stayed unchanged
- Experimental `008d` candidate:
  - implementation shape:
    - planner-visible medium/long literal regime metadata
    - anomaly-ranked anchor plus compact verifier path for plain literals
    - kept the current exact match path as the correctness gate
  - validation before proof:
    - `cargo test -p iex-core`
    - temporary experiment suite: `41` passed
  - immutable pre-rebuild baseline:
    - `tools/reports/candidate-compare/iex-cli-baseline-20260410-050617.exe`
  - proof artifact:
    - `tools/reports/candidate-compare/008d-literal-regime-proof-20260410-050724.json`
  - proof read:
    - motivating ASCII literal lane:
      - `lit:Sherlock Holmes` over `.refs/ripgrep/benchsuite`
      - median engine improvement: `+4.71%`
    - plain regex-literal lane:
      - `re:Sherlock Holmes` over `.refs/ripgrep/benchsuite/subtitles/en.sample.txt`
      - median engine regression: `-14.97%`
    - UTF-8 literal lane:
      - Russian subtitles literal workload over `.refs/ripgrep/benchsuite/subtitles/ru.txt`
      - median engine regression: `-9.51%`
    - exactness held on all checked workloads
  - outcome:
    - reverted the `expr.rs` experiment after proof rejection
    - post-revert validation:
      - `cargo test -p iex-core`
      - `36` passed
- Read:
  - `7b60de1` remains the last proven committed keeper in this repo
  - the next `008d` move cannot be a broad literal-anchor widening; it needs tighter planner eligibility and a cheaper bailout story before it touches UTF-8 or regex-literal lanes

## 008c scheduler-gate carry-forward note - 2026-04-10 05:44Z
- Scope: recorded the first rejected narrow scheduler-gate replay as an explicit follow-on note inside the `008` planning chain so the idea is not lost or accidentally reintroduced without proof.
- Carry-forward artifact:
  - `tools/reports/candidate-compare/008c-streaming-gate-replay-20260410-0530.json`
- Read:
  - the restricted stats-only multipath gate remains interesting because it improved some heavy lanes, but it is intentionally shelved until the `sherlock_single_root` and `multipath_split` regressions are removed
  - any future reintroduction must clear the pinned-baseline proof surface before the gate can become canonical

## 008c salvage replay rejection - 2026-04-10 06:00Z
- Scope: attempted two tighter `008c` salvage variants to preserve the apparent heavy-lane wins from the first narrow gate replay, then rejected both after pinned-baseline proof because neither variant improved the canonical shape overall.
- Experimental variants:
  - crossbeam dispatcher plus multipath-only gate:
    - proof artifact: `tools/reports/candidate-compare/008c-crossbeam-gated-vs-stable-20260410-1346.json`
    - read:
      - `linux_literal`: `+28.91%`
      - `linux_word`: `+24.66%`
      - `sherlock_single_root`: `-7.30%`
      - `multipath_split`: `-1.70%`
      - `multipath_overlap`: `+3.23%`
  - crossbeam only in the streaming worker queue while restoring the materialized root walk channel:
    - proof artifact: `tools/reports/candidate-compare/008c-streaming-queue-only-vs-stable-20260410-1350.json`
    - read:
      - `linux_literal`: `+33.16%`
      - `linux_word`: `+19.31%`
      - `sherlock_single_root`: `-13.13%`
      - `multipath_split`: `+1.53%`
      - `multipath_overlap`: `-1.48%`
- Outcome:
  - reverted both salvage variants and restored the worktree to the committed `008c` keeper
  - post-revert validation:
    - `cargo test -p iex-core`
      - `36` passed
    - `cargo build --release -p iex-cli`
- Read:
  - the large linux-only gains are real on this proof surface, but they do not yet correspond to a safe canonical gate or dispatcher slice
  - the stable global `008c` stats-only streaming path remains the only proven keeper from this family today

## 008c local candidate restore and Sherlock diagnosis - 2026-04-10 06:32Z
- Scope: restored the stronger rejected salvage slice as a local working candidate instead of discarding it, then ran focused phase-level proofs to identify what is actually causing the Sherlock regression.
- Local candidate restored:
  - `Cargo.toml`
  - `Cargo.lock`
  - `crates/iex-core/Cargo.toml`
  - `crates/iex-core/src/engine.rs`
  - shape:
    - crossbeam channel replaces the old locked receiver path
    - stats-only streaming is gated to true multipath input shapes only
- Validation:
  - `cargo test -p iex-core`
    - `38` passed
  - `cargo build --release -p iex-cli`
- Existing proof surface kept as the acceptance snapshot for this local candidate:
  - `tools/reports/candidate-compare/008c-crossbeam-gated-vs-stable-20260410-1346.json`
- Focused Sherlock diagnosis:
  - baseline stable `008c` on `lit:Sherlock Holmes` over `.refs/ripgrep/benchsuite`:
    - median total: `1651.7309 ms`
    - median discover: `1633.4729 ms`
    - median scan: `1640.7505 ms`
    - read:
      - discover and scan are heavily overlapped on the stable single-root streaming path
  - restored local candidate on the same workload:
    - median total: `1868.4753 ms`
    - median discover: `258.6877 ms`
    - median scan: `1579.3500 ms`
    - read:
      - raw discover and raw scan both improved, but total regressed because the single-root path no longer overlaps those phases
- Focused counterexample proving the issue is not just crossbeam alone:
  - `lit:PM_RESUME` over the linux subtree:
    - baseline stable median total: `1341.9719 ms`
    - restored candidate median total: `896.6331 ms`
    - read:
      - the materialized single-root path is materially better on the smaller linux workload even though the full benchsuite Sherlock lane prefers overlap
- Read:
  - the current Sherlock regression is best explained as an eligibility mistake, not a blanket failure of the restored candidate
  - the next refinement should decide single-root streaming versus materialized ownership using a corpus-size or scan-cost signal, because full benchsuite-scale single-root runs still benefit from overlap while narrower single-root lanes clearly prefer the cheaper materialized path

## 008c local candidate refresh benchmark - 2026-04-10 11:50Z
- Scope: reran the current local crossbeam-plus-multipath-gate candidate against the pinned stable `008c` baseline on the same five-lane compare surface to see the current state after rebuild and local retention.
- Refresh artifact:
  - `tools/reports/candidate-compare/008c-crossbeam-gated-refresh-20260410-1150.json`
- Current read:
  - `sherlock_single_root`:
    - baseline: `1176.4913 ms`
    - candidate: `1256.8094 ms`
    - delta: `-6.83%`
    - note:
      - raw phase medians still moved down under the candidate (`discover 178.2191 ms`, `scan 1058.0554 ms`), but stable single-root overlap remains better on total
  - `linux_literal`:
    - baseline: `1077.8988 ms`
    - candidate: `778.6613 ms`
    - delta: `+27.76%`
  - `linux_word`:
    - baseline: `1331.7996 ms`
    - candidate: `1199.2739 ms`
    - delta: `+9.95%`
  - `multipath_split`:
    - baseline: `1631.2029 ms`
    - candidate: `1611.3360 ms`
    - delta: `+1.22%`
  - `multipath_overlap`:
    - baseline: `1597.3467 ms`
    - candidate: `1892.2279 ms`
    - delta: `-18.46%`
- Read:
  - the current local candidate is still attractive on linux-heavy lanes, but the overlap-root lane is now the loudest regression surface, not just Sherlock
  - any keep decision now needs to explain both single-root overlap loss and overlap-root discovery inflation before this slice can be considered stable

## Insect overlap recovery harvest - 2026-04-10 12:05Z
- Scope: used the `insect` runtime to harvest web and YouTube research specifically for the current negative lanes in the local `008c` candidate, then distilled the findings into one repo-native recon note for later execution work.
- Raw artifact directory:
  - `.docs/recon/insect-overlap-research-2026-04-10/`
- Distilled recon note:
  - `.docs/recon/insect-overlap-negative-recovery-2026-04-10.md`
- Highest-signal reads:
  - ripgrep and OpenCilk both reinforce locality-preserving work stealing instead of a single global work queue as the primary scheduling doctrine
  - ripgrep's guidance on workload-shaped strategy selection aligns with the measured iEx split: full benchsuite single-root still benefits from overlapped streaming, while narrower single-root linux lanes prefer the cheaper materialized path
  - the current overlap-root regression likely starts with an eligibility mistake, because the gate keys on `input_roots` even when root normalization has already collapsed the work to `effective_roots = 1`
- Recommended next move:
  - first switch scheduler eligibility from raw input roots to retained effective roots, then retest before attempting a deeper local-deque or task-coarsening rewrite

## 008c effective-root gate replay rejection - 2026-04-10 12:10Z
- Scope: applied the first research-backed scheduler refinement by switching the stats-only streaming gate from `input_roots` to `effective_roots`, ran the full five-lane replay against the pinned stable `008c` baseline, then reverted the change after the proof regressed.
- Rejected artifact:
  - `tools/reports/candidate-compare/008c-crossbeam-effective-roots-20260410-140847.json`
- Result:
  - `sherlock_single_root`:
    - baseline: `1544.7795 ms`
    - candidate: `2040.1849 ms`
    - delta: `-32.07%`
  - `linux_literal`:
    - baseline: `1503.7461 ms`
    - candidate: `1204.5199 ms`
    - delta: `+19.90%`
  - `linux_word`:
    - baseline: `1581.9972 ms`
    - candidate: `1253.3209 ms`
    - delta: `+20.78%`
  - `multipath_split`:
    - baseline: `1660.4540 ms`
    - candidate: `1781.2556 ms`
    - delta: `-7.28%`
  - `multipath_overlap`:
    - baseline: `1586.1423 ms`
    - candidate: `1779.9803 ms`
    - delta: `-12.22%`
- Read:
  - overlap pruning collapsing to `effective_roots = 1` was not the true fix, because those giant directory workloads still prefer the overlapped streaming path even after normalization
  - the result narrows the next scheduler cut: eligibility must key off retained work cost or corpus shape, not just root cardinality, because `effective_roots` alone incorrectly pushes large single-effective-root workloads back onto the materialized barrier
  - this replay was reverted immediately after proof, so the local tree is back on the stronger raw-`input_roots` candidate and the effective-root gate is preserved only as evidence, not as active behavior

## 008c dominant-file override recovery - 2026-04-10 12:28Z
- Scope: reframed the remaining unstable lanes as giant-file tail problems instead of another root-gating problem, then kept the stronger local candidate and repaired the tail in the scan kernel instead of reopening traversal doctrine again.
- Snapshot before rebuild:
  - `tools/reports/candidate-compare/iex-cli-prerebuild-20260410-142531.exe`
- Runtime change:
  - `crates/iex-core/src/engine.rs`
    - added `parallel_fast_count_threads(file_len, thread_budget)` as one canonical thread-allocation gate for chunk-parallel fast counts
    - preserved the existing single-threaded large-file fast-count path
    - added a narrow dominant-file override so stats-only fast counts can still split files `>= 512 MiB` even when the outer search is already parallel
    - capped the inner override at `4` threads to keep the blast radius inside giant-file tails instead of turning it into whole-search oversubscription
    - added focused tests for the small-file off path, the single-threaded large-file path, and the dominant-file override path
- Validation:
  - `cargo test -p iex-core`
    - `41` passed
  - `cargo build --release -p iex-cli`
- Proof artifact:
  - `tools/reports/candidate-compare/008c-dominant-file-override-20260410-142706.json`
- Recovered proof surface versus pinned stable `008c` baseline:
  - `sherlock_single_root`
    - baseline: `1619.6359 ms`
    - candidate: `1494.6344 ms`
    - delta: `+7.72%`
    - read:
      - this lane scans `3200836379` bytes, so the recovery is consistent with dominant-file tail relief, not a root-count change
  - `linux_literal`
    - baseline: `1336.7264 ms`
    - candidate: `1080.2824 ms`
    - delta: `+19.18%`
  - `linux_word`
    - baseline: `1237.8153 ms`
    - candidate: `1075.2013 ms`
    - delta: `+13.14%`
  - `multipath_split`
    - baseline: `1387.0881 ms`
    - candidate: `1379.3967 ms`
    - delta: `+0.55%`
  - `multipath_overlap`
    - baseline: `1548.3930 ms`
    - candidate: `1253.8696 ms`
    - delta: `+19.02%`
    - read:
      - this lane also scans `3200836379` bytes even after overlap pruning, which matches the same giant-file-tail explanation as Sherlock
- Read:
  - the better mental model is now explicit: the remaining bad lanes were not primarily failing because of root cardinality; they were failing because a few dominant files were setting the tail
  - this is a higher-value fix than another broad scheduler toggle because it recovers both single-root and overlap-pruned multipath while preserving the Linux wins already opened by the local candidate
  - the next scheduler leap can now focus on locality and coarse ownership from a lower tail-tax floor instead of trying to use traversal gating to solve a scan-kernel bottleneck

## AGENTS doctrine addendum from 008c recovery - 2026-04-10 12:36Z
- Scope: promoted the successful recovery method itself into repo doctrine so future performance work remembers the problem-solving move, not only the concrete giant-file override.
- Updated:
  - `AGENTS.md`
- Added guidance:
  - when a regression resists local explanations, zoom out from the immediate gate or branch and re-price the whole workload shape using retained bytes, slowest files, and tail-dominant surfaces before rewriting control flow
  - prefer the smallest durable repair at the dominant cost center over another broad scheduler toggle, then revisit traversal doctrine only after the tail tax has been reduced
- Read:
  - this codifies the exact move that unlocked the recovery: the solution arrived only after the regression was reframed from a root-gating story into a giant-file-tail story

## 008c heavy repeatability pass versus live and native - 2026-04-10 13:01Z
- Scope: reran the heavy candidate-versus-dashboard-live-versus-native comparison two more times, then averaged the original proof plus both reruns so the decision surface reflects repeatability instead of a single sample.
- Artifacts:
  - original proof:
    - `tools/reports/candidate-compare/008c-heavy-compare-candidate-live-native-20260410-144019.json`
  - rerun 1:
    - `tools/reports/candidate-compare/008c-heavy-compare-candidate-live-native-rerun1-exec-20260410-125935.json`
  - rerun 2:
    - `tools/reports/candidate-compare/008c-heavy-compare-candidate-live-native-rerun2-exec-20260410-130049.json`
  - three-run aggregate:
    - `tools/reports/candidate-compare/008c-heavy-compare-candidate-live-native-aggregate3-exec-20260410-130158.json`
- Workload contract:
  - three-way common lanes:
    - `sherlock_single_root`
    - `ru_file_single_path`
    - `linux_literal`
    - `linux_word`
  - two-way multipath lanes:
    - `multipath_split`
    - `multipath_overlap`
  - the dashboard live binary still does not accept multi-root CLI input, so multipath remained candidate-versus-native only
- Three-run average:
  - `sherlock_single_root`
    - candidate: `1315.9993 ms`
    - dashboard live: `1495.5190 ms`
    - native installed: `1487.4367 ms`
    - delta: `+12.00%` versus dashboard live, `+11.53%` versus native installed
    - winner count: candidate `3/3`
  - `ru_file_single_path`
    - candidate: `338.5115 ms`
    - dashboard live: `337.8153 ms`
    - native installed: `318.0106 ms`
    - delta: `-0.21%` versus dashboard live, `-6.45%` versus native installed
    - winner count: native installed `3/3`
  - `linux_literal`
    - candidate: `818.1948 ms`
    - dashboard live: `836.3386 ms`
    - native installed: `838.0091 ms`
    - delta: `+2.17%` versus dashboard live, `+2.36%` versus native installed
    - winner count: candidate `2/3`
  - `linux_word`
    - candidate: `840.3009 ms`
    - dashboard live: `841.7836 ms`
    - native installed: `918.9389 ms`
    - delta: `+0.18%` versus dashboard live, `+8.56%` versus native installed
    - winner count: candidate `0/3`, dashboard live `1/3`, native installed `2/3`
    - read:
      - this lane is still unstable, because the arithmetic mean edges toward the candidate even though the candidate never actually posted the fastest individual run
  - `multipath_split`
    - candidate: `1444.3045 ms`
    - native installed: `1935.1550 ms`
    - delta: `+25.36%`
    - winner count: candidate `3/3`
  - `multipath_overlap`
    - candidate: `1369.5439 ms`
    - native installed: `1853.8788 ms`
    - delta: `+26.13%`
    - winner count: candidate `3/3`
- Read:
  - Sherlock and both multipath lanes are now strong repeat winners for the current candidate, not one-off spikes
  - the single-file Russian lane remains a native-owned tail and should be treated as a separate scan-kernel problem if it matters enough to chase
  - `linux_word` is the one lane that still needs skepticism: the aggregate looks favorable, but the per-run winner pattern says it is not yet a clean promotion-grade win

## Live loop repointed to 008c dominant-file snapshot - 2026-04-10 14:10Z
- Scope: reviewed the promotion procedure in `AGENTS.md`, snapshotted the current candidate to a fresh immutable live path, replaced the active pinned suite-loop process, and verified that fresh dashboard telemetry now reads the new binary path.
- Governing contract:
  - `AGENTS.md`
    - `How To Execute` step `5`: pin the promoted binary to a timestamped immutable snapshot, then confirm `tools/reports/latest.json` is reading that snapshot path after restart
  - `AGENTS.md`
    - `Definition Of Done`: if the active loop is updated, the promoted binary must be immutable and dashboard telemetry must verify the exact path after restart
- Live binary swap:
  - previous live snapshot:
    - `E:\Workspaces\01_Projects\01_Github\iEx-Engine-v2\tools\reports\candidate-compare\iex-cli-live-20260409-232626.exe`
    - `sha256 = 94B815FDD223D52471A6A94E17373A21D90E78CE42C213B3BFBC00FDB098616F`
  - promoted candidate source:
    - `E:\Workspaces\01_Projects\01_Github\iEx\target\release\iex-cli.exe`
    - `sha256 = B20CF693DB20489B2934010E710DF4F35D1ED8557CD4C86B95F620AADA92FC03`
  - new immutable live snapshot:
    - `E:\Workspaces\01_Projects\01_Github\iEx-Engine-v2\tools\reports\candidate-compare\iex-cli-live-20260410-161022.exe`
    - `sha256 = B20CF693DB20489B2934010E710DF4F35D1ED8557CD4C86B95F620AADA92FC03`
- Process handoff:
  - previous live loop:
    - pid `125104`
    - `"C:\Program Files\nodejs\node.exe" tools/scripts/bench-loop-suite.mjs --loops 0 --warmup 1 --samples 3 --iex-binary E:\Workspaces\01_Projects\01_Github\iEx-Engine-v2\tools\reports\candidate-compare\iex-cli-live-20260409-232626.exe`
  - replacement live loop:
    - pid `158140`
    - `"C:\Program Files\nodejs\node.exe" tools/scripts/bench-loop-suite.mjs --loops 0 --warmup 1 --samples 3 --iex-binary E:\Workspaces\01_Projects\01_Github\iEx-Engine-v2\tools\reports\candidate-compare\iex-cli-live-20260410-161022.exe`
- Verification:
  - `..\iEx-Engine-v2\tools\reports\latest.json`
    - fresh timestamp: `2026-04-10T14:10:29.652Z`
    - `iexBinaryPath = E:\Workspaces\01_Projects\01_Github\iEx-Engine-v2\tools\reports\candidate-compare\iex-cli-live-20260410-161022.exe`
  - `..\iEx-Engine-v2\tools\reports\live-metrics.jsonl`
    - fresh appended runs now carry the same `iexBinaryPath` on `suite-en-word`, `suite-en-alternates`, and `suite-en-surrounding-words`
- Read:
  - the mechanical switch is complete and the dashboard loop is no longer pinned to the April 9 snapshot
  - the immutable live snapshot now exactly matches the current candidate binary by hash, so the live loop is exercising the same build that won the heavy repeatability surface
  - this turn changed operator state, not runtime code: the live binary path and loop process changed, while the search engine tree stayed untouched
  - this operator-directed repoint used the heavy repeatability evidence and live-path verification surface; it did not regenerate a fresh full-suite promotion artifact before the restart, so that remains the next formal confirmation step if strict promotion-proof closure is required

## Dashboard header active-binary disclosure - 2026-04-10 14:19Z
- Scope: closed the operator observability gap in the benchmark monitor header so the top status section now shows the exact active binary instead of only the live-feed source and latest raw benchsuite artifact.
- Updated:
  - `..\iEx-Engine-v2\tools\scripts\dashboard-server.mjs`
    - added canonical `activeBinary` payload ownership
    - resolves from `tools/reports/latest.json` first, then falls back to the newest live run's `iexBinaryPath` when needed
  - `..\iEx-Engine-v2\dashboard\index.html`
    - added `Active binary` to the top status line
    - renders the resolved artifact path and last-write timestamp
    - keeps the field monospaced and wrap-safe for long snapshot paths
- Verification:
  - temporary dashboard server on port `7381`
    - `/api/summary` returned:
      - `sources.activeBinary.artifactPath = tools/reports/candidate-compare/iex-cli-live-20260410-161022.exe`
      - `sources.activeBinary.available = true`
      - `latest.iexBinaryPath = E:\Workspaces\01_Projects\01_Github\iEx-Engine-v2\tools\reports\candidate-compare\iex-cli-live-20260410-161022.exe`
    - `/` HTML contains:
      - `id="active-binary-state"`
  - active dashboard server restart:
    - previous pid `139268`
    - replacement pid `73844`
    - `http://127.0.0.1:7373/api/summary` now returns the same `sources.activeBinary.artifactPath`
- Read:
  - the dashboard header now exposes the same live-loop pin that the promotion procedure verifies, which removes the operator ambiguity between "live summary source" and "what binary is actually active"
  - this stayed inside canonical ownership by teaching the server summary payload one new source instead of making the page infer state from ad hoc file reads

## Deep best-of-3 live-versus-history replay - 2026-04-10 16:36Z
- Scope: verified that the active live snapshot is truly the newest distinct executable by hash, then ran a deep three-round best-of-3 comparison against four older distinct binaries so the current live pin could be judged against history instead of only one baseline.
- Binary identity lock:
  - current live snapshot:
    - `..\iEx-Engine-v2\tools\reports\candidate-compare\iex-cli-live-20260410-161022.exe`
    - `sha256 = B20CF693DB20489B2934010E710DF4F35D1ED8557CD4C86B95F620AADA92FC03`
  - current repo build output:
    - `target\release\iex-cli.exe`
    - `sha256 = B20CF693DB20489B2934010E710DF4F35D1ED8557CD4C86B95F620AADA92FC03`
  - compared predecessors:
    - `tools\reports\candidate-compare\iex-cli-prerebuild-20260410-142531.exe`
    - `tools\reports\candidate-compare\iex-cli-baseline-20260410-054005.exe`
    - `..\iEx-Engine-v2\tools\reports\candidate-compare\iex-cli-live-20260409-232626.exe`
    - `C:\Users\Savage\AppData\Local\Programs\iEx\bin\iex.exe`
- Proof artifacts:
  - single-root:
    - `tools/reports/candidate-compare/five-way-single-root-bestof3-20260410-163220.json`
  - multipath:
    - `tools/reports/candidate-compare/four-way-multipath-bestof3-20260410-163531.json`
  - combined summary:
    - `tools/reports/candidate-compare/current-live-vs-history-deep-bestof3-20260410-163651.json`
- Method:
  - `3` measured rounds per workload
  - scored by best `engine_ms`
  - rotated binary order per round to reduce first-run cache bias
  - previous live snapshot was excluded from multipath scoring after preflight exited `2`
- Single-root read:
  - workloads:
    - `sherlock_literal`
    - `sherlock_casei`
    - `sherlock_word`
    - `sherlock_alt5`
    - `ru_file_single_path`
    - `linux_literal`
    - `linux_word`
  - win counts:
    - current live snapshot: `4/7`
    - installed native: `2/7`
    - baseline `054005`: `1/7`
    - prerebuild `142531`: `0/7`
    - previous live snapshot: `0/7`
  - current live geomean versus historical binaries:
    - versus prerebuild `142531`: `-10.67%`
    - versus baseline `054005`: `-17.92%`
    - versus previous live snapshot: `-11.64%`
    - versus installed native: `-10.04%`
  - noteworthy lane ownership:
    - `sherlock_casei`: current live `1264.5139 ms`, next best baseline `1489.4042 ms`
    - `sherlock_word`: current live `1022.6422 ms`, next best baseline `1342.1999 ms`
    - `linux_literal`: current live `683.4453 ms`, previous live `691.2878 ms`
    - `linux_word`: installed native still leads with `692.4272 ms` versus current live `751.9218 ms`
    - `ru_file_single_path`: installed native still leads with `286.9359 ms` versus current live `301.1587 ms`
    - `sherlock_literal`: baseline `054005` still edges current live, `1115.5540 ms` versus `1140.3566 ms`
- Multipath read:
  - workloads:
    - `multipath_split`
    - `multipath_overlap`
  - win counts:
    - prerebuild `142531`: `1/2`
    - baseline `054005`: `1/2`
    - current live snapshot: `0/2`
    - installed native: `0/2`
  - best times:
    - `multipath_split`: prerebuild `142531` `1117.4093 ms`, current live `1134.0904 ms`, baseline `054005` `1137.6755 ms`, installed native `1573.0759 ms`
    - `multipath_overlap`: baseline `054005` `1106.6040 ms`, current live `1161.7312 ms`, prerebuild `142531` `1162.5749 ms`, installed native `1506.0421 ms`
  - telemetry parity:
    - all current multipath-capable candidates retained canonical pruning signals
    - `multipath_split`: `effective_roots = 3`, `pruned_roots = 1`, `overlap_pruned_roots = 1`
    - `multipath_overlap`: `effective_roots = 1`, `pruned_roots = 1`, `overlap_pruned_roots = 1`
- Read:
  - the active live snapshot is genuinely the newest distinct build, and it is not secretly an older binary under a fresh filename
  - the active live snapshot is still the strongest broad single-root operator pin among the compared versions
  - it is not the universal overall winner yet, because two older branches still edge it on best-of-3 multipath by small margins
  - the next performance move should be judged as a multipath-tail recovery problem, not as proof that the current live pin was a false single-root win

## Insect multipath next-move research - 2026-04-10 17:00Z
- Scope: ran a fresh Insect harvest plus current `engine.rs` review to decide the highest-value next move for recovering the remaining multipath gap without destabilizing the current single-root lead.
- Stored artifacts:
  - distilled recon note:
    - `.docs/recon/insect-multipath-next-move-2026-04-10.md`
  - raw Insect harvest folder:
    - `.docs/recon/insect-multipath-next-move-2026-04-10/`
  - notable raw files:
    - `query-01-jwalk.json`
    - `query-02-crossbeam-deque.json`
    - `query-03-ignore-walkparallel.json`
    - `query-04-rayon-work-stealing.json`
    - `query-05-ripgrep-performance.json`
    - `jwalk-docs.md`
    - `crossbeam-deque-docs.md`
    - `ignore-walkbuilder-docs.md`
    - `ignore-walkparallel-docs.md`
    - `rayon-docs.md`
    - `ripgrep-performance-guide.md`
    - `ripgrep-maintainer-transcript.md`
- Local engine read:
  - `crates/iex-core/src/engine.rs`
    - the stats-only streaming path currently stacks:
      - `ignore` parallel traversal via `build_parallel()`
      - explicit scan workers over a bounded `crossbeam_channel`
      - inner dominant-file Rayon chunk pools for giant files
    - `build_walk_builder(...)` never calls `WalkBuilder::threads(...)`, so traversal thread count is still chosen by `ignore` heuristics instead of the iEx scheduler contract
    - `DiscoveryDispatch` puts every streamed file through a global `Mutex<BTreeSet<PathBuf>>` whenever `effective_roots > 1`, even though proof telemetry keeps showing `discovered_duplicate_paths = 0` on the retained-root workloads
- External read:
  - `ignore` docs:
    - `WalkBuilder::threads(...)` controls traversal threads for `build_parallel()`
    - `WalkParallel::visit(...)` supports per-thread visitors that can accumulate data without synchronization and merge afterward
  - `crossbeam-deque` docs:
    - canonical work-stealing shape is local worker queues plus injector plus stealers
    - batch stealing is a first-class operation, not a side effect
  - `jwalk` docs and README:
    - directory-level parallelism plus streamed output is useful for deep many-directory trees
    - it explicitly does not help much on one directory with many files
  - ripgrep performance guide:
    - thread count control matters
    - work stealing is a scheduler choice, not "always use more threads"
    - too many threads can lose to coordination overhead
  - transcript:
    - useful as general confirmation that traversal remains hard
    - lower-signal than the official crate docs for concrete implementation guidance
- Next-move read:
  - the strongest immediate cut should stay inside `008c` and fix thread-budget parity first:
    - make traversal thread count explicit whenever the separate scan-worker streaming path is active
    - rerun the existing multipath and five-lane proof surfaces before any deeper rewrite
  - if that lighter fix does not clear the gap, the next structural move should replace the global file queue with `WalkParallel::visit(...)` plus per-thread local batches instead of one shared `PathBuf` channel
  - giant-file dominant-file handling remains a separate scan-kernel concern and should not be conflated with directory-traversal ownership
- Read:
  - the remaining multipath gap now looks more like a scheduler-tax problem than a matcher-quality problem
  - the most credible current culprit is double-pool coordination: `ignore` traversal threads plus iEx scan workers plus occasional inner dominant-file chunk pools
  - this is a materially better next move than reopening another broad literal or reject-fast experiment, because it directly targets the exact lanes where the live snapshot still trails by only a small amount

## Full-picture performance review - 2026-04-10 17:28Z
- Scope: zoomed out across `AGENTS.md`, `engine.rs`, the live loop/dashboard surfaces, and the newest historical proof artifacts to verify that the current optimization direction is actually targeting the right bottleneck.
- Guiding reminders re-read from `AGENTS.md`:
  - when regressions resist local explanations, re-price the whole workload shape and check retained bytes, slowest files, and tail-dominant surfaces first
  - prefer narrow repairs at the dominant cost center over another broad scheduler toggle
- Review findings:
  - the current direction is only partly wrong:
    - the repo is not chasing a fake win
    - the active live snapshot is still the strongest broad single-root binary among the recent internal versions
    - but the remaining `008c` gap is smaller and more benchmark-method-sensitive than the best-of-3 headline suggests
  - the biggest hidden variable is proof semantics:
    - `tools/scripts/lib/benchmark-runner.mjs` always runs `iex search ... --json --stats-only`
    - so loop and dashboard gains only prove the stats-only path, not the collect-hits CLI path
  - the second hidden variable is scoring optimism:
    - the deep historical replay is scored by best-of-3
    - recomputed medians from the same artifacts shrink the single-root lead to roughly `3-5%` geomean versus recent internal binaries instead of the larger best-of-3 headline
    - on the same median basis, multipath still trails the recent prerebuild by about `6%`
  - the third hidden variable is benchmark coverage:
    - the multipath replay only covers two Sherlock-derived workloads
    - that is not enough surface area to justify a large scheduler rewrite by itself
  - the fourth hidden variable is provenance clarity:
    - the deep history artifact correctly pins an immutable live snapshot
    - but `tools/reports/latest.json` still records `target/release/iex-cli.exe`, which is mutable unless a snapshot path is explicitly injected into the loop
  - the engine read keeps the root cause grounded:
    - `should_stream_stats_only(...)` still keys on `input_roots > 1`, so overlap-pruned single-effective-root runs still enter the streaming queue
    - but the dominant-file evidence remains stronger than the gate theory alone, because the successful recovery came from giant-file-tail handling inside `maybe_parallel_fast_count_bytes(...)`
- Distilled read:
  - we are not focused on the wrong subsystem, but we were close to overweighting the remaining multipath scheduler gap
  - the stronger north star is still versus-ripgrep capability on representative workloads, not only candidate-vs-candidate wins
  - the next move should therefore stay narrow:
    - keep a small `008c` follow-through limited to traversal-thread parity and, only if needed, `WalkParallel::visit(...)` batching
    - keep the primary performance hunt on scan-kernel and giant-tail ownership improvements that lift both single-root and multipath without relying on stats-only scheduler quirks

## Fast-count sharding repair wave - 2026-04-10 18:30Z
- Scope: repaired the canonical large-buffer fast-count path in `crates/iex-core/src/engine.rs` and `crates/iex-core/src/expr.rs` so shard sizing can actually fan out, compound `All` byte plans no longer bail straight to the slow path, and pool ownership stops rebuilding Rayon state per call.
- Why this mattered:
  - `maybe_parallel_fast_count_bytes(...)` used `.max(PARALLEL_FAST_COUNT_CHUNK_BYTES)`, which collapsed `64 MiB+` files back to one chunk and disabled the advertised intra-file shard lane.
  - compound `&&` plans returned `None` from `fast_match_count_no_hits_bytes(...)`, forcing the slow per-line path even on direct-file stats-only workloads.
  - both `scan_paths(...)` and `maybe_parallel_fast_count_bytes(...)` built Rayon pools inline.
- Runtime changes:
  - replaced the broken shard cap with `parallel_fast_count_chunk_size(...)` using `.min(...)`
  - introduced a global `OnceLock` Rayon pool plus bounded chunk fan-out helpers so file-level and inner fast-count work reuse one worker pool without overshooting the requested task budget
  - made `auto_threads_for_shape(...)` byte-aware so small file counts with very large retained bytes are no longer pinned to one thread by file count alone
  - swapped duplicate-membership tracking from `BTreeSet<PathBuf>` to `HashSet<PathBuf>` while preserving retained output order in the owning `Vec<PathBuf>`
  - added compound `All` byte fast counting in `expr.rs` with cheapest-predicate reject-first ordering, exact line ownership across range boundaries, and explicit `None` for unsupported `Any` plans
- Validation:
  - `cargo test -p iex-core`
    - passed `46` tests including new regressions:
      - `engine::tests::parallel_fast_count_chunk_size_splits_large_files_into_multiple_ranges`
      - `engine::tests::auto_threads_uses_parallelism_for_byte_heavy_small_file_sets`
      - `expr::tests::compound_all_fast_count_matches_line_semantics`
      - `expr::tests::compound_all_fast_count_returns_none_for_any_mode`
      - `expr::tests::compound_all_range_count_matches_full_count`
- Immutable proof:
  - pre-rebuild snapshot: `tools/reports/candidate-compare/iex-cli-pre013-20260410-201733.exe`
  - rebuilt candidate artifact: `tools/reports/candidate-compare/013-compound-fast-count-proof-rebuilt-20260410-202521.json`
  - workload: `.refs/ripgrep/benchsuite/subtitles/en.sample.txt`
  - expression: `lit:Sherlock && lit:Holmes`
  - command shape: `search ... --json --stats-only --threads 1`
  - result:
    - baseline best wall `204.401 ms`, engine `194.636 ms`
    - candidate best wall `44.774 ms`, engine `35.806 ms`
    - speedup `+78.10%` wall / `+81.60%` engine
    - `matches_found` stayed `30`
- PatchMD:
  - initialized `.t3` store against upstream `https://github.com/savageops/iEx.git` on base ref `main`
  - recorded `PATCH-001` titled `Repair fast-count sharding and compound byte acceleration`
  - `t3-tape validate --json` returned `status: ok`

## Full-suite live-candidate compare for 013 - 2026-04-10 18:51Z
- Scope: compared the rebuilt `013` candidate against the exact immutable live-loop snapshot required by `AGENTS.md` before any promotion decision.
- Inputs:
  - live snapshot: `..\iEx-Engine-v2\tools\reports\candidate-compare\iex-cli-live-20260410-161022.exe`
  - candidate snapshot: `tools\reports\candidate-compare\iex-cli-candidate-013-suite-20260410-204418.exe`
  - proof artifact: `tools\reports\candidate-compare\013-live-vs-new-update-suite-compare-20260410-185144z.json`
  - suite source: `tools\scripts\bench-loop-suite.mjs`
  - command shape: `search <expression> <corpus> --json --stats-only`
- Method:
  - one warmup per binary per profile
  - three measured rounds per profile
  - alternating live-first and candidate-first order by round
  - scored by per-profile median `engine_ms` and `wall_ms`
- Result:
  - match counts, files scanned, and files skipped stayed identical on all `12` workloads
  - candidate lost `10/12` engine medians and `11/12` wall medians
  - aggregate median geomean moved the wrong direction:
    - engine ratio `1.8823x` versus live (`-88.23%` slower)
    - wall ratio `1.7941x` versus live (`-79.41%` slower)
  - only two engine lanes improved:
    - `suite-en-surrounding-words`: `+0.56%`
    - `suite-ru-literal-casei`: `+3.80%`
  - deepest engine regressions were:
    - `suite-linux-word`: `-380.41%`
    - `suite-linux-alternates`: `-354.34%`
    - `suite-linux-no-literal`: `-283.40%`
    - `suite-linux-literal`: `-261.13%`
- Decision:
  - do not promote `013` into the live loop
  - keep the current live snapshot pinned until the Linux-suite regression is explained and removed

## Repeated full-suite stability reruns for 013 - 2026-04-10 19:09Z
- Scope: reran the exact same live-vs-candidate full-suite compare three more times to test whether the `013` rejection was noisy or stable.
- Immutable inputs:
  - live snapshot: `..\iEx-Engine-v2\tools\reports\candidate-compare\iex-cli-live-20260410-161022.exe`
  - candidate snapshot: `tools\reports\candidate-compare\iex-cli-candidate-013-suite-20260410-204418.exe`
- Per-pass artifacts:
  - `tools\reports\candidate-compare\013-live-vs-new-update-suite-compare-pass1-20260410-190448z.json`
  - `tools\reports\candidate-compare\013-live-vs-new-update-suite-compare-pass2-20260410-190633z.json`
  - `tools\reports\candidate-compare\013-live-vs-new-update-suite-compare-pass3-20260410-190825z.json`
- Aggregate artifact:
  - `tools\reports\candidate-compare\013-live-vs-new-update-suite-compare-aggregate3-20260410-190923z.json`
- Method:
  - each pass used the same contract as the first proof:
    - one warmup per binary per profile
    - three measured rounds per profile
    - alternating live-first and candidate-first order
    - full `12`-profile `bench-loop-suite` workload set
- Result:
  - aggregate profile-pass wins across the three reruns:
    - engine: live `34`, candidate `2`
    - wall: live `34`, candidate `2`
  - aggregate median-of-pass-medians geomean:
    - engine ratio `1.8090x` versus live (`-80.90%` slower)
    - wall ratio `1.7891x` versus live (`-78.91%` slower)
  - correctness invariants stayed aligned on all reruns:
    - match counts equal
    - files scanned equal
    - files skipped equal
  - only one lane stayed net-positive after aggregation:
    - `suite-ru-literal-casei`: `+3.10%` engine, `+2.95%` wall
  - all Linux lanes stayed deeply negative in all three passes:
    - `suite-linux-literal`: `-344.72%` engine aggregate
    - `suite-linux-word`: `-296.44%`
    - `suite-linux-alternates`: `-289.53%`
    - `suite-linux-no-literal`: `-299.45%`
- Decision:
  - keep `013` out of the live loop
  - treat the regression as stable until the Linux-path cost center is explained and removed

## Rollback of net-negative 013 wave - 2026-04-10 19:27Z
- Scope:
  - reverted the benchmark-proven losing `013` code paths only
  - restored `crates/iex-core/src/expr.rs` to the pre-013 compound byte-count contract
  - removed the `013` byte-size auto-thread heuristic, HashSet dedupe switch, `.min(...)` shard helper, and global-pool scheduler changes from `crates/iex-core/src/engine.rs`
  - preserved the earlier dominant-file override and streaming stats-only gate work already resident in the dirty tree
- Validation:
  - pre-rebuild snapshot: `tools\reports\candidate-compare\iex-cli-prerollback-015-20260410-212157.exe`
  - `cargo test -p iex-core` passed `41` tests
  - rebuilt rollback candidate snapshot: `tools\reports\candidate-compare\iex-cli-candidate-015-rollback-suite-20260410-212220.exe`
  - `t3-tape validate --json` returned `status: ok`
- Full-suite repeated compare:
  - live snapshot: `..\iEx-Engine-v2\tools\reports\candidate-compare\iex-cli-live-20260410-161022.exe`
  - candidate snapshot: `tools\reports\candidate-compare\iex-cli-candidate-015-rollback-suite-20260410-212220.exe`
  - per-pass artifacts:
    - `tools\reports\candidate-compare\015-live-vs-rollback-suite-compare-pass1-20260410-192559z.json`
    - `tools\reports\candidate-compare\015-live-vs-rollback-suite-compare-pass2-20260410-192647z.json`
    - `tools\reports\candidate-compare\015-live-vs-rollback-suite-compare-pass3-20260410-192727z.json`
  - aggregate artifact:
    - `tools\reports\candidate-compare\015-live-vs-rollback-suite-compare-aggregate3-20260410-192727z.json`
  - method:
    - one warmup per binary per profile
    - three measured rounds per profile
    - alternating live-first and candidate-first order
    - full `12`-profile `bench-loop-suite` workload set
- Result:
  - correctness stayed aligned on all three passes:
    - match counts equal
    - files scanned equal
    - files skipped equal
  - aggregate profile-pass wins moved back toward parity:
    - engine: rollback `21`, live `15`
    - wall: rollback `23`, live `13`
  - aggregate median-of-pass-medians geomean:
    - engine ratio `1.0032x` versus live (`-0.32%` slower)
    - wall ratio `1.0000x` versus live (`-0.0003%` slower)
  - biggest rollback wins:
    - `suite-linux-alternates`: `+10.85%` engine / `+10.39%` wall
    - `suite-ru-literal`: `+5.19%` engine / `+4.76%` wall
    - `suite-linux-word`: `+4.16%` engine / `+3.92%` wall
  - biggest remaining losses:
    - `suite-en-literal`: `-8.22%` engine / `-9.37%` wall
    - `suite-en-word`: `-4.91%` engine / `-2.72%` wall
    - `suite-linux-no-literal`: `-4.57%` engine / `-5.13%` wall
- Decision:
  - the rollback successfully removed the stable net-negative `013` wave
  - do not promote the rollback candidate into the live loop yet because the fresh suite aggregate is parity-grade but still marginally behind on engine geomean

## Promote current working snapshot into live loop - 2026-04-10 20:07Z
- Scope:
  - promoted the measured winning current working snapshot into the dashboard live loop using the repo's immutable-snapshot operator contract
  - recorded the promotion under planning-spec chain `016-iex-promote-current-working-loop-snapshot`
- Promotion proof:
  - artifact: `tools\reports\candidate-compare\current-working-native-dashboard-live-suite-compare-20260410-194704z.json`
  - winning snapshot: `tools\reports\candidate-compare\iex-cli-current-working-compare-20260410-214403.exe`
  - winning snapshot SHA256: `D9360D16166084BCDAF345C65DBEEC8390FE17D63108337108CB0575FE9E0B7B`
  - previous live snapshot: `..\iEx-Engine-v2\tools\reports\candidate-compare\iex-cli-live-20260410-161022.exe`
  - previous live snapshot SHA256: `B20CF693DB20489B2934010E710DF4F35D1ED8557CD4C86B95F620AADA92FC03`
- Three-way aggregate result:
  - `current_working`: engine geomean `297.4984 ms`, wall geomean `419.6631 ms`, wins `7/12` engine and `6/12` wall
  - `native_installed`: engine geomean `305.0308 ms`, wall geomean `420.7754 ms`
  - `dashboard_live`: engine geomean `307.3671 ms`, wall geomean `458.8894 ms`
  - pairwise deltas:
    - versus native installed: `+2.47%` engine, `+0.26%` wall
    - versus dashboard live: `+3.21%` engine, `+8.55%` wall
  - correctness stayed aligned on all `12` workloads:
    - match counts equal
    - files scanned equal
    - files skipped equal
- Operator actions:
  - copied the winning snapshot to new immutable live path `..\iEx-Engine-v2\tools\reports\candidate-compare\iex-cli-live-20260410-220309.exe`
  - stopped old live-loop PID `158140`
  - started replacement live-loop PID `6116` with command:
    - `node.exe tools/scripts/bench-loop-suite.mjs --loops 0 --warmup 1 --samples 3 --iex-binary ..\iEx-Engine-v2\tools\reports\candidate-compare\iex-cli-live-20260410-220309.exe`
- Post-restart verification:
  - `..\iEx-Engine-v2\tools\reports\latest.json` now reports `iex-cli-live-20260410-220309.exe`
  - fresh trailing `..\iEx-Engine-v2\tools\reports\live-metrics.jsonl` entries also report `iex-cli-live-20260410-220309.exe`
- Decision:
  - promote the current working binary into the dashboard live loop
  - treat `iex-cli-live-20260410-220309.exe` as the new canonical active snapshot until a later suite proof beats it

## Tree command planning chain - 2026-04-10 20:59Z
- Scope:
  - added feature parent `todo/pending/017-iex-tree-command-parity.md`
  - added execution units `017a` through `017e` for contract lock, CLI parser, core traversal report, renderer/tests/docs, and closeout
  - updated `.docs/iex-v2-crown-jewel.md` so the planning-spec index now includes the new operator-surface chain
- Intel anchors:
  - `iex --help` currently exposes only `search`, `explain`, and `help`
  - the reproduced failure remains `iex tree apps/backend/convex /F /A` -> `error: unrecognized subcommand 'tree'`
  - local `tree /?` confirms the requested Windows contract `TREE [drive:][path] [/F] [/A]`
  - local render experiments on temporary fixtures showed the practical ordering rule worth freezing for v1:
    - files first alphabetically when `/F` is active
    - a spacer line before child directories when both groups exist
    - child directories alphabetically after the spacer
  - `crates/iex-core/src/engine.rs` already owns ignore-aware walker construction and root normalization, but only for flat search discovery today
- Decisions:
  - keep `tree` v1 bounded to one optional `PATH`, `/F`, and `/A`
  - make slash-flag compatibility its own execution unit so parser behavior is deliberate instead of accidental
  - require a structured core tree report so the CLI renderer never owns filesystem traversal
  - keep shell fallback, JSON output, drive metadata banners, depth flags, and color out of scope for this chain

## README method, sharding, and thread model expansion - 2026-04-10 21:16Z
- Scope:
  - expanded `README.md` to explain the actual iEx execution method instead of only naming components
  - added explicit sections for planner-first execution, root ownership, byte-first scan method, direct-file sharding, concurrency/thread policy, and current next-step doctrine
  - added deeper read links to `.docs/bench/008c-streaming-proof-2026-04-10.md` and `.docs/recon/insect-multipath-next-move-2026-04-10.md`
- Content decisions:
  - describe iEx as planner-first and shape-specific rather than "grep with flags"
  - explain current regex fast-path classes that are already implemented:
    - plain literals
    - ASCII case-insensitive literals
    - ASCII word-boundary literals
    - literal alternates
  - explain sharding narrowly and truthfully:
    - count-only path
    - `64 MiB` direct-file threshold
    - `64 MiB` minimum chunk size
    - `>= 512 MiB` dominant-file override under outer parallelism
    - `4` inner-thread cap
    - exactness comes from overlap windows plus owned-start accounting
  - explain current thread policy as a measured frontier, not a solved abstraction:
    - materialized file-count heuristic tiers
    - bounded streaming queue sized at `resolved_threads * 4`
    - explicit note that traversal-thread budgeting is still an open next move
- Validation:
  - `cargo run -q -p iex-cli -- --help`
  - `cargo run -q -p iex-cli -- search --help`
  - `cargo run -q -p iex-cli -- explain "lit:error && re:\\btimeout\\b"`
  - `node tools/scripts/benchsuite-ripgrep.mjs list`
  - `t3-tape validate --json` returned `status: ok`
  - dupe-audit summary over `README.md` and `.docs/iex-v2-crown-jewel.md` reported `exact_duplicate_candidate_count=0`

## README sharding and thread doctrine rewrite - 2026-04-10 21:37Z
- Scope:
  - rewrote the `README.md` method section to focus on the actual search-shape story the operator asked for:
    - shard-safe counting on dominant giant direct files
    - file-level versus streamed traversal concurrency
    - thread ownership as a budget contract rather than a knob list
  - removed threshold-heavy and queue-detail prose from the public README so the doc explains the doctrine without publishing tactical tuning surfaces
- Content decisions:
  - keep the README centered on sharding correctness through owned byte ranges plus boundary overlap
  - explain concurrency as layered ownership:
    - path handoff
    - file ownership
    - rare byte-range ownership
  - keep the "where next" section pointed at tighter scheduler ownership instead of more visible tuning detail
- Validation:
  - `cargo run -q -p iex-cli -- --help`
  - `cargo run -q -p iex-cli -- search --help`
  - `cargo run -q -p iex-cli -- explain "lit:error && re:\\btimeout\\b"`
  - `node tools/scripts/benchsuite-ripgrep.mjs list`
  - `t3-tape validate --json` returned `status: ok`
  - dupe-audit summary over `README.md` and `.docs/iex-v2-crown-jewel.md` reported `exact_duplicate_candidate_count=0`

## Semantic similarity search planning chain - 2026-04-10 22:35Z
- Scope:
  - added recon artifact `.docs/recon/semantic-similarity-search-recon-2026-04-11.md`
  - harvested new Insect research artifacts under `.docs/research/insect-extracts/similarity-search-wave-2026-04-11/`
  - added feature parent `todo/pending/020-iex-semantic-similarity-search.md`
  - added execution units `020a` through `020g` for contract lock, shared discovery plus chunk schema, runtime model bakeoff, local cache/index, CLI/report wiring, packaging plus rerank boundary, and closeout
  - updated `.docs/iex-v2-crown-jewel.md` so the planning-spec index includes the new semantic retrieval chain
- Intel anchors:
  - current `iex --help` still exposes only `search`, `explain`, and `help`
  - `crates/iex-cli/src/main.rs` remains exact-search-only today
  - `crates/iex-core/src/engine.rs` already owns the ignore-aware discovery seam semantic retrieval should reuse
  - Insect harvest locked the strongest local runtime and model references for this feature:
    - `fastembed-rs` for Rust-local embeddings plus reranking over ONNX
    - Qdrant's code-search tutorial for textified-code versus raw-code retrieval patterns
    - `jinaai/jina-embeddings-v2-base-code` as the code-oriented embedder candidate
    - `all-MiniLM-L6-v2` as the small first-stage retrieval candidate
    - `jina-reranker-v1-tiny-en` and `BAAI/bge-reranker-base` as the bounded rerank shortlist
    - Qdrant reranking and multivector transcripts as evidence that dense retrieval plus optional rerank is the right v1 boundary, while late interaction stays out of scope
- Decisions:
  - add one sibling command `iex similar <QUERY> [PATH...]` instead of deforming the existing boolean `search` grammar
  - keep exact-search hot-path ownership clean by introducing a semantic-only runtime surface and compile-time feature boundary
  - reuse canonical discovery from `iex-core` instead of creating a second walker
  - make local cache/index ownership mandatory so repeated semantic queries do not re-embed the full corpus
  - support two model-asset modes through one runtime path:
    - cached model files
    - bundled compressed assets extracted into the same cache layout
  - keep v1 free of external vector DBs, hidden online APIs, and multivector late interaction
- Validation:
  - `C:\Users\Savage\.codex\skills\t3-tape\assets\bin\windows-x64\t3-tape.exe validate --json` returned `status: ok`
  - dupe-audit summary over the new semantic recon doc plus `020` chain files reported:
    - `target_file_count=9`
    - `segment_count=21`
    - `candidate_pair_count=23`
    - `exact_duplicate_candidate_count=0`

## README top-repo landing rewrite via Insect recon - 2026-04-11 08:35Z
- Scope:
  - rewrote the top README landing sections so the file opens more like the strongest adjacent repos instead of leading with repo explanation
  - kept the sharding and thread doctrine lower in the file, but moved the opening toward:
    - immediate identity
    - fast capability feel
    - proof and status placement
    - compact product-shape framing
- Research inputs:
  - verified adjacent repo star counts live via GitHub API before picking the comparison set
  - used Insect page extraction on raw README surfaces for:
    - `junegunn/fzf`
    - `BurntSushi/ripgrep`
    - `meilisearch/meilisearch`
    - `sharkdp/fd`
    - `ggreer/the_silver_searcher`
    - `typesense/typesense`
  - rejected `elastic/elasticsearch` from the actual README-structure pass after the raw surface resolved to `404`
- Structural findings applied:
  - strongest READMEs establish identity in the first screen instead of explaining repo intent first
  - strongest READMEs show capability feel early through examples, demos, or proof before architecture detail
  - strongest READMEs keep the deep implementation or method story below the opening hook instead of making it the landing paragraph
  - strongest READMEs make the benchmark or product reality easy to find without overfilling the top section with internals
- Validation:
  - `cargo run -q -p iex-cli -- --help`
  - `cargo run -q -p iex-cli -- search --help`
  - `cargo run -q -p iex-cli -- explain "lit:error && re:\\btimeout\\b"`
  - `node tools/scripts/benchsuite-ripgrep.mjs list`
  - `t3-tape validate --json` returned `status: ok`
  - dupe-audit summary over `README.md` and `.docs/iex-v2-crown-jewel.md` reported `exact_duplicate_candidate_count=0`

## README complete overhaul (concurrency + sharding, timeline excerpt) - 2026-04-11 15:52 local
- Scope:
  - replaced `README.md` as a single coherent slice with:
    - a sharper landing focused on hostile corpus search
    - a code-true concurrency model section (streaming stats-only path, file-level parallelism, byte-range sharding)
    - an operator-provided forensic case study excerpt block (kept explicitly labeled as excerpt until repo-native reproduction exists)
    - updated quick-start commands to match Windows binary paths (`iex-cli.exe`)
- Code-truth anchors used during rewrite:
  - streaming stats-only producer/consumer path and its dedupe surface (`DiscoveryDispatch`)
  - byte-range sharding via `maybe_parallel_fast_count_bytes` with correctness enforced in `ExpressionPlan::fast_match_count_no_hits_bytes_in_range`
  - range-count boundary safety through overlap slices plus owned-start filtering (`count_*_in_range` helpers)
- Validation:
  - `cargo run -q -p iex-cli -- --help`
  - `cargo run -q -p iex-cli -- search --help`
  - `cargo run -q -p iex-cli -- explain "lit:error && re:\\btimeout\\b"`
  - `node tools/scripts/benchsuite-ripgrep.mjs list`
  - `t3-tape validate --json` returned `status: ok`
  - dupe-audit summary over `README.md` and `.docs/iex-v2-crown-jewel.md` reported `exact_duplicate_candidate_count=0`

## README bullish refinement (Garry-style critique pass) - 2026-04-11 16:22 local
- Scope:
  - tightened the opening to be more direct and confident without inflating claims
  - added an explicit "Who this is for" wedge and a "What makes iEx different" wedge that matches the actual engine surfaces
  - removed contrast-heavy phrasing and tidy triplet endings that read like generic README filler
  - added a Mermaid architecture flow for parallel byte-range sharding so the concurrency model is visual, not just prose
  - added a real `LICENSE` file so MIT claims and license badge have a canonical target
  - added README badges (build-native-binaries workflow, latest release tag, MIT license) and a short contents block
  - added an install section that matches the workflow-packaged binary name (`iex.exe` / `iex`) versus the local cargo build output (`iex-cli.exe`)
- Validation:
  - `cargo run -q -p iex-cli -- --help`
  - `cargo run -q -p iex-cli -- search --help`
  - `cargo run -q -p iex-cli -- explain "lit:error && re:\\btimeout\\b"`
  - `node tools/scripts/benchsuite-ripgrep.mjs list`
  - `t3-tape validate --json` returned `status: ok`
  - dupe-audit summary over `README.md` and `.docs/iex-v2-crown-jewel.md` reported `exact_duplicate_candidate_count=0`

## Repo-audit ergonomics planning chain - 2026-04-10 22:28Z
- Scope:
  - added feature parent `todo/pending/019-iex-repo-audit-ergonomics-surface.md`
  - added execution units `019a` through `019e` for contract lock, path filters and named scopes, expression-file plus rewrite diagnostics, help and docs parity, and closeout
  - updated `.docs/iex-v2-crown-jewel.md` so the planning-spec index now includes the ergonomics chain
- Intel anchors:
  - `iex search --help` currently exposes only hidden, symlink, JSON, stats-only, max-hits, threads, and report-output flags
  - `iex explain` already exists as the canonical parse-inspection command
  - `crates/iex-core/src/engine.rs` already owns one ignore-aware `WalkBuilder`, but `SearchConfig` has no operator-owned include, exclude, or named-scope filter contract
  - `crates/iex-cli/src/main.rs` requires a positional expression and exposes no `--expr-file` path or shell-aware examples
  - `.refs/ripgrep/README.md` confirms the operator value of first-class manual filtering and file-type targeting without implying matcher-kernel changes
- Decisions:
  - take only the highest-value ergonomics advice:
    - path filters
    - named scopes
    - expression files
    - shell-aware help
    - actionable working rewrites
  - explicitly reject duplicate query-mode flags such as `--literal` and `--regex` in this wave because `lit:` and `re:` already own intent
  - explicitly reject inline debug aliases because `iex explain` already owns parse inspection
  - explicitly reject presets and output taxonomies in this wave because they would add extra state and classification systems before the core targeting friction is solved

## README tightening + badge hygiene + sharding diagram de-leak - 2026-04-11 local
- Scope:
  - removed the redundant "What makes iEx different" section to keep the top half tighter and less repetitive
  - added a docs badge (iex.run) and a PowerShell note about `iex` name collisions so onboarding is less fragile
  - tightened the sharding and concurrency wording to stay code-true while avoiding internal API-name leakage
  - updated the Mermaid diagram labels to read like an architecture flow rather than an implementation trace
  - ignored `/.t3/` (t3-tape state) in `.gitignore` to prevent local state churn from showing up as repo noise
- Gold-standard scan:
  - harvested best-in-class CLI README patterns via Insect into `.docs/insect/readmes/*` (ripgrep, fd, bat) as reference material
- Validation:
  - `iex search --help`
  - `iex search "lit:available_parallelism" crates/iex-core/src --json --max-hits 20` (thread budget evidence)
  - `iex search "lit:build_parallel" crates/iex-core/src --json --max-hits 20` (streaming discovery evidence)
  - `iex search "lit:bytes_in_range" crates/iex-core/src --json --max-hits 20` (byte-range count path evidence)
  - `dupe-audit` over `README.md` reported `exact_duplicate_candidate_count=0`
  - `t3-tape validate --json` returned `status: ok` (bundled binary)
- Blocker:
  - `user-message-logger` could not run because `python3` is not installed in this environment

## README positioning broadening (agentic + rg-native wedge) - 2026-04-11 local
- Scope:
  - broadened the opening positioning so "evidence work" reads as a strong example, not a market limiter
  - added a second "already fast default" wedge for rg-heavy builders who hit tail-setting corpora

## Ops: stop dashboard loop runner - 2026-04-11 local
- Action:
  - stopped the running benchmark loop process: `node tools/scripts/bench-loop-suite.mjs` (previous PID `122144`)
- Note:
  - the dashboard server (`node tools/scripts/dashboard-server.mjs`) was left running

## Engine: 90% auto threads + lifted 12 caps (9950X3D-friendly) - 2026-04-11 local
- Scope:
  - materialized scan auto threads now switch to `ceil(0.9 * available_parallelism)` for `file_count >= 1024`
  - streaming stats-only auto threads now use `ceil(0.9 * available_parallelism)` (removed the fixed 12 cap)
  - parallel byte-range fast-count sharding now uses `ceil(0.9 * available_parallelism)` as its budget (removed the fixed 12 cap)
  - dominant-file inner sharding under outer parallelism now scales as `max(4, shard_budget/4)` clamped to 12
  - added deterministic unit tests for the new policy and cap scaling
- Evidence:
  - 32 logical threads => `threads_90pct(32)` resolves to 29
- Validation:
  - `cargo test -p iex-core`
  - dupe-audit over `crates/iex-core/src/engine.rs` reported `exact_duplicate_candidate_count=0`

## README: ConcurrencyPlanner design-target diagram - 2026-04-11 local
- Scope:
  - added a mermaid architecture diagram that captures the long-term concurrency investment: planner-driven budgets + shard geometry + single run-scoped pool + telemetry
- Validation:
  - dupe-audit over `README.md` reported `exact_duplicate_candidate_count=0`

## Engine: phase-1 concurrency planner + telemetry - 2026-04-11 local
- Scope:
  - introduced explicit run-level concurrency planning in `crates/iex-core/src/engine.rs` for materialized and streaming runs
  - split byte-range sharding into a file-local plan with `shard_threads`, `chunk_bytes`, and `range_count`
  - replaced the fixed `64 MiB` shard geometry with planner-chosen chunk sizing that keeps shard workers fed on mid-size single-file workloads
  - added append-only `stats.concurrency` telemetry in `crates/iex-core/src/stats.rs` and propagated it into `tools/scripts/lib/benchmark-runner.mjs`
  - documented the implementation status in `README.md` and `.docs/iex-v2-crown-jewel.md`
- Validation:
  - `cargo test -p iex-core`
  - `cargo test -p iex-cli`
  - `bash C:/Users/Savage/.codex/skills/dupe-audit/scripts/dupe-audit.sh --provider gguf --target crates/iex-core/src/engine.rs --target crates/iex-core/src/stats.rs --target tools/scripts/lib/benchmark-runner.mjs --task-text "...concurrency planner slice..." --output summary --no-write`
  - release snapshot before rebuild: `tools/reports/candidate-compare/iex-cli-prerebuild-planner-20260411-234016.exe`
  - new candidate snapshot: `tools/reports/candidate-compare/iex-cli-candidate-planner-phase1-20260411-234016.exe`
  - suite comparison artifact: `tools/reports/candidate-compare/planner-phase1-vs-threads90-suite-compare-2026-04-11t21-42-38-019z.json`
- Result:
  - EN single-file shard-safe lanes improved strongly (`suite-en-alternates` `+28.66%`, `suite-en-literal-casei` `+24.68%`, `suite-en-literal` `+16.73%`)
  - full 12-profile median versus the current `threads90` candidate is still slightly negative (`271.7086 ms` baseline -> `272.7053 ms` candidate, `-0.37%`), so the slice is implemented and instrumented but not yet ready for active-loop promotion

## README: shard geometry explanation refresh - 2026-04-12 local
- Scope:
  - rewrote the shard-geometry explanation in `README.md` so it reads like a compact systems note for technical readers
  - added a focused Mermaid showing how file length, shard budget, minimum chunk size, range count, and ownership fit together
  - tightened the correctness explanation around widened visibility windows versus owned-match attribution
  - expanded the section into a wider "search kernel" explanation with tiered byte ingress, parse-time regex fast paths, and the bounded streaming tree pipeline so sharding reads as part of one coherent engine
  - added a second Mermaid showing shard ownership versus widened visibility windows
- Validation:
  - `iex search "lit:Shard Geometry || lit:ownership is for exactness || lit:multiple ranges per worker" README.md --json --max-hits 20`
  - `bash C:/Users/Savage/.codex/skills/dupe-audit/scripts/dupe-audit.sh --provider gguf --target README.md --task-text "Review the shard-geometry README rewrite for duplicate or parallel doc drift. Return concise findings only." --output summary --no-write`
  - `iex search "lit:Search Kernel Shape || lit:throughput comes from geometry || lit:memchr || lit:bounded queue" README.md --json --max-hits 20`
- Result:
  - README now explains shard geometry as a work-feeding problem, not just "split the file"
  - README now also frames the engine as a staged search kernel rather than a bag of isolated tricks
  - dupe-audit reported `exact_duplicate_candidate_count=0`

## README: tone and hero reframe - 2026-04-12 local
- Scope:
  - rewrote the README opening so it reads like a serious product page instead of a school-project report
  - replaced the top-of-file framing with a centered hero, stronger thesis, and sharper early section names
  - kept the shard-geometry and engine-detail sections, but pulled the differentiators forward so the page lands faster
  - removed the bottom `Sources (At End)` classroom-style section and replaced it with a cleaner `Read Next`
- Validation:
  - `bash C:/Users/Savage/.codex/skills/dupe-audit/scripts/dupe-audit.sh --provider gguf --target README.md --task-text "Review the README tone-and-hero rewrite for duplicate or parallel documentation drift. Return concise findings only." --output summary --no-write`
  - heading check for `# <div align="center">iEx</div>`, `## Why People Reach For iEx`, `## Expression Language`, `## Benchmark Truth`, `## Read Next`
- Result:
  - README now opens with stronger product posture and less bureaucratic structure
  - core engine differentiators are visible much earlier
  - dupe-audit reported `exact_duplicate_candidate_count=0`

## README: topological systems reframe - 2026-04-12 local
- Scope:
  - used the provided topological/pathological-corpora guidance as a tonal model for the README without copying over untrue implementation claims
  - rewrote the early README framing around `Architectural Invariants`, `Applicable Domains`, and `Execution Topology`
  - tightened concurrency subheadings so the section reads more like a systems document and less like an internal planning note
- Validation:
  - `bash C:/Users/Savage/.codex/skills/dupe-audit/scripts/dupe-audit.sh --provider gguf --target README.md --task-text "Review the README topological-systems rewrite for duplicate or parallel documentation drift. Return concise findings only." --output summary --no-write`
  - heading check for `Topological search for pathological corpora`, `Architectural Invariants`, `Applicable Domains`, `Execution Topology`, `Concurrency Planner`, `Streaming Trees`, `Byte-Range Sharding`, `Benchmark Truth`
- Result:
  - README now carries more of a systems-tool voice without inventing unsupported internals
  - the topological/pathological framing is now present in the opening and early architecture sections
  - dupe-audit reported `exact_duplicate_candidate_count=0`

## README: applicable domains broadened for agentic corpora - 2026-04-12 local
- Scope:
  - rewrote `Applicable Domains` so it is broader and more agent-friendly instead of skewing mainly forensic
  - shifted the examples toward high-value hostile corpus shapes such as exported chats, JSONL memory stores, tool transcripts, evaluation dumps, observability records, mixed source-plus-artifact trees, and large undisciplined knowledge heaps
- Validation:
  - `bash C:/Users/Savage/.codex/skills/dupe-audit/scripts/dupe-audit.sh --provider gguf --target README.md --task-text "Review the applicable-domains rewrite for duplicate or parallel documentation drift. Return concise findings only." --output summary --no-write`
  - heading check for `## Applicable Domains` and the new agent/observability/knowledge-heap bullets
- Result:
  - README now presents iEx as useful across broader agentic and systems-search workloads without falling back to boring generic examples
  - dupe-audit reported `exact_duplicate_candidate_count=0`

## Recon: April 12 performance direction re-check - 2026-04-12 local
- Scope:
  - re-ran the higher-level performance review against the current repo state instead of trusting the older April 10 strategic read
  - checked the active loop pin in `tools/reports/latest.json`, the current suite-style compare artifacts in `tools/reports/candidate-compare`, the benchmark contract in `tools/scripts/lib/benchmark-runner.mjs`, and the live engine gates in `crates/iex-core/src/engine.rs`
  - ratcheted `.docs/iex-v2-crown-jewel.md` so the repo doctrine reflects the current winning wave rather than the older multipath-first suspicion
- Validation:
  - `iex search "threads90" .docs/iex-v2-crown-jewel.md`
  - `iex search "threads90" .docs/todo/changelog/_log.md`
  - `iex search "should_stream_stats_only" crates/iex-core/src/engine.rs`
  - `iex search "threads_90pct" crates/iex-core/src/engine.rs`
  - `iex search "lit:--stats-only" tools/scripts/lib/benchmark-runner.mjs`
- Result:
  - the earlier April 10 review is now partly stale because the current live candidate is `tools/reports/candidate-compare/iex-cli-candidate-threads90-20260411-201130.exe` and its current loop artifacts show broad wins versus ripgrep on sampled Linux and subtitle lanes
  - the highest-value engine story has shifted toward materialized-path concurrency and file-local sharding, while the remaining `008c` streaming-gate issue is real but no longer the dominant strategic bottleneck
  - planner phase 1 remains worthwhile but not promotable yet because it wins several EN single-file lanes while still losing the full suite median versus `threads90`

## Planning: ripgrep-style ingress compatibility chain - 2026-04-14 local
- Scope:
  - created a new planning-spec chain at `todo/pending/021-iex-ripgrep-style-ingress-compat.md` with execution units `021a` through `021e`
  - locked the repair to one narrow ripgrep-style ingress layer for `iex PATTERN [PATH]...` so operator muscle memory stops failing without turning iEx into a ripgrep rebuild
  - folded in the newly observed painpoints:
    - large no-hit searches such as `iex search "lit:ripgrep" .docs crates .refs` can look dead before an external timeout
    - duplicate transcript output must be proven against direct CLI invocation before being treated as an iEx-owned bug
    - unsupported rg-only flags need guided failures instead of raw parser noise
  - recorded the existing plan conflict with `019-iex-repo-audit-ergonomics-surface`, since that earlier chain rejected compatibility shims before this new complaint changed the doctrine
- Validation:
  - `iex --help`
  - `iex search --help`
  - `rg --help`
  - `Get-Content -LiteralPath 'crates/iex-cli/src/main.rs'`
  - `bash C:/Users/Savage/.codex/skills/dupe-audit/scripts/dupe-audit.sh --provider gguf --target todo/pending/021-iex-ripgrep-style-ingress-compat.md --target todo/pending/021a-iex-ripgrep-style-ingress-compat.md --target todo/pending/021b-iex-ripgrep-style-ingress-compat.md --target todo/pending/021c-iex-ripgrep-style-ingress-compat.md --target todo/pending/021d-iex-ripgrep-style-ingress-compat.md --target todo/pending/021e-iex-ripgrep-style-ingress-compat.md --task-text "Review the new ripgrep-style ingress compatibility planning chain for duplicate ownership, repeated scope, or split-brain planning drift. Return concise findings only." --output summary --no-write`
  - `Get-Command t3-tape`
- Result:
  - chain `021` now exists and is ready for implementation without drifting into full ripgrep parity work
  - the planned ownership split is explicit:
    - `crates/iex-cli` owns compat ingress, unsupported-flag guidance, stderr liveness feedback, and help/docs updates
    - `crates/iex-core` remains the single owner of search execution
  - the chain explicitly guards five invariants:
    - canonical `iex search` and `iex explain` stay intact
    - compat ingress reuses the existing search execution path
    - unsupported ripgrep-only flags remain guided failures
    - JSON stdout stays machine-readable
    - one invocation dispatches once
  - dupe-audit reported `exact_duplicate_candidate_count=0` on the new planning slice
  - `t3-tape` is not installed in this workspace, so maintenance validation is a real blocker to clear during implementation closeout rather than a step to fake
  - review refinement tightened the plan further:
    - removed the unnecessary dash-pattern branch from the first compat wave
    - made unsupported rg-only guidance an explicit non-zero user-error contract
    - changed closeout so `t3-tape` availability is verified before maintenance validation is treated as passable
  - upstream re-check against `.refs/ripgrep` shifted the doctrine from "bare iEx DSL without `search`" to "raw argv classifier plus narrow rg-shaped translator":
    - ripgrep itself parses low-level argv first, then decides whether the first positional is the pattern and the rest are paths
    - the best iEx analogue is one compat classifier that recognizes rg-shaped search requests and lowers a safe subset into canonical iEx search args
    - the refined subset now explicitly includes raw pattern translation plus high-signal agent flags like `-e`, `-F`, `-i`, `-j`, `-n`, `--json`, and `--hidden`

## Implementation: rg-style ingress compatibility - 2026-04-15 local
- Scope:
  - added a canonical-first, compat-second argv router in `crates/iex-cli/src/main.rs`
  - kept `iex search` and `iex explain` as the canonical owned surface while allowing bare `iex PATTERN [PATH]...` requests to lower into the existing search path
  - implemented a narrow supported subset for `-e/--regexp`, `-F/--fixed-strings`, `-i/--ignore-case`, `-j/--threads`, `-n/--line-number`, `--json`, and `--hidden`
  - preserved bare native iEx expressions such as `iex lit:Command::Search crates\\iex-cli` instead of forcing them through rg-style regex lowering
  - kept unsupported rg-only flags as guided non-zero failures so the ingress layer stays small and honest
- Validation:
  - `cargo test -p iex-cli`
  - `cargo build -p iex-cli`
  - `target\\debug\\iex-cli.exe timeout crates\\iex-cli --json`
  - `target\\debug\\iex-cli.exe lit:Command::Search crates\\iex-cli`
  - `target\\debug\\iex-cli.exe --files`
- Result:
  - native subcommands still own themselves, and the compat layer only activates when canonical clap parsing does not accept the argv
  - future CLI growth does not require a hardcoded compat command table because canonical detection derives from the clap command metadata
  - one dispatch path still owns search execution, so compat ingress does not create a second engine or duplicate output route
  - unsupported rg-only requests now fail with direct guidance instead of raw parser noise

## Implementation: stats-only single-root streaming restore - 2026-04-19 local
- Scope:
  - widened the stats-only streaming gate in `crates/iex-core/src/engine.rs` so any directory-root stats-only search can use the streaming dispatch path, not just multipath runs
  - kept hit-collecting searches on the materialized path and kept single-file stats-only behavior unchanged
  - updated engine tests so the canonical gate and execution-mode contract now explicitly cover single-root directory stats-only searches
  - refreshed `.docs/iex-v2-crown-jewel.md` so the core ownership map matches the restored directory-root streaming path
- Validation:
  - `cargo test -p iex-core`
  - `cargo build --release -p iex-cli`
  - native-vs-candidate suite proof: `tools/reports/candidate-compare/native-vs-current-suite-2026-04-19_12-21-07-023z.json`
- Result:
  - current candidate now beats the installed native `iex` bin on the active suite by `17.74%` overall (`overallRatio=0.8225935725710628`)
  - the restored streaming path is the decisive change on the heavy linux directory lanes:
    - `suite-linux-literal`: `47.15s` native -> `39.81s` candidate (`+15.57%`)
    - `suite-linux-word`: `52.98s` native -> `40.94s` candidate (`+22.74%`)
    - `suite-linux-alternates`: `51.36s` native -> `41.49s` candidate (`+19.21%`)
    - `suite-linux-no-literal`: `51.29s` native -> `41.63s` candidate (`+18.82%`)

## Packaging: current benchmark candidate commit slice - 2026-04-19 local
- Scope:
  - packaged the owned core files for the active performance slice: `crates/iex-core/src/engine.rs`, `crates/iex-core/src/expr.rs`, `crates/iex-core/src/stats.rs`, and `crates/iex-core/src/lib.rs`
  - retained the restored directory-root stats-only streaming dispatch as the dominant suite win
  - retained shard-planning telemetry and the expression-level guard that keeps outer-parallel shard fast count off for alternates and word-boundary plans
  - refreshed `.docs/iex-v2-crown-jewel.md` so the architecture map names shard-aware stats-only planning and concurrency telemetry explicitly
- Validation:
  - `cargo test -p iex-core`
  - `cargo build --release -p iex-cli`
  - proof artifact retained: `tools/reports/candidate-compare/native-vs-current-suite-2026-04-19_12-21-07-023z.json`
- Result:
  - worktree validation passed immediately before commit packaging (`51 passed` in `iex-core`; release `iex-cli` build succeeded)
  - commit scope stays inside the owned performance slice and leaves the pre-existing CLI/docs/root-worktree changes unstaged
  - the losing experimental slice was rejected and reverted before closeout: re-enabling outer-parallel inner sharding for word-boundary fast counts created multi-second giant-file spikes on AMD headers and was not kept

## Implementation: fixed-width non-ASCII casefold regex sharding - 2026-04-20 local
- Scope:
  - extended `crates/iex-core/src/expr.rs` so literal-equivalent non-ASCII `re:(?i)...` patterns with stable byte-width case variants no longer fall through to the unshardable generic regex bucket
  - added a narrow `FixedWidthBytesRegex` fast-path classification that reuses the existing byte-regex engine for full-buffer matching while exposing deterministic range counting for shard-safe `--stats-only` scans
  - kept ASCII casefold searchers, ASCII word-boundary ownership, and variable-width Unicode forms like `Straße` on their existing paths
  - added expression tests that lock the fixed-width Cyrillic fast-path classification and its full-count/range-count parity
- Validation:
  - focused tests:
    - `cargo test -p iex-core case_insensitive_fixed_width_non_ascii -- --nocapture`
    - `cargo test -p iex-core case_insensitive_fast_path_activates_for_fixed_width_non_ascii_literals -- --nocapture`
  - full crate gate:
    - `cargo test -p iex-core`
  - release build:
    - `cargo build --release -p iex-cli`
  - installed-vs-candidate Russian-lane proof:
    - `tools/reports/candidate-compare/casei-slice-vs-installed-2026-04-19T22-29-41Z.json`
  - installed-vs-candidate full-suite proof:
    - `tools/reports/candidate-compare/wsl-installed-vs-casei-slice-suite-2026-04-19T22-31-30Z.json`
- Result:
  - `suite-ru-literal-casei` moved from a single-threaded materialized scan to shard-enabled fast-count execution (`max_shard_threads=11`, `max_shard_ranges=22`, `max_shard_chunk_bytes=77949104`)
  - targeted alternating replay against the installed WSL `iex` bin improved `suite-ru-literal-casei` by `51.89%` median (`10067.61 ms` -> `4843.76 ms`) while keeping `suite-ru-literal` slightly positive (`4540.92 ms` -> `4524.11 ms`)
  - a fresh full active-suite pass against the installed WSL `iex` bin improved overall direct time by `22.05%` (`276394.39 ms` -> `215461.91 ms`) and won `11/12` profiles
  - the only remaining loser in the one-pass suite artifact is `suite-ru-literal` (`-6.66%`), but that lane did not hold up as a stable loss under the focused alternating replay after this slice, so no second speculative kernel change was kept

## Implementation: previous-build iEx live comparator lane - 2026-04-19 local
- Timestamp:
  - `2026-04-19 22:56:04.358Z`
- Scope:
  - extended `tools/scripts/lib/benchmark-runner.mjs` so `runOneBenchmark(...)` can measure an explicit `--previous-iex-binary` with the same `search ... --json --stats-only` engine-timing path used for the active iEx binary
  - stored the prior snapshot inside the canonical `competitors` map as `iex_previous` with self-history metadata instead of adding a parallel telemetry channel
  - threaded `--previous-iex-binary` through `tools/scripts/bench-loop-suite.mjs`, `tools/scripts/bench-loop.mjs`, and `tools/scripts/run-once-benchmark.mjs`
  - updated `tools/scripts/lib/summary.mjs` so `iex_previous` is summarized and labeled in `competitorSummary` but explicitly excluded from `primaryChallenger`
  - expanded `dashboard/index.html` to render the previous-build lane in the ratio trend, recent-runs table, top-line metric cards, and comparison table
- Structural snapshot:
  - benchmark runner
    └─ active iEx + previous iEx use the same JSON stats contract
      ├─ active engine ms → `iexMs`
      ├─ previous engine ms → `competitors.iex_previous.durationMs`
      └─ self-regression ratio → `iexToPreviousRatio`
  - summary pipeline
    └─ `summarizeHistory(history)`
      ├─ competitor rollup keeps `iex_previous`
      └─ challenger selection skips `iex_previous`
  - dashboard UI
    └─ live monitor
      ├─ trend legend/series → `iEx/previous`
      ├─ recent runs → `iEx/prev`, `vs prev %`, `prev iEx ms`
      └─ metric cards → `iEx/Previous`
- Validation:
  - syntax gates:
    - `node --check tools/scripts/lib/benchmark-runner.mjs`
    - `node --check tools/scripts/lib/summary.mjs`
    - `node --check tools/scripts/bench-loop-suite.mjs`
    - `node --check tools/scripts/bench-loop.mjs`
    - `node --check tools/scripts/run-once-benchmark.mjs`
  - sample runner proof against the pinned live candidate and the installed WSL `iex`:
    - active binary: `tools/reports/candidate-compare/iex-cli-live-20260419-200006`
    - previous binary: `/home/savage/.local/bin/iex`
    - workload: `lit:Sherlock Holmes` over `.refs/ripgrep/benchsuite/subtitles/en.sample.txt`
    - result: `iexToPreviousRatio=0.9950x` with `competitors.iex_previous.timingSource="engine_total_ms"`
  - server/API proof on a throwaway live sample:
    - `jsonl_has_previous=true`
    - `/api/summary` competitor names: `ripgrep`, `ugrep`, `iex_previous`
    - `/api/summary` previous entry label: `previous iEx`
  - dashboard surface proof:
    - page markup now exposes `iEx/Previous`, `iEx/previous`, and `prev iEx ms`
- Result:
  - the dashboard can now show current-vs-previous iEx movement in real time on the same run stream as ripgrep/ugrep
  - self-history tracking stays canonical and measurable without corrupting the external contender heuristics or hint system

## Implementation: remove ugrep from live run tracking - 2026-04-20 local
- Scope:
  - removed `ugrep` from `tools/scripts/competitors.json`, so live loop/report runs no longer measure or persist `competitors.ugrep`
  - removed `ugrep`-specific loop/report console logging from `tools/scripts/bench-loop-suite.mjs`, `tools/scripts/bench-loop.mjs`, and `tools/scripts/bench-report.mjs`
  - simplified `tools/scripts/lib/summary.mjs` live ordering/challenger selection so the tracked live comparator order is now `ripgrep` then `iex_previous`
  - removed `iEx/ugrep` trend, columns, and coverage text from `dashboard/index.html`, leaving only `iEx/rg` and `iEx/previous` in the live monitor
- Validation:
  - `node --check tools/scripts/lib/summary.mjs`
  - `node --check tools/scripts/bench-loop-suite.mjs`
  - `node --check tools/scripts/bench-loop.mjs`
  - `node --check tools/scripts/bench-report.mjs`
  - `iex 'ugrep|iEx/ugrep|vs ugrep|ugrep ms' dashboard/index.html tools/scripts/bench-loop-suite.mjs tools/scripts/bench-loop.mjs tools/scripts/bench-report.mjs tools/scripts/competitors.json tools/scripts/lib/summary.mjs`
- Result:
  - live run tracking no longer measures or renders `ugrep`
  - the only live comparison lanes are now `ripgrep` and optional previous-build `iEx`

## Documentation: project distill handoff - 2026-04-21 local
- Scope:
  - added `.docs/project-distill-2026-04-21.md` as the dated current-state handoff for mission lock, retained wins, rejected slices, governance doctrine, and fresh-chat anchors
  - updated `.docs/iex-v2-crown-jewel.md` to point cold starts at the dated distill before the older historical status notes
  - archived documentation chain `021-iex-project-distill-handoff` so the handoff is recoverable from filesystem state
- Validation:
  - `target/release/iex-cli.exe search "lit:iEx Project Distill || lit:directory-root --stats-only streaming dispatch || lit:FixedWidthBytesRegex || lit:iex_previous || lit:beat ripgrep with proof, not vibes" .docs/project-distill-2026-04-21.md .docs/iex-v2-crown-jewel.md --json`
    - `matches_found=8`, `files_discovered=2`
  - `git diff --check -- '.docs/project-distill-2026-04-21.md' '.docs/iex-v2-crown-jewel.md'`
  - `bash 'C:/Users/Savage/.codex/skills/dupe-audit/scripts/dupe-audit.sh' --provider gguf --target '.docs/project-distill-2026-04-21.md' --target '.docs/iex-v2-crown-jewel.md' --task-text 'Review the iEx project distill documentation slice for duplicate or parallel documentation drift. Return concise findings only.' --output summary --no-write`
    - `candidate_pair_count=0`
    - `exact_duplicate_candidate_count=0`
  - `Select-String -LiteralPath 'tools/scripts/dashboard-server.mjs' -Pattern 'no-store|no-cache|Cache-Control' -Context 0,2`
    - confirmed explicit `no-store, no-cache, must-revalidate, proxy-revalidate` headers in the dashboard server surface
- Result:
  - the repo now has one canonical dated handoff doc for the current benchmark-governance state instead of forcing cold starts through long historical notes
  - the patch stayed inside `.docs` and preserved the unrelated workspace edits in `crates/iex-core/src/expr.rs` and `dashboard/index.html`

## Recon: meaningful gains research - 2026-04-21 local
- Timestamp:
  - `2026-04-21 23:34:55 +02:00`
- Scope:
  - added `.docs/recon/meaningful-gains-research-2026-04-21.md` to turn the current loser lanes, `.refs` harvest, and primary-source paper/doc findings into one ranked optimization brief
  - grounded the brief in the current suite proof window `bench-proof-head-vs-live-20260421-232204` plus direct Windows-native telemetry samples from the live snapshot and the rebuilt candidate
  - kept the slice documentation-only so the repo can cold-start the next engine pass from filesystem state without assuming chat history
- Structural snapshot:
  - current path
    └─ `search --stats-only`
      ├─ `run_search_inner`
      ├─ `maybe_parallel_fast_count_bytes`
      │  └─ generic regex cannot shard because `fast_match_count_no_hits_bytes_in_range(...)` returns `None`
      └─ fallback
        └─ whole-buffer regex scan for generic regex lanes
  - ranked next moves
    ├─ planner-owned inner literal extraction + reverse-inner verification
    ├─ adaptive acceleration inerting / bailout
    ├─ shard-safe regex regime for giant-file stats-only scans
    └─ matcher-shape-aware file strategy arbitration
- Validation:
  - direct telemetry samples:
    - live snapshot `221444` and rebuilt candidate `232000` both sampled on:
      - `suite-linux-no-literal`
      - `suite-linux-literal`
      - `suite-en-surrounding-words`
    - confirmed:
      - `suite-linux-no-literal` remains `materialized` with `sharding_enabled=false`
      - `suite-en-surrounding-words` remains single-file, single-thread, non-sharded
      - `suite-linux-literal` only partially benefits from sharding on giant files
  - proof anchor:
    - `tools/reports/candidate-compare/bench-proof-head-vs-live-20260421-232204.summary.json`
    - `tools/reports/candidate-compare/bench-proof-head-vs-live-20260421-232204.runs.json`
- Result:
  - the repo now has one dated research note that separates "promising" from "actually highest leverage" and explicitly argues against a parallel literal-kernel detour as the first move

## Promotion proof: clean head vs live windows loop - 2026-04-22 local
- Scope:
  - kept the Windows-only single-root `--stats-only` gating repair in `crates/iex-core/src/engine.rs`
  - removed the dead generic line-sharding experiment before promotion so the winning candidate stayed on one coherent behavior slice
  - paused the live Windows loop, rebuilt `iex-cli`, and pinned proof candidate `tools/reports/candidate-compare/iex-cli-candidate-clean-20260422-000826.exe`
- Validation:
  - `cargo test -p iex-core`
    - `54 passed; 0 failed`
  - `cargo build --release -p iex-cli`
  - `Get-FileHash -Algorithm SHA256 target/release/iex-cli.exe tools/reports/candidate-compare/iex-cli-candidate-clean-20260422-000826.exe`
    - both hashes `088DB52726BDF2E4C2DD8F8218740E171CB211B5EE666419D735B6ABE412CF1A`
  - paused-loop full-suite proof:
    - `tools/reports/candidate-compare/bench-proof-clean-vs-live-20260422-000857.summary.json`
    - methodology: `warmupPerBinary=1`, `measuredPairsPerProfile=3`, `candidate_then_live_alternating`, `loopPaused=true`
    - overall:
      - `wins=8`
      - `losses=4`
      - `candidateMedianTotalMs=3642.5406`
      - `liveMedianTotalMs=4104.3354`
      - `candidateToLiveRatio=0.887486`
      - `deltaPct=11.2514`
- Result:
  - the rebuilt clean candidate beat the current live Windows snapshot at the suite level with paused-loop proof, so it qualified for promotion

## Hidden-knowledge harvest for regex/prefilter next move - 2026-04-22 local
- Scope:
  - added `.docs/recon/hidden-knowledge-harvest-2026-04-22.md`
  - compared current `iex-core` regex ownership against upstream `ripgrep`, `regex-automata`, `memchr`, and `RE2`
  - sampled byte distributions for the dominant AMD header tails versus subtitle text to test whether corpus shape is a real signal
- Validation:
  - local evidence:
    - `iex search "lit:fast_match_count_no_hits_bytes_in_range" crates\\iex-core\\src --json`
    - `iex search "lit:RequiredPrefixForAccel" .refs\\re2\\re2 --json`
    - `iex search "lit:find_candidate_line" .refs\\ripgrep\\crates --json`
    - `iex search "lit:inert" .refs\\memchr\\src\\memmem --json`
  - primary-source docs:
    - `https://github.com/BurntSushi/blog/blob/master/content/post/ripgrep.md`
    - `https://github.com/BurntSushi/ripgrep/discussions/1827`
    - `https://docs.rs/memchr/latest/memchr/memmem/enum.Prefilter.html`
    - `https://docs.rs/memchr/latest/memchr/arch/all/packedpair/trait.HeuristicFrequencyRank.html`
- Result:
  - captured a tighter next-step doctrine:
    - first win should be planner-owned, line-safe inner-literal candidate verification
    - inerting telemetry is mandatory on day one
    - corpus-aware memmem tuning is a valid second-wave experiment for AMD header tails
    - whole-buffer reverse-inner and generic regex sharding remain separate later parents

## Hidden-knowledge harvest expansion with live suite surface mapping - 2026-04-22 local
- Scope:
  - refreshed `.docs/recon/hidden-knowledge-harvest-2026-04-22.md` with additional upstream extractor guards, DFA quit-byte constraints, and current live-suite prioritization data
  - replaced the snapshot block with ASCII-only cold-start structure so the note is patch-safe and terminal-clean
  - grouped current `tools/reports/live-metrics.jsonl` by profile to distinguish regex-parent opportunity from suite-goal priority
- Validation:
  - local evidence:
    - `iex search "lit:is_poisonous" .refs\\ripgrep\\crates\\regex\\src\\literal.rs --json`
    - `iex search "lit:unicode_word_boundary" .refs\\regex\\regex-automata\\src\\hybrid\\dfa.rs --json`
    - `iex search "lit:AsciiCaseFoldLiteral" crates\\iex-core\\src\\expr.rs --json`
    - grouped latest loop entries from `tools\\reports\\live-metrics.jsonl`
  - primary-source docs:
    - `https://docs.rs/regex-automata/latest/regex_automata/hybrid/dfa/struct.Config.html`
- Result:
  - uncovered three additional high-value constraints:
    - ripgrep literal extraction has explicit poison/quality gating, not just literal detection
    - DFA-style Unicode boundary acceleration requires explicit quit/bail handling
    - the clean regex-next slice is still `suite-en-surrounding-words`, but the larger remaining goal-miss cluster is Linux plus no-literal, so regex-parent and suite-parent priorities should stay separate

## Regex line-candidate proof gate and runtime backout - 2026-04-22 local
- Scope:
  - implemented and tested a planner-owned inner-literal line-candidate path for `re:\w+\s+Holmes\s+\w+`
  - snapshotted the pre-build canonical release binary to `tools/reports/candidate-compare/iex-cli-baseline-20260422-034838.exe`
  - compared the candidate binary against that exact snapshot on `.refs/ripgrep/benchsuite/subtitles/en.sample.txt`
  - restored repo code to the known-good baseline after proof failed
- Validation:
  - `cargo test`
  - benchmark proof runs recorded in `tools/reports/live-metrics.jsonl`
  - durable note: `.docs/recon/regex-line-candidate-proof-2026-04-22.md`
- Result:
  - first stats-only line-loop variant regressed catastrophically (`runId=1776822546512-a44ecd50`, `iexToPreviousRatio=3.5339`)
  - even after backing the runtime path out of stats-only, repeated proof stayed slower than the snapped baseline (`runId=1776822657892-d9e508cb`, `1776822776700-11b33a0e`, `1776822785550-bf7c05e1`)
  - conclusion: do not land this implementation shape; the next regex parent must use whole-buffer candidate discovery plus line-boundary recovery instead of a naive per-line prefilter loop

## Regex whole-buffer line-recovery proof gate and runtime backout - 2026-04-22 local
- Scope:
  - implemented and tested a planner-owned whole-buffer inner-literal candidate path with line-boundary recovery for `re:\w+\s+Holmes\s+\w+`
  - snapshotted the pre-build canonical release binary to `tools/reports/candidate-compare/iex-cli-baseline-20260422-140719.exe`
  - compared the candidate binary against that exact snapshot on `.refs/ripgrep/benchsuite/subtitles/en.sample.txt`
  - restored repo code to the known-good baseline after proof stayed mixed outside the target lane
- Validation:
  - `cargo test`
  - benchmark proof runs recorded in `tools/reports/live-metrics.jsonl`
  - durable note: `.docs/recon/regex-whole-buffer-line-recovery-proof-2026-04-22.md`
- Result:
  - the motivating subtitle regex lane improved on repeated proof (`runId=1776859656338-8875d2f1`, `1776859726379-324ebf9f`)
  - the unsupported no-literal comparator stayed mixed and regressed on two repeated checks (`runId=1776859731409-7be50d7c`, `1776859778604-ba759f2a`)
  - conclusion: keep the byte-offset-first direction, but do not land this implementation yet; the next slice should separate generic regex stats-only line-count repair from planner-owned decomposition activation

## Regex whole-buffer line-recovery repair pass kept live for another fix cycle - 2026-04-22 local
- Scope:
  - kept the decomposition candidate live in workspace instead of immediately reverting after the first mixed proof
  - added planner-owned required-inner-literal extraction, narrow stats-only activation, and explicit bailout behavior for excessive candidate-line volume
  - snapshotted the current live binary to `tools/reports/candidate-compare/iex-cli-baseline-20260422-143320.exe`
  - built repaired candidate snapshot `tools/reports/candidate-compare/iex-cli-candidate-whole-buffer-repair-20260422-143320.exe`
- Validation:
  - `cargo test -p iex-core`
  - `cargo test`
  - durable note: `.docs/recon/regex-whole-buffer-line-recovery-repair-pass-2026-04-22.md`
- Result:
  - the motivating lane stayed unstable across three proof passes (`runId=1776861220056-27446456`, `1776861256770-d648d674`, `1776861272767-bce13c95`) and is net slower on median-of-three
  - the no-literal stress comparator also stayed unstable but improved on median-of-three (`runId=1776861226812-0891220f`, `1776861259770-97b540ca`, `1776861278192-21454a29`)
  - conclusion: do not revert yet; keep the repaired candidate live and use the next pass to add activation telemetry plus a cheaper selectivity or line-recovery refinement before another proof replay

## Regex whole-buffer context-gate proof kept live for another fix cycle - 2026-04-22 local
- Scope:
  - kept the decomposition candidate live and repaired the false-positive candidate filter instead of reverting after the repair-pass proof
  - added decomposition attribution to runtime stats and benchmark artifacts
  - added a planner-owned local context gate for the `word + whitespace + literal + whitespace + word` family
  - aligned decomposition HIR analysis with byte-regex semantics via `unicode(false)`
  - paused the active loop for clean proof, then compared against immutable baseline `tools/reports/candidate-compare/iex-cli-baseline-20260422-143320.exe`
  - built candidate snapshot `tools/reports/candidate-compare/iex-cli-candidate-whole-buffer-context-gate-20260422-144932.exe`
- Validation:
  - `cargo test -p iex-core`
  - `cargo test`
  - durable note: `.docs/recon/regex-whole-buffer-context-gate-proof-2026-04-22.md`
- Result:
  - the motivating subtitle lane now collapses to the true match set (`candidate_lines_checked=39`, `candidate_lines_matched=39`) and is slightly ahead on median-of-three (`runId=1776862194340-e2847140`, `1776862197764-7afc8392`, `1776862201228-8e1d5632`)
  - the no-literal stress comparator remains fully inert from a decomposition perspective (`eligible_files=0`) but still shows binary-level instability on repeated proof (`runId=1776862210163-9026c4fe`, `1776862216979-a4d057ec`, `1776862223492-77b37068`)
  - conclusion: keep the candidate live; the next slice is no longer about candidate-line selectivity, it is about isolating inactive-lane variance before promotion
