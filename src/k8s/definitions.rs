use k8s_openapi::api::apps::v1::StatefulSet;
use k8s_openapi::api::core::v1::Service;
use k8s_openapi::api::extensions::v1beta1::Ingress;
use k8s_openapi::serde_json;

const STATEFUL_SET_TEMPLATE: &str = r#"
{
   "apiVersion": "apps/v1",
   "kind": "StatefulSet",
   "metadata": {
      "name": "<name>-stateful-set"
   },
   "spec": {
      "serviceName": "h2o-service",
      "replicas": 3,
      "selector": {
         "matchLabels": {
            "app": "<name>"
         }
      },
      "template": {
         "metadata": {
            "labels": {
               "app": "<name>"
            }
         },
         "spec": {
            "containers": [
               {
                  "name": "<name>",
                  "image": "<docker-img-name>:<docker-img-tag>",
                  "ports": [
                     {
                        "containerPort": 54321,
                        "protocol": "TCP"
                     }
                  ],
                  "readinessProbe": {
                     "httpGet": {
                        "path": "/kubernetes/isLeaderNode",
                        "port": 8081
                     },
                     "initialDelaySeconds": 5,
                     "periodSeconds": 5,
                     "failureThreshold": 1
                  },
                  "env": [
                     {
                        "name": "H2O_KUBERNETES_SERVICE_DNS",
                        "value": "<name>-service.<namespace>.svc.cluster.local"
                     },
                     {
                        "name": "H2O_NODE_LOOKUP_TIMEOUT",
                        "value": "180"
                     },
                     {
                        "name": "H2O_NODE_EXPECTED_COUNT",
                        "value": "3"
                     },
                     {
                        "name": "H2O_KUBERNETES_API_PORT",
                        "value": "8081"
                     }
                  ]
               }
            ]
         }
      }
   }
}
"#;

pub fn h2o_stateful_set(name: &str, namespace: &str, docker_img_name: &str, docker_img_tag: &str) -> StatefulSet {
    let stateful_set_definition = STATEFUL_SET_TEMPLATE.replace("<name>", name)
        .replace("<namespace>", namespace)
        .replace("<docker-img-name>", docker_img_name)
        .replace("<docker-img-tag>", docker_img_tag);

    let stateful_set: StatefulSet = serde_json::from_str(&stateful_set_definition).unwrap();
    return stateful_set;
}

const SERVICE_TEMPLATE: &str = r#"
{
   "apiVersion": "v1",
   "kind": "Service",
   "metadata": {
      "name": "<name>-service",
      "namespace": "<namespace>"
   },
   "spec": {
      "type": "ClusterIP",
      "clusterIP": "None",
      "selector": {
         "app": "h2o-k8s"
      },
      "ports": [
         {
            "protocol": "TCP",
            "port": 80,
            "targetPort": 54321
         }
      ]
   }
}
"#;

pub fn h2o_service(name: &str, namespace: &str) -> Service {
    let service_definition = SERVICE_TEMPLATE.replace("<name>", name)
        .replace("<namespace>", namespace);

    let service: Service = serde_json::from_str(&service_definition).unwrap();
    return service;
}

const INGRESS_TEMPLATE: &str = r#"
{
   "apiVersion": "extensions/v1beta1",
   "kind": "Ingress",
   "metadata": {
      "name": "<name>-ingress",
      "namespace": "<namespace>"
   },
   "spec": {
      "backend": {
         "serviceName": "<name>-service",
         "servicePort": 80
      }
   }
}
"#;

pub fn h2o_ingress(name: &str, namespace: &str) -> Ingress {
    let ingress_definition = INGRESS_TEMPLATE.replace("<name>", name)
        .replace("<namespace>", namespace);

    let ingress: Ingress = serde_json::from_str(&ingress_definition).unwrap();
    return ingress;
}