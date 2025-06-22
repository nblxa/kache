{{- define "kache.name" }}
{{- if .Values.nameOverride }}
{{- .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- else if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- default .Chart.Name .Values.name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}

{{- define "kache.chart" }}
{{- .Chart.Name }}-{{ .Chart.Version | replace "+" "_" }}
{{- end }}

{{- define "kache.selectorLabels" -}}
app.kubernetes.io/name: {{ include "kache.name" . }}
helm.sh/chart: {{ include "kache.chart" . }}
{{- end }}

{{- define "kache.labels" -}}
{{- include "kache.selectorLabels" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{- define "kache.imageTag" }}
{{- if .Values.image.tag }}
{{- .Values.image.tag }}
{{- else }}
{{- .Chart.AppVersion }}
{{- end }}
{{- end }}

{{- define "kache.serviceAccount" }}
{{- if .Values.serviceAccount.name }}
{{- .Values.serviceAccount.name }}
{{- else }}
{{- include "kache.name" . }}
{{- end }}
{{- end }}
