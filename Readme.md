###Ecommerce orders

# Ecommerce orders

### Steps to run:
    install rust, rustup(setup with nightly), diesel and cargo.
    
### optional:
In this project, Used postgresql with docker. to spin postgressql container provided in project use:

    docker-compose up -d postgres
    
else update .env file with postgres url.
## To build and download dependencies, run:
    cargo check

## create db
    diesel migration run

## Run service:
    cargo run

### Note: For all reuqests use Content-type: application/json

## Check service running status in postman
    http://localhost:8000/
this will return 403 Forbidden error,
use Basic Auth from postman with following credentials,

    username: admin
    password: password
    
### List products:
HTTP METHOD: GET

    http:://locahost:8000/product

use Basic Auth from postman with above credentials,
this will return:

    {
    "products": [
        {
            "created_at": "2020-12-03T17:15:55.246846Z",
            "id": 2,
            "name": "new product again"
        },
        {
            "created_at": "2020-12-02T13:51:46.973957Z",
            "id": 1,
            "name": "New Product1"
        }
    ]
    }


### create new product
HTTP METHOD: POST

    https://localhost: 8000/product
with following request body.
    
    {
        "name": "product 1"
    }
    
### list orders
HTTP METHOD: GET

    https://localhost: 8000/orders

Response:

    {
        "orders": [
            {
                "email": "test@example.com",
                "id": 4,
                "line_items": [
                    {
                        "product": {
                            "created_at": "2020-12-02T13:51:46.973957Z",
                            "id": 1,
                            "name": "New Product1"
                        },
                        "quantity": 1
                    },
                    {
                        "product": {
                            "created_at": "2020-12-03T17:15:55.246846Z",
                            "id": 2,
                            "name": "new product again"
                        },
                        "quantity": 4
                    }
                ]
            }
        ]
    }
    
    
### to create orders
HTTP METHOD: POST

    http://localhost:8000/orders
    
With Request body:
    
    {
        "order": {
            "email": "test@example.com",
            "line_items": [
                {
                    "quantity": 5,
                    "product_id": 1
                },
                {
                    "quantity": 2,
                    "product_id": 1
                }
            ]
        }
    }
   


    
