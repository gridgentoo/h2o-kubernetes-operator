# H2O Kubernetes Operator

![Rust](https://github.com/h2oai/h2o-kubernetes/workflows/Rust/badge.svg)

A command line tool to ease the deployment (and undeployment) of H2O open-source machine learning platform [H2O-3](https://github.com/h2oai/h2o-3) to Kubernetes. Currently in beta, with stable basic deployment/undeployment features. Official [H2O Kubernetes Docker images](https://hub.docker.com/r/h2oai/h2o-open-source-k8s) are used.

[Download for Mac/Linux/Windows](https://github.com/h2oai/h2o-kubernetes/releases).

## Usage

![H2O Usage in console](h2ok.gif)

Type `h2ok --help` for an overview of available subcommands. Use the `--help` or `-h` flag in combination with any of the subcommands to receive help for those subcommands, for example `h2ok deploy -h`.

There are three basic commands:
1. `h2ok deploy` - deploys H2O cluster into a Kubernetes cluster,
1. `h2ok undeploy`- removes existing H2O deployment from a Kubernetes cluster,
1. `h2ok ingress` - creates an ingress for existing H2O Kubernetes deployment.

### Deploy
Deploys an H2O cluster into Kubernetes by creating all the necessary components. Once successfully deployed a deployment descriptor file with cluster name is saved. Such a file can be used to undeploy the H2O cluster or built on top of by adding additional services.
If deployment of any of the component fails a rollback of existing components is attempted automatically. If a cluster name is not provided, one is generated automatically.
 
**Mininal example**: `h2ok deploy --cluster_size 3 --version latest`. Kubeconfig environment variable presence is expected.

**Custom kubeconfig and namespace**: `h2ok deploy --cluster-size 3 --version latest --kubeconfig /etc/rancher/k3s/k3s.yaml --namespace default`

The `namespace` option defaults to `default`. If `kubeconfig` is not defined, well-known locations and environment variables are searched.

After each deployment is done, a file with cluster's name and an `.h2ok` suffix is saved to the working directory. Such a file serves as a descriptor of the deployment done and may later be used by `h2ok undeploy -f h2o-deployment-name.h2ok` to automatically undeploy the whole H2O cluster from Kubernetes.

### Undeploy
Undeploys existing deployment from a Kubernetes cluster using deployment descriptor generated during deployment operations. Requires a deployment descriptor file with `.h2ok` suffix.

**Minimal example**: `h2ok undeploy h2o-deployment-name`

### Ingress
Adds an ingress for an existing deployment. Requires a deployment descriptor file with `.h2ok` suffix as an argument. The ingress is set to port 80 and targets the service
associated with the given H2O cluster inside the H2O deployment descriptor automatically. Name of the ingress follows the `<h2o-deployment-name>-ingress` convention.

**Minimal example**: `h2ok ingress h2o-deployment-name`

## Building, testing and running

H2O Kubernetes CLI (`h2ok`) is written in [Rust](https://www.rust-lang.org/), using its standard built-in tools. The build and dependency management tool is therefore [Cargo](https://crates.io/).

- Development build : `cargo build -p h2ok`
- Release build: `cargo build -p h2ok --release`
- Development run: `cargo run -p h2ok -- deploy --namespace default --kubeconfig /etc/rancher/k3s/k3s.yaml --cluster_size 3 -- version latest`
- Test: `cargo test -p h2ok` - please note many tests have prerequisites - running Kubernetes cluster and the `KUBECONFIG` variable set.

## Automated tests
Automated tests are run via GitHub actions - the test environemnt provides the `KUBECONFIG` environment variable with path to a [K3S](https://k3s.io/) Kubernetes instance.
