# Introduction

> **This documentation is primarily to help guide the user experience for Gubernator.** None of
> this stuff actually works right now, but the hope is that it will soon!

## Installing the Gubernator CLI

```
curl https://install.gubernator.org | sh
```

## Connecting to a cluster

### Creating a new local cluster

```
gubernator cluster init
```

**Local clusters are good to have for personal development environments.** Generally, however,
production-ready clusters should be created on cloud platforms like Amazon Web Services, Google
Cloud, and Microsoft Azure.

### Creating a new cluster in the cloud

```
gubernator cluster init --provider="cloud.google.com"
```

### Onboarding to an existing cluster

```
gubernator cluster connect --to="example.gubernator.org"
```

If you just want to connect to an existing cluster, use `gubernator connect` instead.

You will most likely have to go through additional steps to authenticate with the cluster. Once
that has been done, however, your Gubernator config file will be updated.
