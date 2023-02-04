{{- define "maxUnavailEp" }}
{{- $args := . -}}
{{- $namespace := index $args 1}}
{{- $name := index $args 2}}
{{- $lte := index $args 3}}
{{- $image := .Values.image }}
- name: wait-for-{{ $name }}
  image: {{ $image }}
  args: ["wait", "until", "service", "unavailable-endpoints", "--lte", $lte, $name, $namespace]
  resources:
    requests:
      cpu: 100m
      memory: 128M
{{- end -}}

{{- define "minEp" }}
{{- $args := . -}}
{{- $namespace := index $args 1}}
{{- $name := index $args 2}}
{{- $gte := index $args 3}}
{{- $image := .Values.image }}
- name: wait-for-{{ $name }}
  image: {{ $image }}
  args: ["wait", "until", "service", "available-endpoints", "--gte", $gte, $name, $namespace]
  resources:
    requests:
      cpu: 100m
      memory: 128M
{{- end -}}

{{- define "job"}}
{{- $args := . -}}
{{- $namespace := index $args 1}}
{{- $name := index $args 2}}
{{- $image := .Values.image }}
- name: wait-for-{{ $name }}
     image: {{ $image }}
    args: ["wait", "until", "job", "ready", $name, $namespace]
    resources:
        requests:
        cpu: 100m
        memory: 128M
{{- end -}}
