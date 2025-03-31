#!/bin/sh

if ! helm version >/dev/null 2>&1; then
  echo "Helm is not installed. Please, install Helm at https://helm.sh before proceeding."
  exit 1
fi;


nginx_status=$(kubectl get po -n ingress-nginx | grep 'Running\|Completed')
if [ "$nginx_status" == "No resources found in messaging namespace." ]; then
  helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
  helm repo update
  helm install ingress-nginx ingress-nginx/ingress-nginx \
    --namespace ingress-nginx \
    --create-namespace
else
  echo "Skipping Nginx ingress installation."
fi;

echo ""

kubectl get namespace | grep -q "^messaging" || kubectl create namespace messaging
kubectl apply -f messaging/scylladb.yaml
kubectl apply -f messaging/configuration.yaml
kubectl apply -f messaging/deployment.yaml
kubectl apply -f messaging/ingress.yaml

echo ""

kubectl get namespace | grep -q "^web" || kubectl create namespace web
kubectl apply -f web/configuration.yaml
kubectl apply -f web/deployment.yaml
kubectl apply -f web/ingress.yaml

echo "\nDone.";