{{- define "k8sss.wait.until.maxUnavailEp" }}
{{- $args := . -}}
{{- $namespace := index $args 0}}
{{- $name := index $args 1}}
{{- $lte := index $args 2}}
{{- $image := .Values.image }}
- name: wait-for-{{ $name }}
  image: {{ $image }}
  imagePullPolicy: {{ .Values.imagePullPolicy }}
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
{{- $image := .Values.image }}
- name: wait-for-{{ $name }}
  image: {{ $image }}
  imagePullPolicy: {{ .Values.imagePullPolicy }}
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
{{- $image := .Values.image }}
- name: wait-for-{{ $name }}
    image: {{ $image }}
    imagePullPolicy: {{ .Values.imagePullPolicy }}
    args: ["wait", "until", "job", "ready", $name, $namespace]
    resources:
        requests:
        cpu: 100m
        memory: 128M
{{- end -}}
