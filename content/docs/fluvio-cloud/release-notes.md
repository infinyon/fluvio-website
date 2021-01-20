---
title: Fluvio Cloud Release Notes
menu: Release Notes
toc: true
weight: 200
---

This page contains the latest release notes for updates to [Fluvio Cloud].

[Fluvio Cloud]: /signup

## January 19, 2021

Updates:

- Updated Cloud documentation with the latest instructions
- Added a loading spinner to the website UI for cluster installation
- Added `fluvio cloud login` command for downloading Cloud profile

```
$ fluvio cloud login --email="batman@justiceleague.com"
Password:
```

Known issues:

- When consuming from a topic (e.g. via `fluvio consume`), after about 5
  minutes of no activity it may silently disconnect. In this case the
  consumer needs to be restarted.
  
Other notes:

- Please know that after account creation, provisioning your Fluvio account
  may take 5 to 7 minutes before you can run `fluvio cloud login`.
