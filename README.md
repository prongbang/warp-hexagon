# warp-hexagon

## Get customer list

```shell
curl http://localhost:3000/customer
```

Response

```json
{
    "code": 200,
    "message": "success",
    "data": [
        {
            "guid": "4622d0f0-aad4-4c10-a6df-415232141866",
            "first_name": "Leigh",
            "last_name": "Thundercliffe",
            "email": "lthundercliffe0@w3.org",
            "address": "93227 Pankratz Road"
        }
    ]
}
```

## Get customer by guid

```shell
curl http://localhost:3000/customer/4622d0f0-aad4-4c10-a6df-415232141866
```

Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "guid": "4622d0f0-aad4-4c10-a6df-415232141866",
        "first_name": "Leigh",
        "last_name": "Thundercliffe",
        "email": "lthundercliffe0@w3.org",
        "address": "93227 Pankratz Road"
    }
}
```

## Create customer

```shell
curl -X POST http://localhost:3000/customer -H "Content-Type: application/json" -d '{ "guid": "555555555-6666-7777-8888-999999999999", "first_name": "Test", "last_name": "Ter", "email": "test.ter@w3.org", "address": "1234 Bangkok Road" }' 
```

Response

```json
{
  "code": 200,
  "message": "success",
  "data": {
    "guid": "555555555-6666-7777-8888-999999999999",
    "first_name": "Test",
    "last_name": "Ter",
    "email": "test.ter@w3.org",
    "address": "1234 Bangkok Road"
  }
}
```

## Update customer by guid

```shell
curl -X PUT http://localhost:3000/customer/555555555-6666-7777-8888-999999999999 -H "Content-Type: application/json" -d '{ "guid": "555555555-6666-7777-8888-999999999999", "first_name": "Test", "last_name": "Ter Update", "email": "test.ter@w3.org", "address": "1234 Bangkok Road" }' 
```

Response

```json
{
  "code": 200,
  "message": "success",
  "data": {
    "guid": "555555555-6666-7777-8888-999999999999",
    "first_name": "Test",
    "last_name": "Ter Update",
    "email": "test.ter@w3.org",
    "address": "1234 Bangkok Road"
  }
}
```

## Delete customer by guid

```shell
curl -X DELETE http://localhost:3000/customer/555555555-6666-7777-8888-999999999999
```

Response

```json
{
  "code": 200,
  "message": "success",
  "data": null
}
```

## Health check postgres connection

```shell
curl http://localhost:3000/health
```

Response

```json

```