{{- define "k8sss.wait.until.maxUnavailEp" }}
{{- $args := . -}}
{{- $namespace := index $args 0}}
{{- $name := index $args 1}}
{{- $lte := index $args 2}}
- name: wait-for-{{ $name }}
  image: {{ include "k8sss.image" $ }}
  imagePullPolicy: {{ include "k8sss.imagePullPolicy" $ }}
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
- name: wait-for-{{ $name }}
  image: {{ include "k8sss.image" $ }}
  imagePullPolicy: {{ include "k8sss.imagePullPolicy" $ }}
  args: ["wait", "until", "service", "available-endpoints", "--gte", $gte, $name, $namespace]
  resources:
    requests:
      cpu: 100m
      memory: 128M
{{- end -}}

{{- define "k8sss.wait.until.job" }}
{{- $args := . -}}
{{- $namespace := index $args 0}}
{{- $name := index $args 1}}
- name: wait-for-{{ $name }}
  image: {{ include "k8sss.image" $ }}
  imagePullPolicy: {{ include "k8sss.imagePullPolicy" $ }}
    args: ["wait", "until", "job", "ready", $name, $namespace]
    resources:
        requests:
        cpu: 100m
        memory: 128M
{{- end -}}

{{- define "k8sss.image" }}
    {{- if .Values.global -}}
        {{- if $.Values.global.k8sss -}}
            {{- if $.Values.global.k8sss.image -}}
                {{- $.Values.global.k8sss.image -}}
            {{- else -}}
                {{- "ghcr.io/andlaz/k8sss:latest" -}}
            {{- end -}}
        {{- else -}}
            {{- "ghcr.io/andlaz/k8sss:latest" -}}
        {{- end -}}
    {{- else -}}
        {{- "ghcr.io/andlaz/k8sss:latest" -}}
    {{- end -}}
{{- end -}}

{{- define "k8sss.imagePullPolicy" }}
    {{- if .Values.global }}
        {{- if $.Values.global.k8sss }}
            {{- if $.Values.global.k8sss.imagePullPolicy }}
                {{- $.Values.global.k8sss.imagePullPolicy }}
            {{- else }}
                {{- "Always" -}}
            {{- end }}
        {{- else }}
            {{- "Always" -}}
        {{- end }}
    {{- else }}
        {{- "Always" -}}
    {{- end }}
{{- end -}}
