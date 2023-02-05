Kubernetes service status ( `k8sss` /:keɪs/ ) is a small command-line application ( a Kubernetes watcher ) with the purpose of either reflecting on the endpoint count of Kubernetes Services or the outcome of Jobs

It is meant to be run in an init container and will wait until:

- in case of waiting on services, the endpoint count equals or is greater, or the unavailable endpoint count equals or is lower, than the passed-in value, or
- in case of waiting on jobs, the job is completed successfully, or
- it is interrupted ( killed )

It communicates only by exiting with code 0 or continuing running.

## Usage

You need to add an entry to your init containers per external service/job dependency. This could, for example, look like:

```bash
# initContainers:
- name: wait-for-grub
  image: ghcr.io/andlaz/k8sss:latest
  args:
  - wait
  - until
  - service
  - unavailable-endpoints
  - --lte=0
  - grub
  - default
```

or

```bash
# initContainers:
- name: wait-for-ale
  image: ghcr.io/andlaz/k8sss:latest
  args:
  - wait
  - until
  - service
  - available-endpoints
  - --gte=1
  - ale
  - default
```

perhaps

```bash
# initContainers:
- name: wait-for-cleanup-job
  image: ghcr.io/andlaz/k8sss:latest
  args:
  - wait
  - until
  - job
  - ready
  - cleanup-job
  - default
```

For a complete list of commands and arguments, run `k8sss wait until service --help` and `k8sss wait until job --help`

### Helm chart

If you are managing your resources in a helm chart, you can simply depend on the `k8sss` library chart and use the `minEp`, `maxUnavailEp` and `job` helpers to produce the same output as above:

```bash
#     initContainers:
      {{- include "k8sss.wait.until.maxUnavailEp" (list .Release.Namespace "grub" 0) | indent 6 }} # all endpoints must be available
      {{- include "k8sss.wait.until.minEp" (list .Release.Namespace "ale" 1) | indent 6 }} # at least one endpoint must be available
      {{- include "k8sss.wait.until.job" (list .Release.Namespace "cleanup-job") | indent 6 }} # wait for job to complete
```

#### Changing the image in the init container

You can update the k8sss image name and image pull policy by setting the `global.k8sss.image` and `global.k8sss.imagePullPolicy` values on the library chart dependent, e.g. in your Chart.

```bash
dependencies:
- name: k8sss
  version: "~0.2"
  repository: https://oss.andlaz.io/k8sss
```

Then, at helm install time ( or in values.yaml ):
```
helm upgrade --install \
    my-release \
    my-chart \
    --set global.k8sss.image=some/other:image \
    --set global.k8sss.imagePullPolicy=Never # preloaded
```

## Alternatives

- Bash (or other scripting language) polling kubectl in your init containers
- Implement tolerance of external dependencies not being available in the environment at start-up time in your applications, without exiting
- Accept start-up delay due to exponential backoff of leaf node pods in your runtime/service dependency graph
- A service mesh
