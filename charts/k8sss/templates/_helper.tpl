{{- define "k8sss.wait.until.maxUnavailEp" }}
{{- $args := . -}}
{{- $namespace := index $args 0}}
{{- $name := index $args 1}}
{{- $lte := index $args 2}}
{{- $image := $.Values.global.k8sss.image | default "ghcr.io/andlaz/k8sss:latest" }}
- name: wait-for-{{ $name }}
  image: {{ $image }}
  imagePullPolicy: {{ $.Values.global.k8sss.imagePullPolicy | default "Always" }}
  args: ["wait", "until", "service", "unavailable-endpoints", "--lte", $lte, $name, $namespace]
  resources:
    requests:
      cpu: 100m
      memory: 128M
{{- end -}}

{{- define "k8sss.wait.until.minEp" }}
{{- $args := . -}}
{{- $namespace := index $args 0}}
{{- $name := index $args 1}}
{{- $gte := index $args 2}}
{{- $image := $.Values.global.k8sss.image | default "ghcr.io/andlaz/k8sss:latest" }}
- name: wait-for-{{ $name }}
  image: {{ $image }}
  imagePullPolicy: {{ $.Values.global.k8sss.imagePullPolicy | default "Always" }}
  args: ["wait", "until", "service", "available-endpoints", "--gte", $gte, $name, $namespace]
  resources:
    requests:
      cpu: 100m
      memory: 128M
{{- end -}}

{{- define "k8sss.wait.until.job"}}
{{- $args := . -}}
{{- $namespace := index $args 0}}
{{- $name := index $args 1}}
{{- $image := $.Values.global.k8sss.image | default "ghcr.io/andlaz/k8sss:latest" }}
- name: wait-for-{{ $name }}
    image: {{ $image }}
  imagePullPolicy: {{ $.Values.global.k8sss.imagePullPolicy | default "Always" }}
    args: ["wait", "until", "job", "ready", $name, $namespace]
    resources:
        requests:
        cpu: 100m
        memory: 128M
{{- end -}}
