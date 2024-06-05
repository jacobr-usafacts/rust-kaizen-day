# Rust API

### Overview
This app is a result of June 5th, 2024 Kaizen day. It is a simple API, trying to copy platform-services-api

### Running the API
The API currently only runs locally...

You need to have docker and kubernetes (kubectl, kind, and k9s is helpful)

To start the service run `make`. Once the cluster is spun up (check using `kubectl get pods`), run 
`kubectl port-forward service/rust-api-service 8080:80`

You can now access the API locally (`localhost:8080`)

Run `make clean` when done using the API

### Current Routes
GET `localhost:8080/rust-api/chart-data`
- Returns a sample ChartData object of struct ChartData (see below)

POST `localhost:8080/rust-api/chart-data`
- Needs a ChartData object (json) in body
- Returns a status `ok` if the supplied ChartData object is of correct form

```
struct ChartData {
  name: String
  chart_type: String
  chart_data: Array[i32]
}
```


## Future Implementations
- Flesh out chart-data routes more, add functionality similar to platform-services-api
- Allow usage of API without having to port-forward (maybe NodePort or minikube)?