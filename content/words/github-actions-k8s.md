---
title: Github Actions and Kubernetes
description: using Github Actions to deploy to Kubernetes
date: 2024-01-16
tags:
    - kubernetes
    - github
    - github-actions
    - docker
    - ci
    - cd
---

Recently, I migrated my site to being built and deployed using GitHub Actions, this approach is useful if you're experiencing slow docker cross-compilation speeds, like those on M1 Macs. Typically my local flow for building new images involves a Makefile that builds the Docker image, pushes it to a registry, and then issues the `kubectl rollout restart deployment DEPLOYMENT_NAME` command. There are already ample resources on how to build and push a Docker image using Github Actions, so I won't cover that here, but I will cover how to restart a deployment or trigger a kubernetes command using Github Actions.

To replicate this flow with GitHub Actions, the first step is understanding Kubernetes Service Accounts. Here's an example of a service account setup that allows GitHub Actions to restart a deployment:

```
apiVersion: v1
kind: ServiceAccount
metadata:
  name: github-actions-deployer
  namespace: default
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
  name: deployment-restart-role
rules:
- apiGroups: ["apps"]
  resources: ["deployments"]
  verbs: ["get", "patch"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: deployment-restart-binding
subjects:
- kind: ServiceAccount
  name: github-actions-deployer
  namespace: default
roleRef:
  kind: ClusterRole
  name: deployment-restart-role
  apiGroup: rbac.authorization.k8s.io
```

These configurations comprise three parts:

1. The service account itself, identified by a name and a namespace.
2. A ClusterRole, allowing the service account to get and patch deployments.
3. A ClusterRoleBinding, binding the service account to the ClusterRole.
Next, generate a token for the service account using:

```
kubectl create token github-actions-deployer --duration=262800h
```

With the token, create a kubeconfig file. A typical configuration looks like this:
```
apiVersion: v1
clusters:
- cluster:
    certificate-authority-data: CA_DATA
    server: https://K8SENDPOINT:6443
  name: default
contexts:
- context:
    cluster: default
    user: default
  name: default
current-context: default
kind: Config
preferences: {}
users:
- name: default
  user:
    token: TOKEN
```
Test the configuration locally with:
```
kubectl --kubeconfig=kubeconfig.yaml get deployment/DEPLOYMENT_NAME
```

Once verified, set up the Github Action by adding the following to your workflow after the image build step:

```
    - uses: actions-hub/kubectl@master
    env:
        KUBE_CONFIG: ${{ secrets.KUBE_CONFIG }}
    with:
        args: rollout restart deployment DEPLOYMENT_NAME
```

Finally, base64 encode the kubeconfig and store it in the Github Actions secrets as`KUBE_CONFIG` in the repository settings. This will allow the Github Action to authenticate with the Kubernetes cluster and restart the deployment.

Looking at the full workflow file, it should look something like this:

```
name: Docker Build and Push
on:
  push:
    branches:
      - main
    tags:
      - 'v*'
env:
  REGISTRY: REGISTRY_URL
  IMAGE_NAME: IMAGE_NAME

jobs:
  build-and-push:
    runs-on: ubuntu-latest

    steps:
      - name: Check out the repo
        uses: actions/checkout@v3

      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v2

      - name: Login to the Docker registry
        uses: docker/login-action@v2
        with:
          registry: ${{ env.REGISTRY }}
          username: ${{ secrets.REGISTRY_USERNAME }}
          password: ${{ secrets.REGISTRY_PASSWORD }}
      - name: Build and push
        uses: docker/build-push-action@v5
        with:
          context: .
          push: true
          tags: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:latest
          cache-from: type=gha
          cache-to: type=gha,mode=max
      - uses: actions-hub/kubectl@master
        env:
          KUBE_CONFIG: ${{ secrets.KUBE_CONFIG }}
        with:
          args: rollout restart deployment DEPLOYMENT_NAME
```

This workflow will build and push the Docker image, then restart the deployment. This is a simple example, but it can be expanded to do more complex tasks, such as deploying to multiple clusters or running tests on the deployment.

## Hitless Deployments

One of the most useful features of Kubernetes is the ability to do hitless deployments. This means that when a new version of a deployment is deployed, the old version is not taken down until the new version is ready. This is done by having multiple replicas of the deployment running at once. This allows for a new version to be deployed, and then the old version to be taken down once the new version is ready.

To do this, a deployment configuation similar to the following can be used:
```
apiVersion: apps/v1
kind: Deployment
metadata:
  name: APP_NAME
spec:
  selector:
    matchLabels:
      app: APP_NAME
  replicas: 2
  minReadySeconds: 1
  template:
    metadata:
      labels:
        app: APP_NAME
    spec:
      containers:
      - name: hamptonmoore
        image: IMAGE_URL
        imagePullPolicy: Always
        ports:
        - containerPort: 3000
        livenessProbe:
          httpGet:
            path: /health
            port: 3000

```

This will create a deployment with two replicas. This means that when a new version is deployed, the old version will not be taken down until the new version is ready. Using the livenessProbe, Kubernetes will know when the new version is ready, and then take down the old version.