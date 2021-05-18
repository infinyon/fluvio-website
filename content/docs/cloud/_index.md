---
title: Fluvio Cloud Release Notes
menu: Release Notes
toc: true
weight: 200
---

This page contains the latest release notes for updates to [Fluvio Cloud].

[Fluvio Cloud]: /signup

## May 6, 2021

###### Updates:

- Upgrade to [Fluvio 0.8.2](https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md#platform-version-082---2020-05-06)

## April 7, 2021

###### Updates:

- Upgrade to [Fluvio 0.7.3](https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md#platform-version-073---2020-04-02)


## March 25, 2021

###### Updates:

- Upgrade to [Fluvio 0.7.2](https://github.com/infinyon/fluvio/blob/master/CHANGELOG.md#platform-version-072---2020-03-23)
- Improved recovery from infrastructure changes and failures
- Bug fixes

## February 26, 2021

###### Updates:

- New styling for signup page
- Internal improvements that speed up cluster provisioning
- Increased cluster stability
- Network improvements to prevent dropped connections

## January 19, 2021

###### Updates:

- Updated Cloud documentation with the latest instructions
- Added a loading spinner to the website UI for cluster installation
- Added `fluvio cloud login` command for downloading Cloud profile

```
$ fluvio cloud login --email="batman@justiceleague.com"
Password:
```

###### Known issues:

- When consuming from a topic (e.g. via `fluvio consume`), after about 5
  minutes of no activity it may silently disconnect. In this case the
  consumer needs to be restarted.
  
###### Other notes:

- Please know that after account creation, provisioning your Fluvio account
  may take 5 to 7 minutes before you can run `fluvio cloud login`.
