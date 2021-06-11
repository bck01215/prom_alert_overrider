# Prometheus Overrides Formatter
This repo templates out overrides to keep your rules files clean.

## Use Case
This was created after I was unable to find a good way to use regex as an exception in prometheus rules. I would either be writing a bunch of conditions for exceptions, or using the unless operator over and over again. This is command that still enables regex in unless statements and avoids the manual need to copy and paste unless statements with multiple lines.

This exclusion is done by leveraging the the `unless` operator in PromQL [(docs here)](https://prometheus.io/docs/prometheus/latest/querying/operators/#logical-set-binary-operators).
## Usage

```
❯ prometheus_alert_overrider

positional arguments:
  overrides          Folder containing the overrides
  rules_file         The file to be templated out
```
Follow the `regex_overrides.yml` file to create a template with a list of exceptions.
To import the template into the rules file use 
```
{{ overrides.template_list_name }}
```
the template must be prepended with overrides to be detected.
In the example
```
          unless(
            {{ overrides.windows_disk_usage }}
          )
```
BECOMES
```
          unless(
            {instance="host.domain.com:9100",volume=~"[F|G]:"}
            or {instance="host2.domain.com:9100", volume=~"C:.Volumes.*"}
            or {instance="host3.domain.com:9100",volume="C:"}
          )

```
## Recommendations

- Use CI/CD to generate the recording rules programatically. A Dockerfile is provided if you wish to build your own image, or you can use the one available from Dockerhub at [bkauffman7/prom_alert_overrider](https://hub.docker.com/repository/docker/bkauffman7/prom_alert_overrider/general).
