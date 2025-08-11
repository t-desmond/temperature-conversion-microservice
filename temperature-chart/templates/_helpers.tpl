{{/*
Common labels
*/}}
{{- define "temperature.labels" -}}
helm.release: {{ .Release.Name }}
helm.chart: {{ include "temperature-chart.chart" . }}
{{ include "temperature.selectorLabels" . }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end -}}

{{/*
Selector labels
*/}}
{{- define "temperature.selectorLabels" -}}
app.kubernetes.io/name: {{ .Chart.Name }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end -}}

{{/*
Chart name and version as used by the chart label.
*/}}
{{- define "temperature-chart.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end -}}
